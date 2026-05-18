/// Wraps `WKUserScriptInjectionTime` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InjectionTime {
    /// Mirrors the `AtDocumentStart` case used by `WKUserScriptInjectionTime`.
    AtDocumentStart,
    /// Mirrors the `AtDocumentEnd` case used by `WKUserScriptInjectionTime`.
    AtDocumentEnd,
}

impl InjectionTime {
    /// Returns the corresponding value from `WKUserScriptInjectionTime`.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        match self {
            Self::AtDocumentStart => 0,
            Self::AtDocumentEnd => 1,
        }
    }
}

/// Wraps `WKUserScript`.
#[derive(Debug, Clone)]
pub struct UserScript {
    /// Mirrors the `source` value exposed by `WKUserScript`.
    pub source: String,
    /// Mirrors the `injection_time` value exposed by `WKUserScript`.
    pub injection_time: InjectionTime,
    /// Mirrors the `main_frame_only` value exposed by `WKUserScript`.
    pub main_frame_only: bool,
    /// Mirrors the `content_world` value exposed by `WKUserScript`.
    pub content_world: Option<String>,
}

impl UserScript {
    /// Creates a value for `WKUserScript`.
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            injection_time: InjectionTime::AtDocumentEnd,
            main_frame_only: true,
            content_world: None,
        }
    }

    /// Sets the corresponding option used by `WKUserScript`.
    #[must_use]
    pub fn with_injection_time(mut self, time: InjectionTime) -> Self {
        self.injection_time = time;
        self
    }

    /// Sets the corresponding option used by `WKUserScript`.
    #[must_use]
    pub fn with_main_frame_only(mut self, value: bool) -> Self {
        self.main_frame_only = value;
        self
    }

    /// Sets the corresponding option used by `WKUserScript`.
    #[must_use]
    pub fn with_content_world(mut self, name: impl Into<String>) -> Self {
        self.content_world = Some(name.into());
        self
    }
}
