use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Wraps `WKNavigationActionPolicy` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum NavigationActionPolicy {
    /// Mirrors the `Cancel` case used by `WKNavigationActionPolicy`.
    Cancel = 0,
    /// Mirrors the `Allow` case used by `WKNavigationActionPolicy`.
    Allow = 1,
    /// Mirrors the `Download` case used by `WKNavigationActionPolicy`.
    Download = 2,
}

impl NavigationActionPolicy {
    /// Returns the corresponding value from `WKNavigationActionPolicy`.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }
}

/// Wraps `WKNavigationResponsePolicy` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum NavigationResponsePolicy {
    /// Mirrors the `Cancel` case used by `WKNavigationResponsePolicy`.
    Cancel = 0,
    /// Mirrors the `Allow` case used by `WKNavigationResponsePolicy`.
    Allow = 1,
    /// Mirrors the `Download` case used by `WKNavigationResponsePolicy`.
    Download = 2,
}

impl NavigationResponsePolicy {
    /// Returns the corresponding value from `WKNavigationResponsePolicy`.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }
}

/// Configures `WKNavigationDelegate`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationDelegateConfig {
    /// Mirrors the `action_policy` value exposed by `WKNavigationDelegate`.
    pub action_policy: NavigationActionPolicy,
    /// Mirrors the `response_policy` value exposed by `WKNavigationDelegate`.
    pub response_policy: NavigationResponsePolicy,
}

impl Default for NavigationDelegateConfig {
    fn default() -> Self {
        Self {
            action_policy: NavigationActionPolicy::Allow,
            response_policy: NavigationResponsePolicy::Allow,
        }
    }
}

/// Wraps `WKNavigationType` values.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
#[serde(rename_all = "camelCase")]
pub enum NavigationType {
    /// Mirrors the `LinkActivated` case used by `WKNavigationType`.
    LinkActivated = 0,
    /// Mirrors the `FormSubmitted` case used by `WKNavigationType`.
    FormSubmitted = 1,
    /// Mirrors the `BackForward` case used by `WKNavigationType`.
    BackForward = 2,
    /// Mirrors the `Reload` case used by `WKNavigationType`.
    Reload = 3,
    /// Mirrors the `FormResubmitted` case used by `WKNavigationType`.
    FormResubmitted = 4,
    /// Mirrors the `Other` case used by `WKNavigationType`.
    #[default]
    #[serde(other)]
    Other = -1,
}

impl NavigationType {
    /// Returns the corresponding value from `WKNavigationType`.
    #[must_use]
    pub const fn as_raw(self) -> i64 {
        self as i64
    }

    /// Creates a value for `WKNavigationType`.
    #[must_use]
    pub const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::LinkActivated,
            1 => Self::FormSubmitted,
            2 => Self::BackForward,
            3 => Self::Reload,
            4 => Self::FormResubmitted,
            _ => Self::Other,
        }
    }
}

/// Wraps `WKFrameInfo`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FrameInfo {
    /// Mirrors the `main_frame` value exposed by `WKFrameInfo`.
    pub main_frame: bool,
    /// Mirrors the `request_url` value exposed by `WKFrameInfo`.
    pub request_url: String,
    /// Mirrors the `request_method` value exposed by `WKFrameInfo`.
    pub request_method: String,
    /// Mirrors the `security_origin_protocol` value exposed by `WKFrameInfo`.
    pub security_origin_protocol: Option<String>,
    /// Mirrors the `security_origin_host` value exposed by `WKFrameInfo`.
    pub security_origin_host: Option<String>,
    /// Mirrors the `security_origin_port` value exposed by `WKFrameInfo`.
    pub security_origin_port: Option<i64>,
    /// Mirrors the `webview_url` value exposed by `WKFrameInfo`.
    pub webview_url: Option<String>,
}

/// Wraps `WKNavigationAction`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationAction {
    /// Mirrors the `navigation_type` value exposed by `WKNavigationAction`.
    pub navigation_type: NavigationType,
    /// Mirrors the `navigation_type_raw_value` value exposed by `WKNavigationAction`.
    pub navigation_type_raw_value: i64,
    /// Mirrors the `request_url` value exposed by `WKNavigationAction`.
    pub request_url: String,
    /// Mirrors the `request_method` value exposed by `WKNavigationAction`.
    pub request_method: String,
    /// Mirrors the `request_headers` value exposed by `WKNavigationAction`.
    #[serde(default)]
    pub request_headers: BTreeMap<String, String>,
    /// Mirrors the `source_frame` value exposed by `WKNavigationAction`.
    pub source_frame: FrameInfo,
    /// Mirrors the `target_frame` value exposed by `WKNavigationAction`.
    pub target_frame: Option<FrameInfo>,
    /// Mirrors the `should_perform_download` value exposed by `WKNavigationAction`.
    pub should_perform_download: bool,
    /// Mirrors the `modifier_flags` value exposed by `WKNavigationAction`.
    pub modifier_flags: u64,
    /// Mirrors the `button_number` value exposed by `WKNavigationAction`.
    pub button_number: i64,
    /// Mirrors the `content_rule_list_redirect` value exposed by `WKNavigationAction`.
    pub content_rule_list_redirect: Option<bool>,
}

impl Default for NavigationAction {
    fn default() -> Self {
        Self {
            navigation_type: NavigationType::Other,
            navigation_type_raw_value: NavigationType::Other.as_raw(),
            request_url: String::new(),
            request_method: String::new(),
            request_headers: BTreeMap::new(),
            source_frame: FrameInfo::default(),
            target_frame: None,
            should_perform_download: false,
            modifier_flags: 0,
            button_number: 0,
            content_rule_list_redirect: None,
        }
    }
}

/// Wraps `WKNavigationResponse`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigationResponse {
    /// Mirrors the `for_main_frame` value exposed by `WKNavigationResponse`.
    pub for_main_frame: bool,
    /// Mirrors the `url` value exposed by `WKNavigationResponse`.
    pub url: String,
    /// Mirrors the `mime_type` value exposed by `WKNavigationResponse`.
    pub mime_type: Option<String>,
    /// Mirrors the `expected_content_length` value exposed by `WKNavigationResponse`.
    pub expected_content_length: i64,
    /// Mirrors the `text_encoding_name` value exposed by `WKNavigationResponse`.
    pub text_encoding_name: Option<String>,
    /// Mirrors the `status_code` value exposed by `WKNavigationResponse`.
    pub status_code: Option<i64>,
    /// Mirrors the `headers` value exposed by `WKNavigationResponse`.
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    /// Mirrors the `can_show_mime_type` value exposed by `WKNavigationResponse`.
    pub can_show_mime_type: bool,
}

/// Wraps `WKNavigationDelegate` values.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NavigationEventKind {
    /// Mirrors the `DidStartProvisional` case used by `WKNavigationDelegate`.
    DidStartProvisional,
    /// Mirrors the `DidReceiveServerRedirect` case used by `WKNavigationDelegate`.
    DidReceiveServerRedirect,
    /// Mirrors the `DidCommit` case used by `WKNavigationDelegate`.
    DidCommit,
    /// Mirrors the `DidFinish` case used by `WKNavigationDelegate`.
    DidFinish,
    /// Mirrors the `DidFail` case used by `WKNavigationDelegate`.
    DidFail,
    /// Mirrors the `DidFailProvisional` case used by `WKNavigationDelegate`.
    DidFailProvisional,
    /// Mirrors the `DecidePolicyForAction` case used by `WKNavigationDelegate`.
    DecidePolicyForAction,
    /// Mirrors the `DecidePolicyForResponse` case used by `WKNavigationDelegate`.
    DecidePolicyForResponse,
    /// Mirrors the `ProcessDidTerminate` case used by `WKNavigationDelegate`.
    ProcessDidTerminate,
    /// Mirrors the `NavigationActionDidBecomeDownload` case used by `WKNavigationDelegate`.
    NavigationActionDidBecomeDownload,
    /// Mirrors the `NavigationResponseDidBecomeDownload` case used by `WKNavigationDelegate`.
    NavigationResponseDidBecomeDownload,
    /// Mirrors the `Unknown` case used by `WKNavigationDelegate`.
    #[serde(other)]
    Unknown,
}

/// Captures data returned by `WKNavigationDelegate`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEvent {
    /// Mirrors the `kind` value exposed by `WKNavigationDelegate`.
    pub kind: NavigationEventKind,
    /// Mirrors the `url` value exposed by `WKNavigationDelegate`.
    pub url: String,
    /// Mirrors the `error` value exposed by `WKNavigationDelegate`.
    pub error: Option<String>,
    /// Mirrors the `navigation_type` value exposed by `WKNavigationDelegate`.
    pub navigation_type: Option<i64>,
    /// Mirrors the `status_code` value exposed by `WKNavigationDelegate`.
    pub status_code: Option<i64>,
    /// Mirrors the `navigation_action` value exposed by `WKNavigationDelegate`.
    pub navigation_action: Option<NavigationAction>,
    /// Mirrors the `navigation_response` value exposed by `WKNavigationDelegate`.
    pub navigation_response: Option<NavigationResponse>,
}

impl NavigationEvent {
    /// Mirrors the corresponding `WKNavigationDelegate` API.
    #[must_use]
    pub fn unknown() -> Self {
        Self {
            kind: NavigationEventKind::Unknown,
            url: String::new(),
            error: None,
            navigation_type: None,
            status_code: None,
            navigation_action: None,
            navigation_response: None,
        }
    }

    /// Returns the corresponding value from `WKNavigationDelegate`.
    #[must_use]
    pub fn navigation_type_enum(&self) -> Option<NavigationType> {
        self.navigation_type.map(NavigationType::from_raw)
    }
}
