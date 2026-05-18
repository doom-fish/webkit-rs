use crate::geometry::Rect;

/// Configures `WKPDFConfiguration`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PDFConfiguration {
    /// Mirrors the `rect` value exposed by `WKPDFConfiguration`.
    pub rect: Option<Rect>,
    /// Mirrors the `allow_transparent_background` value exposed by `WKPDFConfiguration`.
    pub allow_transparent_background: bool,
}

impl PDFConfiguration {
    /// Creates a value for `WKPDFConfiguration`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rect: None,
            allow_transparent_background: false,
        }
    }

    /// Returns the corresponding value from `WKPDFConfiguration`.
    #[must_use]
    pub const fn rect(&self) -> Option<Rect> {
        self.rect
    }

    /// Returns the corresponding value from `WKPDFConfiguration`.
    #[must_use]
    pub const fn allows_transparent_background(&self) -> bool {
        self.allow_transparent_background
    }

    /// Sets the corresponding option used by `WKPDFConfiguration`.
    #[must_use]
    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = Some(rect);
        self
    }

    /// Sets the corresponding option used by `WKPDFConfiguration`.
    #[must_use]
    pub fn with_allow_transparent_background(mut self, value: bool) -> Self {
        self.allow_transparent_background = value;
        self
    }
}
