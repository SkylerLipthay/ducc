extern crate ducc;
#[macro_use]
extern crate serde;

pub mod error;
pub mod ser;
pub mod de;

pub use error::{Error, Result};
pub use ser::Serializer;

use ducc::{Ducc, Value, Result as DuccResult};

pub fn to_value<T: serde::Serialize>(ducc: &Ducc, value: T) -> DuccResult<Value> {
    let serializer = ser::Serializer { ducc };
    Ok(value.serialize(serializer)?)
}

pub fn from_value<'de, T: serde::Deserialize<'de>>(value: Value<'de>) -> DuccResult<T> {
    let deserializer = de::Deserializer { value };
    Ok(T::deserialize(deserializer)?)
}
