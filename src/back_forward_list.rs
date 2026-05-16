use core::ffi::c_char;

use serde::Deserialize;

use crate::private::take_json_or_default;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackForwardListItem {
    pub relative_index: isize,
    pub url: String,
    pub title: Option<String>,
    pub initial_url: String,
}

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

    #[must_use]
    pub fn items(&self) -> &[BackForwardListItem] {
        &self.items
    }

    #[must_use]
    pub fn current_item(&self) -> Option<&BackForwardListItem> {
        self.item_at_index(0)
    }

    #[must_use]
    pub fn back_item(&self) -> Option<&BackForwardListItem> {
        self.item_at_index(-1)
    }

    #[must_use]
    pub fn forward_item(&self) -> Option<&BackForwardListItem> {
        self.item_at_index(1)
    }

    #[must_use]
    pub fn item_at_index(&self, index: isize) -> Option<&BackForwardListItem> {
        self.items.iter().find(|item| item.relative_index == index)
    }

    #[must_use]
    pub fn back_list(&self) -> Vec<&BackForwardListItem> {
        self.items
            .iter()
            .filter(|item| item.relative_index < 0)
            .collect()
    }

    #[must_use]
    pub fn forward_list(&self) -> Vec<&BackForwardListItem> {
        self.items
            .iter()
            .filter(|item| item.relative_index > 0)
            .collect()
    }
}
