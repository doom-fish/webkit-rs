use serde::Deserialize;

/// Wraps `WKScriptMessage`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptMessage {
    /// Mirrors the `name` value exposed by `WKScriptMessage`.
    pub name: String,
    /// Mirrors the `body` value exposed by `WKScriptMessage`.
    pub body: String,
    /// Mirrors the `frame_url` value exposed by `WKScriptMessage`.
    pub frame_url: String,
    /// Mirrors the `is_main_frame` value exposed by `WKScriptMessage`.
    pub is_main_frame: bool,
    /// Mirrors the `world` value exposed by `WKScriptMessage`.
    pub world: Option<String>,
}
