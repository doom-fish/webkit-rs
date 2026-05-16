use core::ffi::c_void;
use core::ptr;

use crate::ffi;
use crate::private::to_cstring;
use crate::user_script::UserScript;

/// Wrapper around `WKWebViewConfiguration`.
pub struct WebViewConfiguration(*mut c_void);

// SAFETY: The raw pointer is an opaque Swift object managed via retain/release.
unsafe impl Send for WebViewConfiguration {}
// SAFETY: The bridge serialises access through WebKit's main-thread APIs.
unsafe impl Sync for WebViewConfiguration {}

impl Default for WebViewConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl WebViewConfiguration {
    #[must_use]
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wk_config_new() };
        Self(ptr)
    }

    #[must_use]
    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.0
    }

    pub fn set_application_name_for_user_agent(&self, name: &str) {
        let c_name = to_cstring(name);
        unsafe { ffi::wk_config_set_application_name(self.0, c_name.as_ptr()) }
    }

    pub fn set_allows_airplay_for_media_playback(&self, value: bool) {
        unsafe { ffi::wk_config_set_allows_airplay(self.0, value) }
    }

    pub fn set_allows_content_javascript(&self, value: bool) {
        unsafe { ffi::wk_config_set_allows_content_javascript(self.0, value) }
    }

    /// Use a non-persistent (ephemeral) website data store for this
    /// configuration.
    pub fn use_nonpersistent_data_store(&self) {
        unsafe { ffi::wk_config_use_nonpersistent_data_store(self.0) }
    }

    /// Inject a user script into all pages loaded by web views using this
    /// configuration.
    pub fn add_user_script(&self, script: &UserScript) {
        let source = to_cstring(&script.source);
        unsafe {
            ffi::wk_config_add_user_script(
                self.0,
                source.as_ptr(),
                script.injection_time.as_raw(),
                script.main_frame_only,
            );
        }
    }

    /// Register a message handler name so JavaScript can call
    /// `window.webkit.messageHandlers.<name>.postMessage(...)` and Rust can
    /// receive the message via [`crate::webview::WebView::set_message_handler`].
    pub fn add_message_handler(&self, name: &str) {
        let c_name = to_cstring(name);
        unsafe { ffi::wk_config_add_message_handler_name(self.0, c_name.as_ptr()) }
    }
}

impl Drop for WebViewConfiguration {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { ffi::wk_config_release(self.0) }
            self.0 = ptr::null_mut();
        }
    }
}
