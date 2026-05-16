#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InjectionTime {
    AtDocumentStart,
    AtDocumentEnd,
}

impl InjectionTime {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        match self {
            Self::AtDocumentStart => 0,
            Self::AtDocumentEnd => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserScript {
    pub source: String,
    pub injection_time: InjectionTime,
    pub main_frame_only: bool,
    pub content_world: Option<String>,
}

impl UserScript {
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            injection_time: InjectionTime::AtDocumentEnd,
            main_frame_only: true,
            content_world: None,
        }
    }

    #[must_use]
    pub fn with_injection_time(mut self, time: InjectionTime) -> Self {
        self.injection_time = time;
        self
    }

    #[must_use]
    pub fn with_main_frame_only(mut self, value: bool) -> Self {
        self.main_frame_only = value;
        self
    }

    #[must_use]
    pub fn with_content_world(mut self, name: impl Into<String>) -> Self {
        self.content_world = Some(name.into());
        self
    }
}
