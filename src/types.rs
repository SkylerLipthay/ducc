use ducc::Ducc;
use error::Result;
use ffi;
use std::fmt;
use value::{Value, Values};

pub struct Ref<'ducc> {
    pub ducc: &'ducc Ducc,
    pub stash_key: ffi::duk_uarridx_t,
}

impl<'ducc> fmt::Debug for Ref<'ducc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ref({})", self.stash_key)
    }
}

impl<'ducc> Clone for Ref<'ducc> {
    fn clone(&self) -> Self {
        unsafe { self.ducc.clone_ref(self) }
    }
}

impl<'ducc> Drop for Ref<'ducc> {
    fn drop(&mut self) {
        unsafe { self.ducc.drop_ref(self); }
    }
}

pub type Callback<'ducc, 'a> =
    Box<Fn(&'ducc Ducc, Value<'ducc>, Values<'ducc>) -> Result<Value<'ducc>> + 'a>;
