use serde::{Deserialize, Serialize};

/// Configuration for `WKWebView.findString(_:withConfiguration:)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindConfiguration {
    pub backwards: bool,
    pub case_sensitive: bool,
    pub wraps: bool,
}

impl Default for FindConfiguration {
    fn default() -> Self {
        Self {
            backwards: false,
            case_sensitive: false,
            wraps: true,
        }
    }
}

impl FindConfiguration {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Result returned by a find-in-page operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FindResult {
    pub match_found: bool,
}
