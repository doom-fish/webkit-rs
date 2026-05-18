#![allow(clippy::missing_panics_doc, clippy::struct_excessive_bools)]

use core::ffi::c_void;
use core::ptr;
use std::collections::BTreeMap;
use std::ops::{BitOr, BitOrAssign};
use std::path::Path;
use std::sync::{Arc, OnceLock};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::WebViewConfiguration;
use crate::error::WebKitError;
use crate::ffi;
use crate::geometry::Rect;
use crate::private::{
    maybe_take_error, take_json_or_default, take_optional_string, to_cstring, to_json_cstring,
};
use crate::snapshot_configuration::SnapshotConfiguration;
use crate::website_data_store::WebsiteDataStore;
use crate::webview::WebView;

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
        /// Wraps a `WKWebExtension*` value exposed by WebKit.
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates a value for `WKWebExtension`.
            #[must_use]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// Returns the corresponding value from `WKWebExtension`.
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
        /// Wraps a `WKWebExtension*` value exposed by WebKit.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
        #[serde(transparent)]
        pub struct $name($inner);

        impl $name {
            /// Creates a value for `WKWebExtension`.
            #[must_use]
            pub const fn from_bits(bits: $inner) -> Self {
                Self(bits)
            }

            /// Returns the corresponding value from `WKWebExtension`.
            #[must_use]
            pub const fn bits(self) -> $inner {
                self.0
            }

            /// Returns whether this `WKWebExtension` value contains the provided flags.
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
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn active_tab() -> Self {
        Self(web_extension_constants().permission_active_tab.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn alarms() -> Self {
        Self(web_extension_constants().permission_alarms.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn clipboard_write() -> Self {
        Self(web_extension_constants().permission_clipboard_write.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn context_menus() -> Self {
        Self(web_extension_constants().permission_context_menus.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn cookies() -> Self {
        Self(web_extension_constants().permission_cookies.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn declarative_net_request() -> Self {
        Self(
            web_extension_constants()
                .permission_declarative_net_request
                .clone(),
        )
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn declarative_net_request_feedback() -> Self {
        Self(
            web_extension_constants()
                .permission_declarative_net_request_feedback
                .clone(),
        )
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn declarative_net_request_with_host_access() -> Self {
        Self(
            web_extension_constants()
                .permission_declarative_net_request_with_host_access
                .clone(),
        )
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn menus() -> Self {
        Self(web_extension_constants().permission_menus.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn native_messaging() -> Self {
        Self(
            web_extension_constants()
                .permission_native_messaging
                .clone(),
        )
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn scripting() -> Self {
        Self(web_extension_constants().permission_scripting.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn storage() -> Self {
        Self(web_extension_constants().permission_storage.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn tabs() -> Self {
        Self(web_extension_constants().permission_tabs.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn unlimited_storage() -> Self {
        Self(
            web_extension_constants()
                .permission_unlimited_storage
                .clone(),
        )
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn web_navigation() -> Self {
        Self(web_extension_constants().permission_web_navigation.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionPermission` API.
    #[must_use]
    pub fn web_request() -> Self {
        Self(web_extension_constants().permission_web_request.clone())
    }
}

impl WebExtensionDataType {
    /// Mirrors the corresponding `WKWebExtensionDataType` API.
    #[must_use]
    pub fn local() -> Self {
        Self(web_extension_constants().data_type_local.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionDataType` API.
    #[must_use]
    pub fn session() -> Self {
        Self(web_extension_constants().data_type_session.clone())
    }
    /// Mirrors the corresponding `WKWebExtensionDataType` API.
    #[must_use]
    pub fn synchronized() -> Self {
        Self(web_extension_constants().data_type_synchronized.clone())
    }
}

impl WebExtensionContextNotificationUserInfoKey {
    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn permissions() -> Self {
        Self(
            web_extension_constants()
                .notification_user_info_key_permissions
                .clone(),
        )
    }
    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn match_patterns() -> Self {
        Self(
            web_extension_constants()
                .notification_user_info_key_match_patterns
                .clone(),
        )
    }
}

/// Wraps `WKWebExtension` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionError {
    /// Mirrors the `Unknown` case used by `WKWebExtension`.
    Unknown = 1,
    /// Mirrors the `ResourceNotFound` case used by `WKWebExtension`.
    ResourceNotFound = 2,
    /// Mirrors the `InvalidResourceCodeSignature` case used by `WKWebExtension`.
    InvalidResourceCodeSignature = 3,
    /// Mirrors the `InvalidManifest` case used by `WKWebExtension`.
    InvalidManifest = 4,
    /// Mirrors the `UnsupportedManifestVersion` case used by `WKWebExtension`.
    UnsupportedManifestVersion = 5,
    /// Mirrors the `InvalidManifestEntry` case used by `WKWebExtension`.
    InvalidManifestEntry = 6,
    /// Mirrors the `InvalidDeclarativeNetRequestEntry` case used by `WKWebExtension`.
    InvalidDeclarativeNetRequestEntry = 7,
    /// Mirrors the `InvalidBackgroundPersistence` case used by `WKWebExtension`.
    InvalidBackgroundPersistence = 8,
    /// Mirrors the `InvalidArchive` case used by `WKWebExtension`.
    InvalidArchive = 9,
}

impl WebExtensionError {
    /// Returns the corresponding value from `WKWebExtension`.
    #[must_use]
    pub fn domain() -> &'static str {
        web_extension_constants()
            .web_extension_error_domain
            .as_str()
    }
}

/// Wraps `WKWebExtensionContext` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionContextError {
    /// Mirrors the `Unknown` case used by `WKWebExtensionContext`.
    Unknown = 1,
    /// Mirrors the `AlreadyLoaded` case used by `WKWebExtensionContext`.
    AlreadyLoaded = 2,
    /// Mirrors the `NotLoaded` case used by `WKWebExtensionContext`.
    NotLoaded = 3,
    /// Mirrors the `BaseUrlAlreadyInUse` case used by `WKWebExtensionContext`.
    BaseUrlAlreadyInUse = 4,
    /// Mirrors the `NoBackgroundContent` case used by `WKWebExtensionContext`.
    NoBackgroundContent = 5,
    /// Mirrors the `BackgroundContentFailedToLoad` case used by `WKWebExtensionContext`.
    BackgroundContentFailedToLoad = 6,
}

impl WebExtensionContextError {
    /// Returns the corresponding value from `WKWebExtensionContext`.
    #[must_use]
    pub fn domain() -> &'static str {
        web_extension_constants()
            .web_extension_context_error_domain
            .as_str()
    }
}

/// Wraps `WKWebExtensionDataRecord` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionDataRecordError {
    /// Mirrors the `Unknown` case used by `WKWebExtensionDataRecord`.
    Unknown = 1,
    /// Mirrors the `LocalStorageFailed` case used by `WKWebExtensionDataRecord`.
    LocalStorageFailed = 2,
    /// Mirrors the `SessionStorageFailed` case used by `WKWebExtensionDataRecord`.
    SessionStorageFailed = 3,
    /// Mirrors the `SynchronizedStorageFailed` case used by `WKWebExtensionDataRecord`.
    SynchronizedStorageFailed = 4,
}

impl WebExtensionDataRecordError {
    /// Returns the corresponding value from `WKWebExtensionDataRecord`.
    #[must_use]
    pub fn domain() -> &'static str {
        web_extension_constants()
            .web_extension_data_record_error_domain
            .as_str()
    }
}

/// Wraps `WKWebExtensionMatchPattern` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionMatchPatternError {
    /// Mirrors the `Unknown` case used by `WKWebExtensionMatchPattern`.
    Unknown = 1,
    /// Mirrors the `InvalidScheme` case used by `WKWebExtensionMatchPattern`.
    InvalidScheme = 2,
    /// Mirrors the `InvalidHost` case used by `WKWebExtensionMatchPattern`.
    InvalidHost = 3,
    /// Mirrors the `InvalidPath` case used by `WKWebExtensionMatchPattern`.
    InvalidPath = 4,
}

impl WebExtensionMatchPatternError {
    /// Returns the corresponding value from `WKWebExtensionMatchPattern`.
    #[must_use]
    pub fn domain() -> &'static str {
        web_extension_constants()
            .web_extension_match_pattern_error_domain
            .as_str()
    }
}

/// Wraps `WKWebExtensionMessagePort` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionMessagePortError {
    /// Mirrors the `Unknown` case used by `WKWebExtensionMessagePort`.
    Unknown = 1,
    /// Mirrors the `NotConnected` case used by `WKWebExtensionMessagePort`.
    NotConnected = 2,
    /// Mirrors the `MessageInvalid` case used by `WKWebExtensionMessagePort`.
    MessageInvalid = 3,
}

impl WebExtensionMessagePortError {
    /// Returns the corresponding value from `WKWebExtensionMessagePort`.
    #[must_use]
    pub fn domain() -> &'static str {
        web_extension_constants()
            .web_extension_message_port_error_domain
            .as_str()
    }
}

/// Wraps `WKWebExtensionContext` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionContextPermissionStatus {
    /// Mirrors the `DeniedExplicitly` case used by `WKWebExtensionContext`.
    DeniedExplicitly = -3,
    /// Mirrors the `DeniedImplicitly` case used by `WKWebExtensionContext`.
    DeniedImplicitly = -2,
    /// Mirrors the `RequestedImplicitly` case used by `WKWebExtensionContext`.
    RequestedImplicitly = -1,
    /// Mirrors the `Unknown` case used by `WKWebExtensionContext`.
    Unknown = 0,
    /// Mirrors the `RequestedExplicitly` case used by `WKWebExtensionContext`.
    RequestedExplicitly = 1,
    /// Mirrors the `GrantedImplicitly` case used by `WKWebExtensionContext`.
    GrantedImplicitly = 2,
    /// Mirrors the `GrantedExplicitly` case used by `WKWebExtensionContext`.
    GrantedExplicitly = 3,
}

bitflag_type!(WebExtensionMatchPatternOptions, u64);

impl WebExtensionMatchPatternOptions {
    /// Mirrors the `NONE` constant used by `WKWebExtensionMatchPattern`.
    pub const NONE: Self = Self::from_bits(0);
    /// Mirrors the `IGNORE_SCHEMES` constant used by `WKWebExtensionMatchPattern`.
    pub const IGNORE_SCHEMES: Self = Self::from_bits(1 << 0);
    /// Mirrors the `IGNORE_PATHS` constant used by `WKWebExtensionMatchPattern`.
    pub const IGNORE_PATHS: Self = Self::from_bits(1 << 1);
    /// Mirrors the `MATCH_BIDIRECTIONALLY` constant used by `WKWebExtensionMatchPattern`.
    pub const MATCH_BIDIRECTIONALLY: Self = Self::from_bits(1 << 2);
}

bitflag_type!(WebExtensionTabChangedProperties, u64);

impl WebExtensionTabChangedProperties {
    /// Mirrors the `NONE` constant used by `WKWebExtensionTab`.
    pub const NONE: Self = Self::from_bits(0);
    /// Mirrors the `LOADING` constant used by `WKWebExtensionTab`.
    pub const LOADING: Self = Self::from_bits(1 << 1);
    /// Mirrors the `MUTED` constant used by `WKWebExtensionTab`.
    pub const MUTED: Self = Self::from_bits(1 << 2);
    /// Mirrors the `PINNED` constant used by `WKWebExtensionTab`.
    pub const PINNED: Self = Self::from_bits(1 << 3);
    /// Mirrors the `PLAYING_AUDIO` constant used by `WKWebExtensionTab`.
    pub const PLAYING_AUDIO: Self = Self::from_bits(1 << 4);
    /// Mirrors the `READER_MODE` constant used by `WKWebExtensionTab`.
    pub const READER_MODE: Self = Self::from_bits(1 << 5);
    /// Mirrors the `SIZE` constant used by `WKWebExtensionTab`.
    pub const SIZE: Self = Self::from_bits(1 << 6);
    /// Mirrors the `TITLE` constant used by `WKWebExtensionTab`.
    pub const TITLE: Self = Self::from_bits(1 << 7);
    /// Mirrors the `URL` constant used by `WKWebExtensionTab`.
    pub const URL: Self = Self::from_bits(1 << 8);
    /// Mirrors the `ZOOM_FACTOR` constant used by `WKWebExtensionTab`.
    pub const ZOOM_FACTOR: Self = Self::from_bits(1 << 9);
}

/// Wraps `WKWebExtensionWindow` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionWindowType {
    /// Mirrors the `Normal` case used by `WKWebExtensionWindow`.
    Normal = 0,
    /// Mirrors the `Popup` case used by `WKWebExtensionWindow`.
    Popup = 1,
}

/// Wraps `WKWebExtensionWindow` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
pub enum WebExtensionWindowState {
    /// Mirrors the `Normal` case used by `WKWebExtensionWindow`.
    Normal = 0,
    /// Mirrors the `Minimized` case used by `WKWebExtensionWindow`.
    Minimized = 1,
    /// Mirrors the `Maximized` case used by `WKWebExtensionWindow`.
    Maximized = 2,
    /// Mirrors the `Fullscreen` case used by `WKWebExtensionWindow`.
    Fullscreen = 3,
}

fn unsupported_web_extension_delegate_method(method: &str) -> WebKitError {
    WebKitError::Unsupported(format!(
        "web extension delegate method `{method}` is not implemented"
    ))
}

/// Result payload returned by permission-prompt style web extension delegate methods.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionGrant<T> {
    /// Mirrors the `allowed` value exposed by `WKWebExtensionContext`.
    #[serde(default)]
    pub allowed: Vec<T>,
    /// Mirrors the `expiration_date` value exposed by `WKWebExtensionContext`.
    pub expiration_date: Option<String>,
}

impl<T> Default for WebExtensionGrant<T> {
    fn default() -> Self {
        Self {
            allowed: Vec::new(),
            expiration_date: None,
        }
    }
}

/// Type alias used with `WKWebExtensionContext` permission grants.
pub type WebExtensionPermissionGrant = WebExtensionGrant<WebExtensionPermission>;
/// Type alias used with `WKWebExtensionContext` permission grants.
pub type WebExtensionUrlGrant = WebExtensionGrant<String>;
/// Type alias used with `WKWebExtensionContext` permission grants.
pub type WebExtensionMatchPatternGrant = WebExtensionGrant<WebExtensionMatchPattern>;
/// Shared handle used with `WKWebExtensionTab`.
pub type WebExtensionTabHandle = Arc<dyn WebExtensionTab>;
/// Shared handle used with `WKWebExtensionWindow`.
pub type WebExtensionWindowHandle = Arc<dyn WebExtensionWindow>;
/// Shared handle used with `WKWebView`.
pub type WebExtensionWebViewHandle = Arc<WebView>;

/// Size reported by `WKWebExtensionTab`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct WebExtensionSize {
    /// Mirrors the `width` value exposed by `WKWebExtensionAction`.
    pub width: f64,
    /// Mirrors the `height` value exposed by `WKWebExtensionAction`.
    pub height: f64,
}

impl WebExtensionSize {
    /// Creates a value for `WKWebExtensionAction`.
    #[must_use]
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

/// PNG snapshot bytes returned by `WKWebExtensionTab::takeSnapshot`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionTabSnapshot {
    /// Mirrors the `png_data` value exposed by `WKWebExtensionTab`.
    #[serde(default)]
    pub png_data: Vec<u8>,
}

/// Trait mirroring `WKWebExtensionControllerDelegate`.
pub trait WebExtensionControllerDelegate: Send + Sync {
    /// Returns the corresponding value from `WKWebExtensionControllerDelegate`.
    fn open_windows_for_context(
        &self,
        _controller: &WebExtensionController,
        _extension_context: &WebExtensionContext,
    ) -> Vec<WebExtensionWindowHandle> {
        Vec::new()
    }

    /// Returns the corresponding value from `WKWebExtensionControllerDelegate`.
    fn focused_window_for_context(
        &self,
        _controller: &WebExtensionController,
        _extension_context: &WebExtensionContext,
    ) -> Option<WebExtensionWindowHandle> {
        None
    }

    /// Calls the corresponding `WKWebExtensionControllerDelegate` API.
    fn open_new_window(
        &self,
        _controller: &WebExtensionController,
        _configuration: &WebExtensionWindowConfiguration,
        _extension_context: &WebExtensionContext,
    ) -> Result<Option<WebExtensionWindowHandle>, WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionControllerDelegate.openNewWindowUsingConfiguration",
        ))
    }

    /// Calls the corresponding `WKWebExtensionControllerDelegate` API.
    fn open_new_tab(
        &self,
        _controller: &WebExtensionController,
        _configuration: &WebExtensionTabConfiguration,
        _extension_context: &WebExtensionContext,
    ) -> Result<Option<WebExtensionTabHandle>, WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionControllerDelegate.openNewTabUsingConfiguration",
        ))
    }

    /// Calls the corresponding `WKWebExtensionControllerDelegate` API.
    fn open_options_page(
        &self,
        _controller: &WebExtensionController,
        _extension_context: &WebExtensionContext,
    ) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionControllerDelegate.openOptionsPageForExtensionContext",
        ))
    }

    /// Calls the corresponding `WKWebExtensionControllerDelegate` API.
    fn prompt_for_permissions(
        &self,
        _controller: &WebExtensionController,
        _permissions: &[WebExtensionPermission],
        _tab: Option<&dyn WebExtensionTab>,
        _extension_context: &WebExtensionContext,
    ) -> WebExtensionPermissionGrant {
        WebExtensionPermissionGrant::default()
    }

    /// Calls the corresponding `WKWebExtensionControllerDelegate` API.
    fn prompt_for_permission_to_access_urls(
        &self,
        _controller: &WebExtensionController,
        _urls: &[String],
        _tab: Option<&dyn WebExtensionTab>,
        _extension_context: &WebExtensionContext,
    ) -> WebExtensionUrlGrant {
        WebExtensionUrlGrant::default()
    }

    /// Calls the corresponding `WKWebExtensionControllerDelegate` API.
    fn prompt_for_permission_match_patterns(
        &self,
        _controller: &WebExtensionController,
        _match_patterns: &[WebExtensionMatchPattern],
        _tab: Option<&dyn WebExtensionTab>,
        _extension_context: &WebExtensionContext,
    ) -> WebExtensionMatchPatternGrant {
        WebExtensionMatchPatternGrant::default()
    }

    /// Calls the corresponding `WKWebExtensionControllerDelegate` API.
    fn did_update_action(
        &self,
        _controller: &WebExtensionController,
        _action: &WebExtensionAction,
        _extension_context: &WebExtensionContext,
    ) {
    }

    /// Calls the corresponding `WKWebExtensionControllerDelegate` API.
    fn present_popup_for_action(
        &self,
        _controller: &WebExtensionController,
        _action: &WebExtensionAction,
        _extension_context: &WebExtensionContext,
    ) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionControllerDelegate.presentPopupForAction",
        ))
    }

    /// Calls the corresponding `WKWebExtensionControllerDelegate` API.
    fn send_message(
        &self,
        _controller: &WebExtensionController,
        _message: &Value,
        _application_identifier: Option<&str>,
        _extension_context: &WebExtensionContext,
    ) -> Result<Option<Value>, WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionControllerDelegate.sendMessage",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionControllerDelegate` API.
    fn connect_using_message_port(
        &self,
        _controller: &WebExtensionController,
        _port: &WebExtensionMessagePort,
        _extension_context: &WebExtensionContext,
    ) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionControllerDelegate.connectUsingMessagePort",
        ))
    }
}

/// Trait mirroring `WKWebExtensionTab`.
pub trait WebExtensionTab: Send + Sync {
    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn window(&self, _context: &WebExtensionContext) -> Option<WebExtensionWindowHandle> {
        None
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn index_in_window(&self, _context: &WebExtensionContext) -> usize {
        0
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn parent_tab(&self, _context: &WebExtensionContext) -> Option<WebExtensionTabHandle> {
        None
    }

    /// Sets the corresponding value on `WKWebExtensionTab`.
    fn set_parent_tab(
        &self,
        _parent_tab: Option<WebExtensionTabHandle>,
        _context: &WebExtensionContext,
    ) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.setParentTab",
        ))
    }

    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn webview(&self, _context: &WebExtensionContext) -> Option<WebExtensionWebViewHandle> {
        None
    }

    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn title(&self, _context: &WebExtensionContext) -> Option<String> {
        None
    }

    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn is_pinned(&self, _context: &WebExtensionContext) -> bool {
        false
    }

    /// Sets the corresponding value on `WKWebExtensionTab`.
    fn set_pinned(&self, _pinned: bool, _context: &WebExtensionContext) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.setPinned",
        ))
    }

    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn is_reader_mode_available(&self, _context: &WebExtensionContext) -> bool {
        false
    }

    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn is_reader_mode_active(&self, _context: &WebExtensionContext) -> bool {
        false
    }

    /// Sets the corresponding value on `WKWebExtensionTab`.
    fn set_reader_mode_active(
        &self,
        _active: bool,
        _context: &WebExtensionContext,
    ) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.setReaderModeActive",
        ))
    }

    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn is_playing_audio(&self, _context: &WebExtensionContext) -> bool {
        false
    }

    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn is_muted(&self, _context: &WebExtensionContext) -> bool {
        false
    }

    /// Sets the corresponding value on `WKWebExtensionTab`.
    fn set_muted(&self, _muted: bool, _context: &WebExtensionContext) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.setMuted",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn size(&self, _context: &WebExtensionContext) -> WebExtensionSize {
        WebExtensionSize::default()
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn zoom_factor(&self, _context: &WebExtensionContext) -> f64 {
        1.0
    }

    /// Sets the corresponding value on `WKWebExtensionTab`.
    fn set_zoom_factor(
        &self,
        _zoom_factor: f64,
        _context: &WebExtensionContext,
    ) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.setZoomFactor",
        ))
    }

    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn url(&self, _context: &WebExtensionContext) -> Option<String> {
        None
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn pending_url(&self, _context: &WebExtensionContext) -> Option<String> {
        None
    }

    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn is_loading_complete(&self, _context: &WebExtensionContext) -> bool {
        true
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn detect_webpage_locale(
        &self,
        _context: &WebExtensionContext,
    ) -> Result<Option<String>, WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.detectWebpageLocaleForWebExtensionContext",
        ))
    }

    /// Calls the corresponding `WKWebExtensionTab` API.
    fn take_snapshot(
        &self,
        _configuration: &SnapshotConfiguration,
        _context: &WebExtensionContext,
    ) -> Result<Option<WebExtensionTabSnapshot>, WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.takeSnapshotUsingConfiguration",
        ))
    }

    /// Calls the corresponding `WKWebExtensionTab` API.
    fn load_url(&self, _url: &str, _context: &WebExtensionContext) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.loadURL",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn reload(
        &self,
        _from_origin: bool,
        _context: &WebExtensionContext,
    ) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.reloadFromOrigin",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn go_back(&self, _context: &WebExtensionContext) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.goBackForWebExtensionContext",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn go_forward(&self, _context: &WebExtensionContext) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.goForwardForWebExtensionContext",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn activate(&self, _context: &WebExtensionContext) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.activateForWebExtensionContext",
        ))
    }

    /// Returns the corresponding value from `WKWebExtensionTab`.
    fn is_selected(&self, _context: &WebExtensionContext) -> bool {
        false
    }

    /// Sets the corresponding value on `WKWebExtensionTab`.
    fn set_selected(
        &self,
        _selected: bool,
        _context: &WebExtensionContext,
    ) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.setSelected",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn duplicate(
        &self,
        _configuration: &WebExtensionTabConfiguration,
        _context: &WebExtensionContext,
    ) -> Result<Option<WebExtensionTabHandle>, WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.duplicateUsingConfiguration",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn close(&self, _context: &WebExtensionContext) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionTab.closeForWebExtensionContext",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn should_grant_permissions_on_user_gesture(&self, _context: &WebExtensionContext) -> bool {
        false
    }

    /// Mirrors the corresponding `WKWebExtensionTab` API.
    fn should_bypass_permissions(&self, _context: &WebExtensionContext) -> bool {
        false
    }
}

/// Trait mirroring `WKWebExtensionWindow`.
pub trait WebExtensionWindow: Send + Sync {
    /// Mirrors the corresponding `WKWebExtensionWindow` API.
    fn tabs(&self, _context: &WebExtensionContext) -> Vec<WebExtensionTabHandle> {
        Vec::new()
    }

    /// Mirrors the corresponding `WKWebExtensionWindow` API.
    fn active_tab(&self, _context: &WebExtensionContext) -> Option<WebExtensionTabHandle> {
        None
    }

    /// Returns the corresponding value from `WKWebExtensionWindow`.
    fn window_type(&self, _context: &WebExtensionContext) -> WebExtensionWindowType {
        WebExtensionWindowType::Normal
    }

    /// Returns the corresponding value from `WKWebExtensionWindow`.
    fn window_state(&self, _context: &WebExtensionContext) -> WebExtensionWindowState {
        WebExtensionWindowState::Normal
    }

    /// Sets the corresponding value on `WKWebExtensionWindow`.
    fn set_window_state(
        &self,
        _state: WebExtensionWindowState,
        _context: &WebExtensionContext,
    ) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionWindow.setWindowState",
        ))
    }

    /// Returns the corresponding value from `WKWebExtensionWindow`.
    fn is_private(&self, _context: &WebExtensionContext) -> bool {
        false
    }

    /// Mirrors the corresponding `WKWebExtensionWindow` API.
    fn screen_frame(&self, _context: &WebExtensionContext) -> Rect {
        Rect::default()
    }

    /// Mirrors the corresponding `WKWebExtensionWindow` API.
    fn frame(&self, _context: &WebExtensionContext) -> Rect {
        Rect::default()
    }

    /// Sets the corresponding value on `WKWebExtensionWindow`.
    fn set_frame(&self, _frame: Rect, _context: &WebExtensionContext) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionWindow.setFrame",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionWindow` API.
    fn focus(&self, _context: &WebExtensionContext) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionWindow.focusForWebExtensionContext",
        ))
    }

    /// Mirrors the corresponding `WKWebExtensionWindow` API.
    fn close(&self, _context: &WebExtensionContext) -> Result<(), WebKitError> {
        Err(unsupported_web_extension_delegate_method(
            "WKWebExtensionWindow.closeForWebExtensionContext",
        ))
    }
}

/// Wraps `NSError`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NSErrorInfo {
    /// Mirrors the `domain` value exposed by `NSError`.
    pub domain: String,
    /// Mirrors the `code` value exposed by `NSError`.
    pub code: i64,
    /// Mirrors the `description` value exposed by `NSError`.
    pub description: String,
}

/// Captures data returned by `WKWebExtension`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionSummary {
    /// Mirrors the `errors` value exposed by `WKWebExtension`.
    #[serde(default)]
    pub errors: Vec<NSErrorInfo>,
    /// Mirrors the `manifest` value exposed by `WKWebExtension`.
    #[serde(default)]
    pub manifest: Value,
    /// Mirrors the `manifest_version` value exposed by `WKWebExtension`.
    pub manifest_version: f64,
    /// Mirrors the `default_locale_identifier` value exposed by `WKWebExtension`.
    pub default_locale_identifier: Option<String>,
    /// Mirrors the `display_name` value exposed by `WKWebExtension`.
    pub display_name: Option<String>,
    /// Mirrors the `display_short_name` value exposed by `WKWebExtension`.
    pub display_short_name: Option<String>,
    /// Mirrors the `display_version` value exposed by `WKWebExtension`.
    pub display_version: Option<String>,
    /// Mirrors the `display_description` value exposed by `WKWebExtension`.
    pub display_description: Option<String>,
    /// Mirrors the `display_action_label` value exposed by `WKWebExtension`.
    pub display_action_label: Option<String>,
    /// Mirrors the `version` value exposed by `WKWebExtension`.
    pub version: Option<String>,
    /// Mirrors the `requested_permissions` value exposed by `WKWebExtension`.
    #[serde(default)]
    pub requested_permissions: Vec<WebExtensionPermission>,
    /// Mirrors the `optional_permissions` value exposed by `WKWebExtension`.
    #[serde(default)]
    pub optional_permissions: Vec<WebExtensionPermission>,
    /// Mirrors the `requested_permission_match_patterns` value exposed by `WKWebExtension`.
    #[serde(default)]
    pub requested_permission_match_patterns: Vec<String>,
    /// Mirrors the `optional_permission_match_patterns` value exposed by `WKWebExtension`.
    #[serde(default)]
    pub optional_permission_match_patterns: Vec<String>,
    /// Mirrors the `all_requested_match_patterns` value exposed by `WKWebExtension`.
    #[serde(default)]
    pub all_requested_match_patterns: Vec<String>,
    /// Mirrors the `has_background_content` value exposed by `WKWebExtension`.
    pub has_background_content: bool,
    /// Mirrors the `has_persistent_background_content` value exposed by `WKWebExtension`.
    pub has_persistent_background_content: bool,
    /// Mirrors the `has_injected_content` value exposed by `WKWebExtension`.
    pub has_injected_content: bool,
    /// Mirrors the `has_options_page` value exposed by `WKWebExtension`.
    pub has_options_page: bool,
    /// Mirrors the `has_override_new_tab_page` value exposed by `WKWebExtension`.
    pub has_override_new_tab_page: bool,
    /// Mirrors the `has_commands` value exposed by `WKWebExtension`.
    pub has_commands: bool,
    /// Mirrors the `has_content_modification_rules` value exposed by `WKWebExtension`.
    pub has_content_modification_rules: bool,
}

/// Captures data returned by `WKWebExtensionMatchPattern`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionMatchPatternSummary {
    /// Mirrors the `string` value exposed by `WKWebExtensionMatchPattern`.
    pub string: String,
    /// Mirrors the `scheme` value exposed by `WKWebExtensionMatchPattern`.
    pub scheme: Option<String>,
    /// Mirrors the `host` value exposed by `WKWebExtensionMatchPattern`.
    pub host: Option<String>,
    /// Mirrors the `path` value exposed by `WKWebExtensionMatchPattern`.
    pub path: Option<String>,
    /// Mirrors the `matches_all_urls` value exposed by `WKWebExtensionMatchPattern`.
    pub matches_all_urls: bool,
    /// Mirrors the `matches_all_hosts` value exposed by `WKWebExtensionMatchPattern`.
    pub matches_all_hosts: bool,
}

/// Captures data returned by `WKWebExtensionContext`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionContextSummary {
    /// Mirrors the `errors` value exposed by `WKWebExtensionContext`.
    #[serde(default)]
    pub errors: Vec<NSErrorInfo>,
    /// Mirrors the `loaded` value exposed by `WKWebExtensionContext`.
    pub loaded: bool,
    /// Mirrors the `base_url` value exposed by `WKWebExtensionContext`.
    pub base_url: Option<String>,
    /// Mirrors the `unique_identifier` value exposed by `WKWebExtensionContext`.
    pub unique_identifier: String,
    /// Mirrors the `inspectable` value exposed by `WKWebExtensionContext`.
    pub inspectable: bool,
    /// Mirrors the `inspection_name` value exposed by `WKWebExtensionContext`.
    pub inspection_name: Option<String>,
    /// Mirrors the `unsupported_apis` value exposed by `WKWebExtensionContext`.
    #[serde(default)]
    pub unsupported_apis: Vec<String>,
    /// Mirrors the `options_page_url` value exposed by `WKWebExtensionContext`.
    pub options_page_url: Option<String>,
    /// Mirrors the `override_new_tab_page_url` value exposed by `WKWebExtensionContext`.
    pub override_new_tab_page_url: Option<String>,
    /// Mirrors the `has_requested_optional_access_to_all_hosts` value exposed by `WKWebExtensionContext`.
    pub has_requested_optional_access_to_all_hosts: bool,
    /// Mirrors the `has_access_to_private_data` value exposed by `WKWebExtensionContext`.
    pub has_access_to_private_data: bool,
    /// Mirrors the `current_permissions` value exposed by `WKWebExtensionContext`.
    #[serde(default)]
    pub current_permissions: Vec<WebExtensionPermission>,
    /// Mirrors the `current_permission_match_patterns` value exposed by `WKWebExtensionContext`.
    #[serde(default)]
    pub current_permission_match_patterns: Vec<String>,
    /// Mirrors the `has_access_to_all_urls` value exposed by `WKWebExtensionContext`.
    pub has_access_to_all_urls: bool,
    /// Mirrors the `has_access_to_all_hosts` value exposed by `WKWebExtensionContext`.
    pub has_access_to_all_hosts: bool,
    /// Mirrors the `has_injected_content` value exposed by `WKWebExtensionContext`.
    pub has_injected_content: bool,
    /// Mirrors the `has_content_modification_rules` value exposed by `WKWebExtensionContext`.
    pub has_content_modification_rules: bool,
    /// Mirrors the `webview_configuration_available` value exposed by `WKWebExtensionContext`.
    pub webview_configuration_available: bool,
}

/// Wraps `WKWebExtensionAction`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionAction {
    /// Mirrors the `label` value exposed by `WKWebExtensionAction`.
    pub label: String,
    /// Mirrors the `badge_text` value exposed by `WKWebExtensionAction`.
    pub badge_text: String,
    /// Mirrors the `has_unread_badge_text` value exposed by `WKWebExtensionAction`.
    pub has_unread_badge_text: bool,
    /// Mirrors the `inspection_name` value exposed by `WKWebExtensionAction`.
    pub inspection_name: Option<String>,
    /// Mirrors the `enabled` value exposed by `WKWebExtensionAction`.
    pub enabled: bool,
    /// Mirrors the `presents_popup` value exposed by `WKWebExtensionAction`.
    pub presents_popup: bool,
    /// Mirrors the `associated_tab_available` value exposed by `WKWebExtensionAction`.
    pub associated_tab_available: bool,
    /// Mirrors the `popup_webview_available` value exposed by `WKWebExtensionAction`.
    pub popup_webview_available: bool,
}

/// Wraps `WKWebExtensionCommand`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionCommand {
    /// Mirrors the `identifier` value exposed by `WKWebExtensionCommand`.
    pub identifier: String,
    /// Mirrors the `title` value exposed by `WKWebExtensionCommand`.
    pub title: String,
    /// Mirrors the `activation_key` value exposed by `WKWebExtensionCommand`.
    pub activation_key: Option<String>,
    /// Mirrors the `modifier_flags` value exposed by `WKWebExtensionCommand`.
    pub modifier_flags: u64,
}

/// Wraps `WKWebExtensionDataRecord`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionDataRecord {
    /// Mirrors the `display_name` value exposed by `WKWebExtensionDataRecord`.
    pub display_name: String,
    /// Mirrors the `unique_identifier` value exposed by `WKWebExtensionDataRecord`.
    pub unique_identifier: String,
    /// Mirrors the `contained_data_types` value exposed by `WKWebExtensionDataRecord`.
    #[serde(default)]
    pub contained_data_types: Vec<WebExtensionDataType>,
    /// Mirrors the `errors` value exposed by `WKWebExtensionDataRecord`.
    #[serde(default)]
    pub errors: Vec<NSErrorInfo>,
    /// Mirrors the `total_size_in_bytes` value exposed by `WKWebExtensionDataRecord`.
    pub total_size_in_bytes: u64,
    /// Mirrors the `size_in_bytes_by_type` value exposed by `WKWebExtensionDataRecord`.
    #[serde(default)]
    pub size_in_bytes_by_type: BTreeMap<String, u64>,
}

impl WebExtensionDataRecord {
    /// Returns the corresponding value from `WKWebExtensionDataRecord`.
    #[must_use]
    pub fn size_in_bytes_of_types(&self, data_types: &[WebExtensionDataType]) -> u64 {
        data_types
            .iter()
            .filter_map(|data_type| self.size_in_bytes_by_type.get(data_type.as_str()))
            .copied()
            .sum()
    }
}

/// Configures `WKWebExtensionTab`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionTabConfiguration {
    /// Mirrors the `has_window` value exposed by `WKWebExtensionTab`.
    pub has_window: bool,
    /// Mirrors the `index` value exposed by `WKWebExtensionTab`.
    pub index: usize,
    /// Mirrors the `has_parent_tab` value exposed by `WKWebExtensionTab`.
    pub has_parent_tab: bool,
    /// Mirrors the `url` value exposed by `WKWebExtensionTab`.
    pub url: Option<String>,
    /// Mirrors the `should_be_active` value exposed by `WKWebExtensionTab`.
    pub should_be_active: bool,
    /// Mirrors the `should_add_to_selection` value exposed by `WKWebExtensionTab`.
    pub should_add_to_selection: bool,
    /// Mirrors the `should_be_pinned` value exposed by `WKWebExtensionTab`.
    pub should_be_pinned: bool,
    /// Mirrors the `should_be_muted` value exposed by `WKWebExtensionTab`.
    pub should_be_muted: bool,
    /// Mirrors the `should_reader_mode_be_active` value exposed by `WKWebExtensionTab`.
    pub should_reader_mode_be_active: bool,
}

/// Configures `WKWebExtensionWindow`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionWindowConfiguration {
    /// Mirrors the `window_type` value exposed by `WKWebExtensionWindow`.
    pub window_type: Option<WebExtensionWindowType>,
    /// Mirrors the `window_state` value exposed by `WKWebExtensionWindow`.
    pub window_state: Option<WebExtensionWindowState>,
    /// Mirrors the `frame` value exposed by `WKWebExtensionWindow`.
    pub frame: Rect,
    /// Mirrors the `tab_urls` value exposed by `WKWebExtensionWindow`.
    #[serde(default)]
    pub tab_urls: Vec<String>,
    /// Mirrors the `tab_count` value exposed by `WKWebExtensionWindow`.
    pub tab_count: usize,
    /// Mirrors the `should_be_focused` value exposed by `WKWebExtensionWindow`.
    pub should_be_focused: bool,
    /// Mirrors the `should_be_private` value exposed by `WKWebExtensionWindow`.
    pub should_be_private: bool,
}

/// Wraps `WKWebExtension`.
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

    /// Creates a value for `WKWebExtension`.
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

    /// Creates a value for `WKWebExtension`.
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

    /// Returns the corresponding value from `WKWebExtension`.
    #[must_use]
    pub fn summary(&self) -> WebExtensionSummary {
        unsafe { take_json_or_default(ffi::wk_web_extension_copy_summary_json(self.ptr)) }
    }

    /// Returns the corresponding value from `WKWebExtension`.
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

/// Wraps `WKWebExtensionMatchPattern`.
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

    /// Calls the corresponding `WKWebExtensionMatchPattern` API.
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

    /// Returns the corresponding value from `WKWebExtensionMatchPattern`.
    #[must_use]
    pub fn all_urls() -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_match_pattern_all_urls() })
            .expect("wk_web_extension_match_pattern_all_urls returned null")
    }

    /// Returns the corresponding value from `WKWebExtensionMatchPattern`.
    #[must_use]
    pub fn all_hosts_and_schemes() -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_match_pattern_all_hosts_and_schemes() })
            .expect("wk_web_extension_match_pattern_all_hosts_and_schemes returned null")
    }

    /// Creates a value for `WKWebExtensionMatchPattern`.
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

    /// Sets the corresponding option used by `WKWebExtensionMatchPattern`.
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

    /// Returns the corresponding value from `WKWebExtensionMatchPattern`.
    #[must_use]
    pub fn summary(&self) -> WebExtensionMatchPatternSummary {
        unsafe {
            take_json_or_default(ffi::wk_web_extension_match_pattern_copy_summary_json(
                self.ptr,
            ))
        }
    }

    /// Calls the corresponding `WKWebExtensionMatchPattern` API.
    #[must_use]
    pub fn matches_url(&self, url: &str) -> bool {
        self.matches_url_with_options(url, WebExtensionMatchPatternOptions::NONE)
    }

    /// Calls the corresponding `WKWebExtensionMatchPattern` API.
    #[must_use]
    pub fn matches_url_with_options(
        &self,
        url: &str,
        options: WebExtensionMatchPatternOptions,
    ) -> bool {
        let url = to_cstring(url);
        unsafe {
            ffi::wk_web_extension_match_pattern_matches_url(self.ptr, url.as_ptr(), options.bits())
        }
    }

    /// Calls the corresponding `WKWebExtensionMatchPattern` API.
    #[must_use]
    pub fn matches_pattern(&self, other: &Self) -> bool {
        self.matches_pattern_with_options(other, WebExtensionMatchPatternOptions::NONE)
    }

    /// Calls the corresponding `WKWebExtensionMatchPattern` API.
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

/// Configures `WKWebExtensionController.Configuration`.
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

    /// Creates a value for `WKWebExtensionController.Configuration`.
    #[must_use]
    pub fn default_configuration() -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_controller_configuration_default() })
            .expect("wk_web_extension_controller_configuration_default returned null")
    }

    /// Mirrors the corresponding `WKWebExtensionController.Configuration` API.
    #[must_use]
    pub fn non_persistent_configuration() -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_controller_configuration_nonpersistent() })
            .expect("wk_web_extension_controller_configuration_nonpersistent returned null")
    }

    /// Mirrors the corresponding `WKWebExtensionController.Configuration` API.
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

    /// Returns the corresponding value from `WKWebExtensionController.Configuration`.
    #[must_use]
    pub fn summary(&self) -> WebExtensionControllerConfigurationSummary {
        unsafe {
            take_json_or_default(
                ffi::wk_web_extension_controller_configuration_copy_summary_json(self.ptr),
            )
        }
    }

    /// Sets the corresponding value on `WKWebExtensionController.Configuration`.
    pub fn set_webview_configuration(&self, configuration: &WebViewConfiguration) {
        unsafe {
            ffi::wk_web_extension_controller_configuration_set_webview_configuration(
                self.ptr,
                configuration.as_ptr(),
            );
        }
    }

    /// Returns the corresponding value from `WKWebExtensionController.Configuration`.
    #[must_use]
    pub fn webview_configuration(&self) -> Option<WebViewConfiguration> {
        WebViewConfiguration::from_ptr(unsafe {
            ffi::wk_web_extension_controller_configuration_copy_webview_configuration(self.ptr)
        })
    }

    /// Sets the corresponding value on `WKWebExtensionController.Configuration`.
    pub fn set_default_website_data_store(&self, store: &WebsiteDataStore) {
        unsafe {
            ffi::wk_web_extension_controller_configuration_set_default_website_data_store(
                self.ptr,
                store.as_ptr(),
            );
        }
    }

    /// Returns the corresponding value from `WKWebExtensionController.Configuration`.
    #[must_use]
    pub fn default_website_data_store(&self) -> Option<WebsiteDataStore> {
        WebsiteDataStore::from_ptr(unsafe {
            ffi::wk_web_extension_controller_configuration_copy_default_website_data_store(self.ptr)
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

/// Captures data returned by `WKWebExtensionController.Configuration`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebExtensionControllerConfigurationSummary {
    /// Mirrors the `persistent` value exposed by `WKWebExtensionController.Configuration`.
    pub persistent: bool,
    /// Mirrors the `identifier` value exposed by `WKWebExtensionController.Configuration`.
    pub identifier: Option<String>,
}

/// Wraps `WKWebExtensionController`.
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

    /// Creates a value for `WKWebExtensionController`.
    #[must_use]
    pub fn new() -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_controller_new() })
            .expect("wk_web_extension_controller_new returned null")
    }

    /// Sets the corresponding option used by `WKWebExtensionController`.
    #[must_use]
    pub fn with_configuration(configuration: &WebExtensionControllerConfiguration) -> Self {
        Self::from_ptr(unsafe {
            ffi::wk_web_extension_controller_with_configuration(configuration.as_ptr())
        })
        .expect("wk_web_extension_controller_with_configuration returned null")
    }

    /// Mirrors the corresponding `WKWebExtensionController` API.
    #[must_use]
    pub fn configuration(&self) -> Option<WebExtensionControllerConfiguration> {
        WebExtensionControllerConfiguration::from_ptr(unsafe {
            ffi::wk_web_extension_controller_copy_configuration(self.ptr)
        })
    }

    /// Mirrors the corresponding `WKWebExtensionController` API.
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

    /// Mirrors the corresponding `WKWebExtensionController` API.
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

    /// Mirrors the corresponding `WKWebExtensionController` API.
    #[must_use]
    pub fn extension_context_for_extension(
        &self,
        extension: &WebExtension,
    ) -> Option<WebExtensionContext> {
        WebExtensionContext::from_ptr(unsafe {
            ffi::wk_web_extension_controller_copy_context_for_extension(self.ptr, extension.ptr)
        })
    }

    /// Mirrors the corresponding `WKWebExtensionController` API.
    #[must_use]
    pub fn extension_context_for_url(&self, url: &str) -> Option<WebExtensionContext> {
        let url = to_cstring(url);
        WebExtensionContext::from_ptr(unsafe {
            ffi::wk_web_extension_controller_copy_context_for_url(self.ptr, url.as_ptr())
        })
    }

    /// Returns the corresponding value from `WKWebExtensionController`.
    #[must_use]
    pub fn all_extension_data_types() -> Vec<WebExtensionDataType> {
        unsafe { take_json_or_default(ffi::wk_web_extension_controller_copy_all_data_types_json()) }
    }

    /// Calls the corresponding `WKWebExtensionController` API.
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

    /// Mirrors the corresponding `WKWebExtensionController` API.
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

    /// Calls the corresponding `WKWebExtensionController` API.
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

/// Wraps `WKWebExtensionContext`.
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

    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn for_extension(extension: &WebExtension) -> Self {
        Self::from_ptr(unsafe { ffi::wk_web_extension_context_new_for_extension(extension.ptr) })
            .expect("wk_web_extension_context_new_for_extension returned null")
    }

    /// Returns the corresponding value from `WKWebExtensionContext`.
    #[must_use]
    pub fn summary(&self) -> WebExtensionContextSummary {
        unsafe { take_json_or_default(ffi::wk_web_extension_context_copy_summary_json(self.ptr)) }
    }

    /// Sets the corresponding value on `WKWebExtensionContext`.
    pub fn set_base_url(&self, url: &str) -> Result<(), WebKitError> {
        let url = to_cstring(url);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_web_extension_context_set_base_url(self.ptr, url.as_ptr(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Sets the corresponding value on `WKWebExtensionContext`.
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

    /// Sets the corresponding value on `WKWebExtensionContext`.
    pub fn set_inspectable(&self, value: bool) {
        unsafe { ffi::wk_web_extension_context_set_inspectable(self.ptr, value) }
    }

    /// Sets the corresponding value on `WKWebExtensionContext`.
    pub fn set_inspection_name(&self, name: Option<&str>) {
        let name = name.map(to_cstring);
        let name_ptr = name.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        unsafe { ffi::wk_web_extension_context_set_inspection_name(self.ptr, name_ptr) }
    }

    /// Sets the corresponding value on `WKWebExtensionContext`.
    pub fn set_unsupported_apis(&self, apis: &[String]) {
        let apis_json = to_json_cstring(apis);
        unsafe {
            ffi::wk_web_extension_context_set_unsupported_apis_json(self.ptr, apis_json.as_ptr());
        }
    }

    /// Sets the corresponding value on `WKWebExtensionContext`.
    pub fn set_requested_optional_access_to_all_hosts(&self, value: bool) {
        unsafe {
            ffi::wk_web_extension_context_set_requested_optional_access_to_all_hosts(
                self.ptr, value,
            );
        }
    }

    /// Sets the corresponding value on `WKWebExtensionContext`.
    pub fn set_access_to_private_data(&self, value: bool) {
        unsafe { ffi::wk_web_extension_context_set_access_to_private_data(self.ptr, value) }
    }

    /// Returns the corresponding value from `WKWebExtensionContext`.
    #[must_use]
    pub fn webview_configuration(&self) -> Option<WebViewConfiguration> {
        WebViewConfiguration::from_ptr(unsafe {
            ffi::wk_web_extension_context_copy_webview_configuration(self.ptr)
        })
    }

    /// Returns the corresponding value from `WKWebExtensionContext`.
    #[must_use]
    pub fn has_permission(&self, permission: &WebExtensionPermission) -> bool {
        let permission = to_cstring(permission.as_str());
        unsafe { ffi::wk_web_extension_context_has_permission(self.ptr, permission.as_ptr()) }
    }

    /// Returns the corresponding value from `WKWebExtensionContext`.
    #[must_use]
    pub fn has_access_to_url(&self, url: &str) -> bool {
        let url = to_cstring(url);
        unsafe { ffi::wk_web_extension_context_has_access_to_url(self.ptr, url.as_ptr()) }
    }

    /// Returns the corresponding value from `WKWebExtensionContext`.
    #[must_use]
    pub fn permission_status_for_permission(
        &self,
        permission: &WebExtensionPermission,
    ) -> WebExtensionContextPermissionStatus {
        let permission = to_cstring(permission.as_str());
        match unsafe {
            ffi::wk_web_extension_context_permission_status_for_permission(
                self.ptr,
                permission.as_ptr(),
            )
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

    /// Sets the corresponding value on `WKWebExtensionContext`.
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

    /// Returns the corresponding value from `WKWebExtensionContext`.
    #[must_use]
    pub fn permission_status_for_url(&self, url: &str) -> WebExtensionContextPermissionStatus {
        let url = to_cstring(url);
        match unsafe {
            ffi::wk_web_extension_context_permission_status_for_url(self.ptr, url.as_ptr())
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

    /// Sets the corresponding value on `WKWebExtensionContext`.
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

    /// Returns the corresponding value from `WKWebExtensionContext`.
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

    /// Sets the corresponding value on `WKWebExtensionContext`.
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

    /// Calls the corresponding `WKWebExtensionContext` API.
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

    /// Returns the corresponding value from `WKWebExtensionContext`.
    #[must_use]
    pub fn action(&self) -> Option<WebExtensionAction> {
        unsafe {
            take_json_or_default(ffi::wk_web_extension_context_copy_default_action_json(
                self.ptr,
            ))
        }
    }

    /// Calls the corresponding `WKWebExtensionContext` API.
    pub fn perform_action(&self) {
        unsafe { ffi::wk_web_extension_context_perform_default_action(self.ptr) }
    }

    /// Returns the corresponding value from `WKWebExtensionContext`.
    #[must_use]
    pub fn commands(&self) -> Vec<WebExtensionCommand> {
        unsafe { take_json_or_default(ffi::wk_web_extension_context_copy_commands_json(self.ptr)) }
    }

    /// Calls the corresponding `WKWebExtensionContext` API.
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

    /// Calls the corresponding `WKWebExtensionContext` API.
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
    /// Returns the corresponding value from `WKWebExtensionMessagePort`.
    #[must_use]
    pub fn application_identifier(&self) -> Option<String> {
        unsafe {
            take_optional_string(
                ffi::wk_web_extension_message_port_copy_application_identifier(self.ptr),
            )
        }
    }

    /// Returns the corresponding value from `WKWebExtensionMessagePort`.
    #[must_use]
    pub fn is_disconnected(&self) -> bool {
        unsafe { ffi::wk_web_extension_message_port_is_disconnected(self.ptr) }
    }

    /// Calls the corresponding `WKWebExtensionMessagePort` API.
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

    /// Calls the corresponding `WKWebExtensionMessagePort` API.
    pub fn disconnect(&self) {
        unsafe { ffi::wk_web_extension_message_port_disconnect(self.ptr) }
    }

    /// Calls the corresponding `WKWebExtensionMessagePort` API.
    pub fn disconnect_with_error(&self, message: &str) {
        let message = to_cstring(message);
        unsafe {
            ffi::wk_web_extension_message_port_disconnect_with_error(self.ptr, message.as_ptr());
        }
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
    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn errors_did_update() -> &'static str {
        web_extension_constants()
            .errors_did_update_notification
            .as_str()
    }

    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn permissions_were_granted() -> &'static str {
        web_extension_constants()
            .permissions_were_granted_notification
            .as_str()
    }

    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn permissions_were_denied() -> &'static str {
        web_extension_constants()
            .permissions_were_denied_notification
            .as_str()
    }

    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn granted_permissions_were_removed() -> &'static str {
        web_extension_constants()
            .granted_permissions_were_removed_notification
            .as_str()
    }

    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn denied_permissions_were_removed() -> &'static str {
        web_extension_constants()
            .denied_permissions_were_removed_notification
            .as_str()
    }

    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn permission_match_patterns_were_granted() -> &'static str {
        web_extension_constants()
            .permission_match_patterns_were_granted_notification
            .as_str()
    }

    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn permission_match_patterns_were_denied() -> &'static str {
        web_extension_constants()
            .permission_match_patterns_were_denied_notification
            .as_str()
    }

    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn granted_permission_match_patterns_were_removed() -> &'static str {
        web_extension_constants()
            .granted_permission_match_patterns_were_removed_notification
            .as_str()
    }

    /// Mirrors the corresponding `WKWebExtensionContext` API.
    #[must_use]
    pub fn denied_permission_match_patterns_were_removed() -> &'static str {
        web_extension_constants()
            .denied_permission_match_patterns_were_removed_notification
            .as_str()
    }
}
