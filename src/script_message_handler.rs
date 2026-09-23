use serde::Deserialize;

use crate::content_world::ContentWorld;
use crate::frame::FrameHandle;
use crate::navigation_delegate::FrameInfo;

/// Wraps `WKScriptMessage`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptMessage {
    /// Mirrors the `name` value exposed by `WKScriptMessage`.
    pub name: String,
    /// Mirrors the `body` value exposed by `WKScriptMessage`.
    pub body: String,
    #[allow(missing_docs)]
    pub frame: FrameInfo,
    /// Mirrors the `world` value exposed by `WKScriptMessage`.
    pub world: ContentWorld,
    #[allow(missing_docs)]
    #[serde(skip)]
    pub frame_handle: Option<FrameHandle>,
}
