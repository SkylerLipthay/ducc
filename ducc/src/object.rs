use error::{Error, Result};
use ffi;
use std::marker::PhantomData;
use types::Ref;
use util::{protect_duktape_closure, StackGuard};
use function::Function;
use value::{FromValue, ToValue, ToValues, Value};

/// Reference to a JavaScript object (guaranteed to not be an array or function).
#[derive(Clone, Debug)]
pub struct Object<'ducc>(pub(crate) Ref<'ducc>);

impl<'ducc> Object<'ducc> {
    /// Get an object property value using the given key. Returns `Value::Undefined` if no property
    /// with the key exists.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// * `ToValue::to_value` fails for the key
    /// * The `ToPropertyKey` implementation for the key fails
    pub fn get<K: ToValue<'ducc>, V: FromValue<'ducc>>(&self, key: K) -> Result<V> {
        let ducc = self.0.ducc;
        let key = key.to_value(ducc)?;
        let value = unsafe {
            assert_stack!(ducc.ctx, 0, {
                ducc.push_ref(&self.0);
                ducc.push_value(key);
                protect_duktape_closure(ducc.ctx, 2, 1, |ctx| {
                    ffi::duk_get_prop(ctx, -2);
                })?;
                ducc.pop_value()
            })
        };
        V::from_value(value, ducc)
    }

    /// Sets an object property using the given key and value.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// * `ToValue::to_value` fails for either the key or the value
    /// * The `ToPropertyKey` implementation for the key fails
    pub fn set<K: ToValue<'ducc>, V: ToValue<'ducc>>(&self, key: K, value: V) -> Result<()> {
        let ducc = self.0.ducc;
        let key = key.to_value(ducc)?;
        let value = value.to_value(ducc)?;
        unsafe {
            assert_stack!(ducc.ctx, 0, {
                ducc.push_ref(&self.0);
                ducc.push_value(key);
                ducc.push_value(value);
                protect_duktape_closure(ducc.ctx, 3, 0, |ctx| {
                    ffi::duk_put_prop(ctx, -3);
                })
            })
        }
    }

    /// Defines a property using given key and descriptor
    /// 
    /// # Example
    /// 
    /// ```
    /// # use ducc::{Ducc, PropertyDescriptor};
    /// # let ducc = Ducc::new();
    /// let obj = ducc.create_object();
    /// let get = ducc.create_function(|inv| Ok(24));
    /// obj.define_prop("prop", PropertyDescriptor::new().getter(get)).unwrap();
    /// ```
    pub fn define_prop<K: ToValue<'ducc>>(&self, key: K, desc: PropertyDescriptor<'ducc>) -> Result<()> {
        let ducc = self.0.ducc;
        let key = key.to_value(ducc)?;

        let mut flags = 0;
        flags |= match desc.writable {
            Some(true) => ffi::DUK_DEFPROP_HAVE_WRITABLE | ffi::DUK_DEFPROP_WRITABLE,
            Some(false) => ffi::DUK_DEFPROP_HAVE_WRITABLE,
            None => 0
        };
        flags |= match desc.enumerable {
            Some(true) => ffi::DUK_DEFPROP_HAVE_ENUMERABLE | ffi::DUK_DEFPROP_ENUMERABLE,
            Some(false) => ffi::DUK_DEFPROP_HAVE_ENUMERABLE,
            None => 0
        };
        flags |= match desc.configurable {
            Some(true) => ffi::DUK_DEFPROP_HAVE_CONFIGURABLE | ffi::DUK_DEFPROP_CONFIGURABLE,
            Some(false) => ffi::DUK_DEFPROP_HAVE_CONFIGURABLE,
            None => 0
        };

        unsafe {
            assert_stack!(ducc.ctx, 0, {
                ducc.push_ref(&self.0);
                ducc.push_value(key);
                let mut num_args = 2;
                match desc.source {
                    PropertySource::Undefined => {},
                    PropertySource::Value(val) => {
                        ducc.push_value(val);
                        flags |= ffi::DUK_DEFPROP_HAVE_VALUE;
                        num_args += 1;
                    },
                    PropertySource::GetSet(get, set) => {
                        ducc.push_value(get.to_value(ducc)?);
                        ducc.push_value(set.to_value(ducc)?);
                        flags |=
                            ffi::DUK_DEFPROP_HAVE_GETTER | ffi::DUK_DEFPROP_HAVE_SETTER;
                        num_args += 2;
                    },
                    PropertySource::Get(get) => {
                        ducc.push_value(get.to_value(ducc)?);
                        flags |= ffi::DUK_DEFPROP_HAVE_GETTER;
                        num_args += 1;
                    },
                    PropertySource::Set(set) => {
                        ducc.push_value(set.to_value(ducc)?);
                        flags |= ffi::DUK_DEFPROP_HAVE_SETTER;
                        num_args += 1;
                    }
                }
                protect_duktape_closure(ducc.ctx, num_args, 0, |ctx| {
                    ffi::duk_def_prop(ctx, -num_args, flags);
                })
            })
        }
    }

    /// Removes the given key from the object. This function does nothing if the property does not
    /// exist.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// * `ToValue::to_value` fails for the key
    /// * The `ToPropertyKey` implementation for the key fails
    pub fn remove<K: ToValue<'ducc>>(&self, key: K) -> Result<()> {
        let ducc = self.0.ducc;
        let key = key.to_value(ducc)?;
        unsafe {
            assert_stack!(ducc.ctx, 0, {
                ducc.push_ref(&self.0);
                ducc.push_value(key);
                protect_duktape_closure(ducc.ctx, 2, 0, |ctx| {
                    ffi::duk_del_prop(ctx, -2);
                })
            })
        }
    }

    /// Returns `true` if the given key is a property of the object, `false` otherwise.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// * `ToValue::to_value` fails for the key
    /// * The `ToPropertyKey` implementation for the key fails
    pub fn contains_key<K: ToValue<'ducc>>(&self, key: K) -> Result<bool> {
        let ducc = self.0.ducc;
        let key = key.to_value(ducc)?;
        unsafe {
            assert_stack!(ducc.ctx, 0, {
                ducc.push_ref(&self.0);
                ducc.push_value(key);
                protect_duktape_closure(ducc.ctx, 2, 0, |ctx| {
                    ffi::duk_has_prop(ctx, -2) != 0
                })
            })
        }
    }

    /// Returns the number of elements in the object using the calculation
    /// `Math.floor(ToNumber(obj.length))`. This function can return an error if the `ToNumber`
    /// implementation fails or if the `length` getter fails. Returns `Ok(0)` if the calculation
    /// returns a number (a floating point in JavaScript land) outside of the range of `usize`.
    pub fn len(&self) -> Result<usize> {
        let ducc = self.0.ducc;
        unsafe {
            assert_stack!(ducc.ctx, 0, {
                ducc.push_ref(&self.0);
                protect_duktape_closure(ducc.ctx, 1, 0, |ctx| {
                    ffi::duk_get_length(ctx, -1)
                })
            })
        }
    }

    /// Calls the function at the key with the given arguments, with `this` set to the object.
    /// Returns an error if the value at the key is not a function.
    pub fn call_prop<K, A, R>(&self, key: K, args: A) -> Result<R>
    where
        K: ToValue<'ducc>,
        A: ToValues<'ducc>,
        R: FromValue<'ducc>,
    {
        let value: Value = self.get(key)?;
        if let Some(func) = value.as_function() {
            func.call_method(self.clone(), args)
        } else {
            Err(Error::not_a_function())
        }
    }

    /// Returns an iterator over the object's keys and values, acting like a `for-in` loop: own and
    /// inherited enumerable properties are included, and enumeration order follows the ES2015
    /// `OwnPropertyKeys` enumeration order, applied for each inheritance level.
    pub fn properties<K: FromValue<'ducc>, V: FromValue<'ducc>>(self) -> Properties<'ducc, K, V> {
        let ducc = self.0.ducc;
        unsafe {
            let _sg = StackGuard::new(ducc.ctx);
            ducc.push_ref(&self.0);
            ffi::duk_require_stack(ducc.ctx, 1);
            ffi::duk_enum(ducc.ctx, -1, 0);
            Properties {
                object_enum: ducc.pop_ref(),
                _phantom: PhantomData,
            }
        }
    }
}

enum PropertySource<'ducc> {
    Undefined,
    Value(Value<'ducc>),
    GetSet(Function<'ducc>, Function<'ducc>),
    Get(Function<'ducc>),
    Set(Function<'ducc>),
}

pub struct PropertyDescriptor<'ducc> {
    enumerable: Option<bool>,
    configurable: Option<bool>,
    writable: Option<bool>,
    source: PropertySource<'ducc>
}
impl <'ducc> PropertyDescriptor<'ducc> {
    pub fn new() -> PropertyDescriptor<'ducc> {
        PropertyDescriptor {
            enumerable: None,
            configurable: None,
            writable: None,
            source: PropertySource::Undefined
        }
    }

    /// Whether this property shows up during enumeration of the
    /// properties on the corresponding object.
    /// 
    /// Defaults to `false`
    pub fn enumerable<'a>(&'a mut self, b: bool) -> &'a mut PropertyDescriptor<'ducc> {
        self.enumerable = Some(b);
        self
    }

    /// Whether the type of this property descriptor may be changed and
    /// the property may be deleted from the corresponding object.
    /// 
    /// Defaults to `false`
    pub fn configurable<'a>(&'a mut self, b: bool) -> &'a mut PropertyDescriptor<'ducc> {
        self.configurable = Some(b);
        self
    }

    /// Whether the value associated with the property may be changed with
    /// an assignment operator. Must not be set when using getters or setters.
    /// 
    /// Defaults to `false`
    pub fn writable<'a>(&'a mut self, b: bool) -> &'a mut PropertyDescriptor<'ducc> {
        self.writable = Some(b);
        self
    }

    /// Builds the descriptor with given value for the property
    pub fn value(mut self, value: Value<'ducc>) -> PropertyDescriptor<'ducc> {
        self.source = PropertySource::Value(value);
        self
    }

    /// Builds the descriptor with a getter and a setter
    pub fn getter_setter(mut self, get: Function<'ducc>, set: Function<'ducc>) -> PropertyDescriptor<'ducc> {
        self.source = PropertySource::GetSet(get, set);
        self
    }

    /// Builds the descriptor with a getter
    pub fn getter(mut self, get: Function<'ducc>) -> PropertyDescriptor<'ducc> {
        self.source = PropertySource::Get(get);
        self
    }

    /// Builds the descriptor with a setter
    pub fn setter(mut self, set: Function<'ducc>) -> PropertyDescriptor<'ducc> {
        self.source = PropertySource::Set(set);
        self
    }
}

pub struct Properties<'ducc, K, V> {
    object_enum: Ref<'ducc>,
    _phantom: PhantomData<(K, V)>,
}

impl<'ducc, K, V> Iterator for Properties<'ducc, K, V>
where
    K: FromValue<'ducc>,
    V: FromValue<'ducc>,
{
    type Item = Result<(K, V)>;

    fn next(&mut self) -> Option<Self::Item> {
        let ducc = self.object_enum.ducc;
        unsafe {
            let _sg = StackGuard::new(ducc.ctx);
            ducc.push_ref(&self.object_enum);
            ffi::duk_require_stack(ducc.ctx, 2);
            if ffi::duk_next(ducc.ctx, -1, 1) != 0 {
                let value = match ducc.pop_value().into(ducc) {
                    Ok(value) => value,
                    Err(err) => return Some(Err(err)),
                };
                let key = match ducc.pop_value().into(ducc) {
                    Ok(key) => key,
                    Err(err) => return Some(Err(err)),
                };
                Some(Ok((key, value)))
            } else {
                None
            }
        }
    }
}
