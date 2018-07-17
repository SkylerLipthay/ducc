use ffi;
use std::any::TypeId;
use std::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// A Rust value could not be converted to a Duktape value.
    ToDuktapeConversionError {
        /// Name of the Rust type that could not be converted.
        from: &'static str,
        /// Name of the Duktape type that could not be created.
        to: &'static str,
        /// A string containing more detailed error information. This will be exposed in JavaScript.
        message: Option<String>,
    },
    /// A Duktape value could not be converted to the expected Rust type.
    FromDuktapeConversionError {
        /// Name of the Lua type that could not be converted.
        from: &'static str,
        /// Name of the Rust type that could not be created.
        to: &'static str,
        /// A string containing more detailed error information. This will be exposed in JavaScript.
        message: Option<String>,
    },
    /// An error that occurred within the scripting environment.
    RuntimeError {
        /// A code representing what type of error occurred.
        code: RuntimeErrorCode,
        /// A string representation of the type of error.
        name: String,
        /// An optional (but usually populated) message describing the error.
        message: Option<String>,
        /// The underlying Rust-level error that caused this error.
        cause: Option<Box<Error>>,
    },
    /// A mutable callback has triggered JavaScript code that has called the same mutable callback
    /// again.
    ///
    /// This is an error because a mutable callback can only be borrowed mutably once.
    RecursiveMutCallback,
    /// A custom error that occurs during runtime.
    ///
    /// This can be used for returning user-defined errors from callbacks.
    ExternalError(Box<RuntimeError>),
    /// An error specifying the variable that was called as a function was not a function.
    NotAFunction,
}

impl Error {
    pub(crate) fn into_runtime_error_desc(self) -> RuntimeErrorDesc {
        RuntimeErrorDesc {
            code: self.runtime_code(),
            name: self.runtime_name(),
            message: self.runtime_message(),
            cause: Box::new(self),
        }
    }

    fn runtime_code(&self) -> RuntimeErrorCode {
        RuntimeErrorCode::Error
    }

    fn runtime_name(&self) -> String {
        "Error".into()
    }

    fn runtime_message(&self) -> Option<String> {
        None
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
    Eval,
    Range,
    Reference,
    Syntax,
    Type,
    Uri,
}

impl RuntimeErrorCode {
    pub(crate) fn from_duk_errcode(code: ffi::duk_errcode_t) -> RuntimeErrorCode {
        match code as u32 {
            ffi::DUK_ERR_ERROR => RuntimeErrorCode::Error,
            ffi::DUK_ERR_EVAL_ERROR => RuntimeErrorCode::Eval,
            ffi::DUK_ERR_RANGE_ERROR => RuntimeErrorCode::Range,
            ffi::DUK_ERR_REFERENCE_ERROR => RuntimeErrorCode::Reference,
            ffi::DUK_ERR_SYNTAX_ERROR => RuntimeErrorCode::Syntax,
            ffi::DUK_ERR_TYPE_ERROR => RuntimeErrorCode::Type,
            ffi::DUK_ERR_URI_ERROR => RuntimeErrorCode::Uri,
            _ => RuntimeErrorCode::Error,
        }
    }

    pub(crate) fn to_duk_errcode(&self) -> ffi::duk_errcode_t {
        (match *self {
            RuntimeErrorCode::Error => ffi::DUK_ERR_ERROR,
            RuntimeErrorCode::Eval => ffi::DUK_ERR_EVAL_ERROR,
            RuntimeErrorCode::Range => ffi::DUK_ERR_RANGE_ERROR,
            RuntimeErrorCode::Reference => ffi::DUK_ERR_REFERENCE_ERROR,
            RuntimeErrorCode::Syntax => ffi::DUK_ERR_SYNTAX_ERROR,
            RuntimeErrorCode::Type => ffi::DUK_ERR_TYPE_ERROR,
            RuntimeErrorCode::Uri => ffi::DUK_ERR_URI_ERROR,
        }) as ffi::duk_errcode_t
    }
}

impl fmt::Display for RuntimeErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeErrorCode::Error => write!(f, "Error"),
            RuntimeErrorCode::Eval => write!(f, "EvalError"),
            RuntimeErrorCode::Range => write!(f, "RangeError"),
            RuntimeErrorCode::Reference => write!(f, "ReferenceError"),
            RuntimeErrorCode::Syntax => write!(f, "SyntaxError"),
            RuntimeErrorCode::Type => write!(f, "TypeError"),
            RuntimeErrorCode::Uri => write!(f, "URIError"),
        }
    }
}

pub trait RuntimeError: fmt::Debug {
    fn code(&self) -> RuntimeErrorCode {
        RuntimeErrorCode::Error
    }

    fn name(&self) -> String {
        self.code().to_string()
    }

    fn message(&self) -> Option<String> {
        None
    }

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

impl<T: RuntimeError + 'static> From<T> for Error {
    fn from(error: T) -> Error {
        Error::ExternalError(Box::new(error))
    }
}
