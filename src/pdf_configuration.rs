use crate::geometry::Rect;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PDFConfiguration {
    pub rect: Option<Rect>,
    pub allow_transparent_background: bool,
}

impl PDFConfiguration {
    #[must_use]
    pub fn new() -> Self {
        Self {
            rect: None,
            allow_transparent_background: false,
        }
    }

    #[must_use]
    pub const fn rect(&self) -> Option<Rect> {
        self.rect
    }

    #[must_use]
    pub const fn allows_transparent_background(&self) -> bool {
        self.allow_transparent_background
    }

    #[must_use]
    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = Some(rect);
        self
    }

    #[must_use]
    pub fn with_allow_transparent_background(mut self, value: bool) -> Self {
        self.allow_transparent_background = value;
        self
    }
}
