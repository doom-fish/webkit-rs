#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;
/// Rust bindings for `NSAttributedString`.
pub mod attributed_string;
/// Rust bindings for `WKBackForwardList`.
pub mod back_forward_list;
/// Rust bindings for `WKWebViewConfiguration`.
pub mod config;
/// Rust bindings for `WKContentRuleListStore`.
pub mod content_rule_list_store;
/// Rust bindings for WebKit context-menu and preview metadata types.
pub mod context_menu;
/// Rust bindings for `WKDownload`.
pub mod download;
/// Rust bindings for `WKErrorDomain`.
pub mod error;
pub mod ffi;
/// Rust bindings for `WKFindConfiguration`.
pub mod find;
/// Rust bindings for `CGRect`.
pub mod geometry;
/// Rust bindings for `WKHTTPCookieStore`.
pub mod http_cookie_store;
/// Rust bindings for `WKNavigation`.
pub mod navigation;
/// Rust bindings for `WKNavigationDelegate`.
pub mod navigation_delegate;
/// Rust bindings for `WKPDFConfiguration`.
pub mod pdf_configuration;
/// Rust bindings for `WKPreferences`.
pub mod preferences;
mod private;
/// Rust bindings for Network.framework proxy configuration handles used by `WKWebsiteDataStore`.
pub mod proxy_configuration;
/// Rust bindings for `WKScriptMessage`.
pub mod script_message_handler;
/// Rust bindings for `WKSnapshotConfiguration`.
pub mod snapshot_configuration;
/// Rust bindings for `WKUIDelegate`.
pub mod ui_delegate;
/// Rust bindings for `WKURLSchemeHandler`.
pub mod url_scheme;
/// Rust bindings for `WKUserScript`.
pub mod user_script;
/// Rust bindings for `WKWebExtension`.
pub mod web_extension;
/// Rust bindings for `WKWebsiteDataStore`.
pub mod website_data_store;
/// Rust bindings for `WKWebView`.
pub mod webview;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub use async_api::{
    AsyncContentRuleListStore, AsyncDownload, AsyncHttpCookieStore, AsyncWebView,
    AsyncWebsiteDataStore, CallAsyncJavaScriptFuture, CancelWithResumeDataFuture,
    CompileRuleListFuture, CreatePdfFuture, CreateWebArchiveFuture, DownloadCancelFuture,
    EvaluateJavaScriptFuture, FetchDataRecordsFuture, FindStringFuture, GetAllCookiesFuture,
    RemoveDataFuture, TakeSnapshotFuture,
};
pub use attributed_string::{
    AttributedString, AttributedStringCompletionHandler, AttributedStringLoadOptions,
    HtmlLoadRequest, READ_ACCESS_URL_DOCUMENT_OPTION,
};
pub use back_forward_list::{BackForwardList, BackForwardListItem};
pub use config::{AudiovisualMediaTypes, UserInterfaceDirectionPolicy, WebViewConfiguration};
pub use content_rule_list_store::{ContentRuleList, ContentRuleListStore};
pub use context_menu::{ContextMenuElementInfo, PreviewActionItem, PreviewElementInfo};
pub use download::{Download, DownloadEvent, DownloadRedirectPolicy};
pub use error::{WebKitError, WebKitErrorCode, WEBKIT_ERROR_DOMAIN};
pub use find::{FindConfiguration, FindResult, TextFinderAction};
pub use geometry::Rect;
pub use http_cookie_store::{Cookie, CookiePolicy, CookieStoreEvent, HttpCookieStore};
pub use navigation::Navigation;
pub use navigation_delegate::{
    BackForwardListNavigationEvent, BackForwardListNavigationPolicy, FrameInfo, NavigationAction,
    NavigationActionPolicy, NavigationDelegateConfig, NavigationEvent, NavigationEventKind,
    NavigationResponse, NavigationResponsePolicy, NavigationType,
};
pub use pdf_configuration::PDFConfiguration;
pub use preferences::{InactiveSchedulingPolicy, Preferences, UpgradeToHTTPSPolicy};
pub use proxy_configuration::{ProxyConfiguration, ProxyConfigurationSummary};
pub use script_message_handler::ScriptMessage;
pub use snapshot_configuration::SnapshotConfiguration;
pub use ui_delegate::{
    MediaCaptureType, OpenPanelParameters, PermissionDecision, SecurityOrigin, UIDelegateConfig,
    UIDelegateEvent, UIDelegateEventDetail, WindowFeatures,
};
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
pub use webview::{
    FullscreenState, MediaCaptureState, MediaPlaybackState, WebView, WebViewDataType,
};

/// Convenience re-exports for the WebKit bindings.
pub mod prelude {
    #[cfg(feature = "async")]
    pub use crate::async_api::{
        AsyncContentRuleListStore, AsyncDownload, AsyncHttpCookieStore, AsyncWebView,
        AsyncWebsiteDataStore, CallAsyncJavaScriptFuture, CancelWithResumeDataFuture,
        CompileRuleListFuture, CreatePdfFuture, CreateWebArchiveFuture, DownloadCancelFuture,
        EvaluateJavaScriptFuture, FetchDataRecordsFuture, FindStringFuture, GetAllCookiesFuture,
        RemoveDataFuture, TakeSnapshotFuture,
    };
    pub use crate::attributed_string::{
        AttributedString, AttributedStringCompletionHandler, AttributedStringLoadOptions,
        HtmlLoadRequest, READ_ACCESS_URL_DOCUMENT_OPTION,
    };
    pub use crate::back_forward_list::{BackForwardList, BackForwardListItem};
    pub use crate::config::{
        AudiovisualMediaTypes, UserInterfaceDirectionPolicy, WebViewConfiguration,
    };
    pub use crate::content_rule_list_store::{ContentRuleList, ContentRuleListStore};
    pub use crate::context_menu::{ContextMenuElementInfo, PreviewActionItem, PreviewElementInfo};
    pub use crate::download::{Download, DownloadEvent, DownloadRedirectPolicy};
    pub use crate::error::{WebKitError, WebKitErrorCode, WEBKIT_ERROR_DOMAIN};
    pub use crate::find::{FindConfiguration, FindResult, TextFinderAction};
    pub use crate::geometry::Rect;
    pub use crate::http_cookie_store::{Cookie, CookiePolicy, CookieStoreEvent, HttpCookieStore};
    pub use crate::navigation::Navigation;
    pub use crate::navigation_delegate::{
        BackForwardListNavigationEvent, BackForwardListNavigationPolicy, FrameInfo,
        NavigationAction, NavigationActionPolicy, NavigationDelegateConfig, NavigationEvent,
        NavigationEventKind, NavigationResponse, NavigationResponsePolicy, NavigationType,
    };
    pub use crate::pdf_configuration::PDFConfiguration;
    pub use crate::preferences::{InactiveSchedulingPolicy, Preferences, UpgradeToHTTPSPolicy};
    pub use crate::proxy_configuration::{ProxyConfiguration, ProxyConfigurationSummary};
    pub use crate::script_message_handler::ScriptMessage;
    pub use crate::snapshot_configuration::SnapshotConfiguration;
    pub use crate::ui_delegate::{
        MediaCaptureType, OpenPanelParameters, PermissionDecision, SecurityOrigin,
        UIDelegateConfig, UIDelegateEvent, UIDelegateEventDetail, WindowFeatures,
    };
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
    pub use crate::webview::{
        FullscreenState, MediaCaptureState, MediaPlaybackState, WebView, WebViewDataType,
    };
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
