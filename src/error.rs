use std::fmt;

use crate::ffi::status;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebKitError {
    InvalidArgument(String),
    Unsupported(String),
    TimedOut(String),
    FrameworkError(String),
    Unknown(String),
}

impl WebKitError {
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidArgument(message)
            | Self::Unsupported(message)
            | Self::TimedOut(message)
            | Self::FrameworkError(message)
            | Self::Unknown(message) => message,
        }
    }
}

impl fmt::Display for WebKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::Unsupported(message) => write!(f, "unsupported: {message}"),
            Self::TimedOut(message) => write!(f, "timed out: {message}"),
            Self::FrameworkError(message) => write!(f, "framework error: {message}"),
            Self::Unknown(message) => write!(f, "unknown error: {message}"),
        }
    }
}

impl std::error::Error for WebKitError {}

pub(crate) fn error_from_status(code: i32, message: String) -> WebKitError {
    match code {
        status::INVALID_ARGUMENT => WebKitError::InvalidArgument(message),
        status::UNSUPPORTED => WebKitError::Unsupported(message),
        status::TIMED_OUT => WebKitError::TimedOut(message),
        status::FRAMEWORK_ERROR => WebKitError::FrameworkError(message),
        _ => WebKitError::Unknown(message),
    }
}
