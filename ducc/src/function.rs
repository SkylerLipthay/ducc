use ducc::Ducc;
use error::Result;
use ffi;
use object::Object;
use std::panic::{AssertUnwindSafe, catch_unwind};
use types::{Callback, Ref};
use util::{pop_error, push_error};
use value::{FromValue, ToValue, ToValues, Value, Values};

/// Reference to a JavaScript function.
#[derive(Clone, Debug)]
pub struct Function<'ducc>(pub(crate) Ref<'ducc>);

impl<'ducc> Function<'ducc> {
    /// Calls the function with the given arguments, with `this` set to `undefined`.
    pub fn call<A, R>(&self, args: A) -> Result<R>
    where
        A: ToValues<'ducc>,
        R: FromValue<'ducc>,
    {
        self.call_method(Value::Undefined, args)
    }

    /// Calls the function with the given `this` and arguments.
    pub fn call_method<T, A, R>(&self, this: T, args: A) -> Result<R>
    where
        T: ToValue<'ducc>,
        A: ToValues<'ducc>,
        R: FromValue<'ducc>,
    {
        let ducc = self.0.ducc;
        let this = this.to_value(ducc)?;
        let args = args.to_values(ducc)?;
        let num_args = args.len() as ffi::duk_idx_t;

        unsafe {
            assert_stack!(ducc.ctx, 0, {
                ducc.push_ref(&self.0);
                ducc.push_value(this);
                for arg in args.into_vec().into_iter() {
                    ducc.push_value(arg);
                }

                ffi::duk_require_stack(ducc.ctx, 1);
                if ffi::duk_pcall_method(ducc.ctx, num_args) == 0 {
                    FromValue::from_value(ducc.pop_value(), ducc)
                } else {
                    Err(pop_error(ducc.ctx))
                }
            })
        }
    }

    /// Calls the function as if it is a constructor function with given arguments
    pub fn call_new<A, R>(&self, args: A) -> Result<R>
    where
        A: ToValues<'ducc>,
        R: FromValue<'ducc>,
    {
        let ducc = self.0.ducc;
        let args = args.to_values(ducc)?;
        let num_args = args.len() as ffi::duk_idx_t;

        unsafe {
            assert_stack!(ducc.ctx, 0, {
                ducc.push_ref(&self.0);
                for arg in args.into_vec().into_iter() {
                    ducc.push_value(arg);
                }

                ffi::duk_require_stack(ducc.ctx, 1);
                if ffi::duk_pnew(ducc.ctx, num_args) == 0 {
                    FromValue::from_value(ducc.pop_value(), ducc)
                } else {
                    Err(pop_error(ducc.ctx))
                }
            })
        }
    }

    /// Consumes the function and returns it as a JavaScript object. This is inexpensive, since a
    /// function *is* an object.
    pub fn into_object(self) -> Object<'ducc> {
        Object(self.0)
    }
}

pub struct Invocation<'ducc> {
    pub ducc: &'ducc Ducc,
    pub this: Value<'ducc>,
    pub args: Values<'ducc>,
}

const FUNC: [i8; 6] = hidden_i8str!('f', 'u', 'n', 'c');

pub(crate) fn create_callback<'ducc, 'callback>(
    ducc: &'ducc Ducc,
    func: Callback<'callback, 'static>,
) -> Function<'ducc> {
    unsafe extern "C" fn wrapper(ctx: *mut ffi::duk_context) -> ffi::duk_ret_t {
        assert_stack!(ctx, 1, {
            ffi::duk_require_stack(ctx, 2);

            let ducc = Ducc { ctx, is_top: false };
            let num_args = ffi::duk_get_top(ctx) as usize;
            let mut args = Vec::with_capacity(num_args);
            for i in 0..num_args {
                ffi::duk_dup(ctx, i as ffi::duk_idx_t);
                args.push(ducc.pop_value());
            }

            ffi::duk_push_current_function(ctx);
            ffi::duk_get_prop_string(ctx, -1, FUNC.as_ptr() as *const _);
            let func_ptr = ffi::duk_get_pointer(ctx, -1) as *mut Callback;
            ffi::duk_pop_n(ctx, 2);

            ffi::duk_push_this(ctx);
            let this = ducc.pop_value();

            let inner = || (*func_ptr)(&ducc, this, Values::from_vec(args));
            let result = match catch_unwind(AssertUnwindSafe(inner)) {
                Ok(result) => result,
                Err(_) => {
                    ffi::duk_fatal_raw(ctx, cstr!("panic occurred during script execution"));
                    unreachable!();
                },
            };

            match result {
                Ok(value) => {
                    ducc.push_value(value);
                    1
                },
                Err(error) => {
                    push_error(ctx, error);
                    -1
                },
            }
        })
    }

    unsafe extern "C" fn finalizer(ctx: *mut ffi::duk_context) -> ffi::duk_ret_t {
        ffi::duk_require_stack(ctx, 1);
        ffi::duk_get_prop_string(ctx, 0, FUNC.as_ptr() as *const _);
        let callback = Box::from_raw(ffi::duk_get_pointer(ctx, -1) as *mut Callback);
        drop(callback);
        ffi::duk_pop(ctx);
        ffi::duk_push_undefined(ctx);
        ffi::duk_put_prop_string(ctx, 0, FUNC.as_ptr() as *const _);
        0
    }

    unsafe {
        assert_stack!(ducc.ctx, 0, {
            ffi::duk_require_stack(ducc.ctx, 2);
            ffi::ducc_push_c_function_nothrow(ducc.ctx, Some(wrapper), ffi::DUK_VARARGS);
            ffi::duk_push_pointer(ducc.ctx, Box::into_raw(Box::new(func)) as *mut _);
            ffi::duk_put_prop_string(ducc.ctx, -2, FUNC.as_ptr() as *const _);
            ffi::duk_push_c_function(ducc.ctx, Some(finalizer), 1);
            ffi::duk_set_finalizer(ducc.ctx, -2);
            Function(ducc.pop_ref())
        })
    }
}
