use core::ffi::c_char;

use serde::Deserialize;

use crate::private::take_json_or_default;

/// Wraps `WKBackForwardListItem`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardListItem {
    /// Mirrors the `relative_index` value exposed by `WKBackForwardListItem`.
    pub relative_index: isize,
    /// Mirrors the `url` value exposed by `WKBackForwardListItem`.
    pub url: String,
    /// Mirrors the `title` value exposed by `WKBackForwardListItem`.
    pub title: Option<String>,
    /// Mirrors the `initial_url` value exposed by `WKBackForwardListItem`.
    pub initial_url: String,
}

/// Wraps `WKBackForwardList`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BackForwardList {
    items: Vec<BackForwardListItem>,
}

impl BackForwardList {
    #[must_use]
    pub(crate) unsafe fn from_json_ptr(ptr: *mut c_char) -> Self {
        let items = take_json_or_default(ptr);
        Self { items }
    }

    /// Returns the corresponding value from `WKBackForwardList`.
    #[must_use]
    pub fn items(&self) -> &[BackForwardListItem] {
        &self.items
    }

    /// Returns the corresponding value from `WKBackForwardList`.
    #[must_use]
    pub fn current_item(&self) -> Option<&BackForwardListItem> {
        self.item_at_index(0)
    }

    /// Returns the corresponding value from `WKBackForwardList`.
    #[must_use]
    pub fn back_item(&self) -> Option<&BackForwardListItem> {
        self.item_at_index(-1)
    }

    /// Returns the corresponding value from `WKBackForwardList`.
    #[must_use]
    pub fn forward_item(&self) -> Option<&BackForwardListItem> {
        self.item_at_index(1)
    }

    /// Mirrors the corresponding `WKBackForwardList` API.
    #[must_use]
    pub fn item_at_index(&self, index: isize) -> Option<&BackForwardListItem> {
        self.items.iter().find(|item| item.relative_index == index)
    }

    /// Returns the corresponding value from `WKBackForwardList`.
    #[must_use]
    pub fn back_list(&self) -> Vec<&BackForwardListItem> {
        self.items
            .iter()
            .filter(|item| item.relative_index < 0)
            .collect()
    }

    /// Returns the corresponding value from `WKBackForwardList`.
    #[must_use]
    pub fn forward_list(&self) -> Vec<&BackForwardListItem> {
        self.items
            .iter()
            .filter(|item| item.relative_index > 0)
            .collect()
    }
}
