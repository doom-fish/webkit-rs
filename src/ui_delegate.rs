use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UIDelegateConfig {
    pub confirm_response: bool,
    pub prompt_response: Option<String>,
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
