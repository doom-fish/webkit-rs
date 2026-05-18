use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Configures `WKUIDelegate`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UIDelegateConfig {
    /// Mirrors the `confirm_response` value exposed by `WKUIDelegate`.
    pub confirm_response: bool,
    /// Mirrors the `prompt_response` value exposed by `WKUIDelegate`.
    pub prompt_response: Option<String>,
}

/// Wraps `WKPermissionDecision` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionDecision {
    /// Mirrors the `Prompt` case used by `WKPermissionDecision`.
    Prompt,
    /// Mirrors the `Grant` case used by `WKPermissionDecision`.
    Grant,
    /// Mirrors the `Deny` case used by `WKPermissionDecision`.
    Deny,
}

/// Wraps `WKMediaCaptureType` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaCaptureType {
    /// Mirrors the `Camera` case used by `WKMediaCaptureType`.
    Camera,
    /// Mirrors the `Microphone` case used by `WKMediaCaptureType`.
    Microphone,
    /// Mirrors the `CameraAndMicrophone` case used by `WKMediaCaptureType`.
    CameraAndMicrophone,
}

/// Captures data returned by `WKOpenPanelParameters`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenPanelParameters {
    /// Mirrors the `allows_multiple_selection` value exposed by `WKOpenPanelParameters`.
    pub allows_multiple_selection: bool,
    /// Mirrors the `allows_directories` value exposed by `WKOpenPanelParameters`.
    pub allows_directories: bool,
}

/// Wraps `WKSecurityOrigin`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityOrigin {
    /// Mirrors the `protocol` value exposed by `WKSecurityOrigin`.
    pub protocol: String,
    /// Mirrors the `host` value exposed by `WKSecurityOrigin`.
    pub host: String,
    /// Mirrors the `port` value exposed by `WKSecurityOrigin`.
    pub port: i64,
}

/// Wraps `WKWindowFeatures`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WindowFeatures {
    /// Mirrors the `menu_bar_visibility` value exposed by `WKWindowFeatures`.
    pub menu_bar_visibility: Option<bool>,
    /// Mirrors the `status_bar_visibility` value exposed by `WKWindowFeatures`.
    pub status_bar_visibility: Option<bool>,
    /// Mirrors the `toolbars_visibility` value exposed by `WKWindowFeatures`.
    pub toolbars_visibility: Option<bool>,
    /// Mirrors the `allows_resizing` value exposed by `WKWindowFeatures`.
    pub allows_resizing: Option<bool>,
    /// Mirrors the `x` value exposed by `WKWindowFeatures`.
    pub x: Option<f64>,
    /// Mirrors the `y` value exposed by `WKWindowFeatures`.
    pub y: Option<f64>,
    /// Mirrors the `width` value exposed by `WKWindowFeatures`.
    pub width: Option<f64>,
    /// Mirrors the `height` value exposed by `WKWindowFeatures`.
    pub height: Option<f64>,
}

/// Captures data returned by `WKUIDelegate`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UIDelegateEvent {
    /// Mirrors the `kind` value exposed by `WKUIDelegate`.
    pub kind: String,
    /// Mirrors the `message` value exposed by `WKUIDelegate`.
    pub message: Option<String>,
    /// Mirrors the `prompt` value exposed by `WKUIDelegate`.
    pub prompt: Option<String>,
    /// Mirrors the `default_text` value exposed by `WKUIDelegate`.
    pub default_text: Option<String>,
    /// Mirrors the `frame_url` value exposed by `WKUIDelegate`.
    pub frame_url: Option<String>,
    /// Mirrors the `response` value exposed by `WKUIDelegate`.
    pub response: Option<Value>,
    /// Mirrors the `allows_multiple_selection` value exposed by `WKUIDelegate`.
    pub allows_multiple_selection: Option<bool>,
    /// Mirrors the `allows_directories` value exposed by `WKUIDelegate`.
    pub allows_directories: Option<bool>,
    /// Mirrors the `host` value exposed by `WKUIDelegate`.
    pub host: Option<String>,
    /// Mirrors the `field` value exposed by `WKUIDelegate`.
    pub r#type: Option<i64>,
}

impl UIDelegateEvent {
    /// Returns the corresponding value from `WKUIDelegate`.
    #[must_use]
    pub fn open_panel_parameters(&self) -> Option<OpenPanelParameters> {
        (self.kind == "openPanel").then(|| OpenPanelParameters {
            allows_multiple_selection: self.allows_multiple_selection.unwrap_or(false),
            allows_directories: self.allows_directories.unwrap_or(false),
        })
    }
}

/// Captures data returned by `WKUIDelegate`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UIDelegateEventDetail {
    /// Mirrors the `kind` value exposed by `WKUIDelegate`.
    pub kind: String,
    /// Mirrors the `message` value exposed by `WKUIDelegate`.
    pub message: Option<String>,
    /// Mirrors the `prompt` value exposed by `WKUIDelegate`.
    pub prompt: Option<String>,
    /// Mirrors the `default_text` value exposed by `WKUIDelegate`.
    pub default_text: Option<String>,
    /// Mirrors the `frame_url` value exposed by `WKUIDelegate`.
    pub frame_url: Option<String>,
    /// Mirrors the `response` value exposed by `WKUIDelegate`.
    pub response: Option<Value>,
    /// Mirrors the `open_panel_parameters` value exposed by `WKUIDelegate`.
    pub open_panel_parameters: Option<OpenPanelParameters>,
    /// Mirrors the `security_origin` value exposed by `WKUIDelegate`.
    pub security_origin: Option<SecurityOrigin>,
    /// Mirrors the `media_capture_type` value exposed by `WKUIDelegate`.
    pub media_capture_type: Option<MediaCaptureType>,
    /// Mirrors the `permission_decision` value exposed by `WKUIDelegate`.
    pub permission_decision: Option<PermissionDecision>,
    /// Mirrors the `window_features` value exposed by `WKUIDelegate`.
    pub window_features: Option<WindowFeatures>,
}
