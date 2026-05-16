#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

pub mod config;
pub mod error;
pub mod ffi;
mod private;
pub mod user_script;
pub mod webview;

pub use config::WebViewConfiguration;
pub use error::WebKitError;
pub use user_script::{InjectionTime, UserScript};
pub use webview::{NavigationEvent, NavigationEventKind, WebView};

pub mod prelude {
    pub use crate::config::WebViewConfiguration;
    pub use crate::error::WebKitError;
    pub use crate::user_script::{InjectionTime, UserScript};
    pub use crate::webview::{NavigationEvent, NavigationEventKind, WebView};
}

/// Pump the main run loop for the given duration.
///
/// Call this after operations that require the run loop to process events
/// (for example, receiving script messages posted by JavaScript).
pub fn pump_run_loop(seconds: f64) {
    unsafe { ffi::wk_run_loop_pump(seconds) }
}

/// Initialise `NSApplication.shared` and set activation policy to
/// `.prohibited` (headless). Must be called once before creating any
/// [`WebView`].
pub fn init_app() {
    unsafe { ffi::wk_init_app() }
}
