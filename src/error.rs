use std::fmt;

use crate::ffi::status;

pub const WEBKIT_ERROR_DOMAIN: &str = "WKErrorDomain";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum WebKitErrorCode {
    Unknown = 1,
    WebContentProcessTerminated = 2,
    WebViewInvalidated = 3,
    JavaScriptExceptionOccurred = 4,
    JavaScriptResultTypeIsUnsupported = 5,
    ContentRuleListStoreCompileFailed = 6,
    ContentRuleListStoreLookUpFailed = 7,
    ContentRuleListStoreRemoveFailed = 8,
    ContentRuleListStoreVersionMismatch = 9,
    AttributedStringContentFailedToLoad = 10,
    AttributedStringContentLoadTimedOut = 11,
    JavaScriptInvalidFrameTarget = 12,
    NavigationAppBoundDomain = 13,
    JavaScriptAppBoundDomain = 14,
    DuplicateCredential = 15,
    MalformedCredential = 16,
    CredentialNotFound = 17,
}

impl WebKitErrorCode {
    #[must_use]
    pub const fn as_raw(self) -> i64 {
        self as i64
    }

    #[must_use]
    pub const fn from_raw(raw: i64) -> Option<Self> {
        match raw {
            1 => Some(Self::Unknown),
            2 => Some(Self::WebContentProcessTerminated),
            3 => Some(Self::WebViewInvalidated),
            4 => Some(Self::JavaScriptExceptionOccurred),
            5 => Some(Self::JavaScriptResultTypeIsUnsupported),
            6 => Some(Self::ContentRuleListStoreCompileFailed),
            7 => Some(Self::ContentRuleListStoreLookUpFailed),
            8 => Some(Self::ContentRuleListStoreRemoveFailed),
            9 => Some(Self::ContentRuleListStoreVersionMismatch),
            10 => Some(Self::AttributedStringContentFailedToLoad),
            11 => Some(Self::AttributedStringContentLoadTimedOut),
            12 => Some(Self::JavaScriptInvalidFrameTarget),
            13 => Some(Self::NavigationAppBoundDomain),
            14 => Some(Self::JavaScriptAppBoundDomain),
            15 => Some(Self::DuplicateCredential),
            16 => Some(Self::MalformedCredential),
            17 => Some(Self::CredentialNotFound),
            _ => None,
        }
    }

    #[must_use]
    pub const fn domain() -> &'static str {
        WEBKIT_ERROR_DOMAIN
    }
}

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

pub(crate) const fn status_from_error(error: &WebKitError) -> i32 {
    match error {
        WebKitError::InvalidArgument(_) => status::INVALID_ARGUMENT,
        WebKitError::Unsupported(_) => status::UNSUPPORTED,
        WebKitError::TimedOut(_) => status::TIMED_OUT,
        WebKitError::FrameworkError(_) => status::FRAMEWORK_ERROR,
        WebKitError::Unknown(_) => status::UNKNOWN,
    }
}
