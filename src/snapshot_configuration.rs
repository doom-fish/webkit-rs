use crate::geometry::Rect;

/// Configures `WKSnapshotConfiguration`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SnapshotConfiguration {
    /// Mirrors the `rect` value exposed by `WKSnapshotConfiguration`.
    pub rect: Option<Rect>,
    /// Mirrors the `snapshot_width` value exposed by `WKSnapshotConfiguration`.
    pub snapshot_width: Option<f64>,
    /// Mirrors the `after_screen_updates` value exposed by `WKSnapshotConfiguration`.
    pub after_screen_updates: bool,
}

impl SnapshotConfiguration {
    /// Creates a value for `WKSnapshotConfiguration`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rect: None,
            snapshot_width: None,
            after_screen_updates: true,
        }
    }

    /// Returns the corresponding value from `WKSnapshotConfiguration`.
    #[must_use]
    pub const fn rect(&self) -> Option<Rect> {
        self.rect
    }

    /// Returns the corresponding value from `WKSnapshotConfiguration`.
    #[must_use]
    pub const fn snapshot_width(&self) -> Option<f64> {
        self.snapshot_width
    }

    /// Returns the corresponding value from `WKSnapshotConfiguration`.
    #[must_use]
    pub const fn after_screen_updates(&self) -> bool {
        self.after_screen_updates
    }

    /// Sets the corresponding option used by `WKSnapshotConfiguration`.
    #[must_use]
    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = Some(rect);
        self
    }

    /// Sets the corresponding option used by `WKSnapshotConfiguration`.
    #[must_use]
    pub fn with_snapshot_width(mut self, snapshot_width: f64) -> Self {
        self.snapshot_width = Some(snapshot_width);
        self
    }

    /// Sets the corresponding option used by `WKSnapshotConfiguration`.
    #[must_use]
    pub fn with_after_screen_updates(mut self, value: bool) -> Self {
        self.after_screen_updates = value;
        self
    }
}
