use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptMessage {
    pub name: String,
    pub body: String,
    pub frame_url: String,
    pub is_main_frame: bool,
    pub world: Option<String>,
}
