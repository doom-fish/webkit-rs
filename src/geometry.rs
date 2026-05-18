use serde::{Deserialize, Serialize};

/// Wraps `CGRect`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    /// Mirrors the `x` value exposed by `CGRect`.
    pub x: f64,
    /// Mirrors the `y` value exposed by `CGRect`.
    pub y: f64,
    /// Mirrors the `width` value exposed by `CGRect`.
    pub width: f64,
    /// Mirrors the `height` value exposed by `CGRect`.
    pub height: f64,
}

impl Rect {
    /// Creates a value for `CGRect`.
    #[must_use]
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
