use array::Array;
use bytes::Bytes;
use ducc::Ducc;
use error::Result;
use function::Function;
use object::Object;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::{slice, vec};
use string::String;

/// A single JavaScript value.
///
/// It is a logic error to share a `Value` with an internal reference to its parent `Ducc` between
/// two separate `Ducc`s and doing so will result in a panic.
#[derive(Clone, Debug)]
pub enum Value<'ducc> {
    /// The JavaScript value `undefined`.
    Undefined,
    /// The JavaScript value `null`.
    Null,
    /// The JavaScript value `true` or `false`.
    Boolean(bool),
    /// A JavaScript floating point number.
    Number(f64),
    /// An interned JavaScript string, managed by Duktape. Contains an internal reference to its
    /// parent `Ducc`.
    String(String<'ducc>),
    /// Reference to a JavaScript function. Contains an internal reference to its parent `Ducc`.
    Function(Function<'ducc>),
    /// Reference to a JavaScript array. Contains an internal reference to its parent `Ducc`.
    Array(Array<'ducc>),
    /// Reference to a JavaScript object (guaranteed to not be an array or function). Contains an
    /// internal reference to its parent `Ducc`.
    Object(Object<'ducc>),
    /// Reference to a JavaScript `Uint8Array`. Contains an internal reference to its parent `Ducc`.
    Bytes(Bytes<'ducc>),
}

impl<'ducc> Value<'ducc> {
    /// Returns `true` if this is a `Value::Undefined`, `false` otherwise.
    pub fn is_undefined(&self) -> bool {
        if let Value::Undefined = *self { true } else { false }
    }

    /// Returns `true` if this is a `Value::Null`, `false` otherwise.
    pub fn is_null(&self) -> bool {
        if let Value::Null = *self { true } else { false }
    }

    /// Returns `true` if this is a `Value::Boolean`, `false` otherwise.
    pub fn is_boolean(&self) -> bool {
        if let Value::Boolean(_) = *self { true } else { false }
    }

    /// Returns `true` if this is a `Value::Number`, `false` otherwise.
    pub fn is_number(&self) -> bool {
        if let Value::Number(_) = *self { true } else { false }
    }

    /// Returns `true` if this is a `Value::String`, `false` otherwise.
    pub fn is_string(&self) -> bool {
        if let Value::String(_) = *self { true } else { false }
    }

    /// Returns `true` if this is a `Value::Function`, `false` otherwise.
    pub fn is_function(&self) -> bool {
        if let Value::Function(_) = *self { true } else { false }
    }

    /// Returns `true` if this is a `Value::Array`, `false` otherwise.
    pub fn is_array(&self) -> bool {
        if let Value::Array(_) = *self { true } else { false }
    }

    /// Returns `true` if this is a `Value::Object`, `false` otherwise.
    pub fn is_object(&self) -> bool {
        if let Value::Object(_) = *self { true } else { false }
    }

    /// Returns `true` if this is a `Value::Bytes`, `false` otherwise.
    pub fn is_bytes(&self) -> bool {
        if let Value::Bytes(_) = *self { true } else { false }
    }

    /// Returns `Some(())` if this is a `Value::Undefined`, `None` otherwise.
    pub fn as_undefined(&self) -> Option<()> {
        if let Value::Undefined = *self { Some(()) } else { None }
    }

    /// Returns `Some(())` if this is a `Value::Null`, `None` otherwise.
    pub fn as_null(&self) -> Option<()> {
        if let Value::Undefined = *self { Some(()) } else { None }
    }

    /// Returns `Some` if this is a `Value::Boolean`, `None` otherwise.
    pub fn as_boolean(&self) -> Option<bool> {
        if let Value::Boolean(value) = *self { Some(value) } else { None }
    }

    /// Returns `Some` if this is a `Value::Number`, `None` otherwise.
    pub fn as_number(&self) -> Option<f64> {
        if let Value::Number(value) = *self { Some(value) } else { None }
    }

    /// Returns `Some` if this is a `Value::String`, `None` otherwise.
    pub fn as_string(&self) -> Option<&String<'ducc>> {
        if let Value::String(ref value) = *self { Some(value) } else { None }
    }

    /// Returns `Some` if this is a `Value::Function`, `None` otherwise.
    pub fn as_function(&self) -> Option<&Function<'ducc>> {
        if let Value::Function(ref value) = *self { Some(value) } else { None }
    }

    /// Returns `Some` if this is a `Value::Array`, `None` otherwise.
    pub fn as_array(&self) -> Option<&Array<'ducc>> {
        if let Value::Array(ref value) = *self { Some(value) } else { None }
    }

    /// Returns `Some` if this is a `Value::Object`, `None` otherwise.
    pub fn as_object(&self) -> Option<&Object<'ducc>> {
        if let Value::Object(ref value) = *self { Some(value) } else { None }
    }

    /// Returns `Some` if this is a `Value::Bytes`, `None` otherwise.
    pub fn as_bytes(&self) -> Option<&Bytes<'ducc>> {
        if let Value::Bytes(ref value) = *self { Some(value) } else { None }
    }

    /// A wrapper around `FromValue::from_value`.
    pub fn into<T: FromValue<'ducc>>(self, ducc: &'ducc Ducc) -> Result<T> {
        T::from_value(self, ducc)
    }

    pub(crate) fn type_name(&self) -> &'static str {
        match *self {
            Value::Undefined => "undefined",
            Value::Null => "null",
            Value::Boolean(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Function(_) => "function",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Bytes(_) => "bytes",
        }
    }
}

/// Trait for types convertible to `Value`.
pub trait ToValue<'ducc> {
    /// Performs the conversion.
    fn to_value(self, ducc: &'ducc Ducc) -> Result<Value<'ducc>>;
}

/// Trait for types convertible from `Value`.
pub trait FromValue<'ducc>: Sized {
    /// Performs the conversion.
    fn from_value(value: Value<'ducc>, ducc: &'ducc Ducc) -> Result<Self>;
}

/// A collection of multiple JavaScript values used for interacting with function arguments.
#[derive(Clone, Debug)]
pub struct Values<'ducc>(Vec<Value<'ducc>>);

impl<'ducc> Values<'ducc> {
    /// Creates an empty `Values`.
    pub fn new() -> Values<'ducc> {
        Values(Vec::new())
    }

    pub fn from_vec(vec: Vec<Value<'ducc>>) -> Values<'ducc> {
        Values(vec)
    }

    pub fn into_vec(self) -> Vec<Value<'ducc>> {
        self.0
    }

    pub fn get(&self, index: usize) -> Value<'ducc> {
        self.0.get(index).map(Clone::clone).unwrap_or(Value::Undefined)
    }

    pub fn from<T: FromValue<'ducc>>(&self, ducc: &'ducc Ducc, index: usize) -> Result<T> {
        T::from_value(self.0.get(index).map(Clone::clone).unwrap_or(Value::Undefined), ducc)
    }

    pub fn into<T: FromValues<'ducc>>(self, ducc: &'ducc Ducc) -> Result<T> {
        T::from_values(self, ducc)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Value<'ducc>> {
        self.0.iter()
    }
}

impl<'ducc> FromIterator<Value<'ducc>> for Values<'ducc> {
    fn from_iter<I: IntoIterator<Item = Value<'ducc>>>(iter: I) -> Self {
        Values::from_vec(Vec::from_iter(iter))
    }
}

impl<'ducc> IntoIterator for Values<'ducc> {
    type Item = Value<'ducc>;
    type IntoIter = vec::IntoIter<Value<'ducc>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, 'ducc> IntoIterator for &'a Values<'ducc> {
    type Item = &'a Value<'ducc>;
    type IntoIter = slice::Iter<'a, Value<'ducc>>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

/// Trait for types convertible to any number of JavaScript values.
///
/// This is a generalization of `ToValue`, allowing any number of resulting JavaScript values
/// instead of just one. Any type that implements `ToValue` will automatically implement this trait.
pub trait ToValues<'ducc> {
    /// Performs the conversion.
    fn to_values(self, ducc: &'ducc Ducc) -> Result<Values<'ducc>>;
}

/// Trait for types that can be created from an arbitrary number of JavaScript values.
///
/// This is a generalization of `FromValue`, allowing an arbitrary number of JavaScript values to
/// participate in the conversion. Any type that implements `FromValue` will automatically implement
/// this trait.
pub trait FromValues<'ducc>: Sized {
    /// Performs the conversion.
    ///
    /// In case `values` contains more values than needed to perform the conversion, the excess
    /// values should be ignored. This reflects the semantics of Duktape when calling a function or
    /// assigning values. Similarly, if not enough values are given, conversions should assume that
    /// any missing values are nil.
    fn from_values(values: Values<'ducc>, ducc: &'ducc Ducc) -> Result<Self>;
}

/// Wraps a variable number of `T`s.
///
/// Can be used to work with variadic functions more easily. Using this type as the last argument of
/// a Rust callback will accept any number of arguments from JavaScript and convert them to the type
/// `T` using [`FromValue`]. `Variadic<T>` can also be returned from a callback, returning a
/// variable number of values to JavaScript.
#[derive(Clone, Debug)]
pub struct Variadic<T>(pub(crate) Vec<T>);

impl<T> Variadic<T> {
    /// Creates an empty `Variadic` wrapper containing no values.
    pub fn new() -> Variadic<T> {
        Variadic(Vec::new())
    }

    pub fn from_vec(vec: Vec<T>) -> Variadic<T> {
        Variadic(vec)
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}

impl<T> FromIterator<T> for Variadic<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Variadic(Vec::from_iter(iter))
    }
}

impl<T> IntoIterator for Variadic<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Deref for Variadic<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Variadic<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
