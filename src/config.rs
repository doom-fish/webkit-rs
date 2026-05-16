use core::ffi::c_void;
use core::ptr;

use crate::content_rule_list_store::ContentRuleList;
use crate::ffi;
use crate::preferences::Preferences;
use crate::private::{take_json_or_default, to_cstring, to_json_cstring};
use crate::user_script::UserScript;
use crate::website_data_store::WebsiteDataStore;

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

    #[must_use]
    pub fn application_name_for_user_agent(&self) -> String {
        unsafe { crate::private::take_string(ffi::wk_config_copy_application_name(self.0)) }
    }

    pub fn set_allows_airplay_for_media_playback(&self, value: bool) {
        unsafe { ffi::wk_config_set_allows_airplay(self.0, value) }
    }

    #[must_use]
    pub fn allows_airplay_for_media_playback(&self) -> bool {
        unsafe { ffi::wk_config_get_allows_airplay(self.0) }
    }

    pub fn set_allows_content_javascript(&self, value: bool) {
        unsafe { ffi::wk_config_set_allows_content_javascript(self.0, value) }
    }

    #[must_use]
    pub fn allows_content_javascript(&self) -> bool {
        unsafe { ffi::wk_config_get_allows_content_javascript(self.0) }
    }

    pub fn set_preferences(&self, preferences: &Preferences) {
        let preferences_json = to_json_cstring(preferences);
        unsafe { ffi::wk_config_set_preferences_json(self.0, preferences_json.as_ptr()) }
    }

    #[must_use]
    pub fn preferences(&self) -> Preferences {
        unsafe { take_json_or_default(ffi::wk_config_copy_preferences_json(self.0)) }
    }

    pub fn set_website_data_store(&self, store: &WebsiteDataStore) {
        unsafe { ffi::wk_config_set_website_data_store(self.0, store.as_ptr()) }
    }

    #[must_use]
    pub fn website_data_store(&self) -> Option<WebsiteDataStore> {
        WebsiteDataStore::from_ptr(unsafe { ffi::wk_config_copy_website_data_store(self.0) })
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
        let content_world = script.content_world.as_deref().map(to_cstring);
        let content_world_ptr = content_world.as_ref().map_or(ptr::null(), |name| name.as_ptr());
        unsafe {
            ffi::wk_config_add_user_script(
                self.0,
                source.as_ptr(),
                script.injection_time.as_raw(),
                script.main_frame_only,
                content_world_ptr,
            );
        }
    }

    pub fn remove_all_user_scripts(&self) {
        unsafe { ffi::wk_config_remove_all_user_scripts(self.0) }
    }

    pub fn add_content_rule_list(&self, rule_list: &ContentRuleList) {
        unsafe { ffi::wk_config_add_content_rule_list(self.0, rule_list.as_ptr()) }
    }

    pub fn remove_content_rule_list(&self, rule_list: &ContentRuleList) {
        unsafe { ffi::wk_config_remove_content_rule_list(self.0, rule_list.as_ptr()) }
    }

    pub fn remove_all_content_rule_lists(&self) {
        unsafe { ffi::wk_config_remove_all_content_rule_lists(self.0) }
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
