use core::ffi::c_void;
use core::ptr;
use std::path::Path;

use crate::error::WebKitError;
use crate::ffi;
use crate::private::{maybe_take_error, take_json_or_default, take_string, to_cstring};

/// Wraps `WKContentRuleList`.
pub struct ContentRuleList {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for ContentRuleList {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for ContentRuleList {}

impl ContentRuleList {
    #[must_use]
    pub(crate) fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Returns the corresponding value from `WKContentRuleList`.
    #[must_use]
    pub fn identifier(&self) -> String {
        unsafe { take_string(ffi::wk_content_rule_list_copy_identifier(self.ptr)) }
    }
}

impl Drop for ContentRuleList {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_content_rule_list_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}

/// Wraps `WKContentRuleListStore`.
pub struct ContentRuleListStore {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for ContentRuleListStore {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for ContentRuleListStore {}

impl Default for ContentRuleListStore {
    fn default() -> Self {
        Self::default_store()
    }
}

impl ContentRuleListStore {
    #[cfg(feature = "async")]
    #[must_use]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Create the default `WKContentRuleListStore`.
    ///
    /// # Panics
    /// Panics if the Swift bridge fails to construct the store.
    #[must_use]
    pub fn default_store() -> Self {
        let ptr = unsafe { ffi::wk_content_rule_list_store_default() };
        assert!(
            !ptr.is_null(),
            "wk_content_rule_list_store_default returned null"
        );
        Self { ptr }
    }

    /// Create a content-rule-list store rooted at the given path.
    ///
    /// # Panics
    /// Panics if the Swift bridge fails to construct the store.
    #[must_use]
    pub fn with_path(path: impl AsRef<Path>) -> Self {
        let c_path = to_cstring(&path.as_ref().to_string_lossy());
        let ptr = unsafe { ffi::wk_content_rule_list_store_with_path(c_path.as_ptr()) };
        assert!(
            !ptr.is_null(),
            "wk_content_rule_list_store_with_path returned null"
        );
        Self { ptr }
    }

    /// Calls the corresponding `WKContentRuleListStore` API.
    pub fn compile(
        &self,
        identifier: &str,
        encoded_rule_list: &str,
    ) -> Result<ContentRuleList, WebKitError> {
        let c_identifier = to_cstring(identifier);
        let c_rule_list = to_cstring(encoded_rule_list);
        let mut out_rule_list = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_content_rule_list_store_compile(
                self.ptr,
                c_identifier.as_ptr(),
                c_rule_list.as_ptr(),
                &mut out_rule_list,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        ContentRuleList::from_ptr(out_rule_list).ok_or_else(|| {
            WebKitError::FrameworkError("compile returned no content rule list".to_owned())
        })
    }

    /// Calls the corresponding `WKContentRuleListStore` API.
    pub fn lookup(&self, identifier: &str) -> Result<Option<ContentRuleList>, WebKitError> {
        let c_identifier = to_cstring(identifier);
        let mut out_rule_list = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_content_rule_list_store_lookup(
                self.ptr,
                c_identifier.as_ptr(),
                &mut out_rule_list,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(ContentRuleList::from_ptr(out_rule_list))
    }

    /// Mirrors the corresponding `WKContentRuleListStore` API.
    pub fn remove(&self, identifier: &str) -> Result<(), WebKitError> {
        let c_identifier = to_cstring(identifier);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_content_rule_list_store_remove(self.ptr, c_identifier.as_ptr(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Returns the corresponding value from `WKContentRuleListStore`.
    pub fn available_identifiers(&self) -> Result<Vec<String>, WebKitError> {
        let mut out_json = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_content_rule_list_store_copy_available_identifiers_json(
                self.ptr,
                &mut out_json,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_json_or_default(out_json) })
    }
}

impl Drop for ContentRuleListStore {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_content_rule_list_store_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}
