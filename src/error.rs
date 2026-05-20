use std::fmt;

use crate::ffi::status;

/// Mirrors the `WEBKIT_ERROR_DOMAIN` constant used by `WKErrorDomain`.
pub const WEBKIT_ERROR_DOMAIN: &str = "WKErrorDomain";

/// Wraps `WKErrorCode` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum WebKitErrorCode {
    /// Mirrors the `Unknown` case used by `WKErrorCode`.
    Unknown = 1,
    /// Mirrors the `WebContentProcessTerminated` case used by `WKErrorCode`.
    WebContentProcessTerminated = 2,
    /// Mirrors the `WebViewInvalidated` case used by `WKErrorCode`.
    WebViewInvalidated = 3,
    /// Mirrors the `JavaScriptExceptionOccurred` case used by `WKErrorCode`.
    JavaScriptExceptionOccurred = 4,
    /// Mirrors the `JavaScriptResultTypeIsUnsupported` case used by `WKErrorCode`.
    JavaScriptResultTypeIsUnsupported = 5,
    /// Mirrors the `ContentRuleListStoreCompileFailed` case used by `WKErrorCode`.
    ContentRuleListStoreCompileFailed = 6,
    /// Mirrors the `ContentRuleListStoreLookUpFailed` case used by `WKErrorCode`.
    ContentRuleListStoreLookUpFailed = 7,
    /// Mirrors the `ContentRuleListStoreRemoveFailed` case used by `WKErrorCode`.
    ContentRuleListStoreRemoveFailed = 8,
    /// Mirrors the `ContentRuleListStoreVersionMismatch` case used by `WKErrorCode`.
    ContentRuleListStoreVersionMismatch = 9,
    /// Mirrors the `AttributedStringContentFailedToLoad` case used by `WKErrorCode`.
    AttributedStringContentFailedToLoad = 10,
    /// Mirrors the `AttributedStringContentLoadTimedOut` case used by `WKErrorCode`.
    AttributedStringContentLoadTimedOut = 11,
    /// Mirrors the `JavaScriptInvalidFrameTarget` case used by `WKErrorCode`.
    JavaScriptInvalidFrameTarget = 12,
    /// Mirrors the `NavigationAppBoundDomain` case used by `WKErrorCode`.
    NavigationAppBoundDomain = 13,
    /// Mirrors the `JavaScriptAppBoundDomain` case used by `WKErrorCode`.
    JavaScriptAppBoundDomain = 14,
    /// Mirrors the `DuplicateCredential` case used by `WKErrorCode`.
    DuplicateCredential = 15,
    /// Mirrors the `MalformedCredential` case used by `WKErrorCode`.
    MalformedCredential = 16,
    /// Mirrors the `CredentialNotFound` case used by `WKErrorCode`.
    CredentialNotFound = 17,
}

impl WebKitErrorCode {
    /// Returns the corresponding value from `WKErrorCode`.
    #[must_use]
    pub const fn as_raw(self) -> i64 {
        self as i64
    }

    /// Creates a value for `WKErrorCode`.
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

    /// Returns the corresponding value from `WKErrorCode`.
    #[must_use]
    pub const fn domain() -> &'static str {
        WEBKIT_ERROR_DOMAIN
    }
}

/// Wraps `WKErrorDomain` values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebKitError {
    /// Mirrors the `InvalidArgument` case used by `WKErrorDomain`.
    InvalidArgument(String),
    /// Mirrors the `Unsupported` case used by `WKErrorDomain`.
    Unsupported(String),
    /// Mirrors the `TimedOut` case used by `WKErrorDomain`.
    TimedOut(String),
    /// Mirrors the `FrameworkError` case used by `WKErrorDomain`.
    FrameworkError(String),
    /// Mirrors the `Unknown` case used by `WKErrorDomain`.
    Unknown(String),
}

impl WebKitError {
    /// Returns the corresponding value from `WKErrorDomain`.
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

#[cfg(test)]
mod tests {
    use super::{
        error_from_status, status_from_error, WebKitError, WebKitErrorCode, WEBKIT_ERROR_DOMAIN,
    };
    use crate::ffi::status;

    #[test]
    fn error_codes_round_trip_and_share_the_webkit_domain() {
        assert_eq!(WebKitErrorCode::Unknown.as_raw(), 1);
        assert_eq!(WebKitErrorCode::from_raw(1), Some(WebKitErrorCode::Unknown));
        assert_eq!(WebKitErrorCode::from_raw(17), Some(WebKitErrorCode::CredentialNotFound));
        assert_eq!(WebKitErrorCode::from_raw(99), None);
        assert_eq!(WebKitErrorCode::domain(), WEBKIT_ERROR_DOMAIN);
    }

    #[test]
    fn error_status_mapping_and_display_messages_match() {
        let invalid = error_from_status(status::INVALID_ARGUMENT, "bad url".to_owned());
        let unsupported = error_from_status(status::UNSUPPORTED, "feature".to_owned());
        let timed_out = error_from_status(status::TIMED_OUT, "search".to_owned());
        let framework = error_from_status(status::FRAMEWORK_ERROR, "bridge".to_owned());
        let unknown = error_from_status(42, "mystery".to_owned());

        assert_eq!(invalid.to_string(), "invalid argument: bad url");
        assert_eq!(unsupported.to_string(), "unsupported: feature");
        assert_eq!(timed_out.to_string(), "timed out: search");
        assert_eq!(framework.to_string(), "framework error: bridge");
        assert_eq!(unknown.to_string(), "unknown error: mystery");
        assert_eq!(status_from_error(&invalid), status::INVALID_ARGUMENT);
        assert_eq!(status_from_error(&unsupported), status::UNSUPPORTED);
        assert_eq!(status_from_error(&timed_out), status::TIMED_OUT);
        assert_eq!(status_from_error(&framework), status::FRAMEWORK_ERROR);
        assert_eq!(status_from_error(&unknown), status::UNKNOWN);
    }

    #[test]
    fn message_accessor_returns_payload_and_error_source_is_none() {
        let error = WebKitError::FrameworkError("bridge failure".to_owned());
        let as_std_error: &dyn std::error::Error = &error;

        assert_eq!(error.message(), "bridge failure");
        assert!(as_std_error.source().is_none());
    }
}
