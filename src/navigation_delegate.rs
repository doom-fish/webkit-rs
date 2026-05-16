use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum NavigationActionPolicy {
    Cancel = 0,
    Allow = 1,
    Download = 2,
}

impl NavigationActionPolicy {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum NavigationResponsePolicy {
    Cancel = 0,
    Allow = 1,
    Download = 2,
}

impl NavigationResponsePolicy {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationDelegateConfig {
    pub action_policy: NavigationActionPolicy,
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i64)]
#[serde(rename_all = "camelCase")]
pub enum NavigationType {
    LinkActivated = 0,
    FormSubmitted = 1,
    BackForward = 2,
    Reload = 3,
    FormResubmitted = 4,
    #[default]
    #[serde(other)]
    Other = -1,
}

impl NavigationType {
    #[must_use]
    pub const fn as_raw(self) -> i64 {
        self as i64
    }

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FrameInfo {
    pub main_frame: bool,
    pub request_url: String,
    pub request_method: String,
    pub security_origin_protocol: Option<String>,
    pub security_origin_host: Option<String>,
    pub security_origin_port: Option<i64>,
    pub webview_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationAction {
    pub navigation_type: NavigationType,
    pub navigation_type_raw_value: i64,
    pub request_url: String,
    pub request_method: String,
    #[serde(default)]
    pub request_headers: BTreeMap<String, String>,
    pub source_frame: FrameInfo,
    pub target_frame: Option<FrameInfo>,
    pub should_perform_download: bool,
    pub modifier_flags: u64,
    pub button_number: i64,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NavigationResponse {
    pub for_main_frame: bool,
    pub url: String,
    pub mime_type: Option<String>,
    pub expected_content_length: i64,
    pub text_encoding_name: Option<String>,
    pub status_code: Option<i64>,
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    pub can_show_mime_type: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NavigationEventKind {
    DidStartProvisional,
    DidReceiveServerRedirect,
    DidCommit,
    DidFinish,
    DidFail,
    DidFailProvisional,
    DecidePolicyForAction,
    DecidePolicyForResponse,
    ProcessDidTerminate,
    NavigationActionDidBecomeDownload,
    NavigationResponseDidBecomeDownload,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NavigationEvent {
    pub kind: NavigationEventKind,
    pub url: String,
    pub error: Option<String>,
    pub navigation_type: Option<i64>,
    pub status_code: Option<i64>,
    pub navigation_action: Option<NavigationAction>,
    pub navigation_response: Option<NavigationResponse>,
}

impl NavigationEvent {
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

    #[must_use]
    pub fn navigation_type_enum(&self) -> Option<NavigationType> {
        self.navigation_type.map(NavigationType::from_raw)
    }
}
