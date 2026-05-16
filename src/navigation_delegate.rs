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
        }
    }
}
