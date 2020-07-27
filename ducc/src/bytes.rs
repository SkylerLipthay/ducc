use ffi;
use object::Object;
use std::slice;
use types::Ref;

/// Reference to a JavaScript `Uint8Array`.
#[derive(Clone, Debug)]
pub struct Bytes<'ducc>(pub(crate) Ref<'ducc>);

impl<'ducc> Bytes<'ducc> {
    /// Extracts the byte data from JavaScript into a `Vec<u8>`.
    pub fn to_vec(&self) -> Vec<u8> {
        unsafe {
            let ducc = self.0.ducc;
            let ctx = ducc.ctx;
            assert_stack!(ctx, 0, {
                ducc.push_ref(&self.0);
                assert!(ffi::duk_is_buffer_data(ctx, -1) != 0);
                let mut len = 0;
                let data = ffi::duk_get_buffer_data(ctx, -1, &mut len);
                assert!(!data.is_null());
                let bytes = slice::from_raw_parts(data as *const u8, len as usize);
                ffi::duk_pop(ctx);
                bytes.to_vec()
            })
        }
    }

    /// Consumes the buffer and returns it as a JavaScript object. This is inexpensive, since a
    /// buffer *is* an object.
    pub fn into_object(self) -> Object<'ducc> {
        Object(self.0)
    }
}
