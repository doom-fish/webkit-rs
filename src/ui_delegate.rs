use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UIDelegateConfig {
    pub confirm_response: bool,
    pub prompt_response: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PermissionDecision {
    Prompt,
    Grant,
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaCaptureType {
    Camera,
    Microphone,
    CameraAndMicrophone,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenPanelParameters {
    pub allows_multiple_selection: bool,
    pub allows_directories: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityOrigin {
    pub protocol: String,
    pub host: String,
    pub port: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WindowFeatures {
    pub menu_bar_visibility: Option<bool>,
    pub status_bar_visibility: Option<bool>,
    pub toolbars_visibility: Option<bool>,
    pub allows_resizing: Option<bool>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UIDelegateEvent {
    pub kind: String,
    pub message: Option<String>,
    pub prompt: Option<String>,
    pub default_text: Option<String>,
    pub frame_url: Option<String>,
    pub response: Option<Value>,
    pub allows_multiple_selection: Option<bool>,
    pub allows_directories: Option<bool>,
    pub host: Option<String>,
    pub r#type: Option<i64>,
}

impl UIDelegateEvent {
    #[must_use]
    pub fn open_panel_parameters(&self) -> Option<OpenPanelParameters> {
        (self.kind == "openPanel").then(|| OpenPanelParameters {
            allows_multiple_selection: self.allows_multiple_selection.unwrap_or(false),
            allows_directories: self.allows_directories.unwrap_or(false),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UIDelegateEventDetail {
    pub kind: String,
    pub message: Option<String>,
    pub prompt: Option<String>,
    pub default_text: Option<String>,
    pub frame_url: Option<String>,
    pub response: Option<Value>,
    pub open_panel_parameters: Option<OpenPanelParameters>,
    pub security_origin: Option<SecurityOrigin>,
    pub media_capture_type: Option<MediaCaptureType>,
    pub permission_decision: Option<PermissionDecision>,
    pub window_features: Option<WindowFeatures>,
}
