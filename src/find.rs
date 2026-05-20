use serde::{Deserialize, Serialize};

/// Wraps `WKWebView` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TextFinderAction {
    /// Mirrors the `ShowFindInterface` case used by `WKWebView`.
    ShowFindInterface = 1,
    /// Mirrors the `NextMatch` case used by `WKWebView`.
    NextMatch = 2,
    /// Mirrors the `PreviousMatch` case used by `WKWebView`.
    PreviousMatch = 3,
    /// Mirrors the `ReplaceAll` case used by `WKWebView`.
    ReplaceAll = 4,
    /// Mirrors the `Replace` case used by `WKWebView`.
    Replace = 5,
    /// Mirrors the `ReplaceAndFind` case used by `WKWebView`.
    ReplaceAndFind = 6,
    /// Mirrors the `SetSearchString` case used by `WKWebView`.
    SetSearchString = 7,
    /// Mirrors the `ReplaceAllInSelection` case used by `WKWebView`.
    ReplaceAllInSelection = 8,
    /// Mirrors the `SelectAll` case used by `WKWebView`.
    SelectAll = 9,
    /// Mirrors the `SelectAllInSelection` case used by `WKWebView`.
    SelectAllInSelection = 10,
    /// Mirrors the `HideFindInterface` case used by `WKWebView`.
    HideFindInterface = 11,
    /// Mirrors the `ShowReplaceInterface` case used by `WKWebView`.
    ShowReplaceInterface = 12,
    /// Mirrors the `HideReplaceInterface` case used by `WKWebView`.
    HideReplaceInterface = 13,
}

impl TextFinderAction {
    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }
}

/// Configuration for `WKWebView.findString(_:withConfiguration:)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindConfiguration {
    /// Mirrors the `backwards` value exposed by `WKFindConfiguration`.
    pub backwards: bool,
    /// Mirrors the `case_sensitive` value exposed by `WKFindConfiguration`.
    pub case_sensitive: bool,
    /// Mirrors the `wraps` value exposed by `WKFindConfiguration`.
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
    /// Creates a value for `WKFindConfiguration`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the corresponding option used by `WKFindConfiguration`.
    #[must_use]
    pub const fn with_backwards(mut self, backwards: bool) -> Self {
        self.backwards = backwards;
        self
    }

    /// Sets the corresponding option used by `WKFindConfiguration`.
    #[must_use]
    pub const fn with_case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    /// Sets the corresponding option used by `WKFindConfiguration`.
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
    /// Mirrors the `match_found` value exposed by `WKFindResult`.
    pub match_found: bool,
}

impl FindResult {
    /// Returns the corresponding value from `WKFindResult`.
    #[must_use]
    pub const fn found(self) -> bool {
        self.match_found
    }
}

#[cfg(test)]
mod tests {
    use super::{FindConfiguration, FindResult, TextFinderAction};

    #[test]
    fn text_finder_action_raw_values_are_stable() {
        let cases = [
            (TextFinderAction::ShowFindInterface, 1),
            (TextFinderAction::ReplaceAll, 4),
            (TextFinderAction::ReplaceAndFind, 6),
            (TextFinderAction::HideReplaceInterface, 13),
        ];

        for (action, raw) in cases {
            assert_eq!(action.as_raw(), raw);
        }
    }

    #[test]
    fn find_configuration_default_and_builder_methods_work_together() {
        let configuration = FindConfiguration::new()
            .with_backwards(true)
            .with_case_sensitive(true)
            .with_wraps(false);

        assert_eq!(FindConfiguration::default(), FindConfiguration { backwards: false, case_sensitive: false, wraps: true });
        assert!(configuration.backwards);
        assert!(configuration.case_sensitive);
        assert!(!configuration.wraps);
    }

    #[test]
    fn find_result_found_reflects_match_flag() {
        assert!(FindResult { match_found: true }.found());
        assert!(!FindResult { match_found: false }.found());
    }
}
