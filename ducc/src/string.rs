use cesu8::from_cesu8;
use error::{Error, Result};
use ffi;
use std::slice;
use std::borrow::Cow;
use std::string::String as StdString;
use types::Ref;

/// An immutable, interned JavaScript string managed by Duktape.
///
/// Unlike Rust strings, Duktape strings are CESU-8 (not UTF-8).
#[derive(Clone, Debug)]
pub struct String<'ducc>(pub(crate) Ref<'ducc>);

impl<'ducc> String<'ducc> {
    /// Returns a Rust string converted from the Duktape string as long as it can be converted from
    /// CESU-8 to UTF-8.
    pub fn to_string(&self) -> Result<StdString> {
        match from_cesu8(self.as_bytes()) {
            Ok(string) => Ok(string.into_owned()),
            Err(_) => Err(Error::from_js_conversion("string", "String"))
        }
    }

    /// Returns a Rust string converted from the Duktape string as long as it can be converted from
    /// CESU-8 to UTF-8.
    ///
    /// If the underlying Duktape string is already a valid UTF-8 string, this function
    /// will return a direct pointer to the underlying character data (i.e. no string data
    /// will be cloned).
    ///
    /// Otherwise, returns a copy of the string converted to UTF-8.
    pub fn to_str(&self) -> Result<Cow<str>> {
        match from_cesu8(self.as_bytes()) {
            Ok(string) => Ok(string),
            Err(_) => Err(Error::from_js_conversion("string", "String"))
        }
    }

    /// Returns the bytes that make up this string, without a trailing nul byte. This is a CESU-8
    /// string.
    pub fn as_bytes(&self) -> &[u8] {
        let with_nul = self.as_bytes_with_nul();
        &with_nul[..with_nul.len() - 1]
    }

    /// Returns the bytes that make up this string, including a trailing nul byte. This is a CESU-8
    /// string.
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        // Strings are interned and cannot be modified, so the returned reference to its bytes is
        // guaranteed to live as long as the reference lives.
        unsafe {
            let ducc = self.0.ducc;
            let ctx = ducc.ctx;
            assert_stack!(ctx, 0, {
                ducc.push_ref(&self.0);
                assert!(ffi::duk_is_string(ctx, -1) != 0);
                let mut len = 0;
                let data = ffi::duk_get_lstring(ctx, -1, &mut len);
                assert!(!data.is_null());
                let bytes = slice::from_raw_parts(data as *const u8, len + 1 as usize);
                assert!(bytes[bytes.len() - 1] == 0);
                ffi::duk_pop(ctx);
                bytes
            })
        }
    }
}

impl<'ducc> AsRef<[u8]> for String<'ducc> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

// Duktape strings are basically &[u8] slices, so implement PartialEq for anything resembling that.
//
// This makes our `String` comparable with `Vec<u8>`, `[u8]`, `&str`, `String`, and `ducc::String`
// itself.
//
// The only downside is that this disallows a comparison with `Cow<str>`, as that only implements
// `AsRef<str>`, which collides with this impl. Requiring `AsRef<str>` would fix that, but would
// limit us in other ways.
impl<'ducc, T: AsRef<[u8]>> PartialEq<T> for String<'ducc> {
    fn eq(&self, other: &T) -> bool {
        self.as_bytes() == other.as_ref()
    }
}
