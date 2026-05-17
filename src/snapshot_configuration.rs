use crate::geometry::Rect;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SnapshotConfiguration {
    pub rect: Option<Rect>,
    pub snapshot_width: Option<f64>,
    pub after_screen_updates: bool,
}

impl SnapshotConfiguration {
    #[must_use]
    pub fn new() -> Self {
        Self {
            rect: None,
            snapshot_width: None,
            after_screen_updates: true,
        }
    }

    #[must_use]
    pub const fn rect(&self) -> Option<Rect> {
        self.rect
    }

    #[must_use]
    pub const fn snapshot_width(&self) -> Option<f64> {
        self.snapshot_width
    }

    #[must_use]
    pub const fn after_screen_updates(&self) -> bool {
        self.after_screen_updates
    }

    #[must_use]
    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = Some(rect);
        self
    }

    #[must_use]
    pub fn with_snapshot_width(mut self, snapshot_width: f64) -> Self {
        self.snapshot_width = Some(snapshot_width);
        self
    }

    #[must_use]
    pub fn with_after_screen_updates(mut self, value: bool) -> Self {
        self.after_screen_updates = value;
        self
    }
}
