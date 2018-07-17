use object::Object;
use types::Ref;

/// Reference to a JavaScript `Uint8Array`.
#[derive(Clone, Debug)]
pub struct Bytes<'ducc>(pub(crate) Ref<'ducc>);

impl<'ducc> Bytes<'ducc> {
    /// Consumes the buffer and returns it as a JavaScript object. This is inexpensive, since a
    /// buffer *is* an object.
    pub fn into_object(self) -> Object<'ducc> {
        Object(self.0)
    }
}
