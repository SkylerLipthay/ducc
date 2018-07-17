use ffi;
use std::any::TypeId;
use std::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    /// The underlying type of error.
    pub kind: ErrorKind,
    /// An optional context message describing the error. This corresponds to the JavaScript
    /// `Error`'s `message` property.
    pub context: Option<String>,
}

#[derive(Debug)]
pub enum ErrorKind {
    /// A Rust value could not be converted to a JavaScript value.
    ToJsConversionError {
        /// Name of the Rust type that could not be converted.
        from: &'static str,
        /// Name of the JavaScript type that could not be created.
        to: &'static str,
    },
    /// A JavaScript value could not be converted to the expected Rust type.
    FromJsConversionError {
        /// Name of the JavaScript type that could not be converted.
        from: &'static str,
        /// Name of the Rust type that could not be created.
        to: &'static str,
    },
    /// An error that occurred within the scripting environment.
    RuntimeError {
        /// A code representing what type of error occurred.
        code: RuntimeErrorCode,
        /// A string representation of the type of error.
        name: String,
    },
    /// A mutable callback has triggered JavaScript code that has called the same mutable callback
    /// again.
    ///
    /// This is an error because a mutable callback can only be borrowed mutably once.
    RecursiveMutCallback,
    /// A custom error that occurs during runtime.
    ///
    /// This can be used for returning user-defined errors from callbacks.
    ExternalError(Box<RuntimeError + 'static>),
    /// An error specifying the variable that was called as a function was not a function.
    NotAFunction,
}

impl Error {
    /// Creates an `Error` from any type that implements `RuntimeError`.
    pub fn external<T: RuntimeError + 'static>(error: T) -> Error {
        Error {
            kind: ErrorKind::ExternalError(Box::new(error)),
            context: None,
        }
    }

    pub fn from_js_conversion(from: &'static str, to: &'static str) -> Error {
        Error {
            kind: ErrorKind::FromJsConversionError { from, to },
            context: None,
        }
    }

    pub fn to_js_conversion(from: &'static str, to: &'static str) -> Error {
        Error {
            kind: ErrorKind::ToJsConversionError { from, to },
            context: None,
        }
    }

    pub fn recursive_mut_callback() -> Error {
        Error { kind: ErrorKind::RecursiveMutCallback, context: None }
    }

    pub fn not_a_function() -> Error {
        Error { kind: ErrorKind::NotAFunction, context: None }
    }

    pub(crate) fn into_runtime_error_desc(self) -> RuntimeErrorDesc {
        RuntimeErrorDesc {
            code: self.runtime_code(),
            name: self.runtime_name(),
            message: self.runtime_message(),
            cause: Box::new(self),
        }
    }

    fn runtime_code(&self) -> RuntimeErrorCode {
        match &self.kind {
            ErrorKind::ToJsConversionError { .. } => RuntimeErrorCode::TypeError,
            ErrorKind::FromJsConversionError { .. } => RuntimeErrorCode::TypeError,
            ErrorKind::NotAFunction => RuntimeErrorCode::TypeError,
            ErrorKind::ExternalError(err) => err.code(),
            _ => RuntimeErrorCode::Error
        }
    }

    fn runtime_name(&self) -> String {
        match &self.kind {
            ErrorKind::ExternalError(err) => err.name(),
            _ => self.runtime_code().to_string()
        }
    }

    fn runtime_message(&self) -> Option<String> {
        let mut message = String::new();

        if let Some(ref context) = self.context {
            message.push_str(context);
        }

        if let ErrorKind::ExternalError(ref error) = self.kind {
            if let Some(ref ext_message) = error.message() {
                if !message.is_empty() {
                    message.push_str(": ");
                }

                message.push_str(ext_message);
            }
        }

        if !message.is_empty() {
            Some(message)
        } else {
            None
        }
    }
}

pub trait ResultExt {
    fn js_err_context<D: fmt::Display>(self, context: D) -> Self;
    fn js_err_context_with<D: fmt::Display, F: FnOnce(&Error) -> D>(self, op: F) -> Self;
}

impl<T> ResultExt for result::Result<T, Error> {
    fn js_err_context<D: fmt::Display>(self, context: D) -> Self {
        match self {
            Err(mut err) => {
                err.context = Some(context.to_string());
                Err(err)
            },
            result => result,
        }
    }

    fn js_err_context_with<D: fmt::Display, F: FnOnce(&Error) -> D>(self, op: F) -> Self {
        match self {
            Err(mut err) => {
                err.context = Some(op(&err).to_string());
                Err(err)
            },
            result => result,
        }
    }
}

pub struct RuntimeErrorDesc {
    pub code: RuntimeErrorCode,
    pub name: String,
    pub message: Option<String>,
    pub cause: Box<Error>,
}

/// Represents the various types of JavaScript errors that can occur. This corresponds to the
/// `prototype` of the JavaScript error object, and the `name` field is typically derived from it.
#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeErrorCode {
    Error,
    EvalError,
    RangeError,
    ReferenceError,
    SyntaxError,
    TypeError,
    UriError,
}

impl RuntimeErrorCode {
    pub(crate) fn from_duk_errcode(code: ffi::duk_errcode_t) -> RuntimeErrorCode {
        match code as u32 {
            ffi::DUK_ERR_ERROR => RuntimeErrorCode::Error,
            ffi::DUK_ERR_EVAL_ERROR => RuntimeErrorCode::EvalError,
            ffi::DUK_ERR_RANGE_ERROR => RuntimeErrorCode::RangeError,
            ffi::DUK_ERR_REFERENCE_ERROR => RuntimeErrorCode::ReferenceError,
            ffi::DUK_ERR_SYNTAX_ERROR => RuntimeErrorCode::SyntaxError,
            ffi::DUK_ERR_TYPE_ERROR => RuntimeErrorCode::TypeError,
            ffi::DUK_ERR_URI_ERROR => RuntimeErrorCode::UriError,
            _ => RuntimeErrorCode::Error,
        }
    }

    pub(crate) fn to_duk_errcode(&self) -> ffi::duk_errcode_t {
        (match *self {
            RuntimeErrorCode::Error => ffi::DUK_ERR_ERROR,
            RuntimeErrorCode::EvalError => ffi::DUK_ERR_EVAL_ERROR,
            RuntimeErrorCode::RangeError => ffi::DUK_ERR_RANGE_ERROR,
            RuntimeErrorCode::ReferenceError => ffi::DUK_ERR_REFERENCE_ERROR,
            RuntimeErrorCode::SyntaxError => ffi::DUK_ERR_SYNTAX_ERROR,
            RuntimeErrorCode::TypeError => ffi::DUK_ERR_TYPE_ERROR,
            RuntimeErrorCode::UriError => ffi::DUK_ERR_URI_ERROR,
        }) as ffi::duk_errcode_t
    }
}

impl fmt::Display for RuntimeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeErrorCode::Error => write!(f, "Error"),
            RuntimeErrorCode::EvalError => write!(f, "EvalError"),
            RuntimeErrorCode::RangeError => write!(f, "RangeError"),
            RuntimeErrorCode::ReferenceError => write!(f, "ReferenceError"),
            RuntimeErrorCode::SyntaxError => write!(f, "SyntaxError"),
            RuntimeErrorCode::TypeError => write!(f, "TypeError"),
            RuntimeErrorCode::UriError => write!(f, "URIError"),
        }
    }
}

/// A Rust error that can be transformed into a JavaScript error.
pub trait RuntimeError: fmt::Debug {
    /// The prototypical JavaScript error code.
    ///
    /// By default, this method returns `RuntimeErrorCode::Error`.
    fn code(&self) -> RuntimeErrorCode {
        RuntimeErrorCode::Error
    }

    /// The name of the error corresponding to the JavaScript error's `name` property.
    ///
    /// By default, this method returns the string name corresponding to this object's `code()`
    /// return value.
    fn name(&self) -> String {
        self.code().to_string()
    }

    /// An optional message that is set on the JavaScript error's `message` property. This is
    /// automatically appended to the parent `Error`'s `context` field.
    ///
    /// By default, this method returns `None`.
    fn message(&self) -> Option<String> {
        None
    }

    // TODO: Should we support modifying the error object?
    // fn customize<'ducc>(&self, ducc: &'ducc Ducc, object: &'ducc Object<'ducc>) {
    //     let _ = ducc;
    //     let _ = object;
    // }

    #[doc(hidden)]
    fn __private_get_type_id__(&self) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }
}

impl RuntimeError {
    /// Attempts to downcast this failure to a concrete type by reference.
    ///
    /// If the underlying error is not of type `T`, this will return `None`.
    pub fn downcast_ref<T: RuntimeError + 'static>(&self) -> Option<&T> {
        if self.__private_get_type_id__() == TypeId::of::<T>() {
            unsafe { Some(&*(self as *const RuntimeError as *const T)) }
        } else {
            None
        }
    }
}

impl RuntimeError for () {
}

impl RuntimeError for String {
    fn message(&self) -> Option<String> {
        Some(self.clone())
    }
}

impl<'a> RuntimeError for &'a str {
    fn message(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl<T: RuntimeError + 'static> From<T> for Error {
    fn from(error: T) -> Error {
        Error::external(error)
    }
}

impl From<ErrorKind> for Error {
    fn from(error: ErrorKind) -> Error {
        Error {
            kind: error,
            context: None,
        }
    }
}
