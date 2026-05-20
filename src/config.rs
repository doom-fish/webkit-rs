use core::ffi::c_void;
use core::ptr;

use std::ops::{BitOr, BitOrAssign};

use crate::content_rule_list_store::ContentRuleList;
use crate::error::WebKitError;
use crate::ffi;
use crate::preferences::Preferences;
use crate::private::{maybe_take_error, take_json_or_default, to_cstring, to_json_cstring};
use crate::user_script::UserScript;
use crate::website_data_store::WebsiteDataStore;

/// Wraps `WKUserInterfaceDirectionPolicy` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum UserInterfaceDirectionPolicy {
    /// Mirrors the `Content` case used by `WKUserInterfaceDirectionPolicy`.
    Content = 0,
    /// Mirrors the `System` case used by `WKUserInterfaceDirectionPolicy`.
    System = 1,
}

impl UserInterfaceDirectionPolicy {
    /// Returns the corresponding value from `WKUserInterfaceDirectionPolicy`.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Creates a value for `WKUserInterfaceDirectionPolicy`.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::System,
            _ => Self::Content,
        }
    }
}

/// Wraps `WKAudiovisualMediaTypes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AudiovisualMediaTypes(u64);

impl AudiovisualMediaTypes {
    /// Mirrors the `NONE` constant used by `WKAudiovisualMediaTypes`.
    pub const NONE: Self = Self(0);
    /// Mirrors the `AUDIO` constant used by `WKAudiovisualMediaTypes`.
    pub const AUDIO: Self = Self(1 << 0);
    /// Mirrors the `VIDEO` constant used by `WKAudiovisualMediaTypes`.
    pub const VIDEO: Self = Self(1 << 1);
    /// Mirrors the `ALL` constant used by `WKAudiovisualMediaTypes`.
    pub const ALL: Self = Self(u64::MAX);

    /// Creates a value for `WKAudiovisualMediaTypes`.
    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns the corresponding value from `WKAudiovisualMediaTypes`.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Returns whether this `WKAudiovisualMediaTypes` value contains the provided flags.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for AudiovisualMediaTypes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for AudiovisualMediaTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

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
    /// Creates a value for `WKWebViewConfiguration`.
    #[must_use]
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wk_config_new() };
        Self(ptr)
    }

    #[must_use]
    pub(crate) fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    #[must_use]
    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.0
    }

    /// Sets the corresponding value on `WKWebViewConfiguration`.
    pub fn set_application_name_for_user_agent(&self, name: &str) {
        let c_name = to_cstring(name);
        unsafe { ffi::wk_config_set_application_name(self.0, c_name.as_ptr()) }
    }

    /// Returns the corresponding value from `WKWebViewConfiguration`.
    #[must_use]
    pub fn application_name_for_user_agent(&self) -> String {
        unsafe { crate::private::take_string(ffi::wk_config_copy_application_name(self.0)) }
    }

    /// Sets the corresponding value on `WKWebViewConfiguration`.
    pub fn set_allows_airplay_for_media_playback(&self, value: bool) {
        unsafe { ffi::wk_config_set_allows_airplay(self.0, value) }
    }

    /// Returns the corresponding value from `WKWebViewConfiguration`.
    #[must_use]
    pub fn allows_airplay_for_media_playback(&self) -> bool {
        unsafe { ffi::wk_config_get_allows_airplay(self.0) }
    }

    /// Sets whether the System Screen Time blocking view should be shown.
    pub fn set_shows_system_screen_time_blocking_view(
        &self,
        value: bool,
    ) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_config_set_shows_system_screen_time_blocking_view(self.0, value, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Returns whether the System Screen Time blocking view should be shown.
    pub fn shows_system_screen_time_blocking_view(&self) -> Result<bool, WebKitError> {
        let mut out_value = false;
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_config_get_shows_system_screen_time_blocking_view(
                self.0,
                &mut out_value,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(out_value)
    }

    /// Sets the corresponding value on `WKWebViewConfiguration`.
    pub fn set_media_types_requiring_user_action_for_playback(
        &self,
        media_types: AudiovisualMediaTypes,
    ) {
        unsafe {
            ffi::wk_config_set_media_types_requiring_user_action_for_playback(
                self.0,
                media_types.bits(),
            );
        }
    }

    /// Returns the corresponding value from `WKWebViewConfiguration`.
    #[must_use]
    pub fn media_types_requiring_user_action_for_playback(&self) -> AudiovisualMediaTypes {
        AudiovisualMediaTypes::from_bits(unsafe {
            ffi::wk_config_get_media_types_requiring_user_action_for_playback(self.0)
        })
    }

    /// Sets the corresponding value on `WKWebViewConfiguration`.
    pub fn set_user_interface_direction_policy(&self, policy: UserInterfaceDirectionPolicy) {
        unsafe { ffi::wk_config_set_user_interface_direction_policy(self.0, policy.as_raw()) }
    }

    /// Returns the corresponding value from `WKWebViewConfiguration`.
    #[must_use]
    pub fn user_interface_direction_policy(&self) -> UserInterfaceDirectionPolicy {
        UserInterfaceDirectionPolicy::from_raw(unsafe {
            ffi::wk_config_get_user_interface_direction_policy(self.0)
        })
    }

    /// Sets the corresponding value on `WKWebViewConfiguration`.
    pub fn set_allows_content_javascript(&self, value: bool) {
        unsafe { ffi::wk_config_set_allows_content_javascript(self.0, value) }
    }

    /// Returns the corresponding value from `WKWebViewConfiguration`.
    #[must_use]
    pub fn allows_content_javascript(&self) -> bool {
        unsafe { ffi::wk_config_get_allows_content_javascript(self.0) }
    }

    /// Sets the corresponding value on `WKWebViewConfiguration`.
    pub fn set_preferences(&self, preferences: &Preferences) {
        let preferences_json = to_json_cstring(preferences);
        unsafe { ffi::wk_config_set_preferences_json(self.0, preferences_json.as_ptr()) }
    }

    /// Mirrors the corresponding `WKWebViewConfiguration` API.
    #[must_use]
    pub fn preferences(&self) -> Preferences {
        unsafe { take_json_or_default(ffi::wk_config_copy_preferences_json(self.0)) }
    }

    /// Sets the corresponding value on `WKWebViewConfiguration`.
    pub fn set_website_data_store(&self, store: &WebsiteDataStore) {
        unsafe { ffi::wk_config_set_website_data_store(self.0, store.as_ptr()) }
    }

    /// Mirrors the corresponding `WKWebViewConfiguration` API.
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
        let content_world_ptr = content_world
            .as_ref()
            .map_or(ptr::null(), |name| name.as_ptr());
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

    /// Calls the corresponding `WKWebViewConfiguration` API.
    pub fn remove_all_user_scripts(&self) {
        unsafe { ffi::wk_config_remove_all_user_scripts(self.0) }
    }

    /// Calls the corresponding `WKWebViewConfiguration` API.
    pub fn add_content_rule_list(&self, rule_list: &ContentRuleList) {
        unsafe { ffi::wk_config_add_content_rule_list(self.0, rule_list.as_ptr()) }
    }

    /// Calls the corresponding `WKWebViewConfiguration` API.
    pub fn remove_content_rule_list(&self, rule_list: &ContentRuleList) {
        unsafe { ffi::wk_config_remove_content_rule_list(self.0, rule_list.as_ptr()) }
    }

    /// Calls the corresponding `WKWebViewConfiguration` API.
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

    /// Register a reply-capable message handler name so JavaScript can await the
    /// Promise returned by `postMessage(...)`.
    pub fn add_message_handler_with_reply(&self, name: &str) {
        let c_name = to_cstring(name);
        unsafe { ffi::wk_config_add_message_handler_with_reply_name(self.0, c_name.as_ptr()) }
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

#[cfg(test)]
mod tests {
    use super::{AudiovisualMediaTypes, UserInterfaceDirectionPolicy};

    #[test]
    fn user_interface_direction_policy_round_trips_raw_values() {
        assert_eq!(UserInterfaceDirectionPolicy::Content.as_raw(), 0);
        assert_eq!(
            UserInterfaceDirectionPolicy::from_raw(0),
            UserInterfaceDirectionPolicy::Content
        );
        assert_eq!(UserInterfaceDirectionPolicy::System.as_raw(), 1);
        assert_eq!(
            UserInterfaceDirectionPolicy::from_raw(1),
            UserInterfaceDirectionPolicy::System
        );
        assert_eq!(
            UserInterfaceDirectionPolicy::from_raw(99),
            UserInterfaceDirectionPolicy::Content
        );
    }

    #[test]
    fn audiovisual_media_types_bitflags_combine_and_contain() {
        let mut media_types = AudiovisualMediaTypes::AUDIO;
        media_types |= AudiovisualMediaTypes::VIDEO;

        assert!(media_types.contains(AudiovisualMediaTypes::AUDIO));
        assert!(media_types.contains(AudiovisualMediaTypes::VIDEO));
        assert_eq!(
            media_types.bits(),
            AudiovisualMediaTypes::AUDIO.bits() | AudiovisualMediaTypes::VIDEO.bits()
        );
        assert!(AudiovisualMediaTypes::ALL.contains(media_types));
    }

    #[test]
    fn audiovisual_media_types_default_is_empty() {
        assert_eq!(
            AudiovisualMediaTypes::default(),
            AudiovisualMediaTypes::NONE
        );
        assert_eq!(
            AudiovisualMediaTypes::from_bits(0),
            AudiovisualMediaTypes::NONE
        );
    }
}
