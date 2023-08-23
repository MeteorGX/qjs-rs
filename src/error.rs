use std::ffi::NulError;
use std::fmt;
use std::fmt::Formatter;


#[derive(Debug)]
pub enum JSError {
    CreateRuntime,
    CreateContext,
    CStringError(NulError),
    UnexpectedType,
    Internal(String),
    InvalidUtf8String(std::str::Utf8Error),
    JSException(String),
    PropertyError(String),
    Other(String),
}


impl fmt::Display for JSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateRuntime => write!(f, "could not create runtime"),
            Self::CreateContext => write!(f, "could not create context"),
            Self::CStringError(e) => write!(f, "failed by convert cstring:{:?}", e),
            Self::UnexpectedType => write!(f, "could not convert: received unexpected type"),
            Self::Internal(e) => write!(f, "unhandled JS_TAG value: {}", e),
            Self::InvalidUtf8String(e) => write!(f, "value conversion failed - invalid non-utf8 string: {}", e),
            Self::JSException(e) => write!(f, "JSRuntime exception: {}", e),
            Self::PropertyError(e) => write!(f, "JSContext property: {}", e),
            Self::Other(msg) => write!(f, "{}", msg),
        }
    }
}


impl std::error::Error for JSError {}

impl TryFrom<NulError> for JSError {
    type Error = Self;
    fn try_from(value: NulError) -> Result<Self, Self::Error> {
        Ok(Self::CStringError(value))
    }
}

pub type JSResult<T> = Result<T, JSError>;
