// # Developer notes
//
// * Do not expose any FFI items through this crate.
// * Use `duk_require_stack` directly before any `ffi::duk_*` calls that increase the stack size.
// * Do not instantiate a `duk_context` (via `duk_create_heap` or `duk_create_heap_default`) outside
//   of `Ducc`. Because of some regrettable global state requirements imposed by Duktape, a
//   `duk_context` must be specially configured to avoid undefined behavior. For more information,
//   see `ensure_exec_timeout_check_exists`.

use array::Array;
use bytes::Bytes;
use error::{Error, Result};
use ffi;
use function::{create_callback, Function, Invocation};
use object::Object;
use std::any::Any;
use std::cell::RefCell;
use string::String;
use types::Ref;
use util::{
    create_heap,
    get_any_map,
    get_udata,
    pop_error,
    protect_duktape_closure,
    push_bytes,
    push_str,
    StackGuard,
};
use value::{FromValue, ToValue, Value};

/// The entry point into the JavaScript execution environment.
///
/// The `Duktape` global variable is not available to scripts because it is possible to use its
/// methods to violate security and safety guarantees made by this library.
pub struct Ducc {
    pub(crate) ctx: *mut ffi::duk_context,
    // Internally, a `ctx` can live in multiple `Ducc` instances (see `function::create_callback`),
    // so we need to make sure we only drop the Duktape heap in the top-level "grandparent" `Ducc`.
    pub(crate) is_top: bool,
}

impl Ducc {
    /// Creates a new JavaScript execution environment.
    pub fn new() -> Ducc {
        Ducc { ctx: unsafe { create_heap() }, is_top: true }
    }

    /// Returns the global object.
    pub fn globals(&self) -> Object {
        unsafe {
            assert_stack!(self.ctx, 0, {
                ffi::duk_require_stack(self.ctx, 1);
                ffi::duk_push_global_object(self.ctx);
                Object(self.pop_ref())
            })
        }
    }

    /// Compiles a chunk of JavaScript code and returns it as a function with no arguments.
    ///
    /// The source can be named by setting the `name` parameter. This is generally recommended as it
    /// results in better errors.
    ///
    /// Equivalent to Duktape's `duk_compile` using `DUK_COMPILE_EVAL`.
    pub fn compile(&self, source: &str, name: Option<&str>) -> Result<Function> {
        unsafe {
            assert_stack!(self.ctx, 0, {
                push_str(self.ctx, source)?;
                push_str(self.ctx, name.unwrap_or("input"))?;
                if ffi::duk_pcompile(self.ctx, ffi::DUK_COMPILE_EVAL) == 0 {
                    Ok(Function(self.pop_ref()))
                } else {
                    Err(pop_error(self.ctx))
                }
            })
        }
    }

    /// Executes a chunk of JavaScript code and returns its result.
    ///
    /// This is equivalent to calling `Ducc::compile` and `Function::call` immediately after. The
    /// only difference is that this function supports a `settings` parameter, can be used to
    /// specify one-time execution settings.
    pub fn exec<'ducc, R: FromValue<'ducc>>(
        &'ducc self,
        source: &str,
        name: Option<&str>,
        settings: ExecSettings,
    ) -> Result<R> {
        let func = self.compile(source, name)?;

        let udata = unsafe { get_udata(self.ctx) };
        unsafe { (*udata).set_exec_settings(settings); }

        let result = func.call(());

        unsafe { (*udata).clear_exec_settings(); }

        result.into()
    }

    /// Inserts any sort of keyed value of type `T` into the `Ducc`, typically for later retrieval
    /// from within Rust functions called from within JavaScript. If a value already exists with the
    /// key, it is returned.
    pub fn set_user_data<K, T>(&mut self, key: K, data: T) -> Option<Box<Any + 'static>>
    where
        K: ToString,
        T: Any + 'static,
    {
        unsafe {
            let any_map = get_any_map(self.ctx);
            (*any_map).insert(key.to_string(), Box::new(data))
        }
    }

    /// Returns a user data value by its key, or `None` if no value exists with the key. If a value
    /// exists but it is not of the type `T`, `None` is returned. This is typically used by a Rust
    /// function called from within JavaScript.
    pub fn get_user_data<'ducc, T: Any + 'static>(&'ducc self, key: &str) -> Option<&'ducc T> {
        unsafe {
            let any_map = get_any_map(self.ctx);
            match (*any_map).get(key) {
                Some(data) => data.downcast_ref::<T>(),
                None => None,
            }
        }
    }

    /// Removes and returns a user data value by its key. Returns `None` if no value exists with the
    /// key.
    pub fn remove_user_data(&mut self, key: &str) -> Option<Box<Any + 'static>> {
        unsafe {
            let any_map = get_any_map(self.ctx);
            (*any_map).remove(key)
        }
    }

    /// Wraps a Rust function or closure, creating a callable JavaScript function handle to it.
    ///
    /// The function's return value is always a `Result`: If the function returns `Err`, the error
    /// is raised as a JavaScript error, which can be caught within JavaScript or bubbled up back
    /// into Rust. This allows using the `?` operator to propagate errors through intermediate
    /// JavaScript code.
    ///
    /// If the function returns `Ok`, the contained value will be converted to one or more
    /// JavaScript values. For details on Rust-to-JavaScript conversions, refer to the `ToValue` and
    /// `ToValues` traits.
    pub fn create_function<'ducc, 'callback, R, F>(&'ducc self, func: F) -> Function<'ducc>
    where
        R: ToValue<'callback>,
        F: 'static + Send + Fn(Invocation<'callback>) -> Result<R>,
    {
        create_callback(self, Box::new(move |ducc, this, args| {
            func(Invocation { ducc, this, args })?.to_value(ducc)
        }))
    }

    /// Wraps a mutable Rust closure, creating a callable JavaScript function handle to it.
    ///
    /// This is a version of `create_function` that accepts a FnMut argument. Refer to
    /// `create_function` for more information about the implementation.
    pub fn create_function_mut<'ducc, 'callback, R, F>(&'ducc self, func: F) -> Function<'ducc>
    where
        R: ToValue<'callback>,
        F: 'static + Send + FnMut(Invocation<'callback>) -> Result<R>,
    {
        let func = RefCell::new(func);
        self.create_function(move |invocation| {
            (&mut *func.try_borrow_mut().map_err(|_| Error::recursive_mut_callback())?)(invocation)
        })
    }

    /// Pass a `&str` to Duktape, creating and returning an interned string.
    pub fn create_string(&self, value: &str) -> Result<String> {
        unsafe {
            assert_stack!(self.ctx, 0, {
                push_str(self.ctx, value)?;
                Ok(String(self.pop_ref()))
            })
        }
    }

    /// Pass a `&[u8]` to Duktape, creating and returning an interned `Bytes`.
    pub fn create_bytes(&self, value: &[u8]) -> Result<Bytes> {
        unsafe {
            assert_stack!(self.ctx, 0, {
                push_bytes(self.ctx, value)?;
                Ok(Bytes(self.pop_ref()))
            })
        }
    }

    /// Creates and returns an empty `Object` managed by Duktape.
    pub fn create_object(&self) -> Object {
        unsafe {
            assert_stack!(self.ctx, 0, {
                ffi::duk_require_stack(self.ctx, 1);
                ffi::duk_push_object(self.ctx);
                Object(self.pop_ref())
            })
        }
    }

    /// Creates and returns an empty `Array` managed by Duktape.
    pub fn create_array(&self) -> Array {
        unsafe {
            assert_stack!(self.ctx, 0, {
                ffi::duk_require_stack(self.ctx, 1);
                ffi::duk_push_array(self.ctx);
                Array(self.pop_ref())
            })
        }
    }

    /// Creates and returns an `Object` managed by Duktape filled with the keys and values from an
    /// iterator. Keys are coerced to object properties.
    ///
    /// This is a thin wrapper around `Ducc::create_object` and `Object::set`. See `Object::set` for
    /// how this method might return an error.
    pub fn create_object_from<'ducc, K, V, I>(&'ducc self, iter: I) -> Result<Object<'ducc>>
    where
        K: ToValue<'ducc>,
        V: ToValue<'ducc>,
        I: IntoIterator<Item = (K, V)>,
    {
        let object = self.create_object();
        for (k, v) in iter {
            object.set(k, v)?;
        }
        Ok(object)
    }

    /// Coerces a Duktape value to a string. Nearly all JavaScript values are coercible to strings,
    /// but this may fail with a runtime error under extraordinary circumstances (e.g. if the
    /// Ecmascript `ToString` implementation throws an error).
    pub fn coerce_string<'ducc>(&'ducc self, value: Value<'ducc>) -> Result<String<'ducc>> {
        match value {
            Value::String(s) => Ok(s),
            value => unsafe {
                let _sg = StackGuard::new(self.ctx);
                self.push_value(value);
                protect_duktape_closure(self.ctx, 1, 1, |ctx| ffi::duk_to_string(ctx, -1))?;
                Ok(String(self.pop_ref()))
            },
        }
    }

    /// Coerces a Duktape value to a number. Nearly all JavaScript values are coercible to numbers,
    /// but this may fail with a runtime error under extraordinary circumstances (e.g. if the
    /// Ecmascript `ToNumber` implementation throws an error).
    ///
    /// This will return `std::f64::NAN` if the value has no numerical equivalent.
    pub fn coerce_number<'ducc>(&'ducc self, value: Value<'ducc>) -> Result<f64> {
        match value {
            Value::Number(n) => Ok(n),
            value => unsafe {
                let _sg = StackGuard::new(self.ctx);
                self.push_value(value);
                protect_duktape_closure(self.ctx, 1, 0, |ctx| {
                    ffi::duk_to_number(ctx, -1)
                })
            },
        }
    }

    /// Coerces a Duktape value to a boolean (returns `true` if the value is "truthy", `false`
    /// otherwise).
    pub fn coerce_boolean(&self, value: Value) -> bool {
        unsafe {
            let _sg = StackGuard::new(self.ctx);
            self.push_value(value);
            ffi::duk_to_boolean(self.ctx, -1) != 0
        }
    }

    pub(crate) unsafe fn push_value(&self, value: Value) {
        assert_stack!(self.ctx, 1, {
            match value {
                Value::Undefined => {
                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_push_undefined(self.ctx);
                },
                Value::Null => {
                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_push_null(self.ctx);
                },
                Value::Boolean(b) => {
                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_push_boolean(self.ctx, if b { 1 } else { 0 });
                },
                Value::Number(n) => {
                    ffi::duk_require_stack(self.ctx, 1);
                    ffi::duk_push_number(self.ctx, n);
                },
                Value::String(s) => self.push_ref(&s.0),
                Value::Function(f) => self.push_ref(&f.0),
                Value::Array(a) => self.push_ref(&a.0),
                Value::Object(o) => self.push_ref(&o.0),
                Value::Bytes(b) => self.push_ref(&b.0),
            }
        })
    }

    // Pops the value at the top of the stack and converts it to a `Value`.
    //
    // Returns `Value::Undefined` if `duk_get_type` returns a value type that cannot be decoded.
    pub(crate) unsafe fn pop_value(&self) -> Value {
        assert_stack!(self.ctx, -1, {
            match ffi::duk_get_type(self.ctx, -1) as u32 {
                ffi::DUK_TYPE_UNDEFINED => {
                    ffi::duk_pop(self.ctx);
                    Value::Undefined
                },
                ffi::DUK_TYPE_NULL => {
                    ffi::duk_pop(self.ctx);
                    Value::Null
                },
                ffi::DUK_TYPE_BOOLEAN => {
                    let result = ffi::duk_get_boolean(self.ctx, -1) != 0;
                    ffi::duk_pop(self.ctx);
                    Value::Boolean(result)
                },
                ffi::DUK_TYPE_NUMBER => {
                    let result = ffi::duk_get_number(self.ctx, -1);
                    ffi::duk_pop(self.ctx);
                    Value::Number(result)
                },
                ffi::DUK_TYPE_STRING => {
                    Value::String(String(self.pop_ref()))
                },
                ffi::DUK_TYPE_OBJECT => {
                    if ffi::duk_is_function(self.ctx, -1) != 0 {
                        Value::Function(Function(self.pop_ref()))
                    } else if ffi::duk_is_array(self.ctx, -1) != 0 {
                        Value::Array(Array(self.pop_ref()))
                    } else {
                        Value::Object(Object(self.pop_ref()))
                    }
                },
                ffi::DUK_TYPE_BUFFER => {
                    Value::Bytes(Bytes(self.pop_ref()))
                },
                _ => Value::Undefined,
            }
        })
    }

    pub(crate) unsafe fn push_ref(&self, r: &Ref) {
        assert!(r.ducc.ctx == self.ctx, "`Value` passed from one `Ducc` instance to another");
        assert_stack!(self.ctx, 1, {
            ffi::duk_require_stack(self.ctx, 2);
            ffi::duk_push_heap_stash(self.ctx);
            ffi::duk_get_prop_index(self.ctx, -1, r.stash_key);
            ffi::duk_remove(self.ctx, -2);
        });
    }

    pub(crate) unsafe fn pop_ref(&self) -> Ref {
        const STASH_KEY: [i8; 10] = hidden_i8str!('s', 't', 'a', 's', 'h', 'k', 'e', 'y');

        assert_stack!(self.ctx, -1, {
            ffi::duk_require_stack(self.ctx, 2);
            ffi::duk_push_heap_stash(self.ctx);

            // Generate a unique stash key.
            ffi::duk_get_global_string(self.ctx, STASH_KEY.as_ptr() as *const _);
            let mut stash_key: ffi::duk_uarridx_t = ffi::duk_to_number(self.ctx, -1) as u32;
            ffi::duk_pop(self.ctx);
            let mut has_wrapped = false;
            loop {
                stash_key = stash_key.wrapping_add(1);
                if stash_key == 0 {
                    if has_wrapped {
                        // We searched the entire integer space for an empty heap space. Looks like
                        // we're out of indexable memory! This only happens if the user pushes
                        // `0xFFFFFFFF` references to the heap at once, which is probably at least
                        // 16GB of memory.
                        panic!("out of addressable space in duktape heap");
                    } else {
                        has_wrapped = true;
                    }
                }

                if ffi::duk_has_prop_index(self.ctx, -1, stash_key) == 0 {
                    break;
                }
            }
            ffi::duk_push_uint(self.ctx, stash_key);
            ffi::duk_put_global_string(self.ctx, STASH_KEY.as_ptr() as *const _);

            ffi::duk_dup(self.ctx, -2);
            ffi::duk_put_prop_index(self.ctx, -2, stash_key);
            ffi::duk_pop_2(self.ctx);
            Ref { ducc: self, stash_key: stash_key }
        })
    }

    pub(crate) unsafe fn clone_ref(&self, r: &Ref) -> Ref {
        assert_stack!(self.ctx, 0, {
            self.push_ref(r);
            self.pop_ref()
        })
    }

    pub(crate) unsafe fn drop_ref(&self, r: &mut Ref) {
        assert_stack!(self.ctx, 0, {
            ffi::duk_require_stack(self.ctx, 1);
            ffi::duk_push_heap_stash(self.ctx);
            ffi::duk_del_prop_index(self.ctx, -1, r.stash_key);
            ffi::duk_pop(self.ctx);
        });
    }
}

impl Drop for Ducc {
    fn drop(&mut self) {
        if !self.is_top {
            return;
        }

        unsafe {
            let udata = get_udata(self.ctx);
            let any_map = get_any_map(self.ctx);
            ffi::duk_destroy_heap(self.ctx);
            Box::from_raw(udata);
            Box::from_raw(any_map);
        }
    }
}

/// A list of one-time settings for JavaScript execution.
#[derive(Default)]
pub struct ExecSettings {
    /// An optional closure that returns `true` if the execution should be cancelled as soon as
    /// possible, or `false` if the execution should continue. This is useful for implementing an
    /// execution timeout. This function is only called during JavaScript execution, and will not be
    /// called while execution is within native Rust code.
    pub cancel_fn: Option<Box<Fn() -> bool>>,
}
