#![allow(clippy::missing_panics_doc, clippy::struct_excessive_bools)]

use core::ffi::c_void;
use core::ptr;
use std::collections::BTreeMap;
use std::ops::{BitOr, BitOrAssign};
use std::path::Path;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::WebViewConfiguration;
use crate::error::WebKitError;
use crate::ffi;
use crate::geometry::Rect;
use crate::private::{maybe_take_error, take_json_or_default, take_optional_string, to_cstring, to_json_cstring};
use crate::website_data_store::WebsiteDataStore;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebExtensionConstantsJson {
    web_extension_error_domain: String,
    web_extension_context_error_domain: String,
    web_extension_data_record_error_domain: String,
    web_extension_match_pattern_error_domain: String,
    web_extension_message_port_error_domain: String,
    errors_did_update_notification: String,
    permissions_were_granted_notification: String,
    permissions_were_denied_notification: String,
    granted_permissions_were_removed_notification: String,
    denied_permissions_were_removed_notification: String,
    permission_match_patterns_were_granted_notification: String,
    permission_match_patterns_were_denied_notification: String,
    granted_permission_match_patterns_were_removed_notification: String,
    denied_permission_match_patterns_were_removed_notification: String,
    notification_user_info_key_permissions: String,
    notification_user_info_key_match_patterns: String,
    permission_active_tab: String,
    permission_alarms: String,
    permission_clipboard_write: String,
    permission_context_menus: String,
    permission_cookies: String,
    permission_declarative_net_request: String,
    permission_declarative_net_request_feedback: String,
    permission_declarative_net_request_with_host_access: String,
    permission_menus: String,
    permission_native_messaging: String,
    permission_scripting: String,
    permission_storage: String,
    permission_tabs: String,
    permission_unlimited_storage: String,
    permission_web_navigation: String,
    permission_web_request: String,
    data_type_local: String,
    data_type_session: String,
    data_type_synchronized: String,
}

static WEB_EXTENSION_CONSTANTS: OnceLock<WebExtensionConstantsJson> = OnceLock::new();

fn web_extension_constants() -> &'static WebExtensionConstantsJson {
    WEB_EXTENSION_CONSTANTS.get_or_init(|| unsafe {
        take_json_or_default(ffi::wk_web_extension_copy_constants_json())
    })
}

macro_rules! string_backed_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            #[must_use]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    };
}

macro_rules! bitflag_type {
    ($name:ident, $inner:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
        #[serde(transparent)]
        pub struct $name($inner);

        impl $name {
            #[must_use]
            pub const fn from_bits(bits: $inner) -> Self {
                Self(bits)
            }

            #[must_use]
            pub const fn bits(self) -> $inner {
                self.0
            }

            #[must_use]
            pub const fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }
        }

        impl BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
    };
}

string_backed_type!(WebExtensionPermission);
string_backed_type!(WebExtensionDataType);
string_backed_type!(WebExtensionContextNotificationUserInfoKey);

impl WebExtensionPermission {
    #[must_use]
    pub fn active_tab() -> Self {
        Self(web_extension_constants().permission_active_tab.clone())
    }
    #[must_use]
    pub fn alarms() -> Self {
        Self(web_extension_constants().permission_alarms.clone())
    }
    #[must_use]
    pub fn clipboard_write() -> Self {
        Self(web_extension_constants().permission_clipboard_write.clone())
    }
    #[must_use]
    pub fn context_menus() -> Self {
        Self(web_extension_constants().permission_context_menus.clone())
    }
    #[must_use]
    pub fn cookies() -> Self {
        Self(web_extension_constants().permission_cookies.clone())
    }
    #[must_use]
    pub fn declarative_net_request() -> Self {
        Self(web_extension_constants().permission_declarative_net_request.clone())
    }
    #[must_use]
    pub fn declarative_net_request_feedback() -> Self {
        Self(web_extension_constants().permission_declarative_net_request_feedback.clone())
    }
    #[must_use]
    pub fn declarative_net_request_with_host_access() -> Self {
        Self(web_extension_constants().permission_declarative_net_request_with_host_access.clone())
    }
    #[must_use]
    pub fn menus() -> Self {
        Self(web_extension_constants().permission_menus.clone())
    }
    #[must_use]
    pub fn native_messaging() -> Self {
        Self(web_extension_constants().permission_native_messaging.clone())
    }
    #[must_use]
    pub fn scripting() -> Self {
        Self(web_extension_constants().permission_scripting.clone())
    }
    #[must_use]
    pub fn storage() -> Self {
        Self(web_extension_constants().permission_storage.clone())
    }
    #[must_use]
    pub fn tabs() -> Self {
        Self(web_extension_constants().permission_tabs.clone())
    }
    #[must_use]
    pub fn unlimited_storage() -> Self {
        Self(web_extension_constants().permission_unlimited_storage.clone())
    }
    #[must_use]
    pub fn web_navigation() -> Self {
        Self(web_extension_constants().permission_web_navigation.clone())
    }
    #[must_use]
    pub fn web_request() -> Self {
        Self(web_extension_constants().permission_web_request.clone())
    }
}

impl WebExtensionDataType {
    #[must_use]
    pub fn local() -> Self {
        Self(web_extension_constants().data_type_local.clone())
    }
    #[must_use]
    pub fn session() -> Self {
        Self(web_extension_constants().data_type_session.clone())
    }
    #[must_use]
    pub fn synchronized() -> Self {
        Self(web_extension_constants().data_type_synchronized.clone())
    }
}

impl WebExtensionContextNotificationUserInfoKey {
    #[must_use]
    pub fn permissions() -> Self {
        Self(web_extension_constants().notification_user_info_key_permissions.clone())
    }
    #[must_use]
    pub fn match_patterns() -> Self {
        Self(web_extension_constants().notification_user_info_key_match_patterns.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionError {
    Unknown = 1,
    ResourceNotFound = 2,
    InvalidResourceCodeSignature = 3,
    InvalidManifest = 4,
    UnsupportedManifestVersion = 5,
    InvalidManifestEntry = 6,
    InvalidDeclarativeNetRequestEntry = 7,
    InvalidBackgroundPersistence = 8,
    InvalidArchive = 9,
}

impl WebExtensionError {
    #[must_use]
    pub fn domain() -> &'static str {
        web_extension_constants().web_extension_error_domain.as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionContextError {
    Unknown = 1,
    AlreadyLoaded = 2,
    NotLoaded = 3,
    BaseUrlAlreadyInUse = 4,
    NoBackgroundContent = 5,
    BackgroundContentFailedToLoad = 6,
}

impl WebExtensionContextError {
    #[must_use]
    pub fn domain() -> &'static str {
        web_extension_constants()
            .web_extension_context_error_domain
            .as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionDataRecordError {
    Unknown = 1,
    LocalStorageFailed = 2,
    SessionStorageFailed = 3,
    SynchronizedStorageFailed = 4,
}

impl WebExtensionDataRecordError {
    #[must_use]
    pub fn domain() -> &'static str {
        web_extension_constants()
            .web_extension_data_record_error_domain
            .as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionMatchPatternError {
    Unknown = 1,
    InvalidScheme = 2,
    InvalidHost = 3,
    InvalidPath = 4,
}

impl WebExtensionMatchPatternError {
    #[must_use]
    pub fn domain() -> &'static str {
        web_extension_constants()
            .web_extension_match_pattern_error_domain
            .as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionMessagePortError {
    Unknown = 1,
    NotConnected = 2,
    MessageInvalid = 3,
}

impl WebExtensionMessagePortError {
    #[must_use]
    pub fn domain() -> &'static str {
        web_extension_constants()
            .web_extension_message_port_error_domain
            .as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionContextPermissionStatus {
    DeniedExplicitly = -3,
    DeniedImplicitly = -2,
    RequestedImplicitly = -1,
    Unknown = 0,
    RequestedExplicitly = 1,
    GrantedImplicitly = 2,
    GrantedExplicitly = 3,
}

bitflag_type!(WebExtensionMatchPatternOptions, u64);

impl WebExtensionMatchPatternOptions {
    pub const NONE: Self = Self::from_bits(0);
    pub const IGNORE_SCHEMES: Self = Self::from_bits(1 << 0);
    pub const IGNORE_PATHS: Self = Self::from_bits(1 << 1);
    pub const MATCH_BIDIRECTIONALLY: Self = Self::from_bits(1 << 2);
}

bitflag_type!(WebExtensionTabChangedProperties, u64);

impl WebExtensionTabChangedProperties {
    pub const NONE: Self = Self::from_bits(0);
    pub const LOADING: Self = Self::from_bits(1 << 1);
    pub const MUTED: Self = Self::from_bits(1 << 2);
    pub const PINNED: Self = Self::from_bits(1 << 3);
    pub const PLAYING_AUDIO: Self = Self::from_bits(1 << 4);
    pub const READER_MODE: Self = Self::from_bits(1 << 5);
    pub const SIZE: Self = Self::from_bits(1 << 6);
    pub const TITLE: Self = Self::from_bits(1 << 7);
    pub const URL: Self = Self::from_bits(1 << 8);
    pub const ZOOM_FACTOR: Self = Self::from_bits(1 << 9);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionWindowType {
    Normal = 0,
    Popup = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionWindowState {
    Normal = 0,
    Minimized = 1,
    Maximized = 2,
    Fullscreen = 3,
}

/// Marker trait mirroring `WKWebExtensionControllerDelegate`.
pub trait WebExtensionControllerDelegate: Send + Sync {}

/// Marker trait mirroring `WKWebExtensionTab`.
pub trait WebExtensionTab: Send + Sync {}

/// Marker trait mirroring `WKWebExtensionWindow`.
pub trait WebExtensionWindow: Send + Sync {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NSErrorInfo {
    pub domain: String,
    pub code: i64,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionSummary {
    #[serde(default)]
    pub errors: Vec<NSErrorInfo>,
    #[serde(default)]
    pub manifest: Value,
    pub manifest_version: f64,
    pub default_locale_identifier: Option<String>,
    pub display_name: Option<String>,
    pub display_short_name: Option<String>,
    pub display_version: Option<String>,
    pub display_description: Option<String>,
    pub display_action_label: Option<String>,
    pub version: Option<String>,
    #[serde(default)]
    pub requested_permissions: Vec<WebExtensionPermission>,
    #[serde(default)]
    pub optional_permissions: Vec<WebExtensionPermission>,
    #[serde(default)]
    pub requested_permission_match_patterns: Vec<String>,
    #[serde(default)]
    pub optional_permission_match_patterns: Vec<String>,
    #[serde(default)]
    pub all_requested_match_patterns: Vec<String>,
    pub has_background_content: bool,
    pub has_persistent_background_content: bool,
    pub has_injected_content: bool,
    pub has_options_page: bool,
    pub has_override_new_tab_page: bool,
    pub has_commands: bool,
    pub has_content_modification_rules: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionMatchPatternSummary {
    pub string: String,
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
    pub matches_all_urls: bool,
    pub matches_all_hosts: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionContextSummary {
    #[serde(default)]
    pub errors: Vec<NSErrorInfo>,
    pub loaded: bool,
    pub base_url: Option<String>,
    pub unique_identifier: String,
    pub inspectable: bool,
    pub inspection_name: Option<String>,
    #[serde(default)]
    pub unsupported_apis: Vec<String>,
    pub options_page_url: Option<String>,
    pub override_new_tab_page_url: Option<String>,
    pub has_requested_optional_access_to_all_hosts: bool,
    pub has_access_to_private_data: bool,
    #[serde(default)]
    pub current_permissions: Vec<WebExtensionPermission>,
    #[serde(default)]
    pub current_permission_match_patterns: Vec<String>,
    pub has_access_to_all_urls: bool,
    pub has_access_to_all_hosts: bool,
    pub has_injected_content: bool,
    pub has_content_modification_rules: bool,
    pub webview_configuration_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionAction {
    pub label: String,
    pub badge_text: String,
    pub has_unread_badge_text: bool,
    pub inspection_name: Option<String>,
    pub enabled: bool,
    pub presents_popup: bool,
    pub associated_tab_available: bool,
    pub popup_webview_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionCommand {
    pub identifier: String,
    pub title: String,
    pub activation_key: Option<String>,
    pub modifier_flags: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionDataRecord {
    pub display_name: String,
    pub unique_identifier: String,
    #[serde(default)]
    pub contained_data_types: Vec<WebExtensionDataType>,
    #[serde(default)]
    pub errors: Vec<NSErrorInfo>,
    pub total_size_in_bytes: u64,
    #[serde(default)]
    pub size_in_bytes_by_type: BTreeMap<String, u64>,
}

impl WebExtensionDataRecord {
    #[must_use]
    pub fn size_in_bytes_of_types(&self, data_types: &[WebExtensionDataType]) -> u64 {
        data_types
            .iter()
            .filter_map(|data_type| self.size_in_bytes_by_type.get(data_type.as_str()))
            .copied()
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionTabConfiguration {
    pub has_window: bool,
    pub index: usize,
    pub has_parent_tab: bool,
    pub url: Option<String>,
    pub should_be_active: bool,
    pub should_add_to_selection: bool,
    pub should_be_pinned: bool,
    pub should_be_muted: bool,
    pub should_reader_mode_be_active: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionWindowConfiguration {
    pub window_type: Option<WebExtensionWindowType>,
    pub window_state: Option<WebExtensionWindowState>,
    pub frame: Rect,
    #[serde(default)]
    pub tab_urls: Vec<String>,
    pub tab_count: usize,
    pub should_be_focused: bool,
    pub should_be_private: bool,
}

pub struct WebExtension {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for WebExtension {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for WebExtension {}

impl WebExtension {
    fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    pub fn from_resource_base_url(path: impl AsRef<Path>) -> Result<Self, WebKitError> {
        let path = to_cstring(&path.as_ref().to_string_lossy());
        let mut out_extension = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_create_with_resource_base_url(
                path.as_ptr(),
                &mut out_extension,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_extension).ok_or_else(|| {
            WebKitError::FrameworkError(
                "wk_web_extension_create_with_resource_base_url returned null".to_owned(),
            )
        })
    }

    pub fn from_app_extension_bundle(path: impl AsRef<Path>) -> Result<Self, WebKitError> {
        let path = to_cstring(&path.as_ref().to_string_lossy());
        let mut out_extension = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_create_with_app_extension_bundle(
                path.as_ptr(),
                &mut out_extension,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_extension).ok_or_else(|| {
            WebKitError::FrameworkError(
                "wk_web_extension_create_with_app_extension_bundle returned null".to_owned(),
            )
        })
    }

    #[must_use]
    pub fn summary(&self) -> WebExtensionSummary {
        unsafe { take_json_or_default(ffi::wk_web_extension_copy_summary_json(self.ptr)) }
    }

    #[must_use]
    pub fn supports_manifest_version(&self, manifest_version: f64) -> bool {
        unsafe { ffi::wk_web_extension_supports_manifest_version(self.ptr, manifest_version) }
    }
}

impl Drop for WebExtension {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_web_extension_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct WebExtensionMatchPattern {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for WebExtensionMatchPattern {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for WebExtensionMatchPattern {}

impl WebExtensionMatchPattern {
    fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub fn register_custom_url_scheme(scheme: &str) -> Result<(), WebKitError> {
        let scheme = to_cstring(scheme);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_match_pattern_register_custom_url_scheme(
                scheme.as_ptr(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    #[must_use]
    pub fn all_urls() -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_match_pattern_all_urls() })
            .expect("wk_web_extension_match_pattern_all_urls returned null")
    }

    #[must_use]
    pub fn all_hosts_and_schemes() -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_match_pattern_all_hosts_and_schemes() })
            .expect("wk_web_extension_match_pattern_all_hosts_and_schemes returned null")
    }

    pub fn new(pattern: &str) -> Result<Self, WebKitError> {
        let pattern = to_cstring(pattern);
        let mut out_pattern = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_match_pattern_with_string(
                pattern.as_ptr(),
                &mut out_pattern,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_pattern).ok_or_else(|| {
            WebKitError::FrameworkError(
                "wk_web_extension_match_pattern_with_string returned null".to_owned(),
            )
        })
    }

    pub fn with_components(scheme: &str, host: &str, path: &str) -> Result<Self, WebKitError> {
        let scheme = to_cstring(scheme);
        let host = to_cstring(host);
        let path = to_cstring(path);
        let mut out_pattern = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_match_pattern_with_components(
                scheme.as_ptr(),
                host.as_ptr(),
                path.as_ptr(),
                &mut out_pattern,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_pattern).ok_or_else(|| {
            WebKitError::FrameworkError(
                "wk_web_extension_match_pattern_with_components returned null".to_owned(),
            )
        })
    }

    #[must_use]
    pub fn summary(&self) -> WebExtensionMatchPatternSummary {
        unsafe { take_json_or_default(ffi::wk_web_extension_match_pattern_copy_summary_json(self.ptr)) }
    }

    #[must_use]
    pub fn matches_url(&self, url: &str) -> bool {
        self.matches_url_with_options(url, WebExtensionMatchPatternOptions::NONE)
    }

    #[must_use]
    pub fn matches_url_with_options(
        &self,
        url: &str,
        options: WebExtensionMatchPatternOptions,
    ) -> bool {
        let url = to_cstring(url);
        unsafe { ffi::wk_web_extension_match_pattern_matches_url(self.ptr, url.as_ptr(), options.bits()) }
    }

    #[must_use]
    pub fn matches_pattern(&self, other: &Self) -> bool {
        self.matches_pattern_with_options(other, WebExtensionMatchPatternOptions::NONE)
    }

    #[must_use]
    pub fn matches_pattern_with_options(
        &self,
        other: &Self,
        options: WebExtensionMatchPatternOptions,
    ) -> bool {
        unsafe {
            ffi::wk_web_extension_match_pattern_matches_pattern(
                self.ptr,
                other.as_ptr(),
                options.bits(),
            )
        }
    }
}

impl Drop for WebExtensionMatchPattern {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_web_extension_match_pattern_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct WebExtensionControllerConfiguration {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for WebExtensionControllerConfiguration {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for WebExtensionControllerConfiguration {}

impl WebExtensionControllerConfiguration {
    fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    #[must_use]
    pub fn default_configuration() -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_controller_configuration_default() })
            .expect("wk_web_extension_controller_configuration_default returned null")
    }

    #[must_use]
    pub fn non_persistent_configuration() -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_controller_configuration_nonpersistent() })
            .expect("wk_web_extension_controller_configuration_nonpersistent returned null")
    }

    pub fn configuration_with_identifier(identifier: &str) -> Result<Self, WebKitError> {
        let identifier = to_cstring(identifier);
        let mut out_configuration = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_controller_configuration_with_identifier(
                identifier.as_ptr(),
                &mut out_configuration,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_configuration).ok_or_else(|| {
            WebKitError::FrameworkError(
                "wk_web_extension_controller_configuration_with_identifier returned null"
                    .to_owned(),
            )
        })
    }

    #[must_use]
    pub fn summary(&self) -> WebExtensionControllerConfigurationSummary {
        unsafe {
            take_json_or_default(ffi::wk_web_extension_controller_configuration_copy_summary_json(
                self.ptr,
            ))
        }
    }

    pub fn set_webview_configuration(&self, configuration: &WebViewConfiguration) {
        unsafe {
            ffi::wk_web_extension_controller_configuration_set_webview_configuration(
                self.ptr,
                configuration.as_ptr(),
            );
        }
    }

    #[must_use]
    pub fn webview_configuration(&self) -> Option<WebViewConfiguration> {
        WebViewConfiguration::from_ptr(unsafe {
            ffi::wk_web_extension_controller_configuration_copy_webview_configuration(self.ptr)
        })
    }

    pub fn set_default_website_data_store(&self, store: &WebsiteDataStore) {
        unsafe {
            ffi::wk_web_extension_controller_configuration_set_default_website_data_store(
                self.ptr,
                store.as_ptr(),
            );
        }
    }

    #[must_use]
    pub fn default_website_data_store(&self) -> Option<WebsiteDataStore> {
        WebsiteDataStore::from_ptr(unsafe {
            ffi::wk_web_extension_controller_configuration_copy_default_website_data_store(
                self.ptr,
            )
        })
    }
}

impl Drop for WebExtensionControllerConfiguration {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_web_extension_controller_configuration_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionControllerConfigurationSummary {
    pub persistent: bool,
    pub identifier: Option<String>,
}

pub struct WebExtensionController {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for WebExtensionController {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for WebExtensionController {}

impl Default for WebExtensionController {
    fn default() -> Self {
        Self::new()
    }
}

impl WebExtensionController {
    fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    #[must_use]
    pub fn new() -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_controller_new() })
            .expect("wk_web_extension_controller_new returned null")
    }

    #[must_use]
    pub fn with_configuration(configuration: &WebExtensionControllerConfiguration) -> Self {
        Self::from_ptr(unsafe {
            ffi::wk_web_extension_controller_with_configuration(configuration.as_ptr())
        })
        .expect("wk_web_extension_controller_with_configuration returned null")
    }

    #[must_use]
    pub fn configuration(&self) -> Option<WebExtensionControllerConfiguration> {
        WebExtensionControllerConfiguration::from_ptr(unsafe {
            ffi::wk_web_extension_controller_copy_configuration(self.ptr)
        })
    }

    pub fn load(&self, context: &WebExtensionContext) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_controller_load_context(self.ptr, context.ptr, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn unload(&self, context: &WebExtensionContext) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_controller_unload_context(self.ptr, context.ptr, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    #[must_use]
    pub fn extension_context_for_extension(
        &self,
        extension: &WebExtension,
    ) -> Option<WebExtensionContext> {
        WebExtensionContext::from_ptr(unsafe {
            ffi::wk_web_extension_controller_copy_context_for_extension(self.ptr, extension.ptr)
        })
    }

    #[must_use]
    pub fn extension_context_for_url(&self, url: &str) -> Option<WebExtensionContext> {
        let url = to_cstring(url);
        WebExtensionContext::from_ptr(unsafe {
            ffi::wk_web_extension_controller_copy_context_for_url(self.ptr, url.as_ptr())
        })
    }

    #[must_use]
    pub fn all_extension_data_types() -> Vec<WebExtensionDataType> {
        unsafe { take_json_or_default(ffi::wk_web_extension_controller_copy_all_data_types_json()) }
    }

    pub fn data_records(
        &self,
        data_types: &[WebExtensionDataType],
    ) -> Result<Vec<WebExtensionDataRecord>, WebKitError> {
        let data_types_json = to_json_cstring(data_types);
        let mut out_json = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_controller_copy_data_records_json(
                self.ptr,
                data_types_json.as_ptr(),
                &mut out_json,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_json_or_default(out_json) })
    }

    pub fn data_record_for_context(
        &self,
        data_types: &[WebExtensionDataType],
        context: &WebExtensionContext,
    ) -> Result<Option<WebExtensionDataRecord>, WebKitError> {
        let data_types_json = to_json_cstring(data_types);
        let mut out_json = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_controller_copy_data_record_json_for_context(
                self.ptr,
                data_types_json.as_ptr(),
                context.ptr,
                &mut out_json,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_json_or_default(out_json) })
    }

    pub fn remove_data(
        &self,
        data_types: &[WebExtensionDataType],
        records: &[WebExtensionDataRecord],
    ) -> Result<(), WebKitError> {
        let data_types_json = to_json_cstring(data_types);
        let identifiers = records
            .iter()
            .map(|record| record.unique_identifier.clone())
            .collect::<Vec<_>>();
        let identifiers_json = to_json_cstring(&identifiers);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_controller_remove_data_for_identifiers(
                self.ptr,
                data_types_json.as_ptr(),
                identifiers_json.as_ptr(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }
}

impl Drop for WebExtensionController {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_web_extension_controller_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}

pub struct WebExtensionContext {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for WebExtensionContext {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for WebExtensionContext {}

impl WebExtensionContext {
    fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    #[must_use]
    pub fn for_extension(extension: &WebExtension) -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_context_new_for_extension(extension.ptr) })
            .expect("wk_web_extension_context_new_for_extension returned null")
    }

    #[must_use]
    pub fn summary(&self) -> WebExtensionContextSummary {
        unsafe { take_json_or_default(ffi::wk_web_extension_context_copy_summary_json(self.ptr)) }
    }

    pub fn set_base_url(&self, url: &str) -> Result<(), WebKitError> {
        let url = to_cstring(url);
        let mut out_err = ptr::null_mut();
        let status = unsafe { ffi::wk_web_extension_context_set_base_url(self.ptr, url.as_ptr(), &mut out_err) };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn set_unique_identifier(&self, identifier: &str) -> Result<(), WebKitError> {
        let identifier = to_cstring(identifier);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_context_set_unique_identifier(
                self.ptr,
                identifier.as_ptr(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn set_inspectable(&self, value: bool) {
        unsafe { ffi::wk_web_extension_context_set_inspectable(self.ptr, value) }
    }

    pub fn set_inspection_name(&self, name: Option<&str>) {
        let name = name.map(to_cstring);
        let name_ptr = name.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        unsafe { ffi::wk_web_extension_context_set_inspection_name(self.ptr, name_ptr) }
    }

    pub fn set_unsupported_apis(&self, apis: &[String]) {
        let apis_json = to_json_cstring(apis);
        unsafe { ffi::wk_web_extension_context_set_unsupported_apis_json(self.ptr, apis_json.as_ptr()) }
    }

    pub fn set_requested_optional_access_to_all_hosts(&self, value: bool) {
        unsafe {
            ffi::wk_web_extension_context_set_requested_optional_access_to_all_hosts(self.ptr, value);
        }
    }

    pub fn set_access_to_private_data(&self, value: bool) {
        unsafe { ffi::wk_web_extension_context_set_access_to_private_data(self.ptr, value) }
    }

    #[must_use]
    pub fn webview_configuration(&self) -> Option<WebViewConfiguration> {
        WebViewConfiguration::from_ptr(unsafe {
            ffi::wk_web_extension_context_copy_webview_configuration(self.ptr)
        })
    }

    #[must_use]
    pub fn has_permission(&self, permission: &WebExtensionPermission) -> bool {
        let permission = to_cstring(permission.as_str());
        unsafe { ffi::wk_web_extension_context_has_permission(self.ptr, permission.as_ptr()) }
    }

    #[must_use]
    pub fn has_access_to_url(&self, url: &str) -> bool {
        let url = to_cstring(url);
        unsafe { ffi::wk_web_extension_context_has_access_to_url(self.ptr, url.as_ptr()) }
    }

    #[must_use]
    pub fn permission_status_for_permission(
        &self,
        permission: &WebExtensionPermission,
    ) -> WebExtensionContextPermissionStatus {
        let permission = to_cstring(permission.as_str());
        match unsafe {
            ffi::wk_web_extension_context_permission_status_for_permission(self.ptr, permission.as_ptr())
        } {
            -3 => WebExtensionContextPermissionStatus::DeniedExplicitly,
            -2 => WebExtensionContextPermissionStatus::DeniedImplicitly,
            -1 => WebExtensionContextPermissionStatus::RequestedImplicitly,
            1 => WebExtensionContextPermissionStatus::RequestedExplicitly,
            2 => WebExtensionContextPermissionStatus::GrantedImplicitly,
            3 => WebExtensionContextPermissionStatus::GrantedExplicitly,
            _ => WebExtensionContextPermissionStatus::Unknown,
        }
    }

    pub fn set_permission_status_for_permission(
        &self,
        status: WebExtensionContextPermissionStatus,
        permission: &WebExtensionPermission,
    ) -> Result<(), WebKitError> {
        let permission = to_cstring(permission.as_str());
        let mut out_err = ptr::null_mut();
        let status_code = unsafe {
            ffi::wk_web_extension_context_set_permission_status_for_permission(
                self.ptr,
                status as i64,
                permission.as_ptr(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status_code, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    #[must_use]
    pub fn permission_status_for_url(&self, url: &str) -> WebExtensionContextPermissionStatus {
        let url = to_cstring(url);
        match unsafe { ffi::wk_web_extension_context_permission_status_for_url(self.ptr, url.as_ptr()) } {
            -3 => WebExtensionContextPermissionStatus::DeniedExplicitly,
            -2 => WebExtensionContextPermissionStatus::DeniedImplicitly,
            -1 => WebExtensionContextPermissionStatus::RequestedImplicitly,
            1 => WebExtensionContextPermissionStatus::RequestedExplicitly,
            2 => WebExtensionContextPermissionStatus::GrantedImplicitly,
            3 => WebExtensionContextPermissionStatus::GrantedExplicitly,
            _ => WebExtensionContextPermissionStatus::Unknown,
        }
    }

    pub fn set_permission_status_for_url(
        &self,
        status: WebExtensionContextPermissionStatus,
        url: &str,
    ) -> Result<(), WebKitError> {
        let url = to_cstring(url);
        let mut out_err = ptr::null_mut();
        let status_code = unsafe {
            ffi::wk_web_extension_context_set_permission_status_for_url(
                self.ptr,
                status as i64,
                url.as_ptr(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status_code, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    #[must_use]
    pub fn permission_status_for_match_pattern(
        &self,
        pattern: &WebExtensionMatchPattern,
    ) -> WebExtensionContextPermissionStatus {
        match unsafe {
            ffi::wk_web_extension_context_permission_status_for_match_pattern(self.ptr, pattern.ptr)
        } {
            -3 => WebExtensionContextPermissionStatus::DeniedExplicitly,
            -2 => WebExtensionContextPermissionStatus::DeniedImplicitly,
            -1 => WebExtensionContextPermissionStatus::RequestedImplicitly,
            1 => WebExtensionContextPermissionStatus::RequestedExplicitly,
            2 => WebExtensionContextPermissionStatus::GrantedImplicitly,
            3 => WebExtensionContextPermissionStatus::GrantedExplicitly,
            _ => WebExtensionContextPermissionStatus::Unknown,
        }
    }

    pub fn set_permission_status_for_match_pattern(
        &self,
        status: WebExtensionContextPermissionStatus,
        pattern: &WebExtensionMatchPattern,
    ) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status_code = unsafe {
            ffi::wk_web_extension_context_set_permission_status_for_match_pattern(
                self.ptr,
                status as i64,
                pattern.ptr,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status_code, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn load_background_content(&self) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_context_load_background_content(self.ptr, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    #[must_use]
    pub fn action(&self) -> Option<WebExtensionAction> {
        unsafe { take_json_or_default(ffi::wk_web_extension_context_copy_default_action_json(self.ptr)) }
    }

    pub fn perform_action(&self) {
        unsafe { ffi::wk_web_extension_context_perform_default_action(self.ptr) }
    }

    #[must_use]
    pub fn commands(&self) -> Vec<WebExtensionCommand> {
        unsafe { take_json_or_default(ffi::wk_web_extension_context_copy_commands_json(self.ptr)) }
    }

    pub fn perform_command_by_identifier(&self, identifier: &str) -> Result<(), WebKitError> {
        let identifier = to_cstring(identifier);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_context_perform_command_for_identifier(
                self.ptr,
                identifier.as_ptr(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn perform_command(&self, command: &WebExtensionCommand) -> Result<(), WebKitError> {
        self.perform_command_by_identifier(&command.identifier)
    }
}

impl Drop for WebExtensionContext {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_web_extension_context_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}

/// Safe wrapper around a `WKWebExtensionMessagePort` handle.
pub struct WebExtensionMessagePort {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for WebExtensionMessagePort {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for WebExtensionMessagePort {}

impl WebExtensionMessagePort {
    #[must_use]
    pub fn application_identifier(&self) -> Option<String> {
        unsafe { take_optional_string(ffi::wk_web_extension_message_port_copy_application_identifier(self.ptr)) }
    }

    #[must_use]
    pub fn is_disconnected(&self) -> bool {
        unsafe { ffi::wk_web_extension_message_port_is_disconnected(self.ptr) }
    }

    pub fn send_message(&self, message: &Value) -> Result<(), WebKitError> {
        let message_json = to_json_cstring(message);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_message_port_send_message_json(
                self.ptr,
                message_json.as_ptr(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn disconnect(&self) {
        unsafe { ffi::wk_web_extension_message_port_disconnect(self.ptr) }
    }

    pub fn disconnect_with_error(&self, message: &str) {
        let message = to_cstring(message);
        unsafe { ffi::wk_web_extension_message_port_disconnect_with_error(self.ptr, message.as_ptr()) }
    }
}

impl Drop for WebExtensionMessagePort {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_web_extension_message_port_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}

/// Notification names emitted by `WKWebExtensionContext`.
pub struct WebExtensionContextNotifications;

impl WebExtensionContextNotifications {
    #[must_use]
    pub fn errors_did_update() -> &'static str {
        web_extension_constants().errors_did_update_notification.as_str()
    }

    #[must_use]
    pub fn permissions_were_granted() -> &'static str {
        web_extension_constants()
            .permissions_were_granted_notification
            .as_str()
    }

    #[must_use]
    pub fn permissions_were_denied() -> &'static str {
        web_extension_constants()
            .permissions_were_denied_notification
            .as_str()
    }

    #[must_use]
    pub fn granted_permissions_were_removed() -> &'static str {
        web_extension_constants()
            .granted_permissions_were_removed_notification
            .as_str()
    }

    #[must_use]
    pub fn denied_permissions_were_removed() -> &'static str {
        web_extension_constants()
            .denied_permissions_were_removed_notification
            .as_str()
    }

    #[must_use]
    pub fn permission_match_patterns_were_granted() -> &'static str {
        web_extension_constants()
            .permission_match_patterns_were_granted_notification
            .as_str()
    }

    #[must_use]
    pub fn permission_match_patterns_were_denied() -> &'static str {
        web_extension_constants()
            .permission_match_patterns_were_denied_notification
            .as_str()
    }

    #[must_use]
    pub fn granted_permission_match_patterns_were_removed() -> &'static str {
        web_extension_constants()
            .granted_permission_match_patterns_were_removed_notification
            .as_str()
    }

    #[must_use]
    pub fn denied_permission_match_patterns_were_removed() -> &'static str {
        web_extension_constants()
            .denied_permission_match_patterns_were_removed_notification
            .as_str()
    }
}
