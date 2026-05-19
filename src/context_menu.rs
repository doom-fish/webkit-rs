use core::ffi::c_void;
use std::ptr::NonNull;

use crate::ffi;
use crate::private::{take_optional_string, take_string};

/// Wraps `WKContextMenuElementInfo`.
pub struct ContextMenuElementInfo(NonNull<c_void>);

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for ContextMenuElementInfo {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for ContextMenuElementInfo {}

impl ContextMenuElementInfo {
    /// Wraps a retained `WKContextMenuElementInfo *`.
    ///
    /// # Safety
    /// `ptr` must be either null or a live retained Objective-C object compatible with
    /// `WKContextMenuElementInfo`. The returned wrapper takes ownership of that retain
    /// and releases it on drop.
    #[must_use]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Returns the raw retained `WKContextMenuElementInfo *`.
    #[must_use]
    pub const fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Returns the corresponding value from `WKContextMenuElementInfo`.
    #[must_use]
    pub fn link_url(&self) -> Option<String> {
        unsafe {
            take_optional_string(ffi::wk_context_menu_element_info_copy_link_url(
                self.as_raw(),
            ))
        }
    }
}

impl core::fmt::Debug for ContextMenuElementInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ContextMenuElementInfo")
            .field("link_url", &self.link_url())
            .finish()
    }
}

impl Drop for ContextMenuElementInfo {
    fn drop(&mut self) {
        unsafe { ffi::wk_context_menu_element_info_release(self.as_raw()) }
    }
}

/// Wraps `WKPreviewElementInfo`.
pub struct PreviewElementInfo(NonNull<c_void>);

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for PreviewElementInfo {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for PreviewElementInfo {}

impl PreviewElementInfo {
    /// Wraps a retained `WKPreviewElementInfo *`.
    ///
    /// # Safety
    /// `ptr` must be either null or a live retained Objective-C object compatible with
    /// `WKPreviewElementInfo`. The returned wrapper takes ownership of that retain and
    /// releases it on drop.
    #[must_use]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Returns the raw retained `WKPreviewElementInfo *`.
    #[must_use]
    pub const fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Returns the corresponding value from `WKPreviewElementInfo`.
    #[must_use]
    pub fn link_url(&self) -> Option<String> {
        unsafe { take_optional_string(ffi::wk_preview_element_info_copy_link_url(self.as_raw())) }
    }
}

impl core::fmt::Debug for PreviewElementInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PreviewElementInfo")
            .field("link_url", &self.link_url())
            .finish()
    }
}

impl Drop for PreviewElementInfo {
    fn drop(&mut self) {
        unsafe { ffi::wk_preview_element_info_release(self.as_raw()) }
    }
}

/// Wraps `WKPreviewActionItem`.
pub struct PreviewActionItem(NonNull<c_void>);

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for PreviewActionItem {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for PreviewActionItem {}

impl PreviewActionItem {
    /// Wraps a retained `WKPreviewActionItem`-compatible Objective-C object pointer.
    ///
    /// # Safety
    /// `ptr` must be either null or a live retained Objective-C object compatible with
    /// `WKPreviewActionItem`. The returned wrapper takes ownership of that retain and
    /// releases it on drop.
    #[must_use]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Returns the raw retained `WKPreviewActionItem`-compatible object pointer.
    #[must_use]
    pub const fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Returns the corresponding value from `WKPreviewActionItem`.
    #[must_use]
    pub fn identifier(&self) -> String {
        unsafe { take_string(ffi::wk_preview_action_item_copy_identifier(self.as_raw())) }
    }

    /// Returns the corresponding value from `WKPreviewActionItem`.
    #[must_use]
    pub fn title(&self) -> String {
        unsafe { take_string(ffi::wk_preview_action_item_copy_title(self.as_raw())) }
    }
}

impl core::fmt::Debug for PreviewActionItem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PreviewActionItem")
            .field("identifier", &self.identifier())
            .field("title", &self.title())
            .finish()
    }
}

impl Drop for PreviewActionItem {
    fn drop(&mut self) {
        unsafe { ffi::wk_preview_action_item_release(self.as_raw()) }
    }
}
