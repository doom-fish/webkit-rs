#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

pub mod back_forward_list;
pub mod config;
pub mod content_rule_list_store;
pub mod download;
pub mod error;
pub mod ffi;
pub mod find;
pub mod geometry;
pub mod http_cookie_store;
pub mod navigation;
pub mod navigation_delegate;
pub mod pdf_configuration;
pub mod preferences;
mod private;
pub mod script_message_handler;
pub mod snapshot_configuration;
pub mod ui_delegate;
pub mod url_scheme;
pub mod user_script;
pub mod web_extension;
pub mod website_data_store;
pub mod webview;

pub use back_forward_list::{BackForwardList, BackForwardListItem};
pub use config::WebViewConfiguration;
pub use content_rule_list_store::{ContentRuleList, ContentRuleListStore};
pub use download::{Download, DownloadEvent};
pub use error::WebKitError;
pub use find::{FindConfiguration, FindResult};
pub use geometry::Rect;
pub use http_cookie_store::{Cookie, CookiePolicy, CookieStoreEvent, HttpCookieStore};
pub use navigation::Navigation;
pub use navigation_delegate::{
    FrameInfo, NavigationAction, NavigationActionPolicy, NavigationDelegateConfig, NavigationEvent,
    NavigationEventKind, NavigationResponse, NavigationResponsePolicy, NavigationType,
};
pub use pdf_configuration::PDFConfiguration;
pub use preferences::{InactiveSchedulingPolicy, Preferences, UpgradeToHTTPSPolicy};
pub use script_message_handler::ScriptMessage;
pub use snapshot_configuration::SnapshotConfiguration;
pub use ui_delegate::{UIDelegateConfig, UIDelegateEvent};
pub use url_scheme::{UrlSchemeHandler, UrlSchemeRequest, UrlSchemeResponse, UrlSchemeTask};
pub use user_script::{InjectionTime, UserScript};
pub use web_extension::{
    NSErrorInfo, WebExtension, WebExtensionAction, WebExtensionCommand, WebExtensionContext,
    WebExtensionContextError, WebExtensionContextNotificationUserInfoKey,
    WebExtensionContextNotifications, WebExtensionContextPermissionStatus, WebExtensionController,
    WebExtensionControllerConfiguration, WebExtensionControllerConfigurationSummary,
    WebExtensionControllerDelegate, WebExtensionDataRecord, WebExtensionDataRecordError,
    WebExtensionDataType, WebExtensionError, WebExtensionGrant, WebExtensionMatchPattern,
    WebExtensionMatchPatternError, WebExtensionMatchPatternGrant, WebExtensionMatchPatternOptions,
    WebExtensionMatchPatternSummary, WebExtensionMessagePort, WebExtensionMessagePortError,
    WebExtensionPermission, WebExtensionPermissionGrant, WebExtensionSize, WebExtensionSummary,
    WebExtensionTab, WebExtensionTabChangedProperties, WebExtensionTabConfiguration,
    WebExtensionTabHandle, WebExtensionTabSnapshot, WebExtensionUrlGrant,
    WebExtensionWebViewHandle, WebExtensionWindow, WebExtensionWindowConfiguration,
    WebExtensionWindowHandle, WebExtensionWindowState, WebExtensionWindowType,
};
pub use website_data_store::{WebsiteDataRecord, WebsiteDataStore, WebsiteDataType};
pub use webview::WebView;

pub mod prelude {
    pub use crate::back_forward_list::{BackForwardList, BackForwardListItem};
    pub use crate::config::WebViewConfiguration;
    pub use crate::content_rule_list_store::{ContentRuleList, ContentRuleListStore};
    pub use crate::download::{Download, DownloadEvent};
    pub use crate::error::WebKitError;
    pub use crate::find::{FindConfiguration, FindResult};
    pub use crate::geometry::Rect;
    pub use crate::http_cookie_store::{Cookie, CookiePolicy, CookieStoreEvent, HttpCookieStore};
    pub use crate::navigation::Navigation;
    pub use crate::navigation_delegate::{
        FrameInfo, NavigationAction, NavigationActionPolicy, NavigationDelegateConfig,
        NavigationEvent, NavigationEventKind, NavigationResponse, NavigationResponsePolicy,
        NavigationType,
    };
    pub use crate::pdf_configuration::PDFConfiguration;
    pub use crate::preferences::{InactiveSchedulingPolicy, Preferences, UpgradeToHTTPSPolicy};
    pub use crate::script_message_handler::ScriptMessage;
    pub use crate::snapshot_configuration::SnapshotConfiguration;
    pub use crate::ui_delegate::{UIDelegateConfig, UIDelegateEvent};
    pub use crate::url_scheme::{
        UrlSchemeHandler, UrlSchemeRequest, UrlSchemeResponse, UrlSchemeTask,
    };
    pub use crate::user_script::{InjectionTime, UserScript};
    pub use crate::web_extension::{
        NSErrorInfo, WebExtension, WebExtensionAction, WebExtensionCommand, WebExtensionContext,
        WebExtensionContextError, WebExtensionContextNotificationUserInfoKey,
        WebExtensionContextNotifications, WebExtensionContextPermissionStatus,
        WebExtensionController, WebExtensionControllerConfiguration,
        WebExtensionControllerConfigurationSummary, WebExtensionControllerDelegate,
        WebExtensionDataRecord, WebExtensionDataRecordError, WebExtensionDataType,
        WebExtensionError, WebExtensionGrant, WebExtensionMatchPattern,
        WebExtensionMatchPatternError, WebExtensionMatchPatternGrant,
        WebExtensionMatchPatternOptions, WebExtensionMatchPatternSummary, WebExtensionMessagePort,
        WebExtensionMessagePortError, WebExtensionPermission, WebExtensionPermissionGrant,
        WebExtensionSize, WebExtensionSummary, WebExtensionTab, WebExtensionTabChangedProperties,
        WebExtensionTabConfiguration, WebExtensionTabHandle, WebExtensionTabSnapshot,
        WebExtensionUrlGrant, WebExtensionWebViewHandle, WebExtensionWindow,
        WebExtensionWindowConfiguration, WebExtensionWindowHandle, WebExtensionWindowState,
        WebExtensionWindowType,
    };
    pub use crate::website_data_store::{WebsiteDataRecord, WebsiteDataStore, WebsiteDataType};
    pub use crate::webview::WebView;
}

/// Pump the main run loop for the given duration.
///
/// Call this after operations that require the run loop to process events
/// (for example, receiving script messages posted by JavaScript).
pub fn pump_run_loop(seconds: f64) {
    unsafe { ffi::wk_run_loop_pump(seconds) }
}

/// Initialise `NSApplication.shared` and set activation policy to
/// `.prohibited` (headless). Must be called once before creating any
/// [`WebView`].
pub fn init_app() {
    unsafe { ffi::wk_init_app() }
}
