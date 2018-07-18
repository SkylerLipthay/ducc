use ducc::Error as DuccError;
use serde;
use std::fmt;
use std::error::Error as StdError;
use std::result::Result as StdResult;

#[derive(Debug)]
pub struct Error(DuccError);

pub type Result<T> = StdResult<T, Error>;

impl From<DuccError> for Error {
    fn from(err: DuccError) -> Error {
        Error(err)
    }
}

impl From<Error> for DuccError {
    fn from(err: Error) -> DuccError {
        err.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

impl StdError for Error {
    fn description(&self) -> &'static str {
        "failed to serialize to Ducc value"
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(_msg: T) -> Self {
        Error(DuccError::to_js_conversion("serde", "value"))
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(_msg: T) -> Self {
        Error(DuccError::to_js_conversion("value", "serde"))
    }
}
