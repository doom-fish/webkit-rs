use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextFinderAction {
    ShowFindInterface = 1,
    NextMatch = 2,
    PreviousMatch = 3,
    ReplaceAll = 4,
    Replace = 5,
    ReplaceAndFind = 6,
    SetSearchString = 7,
    ReplaceAllInSelection = 8,
    SelectAll = 9,
    SelectAllInSelection = 10,
    HideFindInterface = 11,
    ShowReplaceInterface = 12,
    HideReplaceInterface = 13,
}

impl TextFinderAction {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }
}

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

    #[must_use]
    pub const fn with_backwards(mut self, backwards: bool) -> Self {
        self.backwards = backwards;
        self
    }

    #[must_use]
    pub const fn with_case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    #[must_use]
    pub const fn with_wraps(mut self, wraps: bool) -> Self {
        self.wraps = wraps;
        self
    }
}

/// Result returned by a find-in-page operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FindResult {
    pub match_found: bool,
}

impl FindResult {
    #[must_use]
    pub const fn found(self) -> bool {
        self.match_found
    }
}
