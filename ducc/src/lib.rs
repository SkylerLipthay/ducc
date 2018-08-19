extern crate cesu8;
extern crate ducc_sys as ffi;

#[macro_use] mod util;
mod array;
mod bytes;
mod conversion;
mod ducc;
mod error;
mod function;
mod object;
mod string;
mod types;
mod value;

#[cfg(test)] mod tests;

pub use array::{Array, Elements};
pub use bytes::Bytes;
pub use ducc::{Ducc, ExecSettings};
pub use error::{Error, ErrorKind, Result, ResultExt, RuntimeError, RuntimeErrorCode};
pub use function::{Function, Invocation};
pub use object::{Object, Properties};
pub use string::String;
pub use value::{FromValue, FromValues, ToValue, ToValues, Value, Values, Variadic};
