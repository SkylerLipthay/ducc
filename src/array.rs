use error::Result;
use ffi;
use object::Object;
use std::marker::PhantomData;
use types::Ref;
use util::protect_duktape_closure;
use value::{FromValue, ToValue};

/// Reference to a JavaScript array.
#[derive(Clone, Debug)]
pub struct Array<'ducc>(pub(crate) Ref<'ducc>);

impl<'ducc> Array<'ducc> {
    /// Consumes the array and returns it as a JavaScript object. This is inexpensive, since an
    /// array *is* an object.
    pub fn into_object(self) -> Object<'ducc> {
        Object(self.0)
    }

    /// Get the value using the given array index. Returns `Value::Undefined` if no element at the
    /// index exists.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// * `FromValue::from_value` fails for the element
    pub fn get<V: FromValue<'ducc>>(&self, index: u32) -> Result<V> {
        let ducc = self.0.ducc;
        let value = unsafe {
            assert_stack!(ducc.ctx, 0, {
                ducc.push_ref(&self.0);
                protect_duktape_closure(ducc.ctx, 1, 1, |ctx| {
                    ffi::duk_get_prop_index(ctx, -1, index);
                })?;
                ducc.pop_value()
            })
        };
        V::from_value(value, ducc)
    }

    /// Sets an array element using the given index and value.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    ///
    /// * `ToValue::to_value` fails for the value
    pub fn set<V: ToValue<'ducc>>(&self, index: u32, value: V) -> Result<()> {
        let ducc = self.0.ducc;
        let value = value.to_value(ducc)?;
        unsafe {
            assert_stack!(ducc.ctx, 0, {
                ducc.push_ref(&self.0);
                ducc.push_value(value);
                protect_duktape_closure(ducc.ctx, 2, 0, |ctx| {
                    ffi::duk_put_prop_index(ctx, -2, index);
                })
            })
        }
    }

    /// Returns the number of elements in the array using the calculation
    /// `Math.floor(ToNumber(array.length))`. This function can return an error if the `ToNumber`
    /// implementation fails or if the `length` getter fails.
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

    /// Pushes an element to the end of the array. This is a shortcut for `set` using `len` as the
    /// index.
    pub fn push<V: ToValue<'ducc>>(&self, value: V) -> Result<()> {
        self.set(self.len()? as u32, value)
    }

    /// Returns an iterator over the array's indexable values.
    pub fn elements<V: FromValue<'ducc>>(self) -> Elements<'ducc, V> {
        Elements {
            array: self,
            index: 0,
            len: None,
            _phantom: PhantomData,
        }
    }
}

pub struct Elements<'ducc, V> {
    array: Array<'ducc>,
    index: u32,
    len: Option<usize>,
    _phantom: PhantomData<V>,
}

impl<'ducc, V: FromValue<'ducc>> Iterator for Elements<'ducc, V> {
    type Item = Result<V>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len.is_none() {
            self.len = Some(match self.array.len() {
                Ok(len) => len,
                Err(err) => return Some(Err(err)),
            });
        }

        if self.index as usize >= self.len.unwrap() {
            return None;
        }

        let result = self.array.get(self.index);
        self.index += 1;
        Some(result)
    }
}
