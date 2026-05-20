use core::ffi::{c_char, c_void, CStr};
use core::ptr;
use std::ops::{BitOr, BitOrAssign};
use std::path::Path;
use std::time::Duration;

use serde_json::Value;

use crate::back_forward_list::BackForwardList;
use crate::config::WebViewConfiguration;
use crate::download::Download;
use crate::error::{status_from_error, WebKitError};
use crate::ffi::{self, WKMsgCallback, WKMsgReplyCallback, WKNavCallback};
use crate::find::{FindConfiguration, FindResult, TextFinderAction};
use crate::navigation::Navigation;
use crate::navigation_delegate::{NavigationDelegateConfig, NavigationEvent};
use crate::pdf_configuration::PDFConfiguration;
use crate::private::{
    maybe_take_error, take_bytes, take_json_or_default, take_string, to_cstring, to_json_cstring,
};
use crate::script_message_handler::ScriptMessage;
use crate::snapshot_configuration::SnapshotConfiguration;
use crate::ui_delegate::{UIDelegateConfig, UIDelegateEvent, UIDelegateEventDetail};

pub use crate::navigation_delegate::NavigationEventKind;

type NavigationHandler = dyn Fn(NavigationEvent) + Send + 'static;
type MessageHandler = dyn Fn(&str, &str) + Send + 'static;
type ReplyMessageHandler =
    dyn Fn(&str, &str) -> Result<Option<Value>, WebKitError> + Send + 'static;

/// Wraps `WKMediaPlaybackState` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MediaPlaybackState {
    /// Mirrors the `None` case used by `WKMediaPlaybackState`.
    None = 0,
    /// Mirrors the `Playing` case used by `WKMediaPlaybackState`.
    Playing = 1,
    /// Mirrors the `Paused` case used by `WKMediaPlaybackState`.
    Paused = 2,
    /// Mirrors the `Suspended` case used by `WKMediaPlaybackState`.
    Suspended = 3,
}

impl MediaPlaybackState {
    /// Creates a value for `WKMediaPlaybackState`.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Playing,
            2 => Self::Paused,
            3 => Self::Suspended,
            _ => Self::None,
        }
    }
}

/// Wraps `WKMediaCaptureState` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MediaCaptureState {
    /// Mirrors the `None` case used by `WKMediaCaptureState`.
    None = 0,
    /// Mirrors the `Active` case used by `WKMediaCaptureState`.
    Active = 1,
    /// Mirrors the `Muted` case used by `WKMediaCaptureState`.
    Muted = 2,
}

impl MediaCaptureState {
    /// Returns the corresponding value from `WKMediaCaptureState`.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Creates a value for `WKMediaCaptureState`.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Active,
            2 => Self::Muted,
            _ => Self::None,
        }
    }
}

/// Wraps `WKFullscreenState` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FullscreenState {
    /// Mirrors the `NotInFullscreen` case used by `WKFullscreenState`.
    NotInFullscreen = 0,
    /// Mirrors the `EnteringFullscreen` case used by `WKFullscreenState`.
    EnteringFullscreen = 1,
    /// Mirrors the `InFullscreen` case used by `WKFullscreenState`.
    InFullscreen = 2,
    /// Mirrors the `ExitingFullscreen` case used by `WKFullscreenState`.
    ExitingFullscreen = 3,
}

impl FullscreenState {
    /// Creates a value for `WKFullscreenState`.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::EnteringFullscreen,
            2 => Self::InFullscreen,
            3 => Self::ExitingFullscreen,
            _ => Self::NotInFullscreen,
        }
    }
}

/// Wraps `WKWebViewDataType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WebViewDataType(u64);

impl WebViewDataType {
    /// Mirrors the `NONE` constant used by `WKWebViewDataType`.
    pub const NONE: Self = Self(0);
    /// Mirrors the `SESSION_STORAGE` constant used by `WKWebViewDataType`.
    pub const SESSION_STORAGE: Self = Self(1 << 0);

    /// Creates a value for `WKWebViewDataType`.
    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns the corresponding value from `WKWebViewDataType`.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Returns whether this `WKWebViewDataType` value contains the provided flags.
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl BitOr for WebViewDataType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for WebViewDataType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

struct NavCallbackHolder {
    f: Box<NavigationHandler>,
}

struct MsgCallbackHolder {
    f: Box<MessageHandler>,
}

struct ReplyMsgCallbackHolder {
    f: Box<ReplyMessageHandler>,
}

fn malloc_c_string(value: &str) -> *mut c_char {
    let sanitized = value.replace('\0', " ");
    let len = sanitized.len() + 1;
    let ptr = unsafe { libc::malloc(len) }.cast::<c_char>();
    if ptr.is_null() {
        return ptr;
    }
    unsafe {
        ptr::copy_nonoverlapping(sanitized.as_ptr().cast::<c_char>(), ptr, sanitized.len());
        *ptr.add(sanitized.len()) = 0;
    }
    ptr
}

// SAFETY: Called by the Swift bridge on the main thread. `user_info` is either
// null or a shared reference to `NavCallbackHolder` whose lifetime is managed
// by the enclosing `WebView`. `event_json` is a bridge-owned null-terminated C
// string valid for the duration of this call.
unsafe extern "C" fn nav_trampoline(user_info: *mut c_void, event_json: *const c_char) {
    if user_info.is_null() || event_json.is_null() {
        return;
    }

    // SAFETY: `user_info` is non-null and points to a live `NavCallbackHolder`.
    let holder = unsafe { &*(user_info.cast::<NavCallbackHolder>()) };
    // SAFETY: `event_json` is non-null and a valid C string for this call.
    let json = unsafe { CStr::from_ptr(event_json) }.to_string_lossy();
    let event = serde_json::from_str::<NavigationEvent>(&json)
        .unwrap_or_else(|_| NavigationEvent::unknown());
    // Catch panics from user-supplied closure to prevent UB across the C ABI.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (holder.f)(event)));
}

// SAFETY: Called by the Swift bridge on the main thread. `user_info` is either
// null or a shared reference to `MsgCallbackHolder` managed by the enclosing
// `WebView`. String pointers are bridge-owned and valid for this call.
unsafe extern "C" fn msg_trampoline(
    user_info: *mut c_void,
    handler_name: *const c_char,
    body: *const c_char,
) {
    if user_info.is_null() {
        return;
    }

    // SAFETY: `user_info` is non-null and points to a live `MsgCallbackHolder`.
    let holder = unsafe { &*(user_info.cast::<MsgCallbackHolder>()) };
    let name = if handler_name.is_null() {
        ""
    } else {
        // SAFETY: `handler_name` is non-null and a valid C string.
        unsafe { CStr::from_ptr(handler_name) }
            .to_str()
            .unwrap_or("")
    };
    let body_str = if body.is_null() {
        ""
    } else {
        // SAFETY: `body` is non-null and a valid C string.
        unsafe { CStr::from_ptr(body) }.to_str().unwrap_or("")
    };
    // Catch panics from user-supplied closure to prevent UB across the C ABI.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (holder.f)(name, body_str)));
}

// SAFETY: Called by the Swift bridge on the main thread. `user_info` is either
// null or a shared reference to `ReplyMsgCallbackHolder` managed by the
// enclosing `WebView`. String pointers are bridge-owned and valid for this
// call. `out_reply` and `out_err` are writable bridge-owned pointers or null.
unsafe extern "C" fn msg_reply_trampoline(
    user_info: *mut c_void,
    handler_name: *const c_char,
    body: *const c_char,
    out_reply: *mut *mut c_char,
    out_err: *mut *mut c_char,
) -> i32 {
    if user_info.is_null() {
        if !out_err.is_null() {
            // SAFETY: `out_err` is non-null and writable.
            unsafe { *out_err = malloc_c_string("missing script message reply handler") };
        }
        return ffi::status::INVALID_ARGUMENT;
    }

    if !out_reply.is_null() {
        // SAFETY: `out_reply` is non-null and writable.
        unsafe { *out_reply = ptr::null_mut() };
    }
    if !out_err.is_null() {
        // SAFETY: `out_err` is non-null and writable.
        unsafe { *out_err = ptr::null_mut() };
    }

    // SAFETY: `user_info` is non-null and points to a live `ReplyMsgCallbackHolder`.
    let holder = unsafe { &*(user_info.cast::<ReplyMsgCallbackHolder>()) };
    let name = if handler_name.is_null() {
        ""
    } else {
        // SAFETY: `handler_name` is non-null and a valid C string.
        unsafe { CStr::from_ptr(handler_name) }
            .to_str()
            .unwrap_or("")
    };
    let body_str = if body.is_null() {
        ""
    } else {
        // SAFETY: `body` is non-null and a valid C string.
        unsafe { CStr::from_ptr(body) }.to_str().unwrap_or("")
    };

    // Catch panics from user-supplied closure to prevent UB across the C ABI.
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (holder.f)(name, body_str))) {
        Err(_) => {
            if !out_err.is_null() {
                // SAFETY: `out_err` is non-null and writable.
                unsafe { *out_err = malloc_c_string("script message reply handler panicked") };
            }
            ffi::status::FRAMEWORK_ERROR
        }
        Ok(Ok(Some(reply))) => match serde_json::to_string(&reply) {
            Ok(json) => {
                if !out_reply.is_null() {
                    // SAFETY: `out_reply` is non-null and writable.
                    unsafe { *out_reply = malloc_c_string(&json) };
                }
                ffi::status::OK
            }
            Err(error) => {
                if !out_err.is_null() {
                    // SAFETY: `out_err` is non-null and writable.
                    unsafe { *out_err = malloc_c_string(&error.to_string()) };
                }
                ffi::status::FRAMEWORK_ERROR
            }
        },
        Ok(Ok(None)) => ffi::status::OK,
        Ok(Err(error)) => {
            if !out_err.is_null() {
                // SAFETY: `out_err` is non-null and writable.
                unsafe { *out_err = malloc_c_string(&error.to_string()) };
            }
            status_from_error(&error)
        }
    }
}

/// Safe wrapper around `WKWebView`.
///
/// All WebKit APIs are driven from the main thread. The Swift bridge uses
/// `DispatchQueue.main` plus run-loop pumping so examples and tests can use the
/// API from ordinary Rust code.
pub struct WebView {
    ptr: *mut c_void,
    nav_holder: Option<Box<NavCallbackHolder>>,
    msg_holder: Option<Box<MsgCallbackHolder>>,
    reply_msg_holder: Option<Box<ReplyMsgCallbackHolder>>,
}

// SAFETY: The Swift bridge ensures all WebKit calls happen on the main thread.
unsafe impl Send for WebView {}
// SAFETY: The Swift bridge serialises all WebKit calls onto the main thread.
unsafe impl Sync for WebView {}

impl WebView {
    /// Create a new offscreen `WKWebView` with the default configuration.
    ///
    /// # Errors
    /// Returns an error if the WebKit framework is unavailable.
    pub fn new_offscreen() -> Result<Self, WebKitError> {
        crate::init_app();
        let ptr = unsafe { ffi::wk_webview_new(ptr::null_mut()) };
        if ptr.is_null() {
            return Err(WebKitError::FrameworkError(
                "wk_webview_new returned null".to_owned(),
            ));
        }

        Ok(Self {
            ptr,
            nav_holder: None,
            msg_holder: None,
            reply_msg_holder: None,
        })
    }

    /// Create a new offscreen `WKWebView` with the supplied configuration.
    ///
    /// # Errors
    /// Returns an error if the WebKit framework is unavailable.
    pub fn with_config(config: &WebViewConfiguration) -> Result<Self, WebKitError> {
        crate::init_app();
        let ptr = unsafe { ffi::wk_webview_new(config.as_ptr()) };
        if ptr.is_null() {
            return Err(WebKitError::FrameworkError(
                "wk_webview_new returned null".to_owned(),
            ));
        }

        Ok(Self {
            ptr,
            nav_holder: None,
            msg_holder: None,
            reply_msg_holder: None,
        })
    }

    #[cfg(feature = "async")]
    #[must_use]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Register a navigation-event callback.
    pub fn set_navigation_handler<F>(&mut self, f: F)
    where
        F: Fn(NavigationEvent) + Send + 'static,
    {
        let holder = Box::new(NavCallbackHolder { f: Box::new(f) });
        let user_info = std::ptr::from_ref(holder.as_ref())
            .cast_mut()
            .cast::<c_void>();
        unsafe {
            ffi::wk_webview_set_nav_callback(
                self.ptr,
                Some(nav_trampoline as WKNavCallback),
                user_info,
            );
        }
        self.nav_holder = Some(holder);
    }

    /// Register a script-message callback.
    pub fn set_message_handler<F>(&mut self, f: F)
    where
        F: Fn(&str, &str) + Send + 'static,
    {
        let holder = Box::new(MsgCallbackHolder { f: Box::new(f) });
        let user_info = std::ptr::from_ref(holder.as_ref())
            .cast_mut()
            .cast::<c_void>();
        unsafe {
            ffi::wk_webview_set_msg_callback(
                self.ptr,
                Some(msg_trampoline as WKMsgCallback),
                user_info,
            );
        }
        self.msg_holder = Some(holder);
    }

    /// Register a reply-capable script-message callback.
    pub fn set_message_handler_with_reply<F>(&mut self, f: F)
    where
        F: Fn(&str, &str) -> Result<Option<Value>, WebKitError> + Send + 'static,
    {
        let holder = Box::new(ReplyMsgCallbackHolder { f: Box::new(f) });
        let user_info = std::ptr::from_ref(holder.as_ref())
            .cast_mut()
            .cast::<c_void>();
        unsafe {
            ffi::wk_webview_set_msg_reply_callback(
                self.ptr,
                Some(msg_reply_trampoline as WKMsgReplyCallback),
                user_info,
            );
        }
        self.reply_msg_holder = Some(holder);
    }

    /// Sets the corresponding value on `WKWebView`.
    pub fn set_navigation_delegate_config(&self, config: &NavigationDelegateConfig) {
        unsafe {
            ffi::wk_webview_set_navigation_delegate_config(
                self.ptr,
                config.action_policy.as_raw(),
                config.response_policy.as_raw(),
            );
        }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn drain_navigation_events(&self) -> Vec<NavigationEvent> {
        unsafe { take_json_or_default(ffi::wk_webview_drain_navigation_events_json(self.ptr)) }
    }

    /// Sets the corresponding value on `WKWebView`.
    pub fn set_ui_delegate_config(&self, config: &UIDelegateConfig) {
        let prompt_response = config.prompt_response.as_deref().map(to_cstring);
        let prompt_response_ptr = prompt_response
            .as_ref()
            .map_or(ptr::null(), |value| value.as_ptr());
        unsafe {
            ffi::wk_webview_set_ui_delegate_config(
                self.ptr,
                config.confirm_response,
                prompt_response_ptr,
            );
        }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn drain_ui_events(&self) -> Vec<UIDelegateEvent> {
        unsafe { take_json_or_default(ffi::wk_webview_drain_ui_events_json(self.ptr)) }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn drain_ui_event_details(&self) -> Vec<UIDelegateEventDetail> {
        unsafe { take_json_or_default(ffi::wk_webview_drain_ui_events_json(self.ptr)) }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn drain_script_messages(&self) -> Vec<ScriptMessage> {
        unsafe { take_json_or_default(ffi::wk_webview_drain_script_messages_json(self.ptr)) }
    }

    /// Load a URL and block until navigation finishes.
    ///
    /// # Errors
    /// Returns an error if the URL is invalid or navigation fails.
    pub fn load_url(&self, url: &str) -> Result<(), WebKitError> {
        self.load_url_with_navigation(url).map(|_| ())
    }

    /// Load a URL and return the completed `WKNavigation` handle.
    ///
    /// # Errors
    /// Returns an error if the URL is invalid or navigation fails.
    pub fn load_url_with_navigation(&self, url: &str) -> Result<Navigation, WebKitError> {
        let c_url = to_cstring(url);
        let mut out_navigation: *mut c_void = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_load_url(self.ptr, c_url.as_ptr(), &mut out_navigation, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Navigation::from_ptr(out_navigation).ok_or_else(|| {
            WebKitError::FrameworkError("load_url returned no navigation handle".to_owned())
        })
    }

    /// Load an HTML string and block until navigation finishes.
    ///
    /// # Errors
    /// Returns an error if navigation fails.
    pub fn load_html(&self, html: &str, base_url: Option<&str>) -> Result<(), WebKitError> {
        self.load_html_with_navigation(html, base_url).map(|_| ())
    }

    /// Load an HTML string and return the completed `WKNavigation` handle.
    ///
    /// # Errors
    /// Returns an error if navigation fails.
    pub fn load_html_with_navigation(
        &self,
        html: &str,
        base_url: Option<&str>,
    ) -> Result<Navigation, WebKitError> {
        let c_html = to_cstring(html);
        let c_base = base_url.map(to_cstring);
        let base_ptr = c_base.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        let mut out_navigation: *mut c_void = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_load_html(
                self.ptr,
                c_html.as_ptr(),
                base_ptr,
                &mut out_navigation,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Navigation::from_ptr(out_navigation).ok_or_else(|| {
            WebKitError::FrameworkError("load_html returned no navigation handle".to_owned())
        })
    }

    /// Load a file URL and block until navigation finishes.
    ///
    /// # Errors
    /// Returns an error if navigation fails.
    pub fn load_file_url(
        &self,
        file_url: impl AsRef<Path>,
        read_access_url: impl AsRef<Path>,
    ) -> Result<Navigation, WebKitError> {
        let c_file_url = to_cstring(&file_url.as_ref().to_string_lossy());
        let c_read_access_url = to_cstring(&read_access_url.as_ref().to_string_lossy());
        let mut out_navigation: *mut c_void = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_load_file_url(
                self.ptr,
                c_file_url.as_ptr(),
                c_read_access_url.as_ptr(),
                &mut out_navigation,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Navigation::from_ptr(out_navigation).ok_or_else(|| {
            WebKitError::FrameworkError("load_file_url returned no navigation handle".to_owned())
        })
    }

    /// Load raw data and block until navigation finishes.
    ///
    /// # Errors
    /// Returns an error if navigation fails.
    pub fn load_data(
        &self,
        data: &[u8],
        mime_type: &str,
        encoding_name: &str,
        base_url: &str,
    ) -> Result<Navigation, WebKitError> {
        let c_mime_type = to_cstring(mime_type);
        let c_encoding_name = to_cstring(encoding_name);
        let c_base_url = to_cstring(base_url);
        let mut out_navigation: *mut c_void = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_load_data(
                self.ptr,
                data.as_ptr(),
                data.len(),
                c_mime_type.as_ptr(),
                c_encoding_name.as_ptr(),
                c_base_url.as_ptr(),
                &mut out_navigation,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Navigation::from_ptr(out_navigation).ok_or_else(|| {
            WebKitError::FrameworkError("load_data returned no navigation handle".to_owned())
        })
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn go_back(&self) -> Option<Navigation> {
        let mut out_navigation: *mut c_void = ptr::null_mut();
        let has_navigation = unsafe { ffi::wk_webview_go_back(self.ptr, &mut out_navigation) };
        if has_navigation {
            Navigation::from_ptr(out_navigation)
        } else {
            None
        }
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn go_forward(&self) -> Option<Navigation> {
        let mut out_navigation: *mut c_void = ptr::null_mut();
        let has_navigation = unsafe { ffi::wk_webview_go_forward(self.ptr, &mut out_navigation) };
        if has_navigation {
            Navigation::from_ptr(out_navigation)
        } else {
            None
        }
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn reload(&self) -> Option<Navigation> {
        let mut out_navigation: *mut c_void = ptr::null_mut();
        let has_navigation = unsafe { ffi::wk_webview_reload(self.ptr, &mut out_navigation) };
        if has_navigation {
            Navigation::from_ptr(out_navigation)
        } else {
            None
        }
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn reload_from_origin(&self) -> Option<Navigation> {
        let mut out_navigation: *mut c_void = ptr::null_mut();
        let has_navigation =
            unsafe { ffi::wk_webview_reload_from_origin(self.ptr, &mut out_navigation) };
        if has_navigation {
            Navigation::from_ptr(out_navigation)
        } else {
            None
        }
    }

    /// Calls the corresponding `WKWebView` API.
    pub fn stop_loading(&self) {
        unsafe { ffi::wk_webview_stop_loading(self.ptr) }
    }

    /// Calls the corresponding `WKWebView` API.
    pub fn perform_go_back_action(&self) {
        unsafe { ffi::wk_webview_perform_go_back_action(self.ptr) }
    }

    /// Calls the corresponding `WKWebView` API.
    pub fn perform_go_forward_action(&self) {
        unsafe { ffi::wk_webview_perform_go_forward_action(self.ptr) }
    }

    /// Calls the corresponding `WKWebView` API.
    pub fn perform_reload_action(&self) {
        unsafe { ffi::wk_webview_perform_reload_action(self.ptr) }
    }

    /// Calls the corresponding `WKWebView` API.
    pub fn perform_reload_from_origin_action(&self) {
        unsafe { ffi::wk_webview_perform_reload_from_origin_action(self.ptr) }
    }

    /// Calls the corresponding `WKWebView` API.
    pub fn perform_stop_loading_action(&self) {
        unsafe { ffi::wk_webview_perform_stop_loading_action(self.ptr) }
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn go_to_back_forward_index(&self, index: isize) -> Option<Navigation> {
        let mut out_navigation: *mut c_void = ptr::null_mut();
        let has_navigation = unsafe {
            ffi::wk_webview_go_to_back_forward_index(self.ptr, index, &mut out_navigation)
        };
        if has_navigation {
            Navigation::from_ptr(out_navigation)
        } else {
            None
        }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn title(&self) -> String {
        unsafe { take_string(ffi::wk_webview_copy_title(self.ptr)) }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn url(&self) -> String {
        unsafe { take_string(ffi::wk_webview_copy_url(self.ptr)) }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn is_loading(&self) -> bool {
        unsafe { ffi::wk_webview_is_loading(self.ptr) }
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn estimated_progress(&self) -> f64 {
        unsafe { ffi::wk_webview_get_estimated_progress(self.ptr) }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn has_only_secure_content(&self) -> bool {
        unsafe { ffi::wk_webview_get_has_only_secure_content(self.ptr) }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn can_go_back(&self) -> bool {
        unsafe { ffi::wk_webview_get_can_go_back(self.ptr) }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn can_go_forward(&self) -> bool {
        unsafe { ffi::wk_webview_get_can_go_forward(self.ptr) }
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn back_forward_list(&self) -> BackForwardList {
        unsafe {
            BackForwardList::from_json_ptr(ffi::wk_webview_copy_back_forward_list_json(self.ptr))
        }
    }

    /// Sets the corresponding value on `WKWebView`.
    pub fn set_custom_user_agent(&self, value: Option<&str>) {
        let value = value.map(to_cstring);
        let value_ptr = value.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        unsafe { ffi::wk_webview_set_custom_user_agent(self.ptr, value_ptr) }
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn custom_user_agent(&self) -> String {
        unsafe { take_string(ffi::wk_webview_copy_custom_user_agent(self.ptr)) }
    }

    /// Sets the corresponding value on `WKWebView`.
    pub fn set_allows_link_preview(&self, value: bool) {
        unsafe { ffi::wk_webview_set_allows_link_preview(self.ptr, value) }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn allows_link_preview(&self) -> bool {
        unsafe { ffi::wk_webview_get_allows_link_preview(self.ptr) }
    }

    /// Sets the corresponding value on `WKWebView`.
    pub fn set_page_zoom(&self, value: f64) {
        unsafe { ffi::wk_webview_set_page_zoom(self.ptr, value) }
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn page_zoom(&self) -> f64 {
        unsafe { ffi::wk_webview_get_page_zoom(self.ptr) }
    }

    /// Sets the corresponding value on `WKWebView`.
    pub fn set_media_type(&self, value: Option<&str>) {
        let value = value.map(to_cstring);
        let value_ptr = value.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        unsafe { ffi::wk_webview_set_media_type(self.ptr, value_ptr) }
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn media_type(&self) -> String {
        unsafe { take_string(ffi::wk_webview_copy_media_type(self.ptr)) }
    }

    /// Sets the corresponding value on `WKWebView`.
    pub fn set_inspectable(&self, value: bool) {
        unsafe { ffi::wk_webview_set_inspectable(self.ptr, value) }
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn is_inspectable(&self) -> bool {
        unsafe { ffi::wk_webview_get_inspectable(self.ptr) }
    }

    /// Mirrors the corresponding `WKWebView` API.
    pub fn request_media_playback_state(&self) -> Result<MediaPlaybackState, WebKitError> {
        let mut out_state = 0;
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_request_media_playback_state(self.ptr, &mut out_state, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(MediaPlaybackState::from_raw(out_state))
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn camera_capture_state(&self) -> MediaCaptureState {
        MediaCaptureState::from_raw(unsafe { ffi::wk_webview_get_camera_capture_state(self.ptr) })
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn microphone_capture_state(&self) -> MediaCaptureState {
        MediaCaptureState::from_raw(unsafe {
            ffi::wk_webview_get_microphone_capture_state(self.ptr)
        })
    }

    /// Sets the corresponding value on `WKWebView`.
    pub fn set_camera_capture_state(&self, state: MediaCaptureState) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_set_camera_capture_state(self.ptr, state.as_raw(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Sets the corresponding value on `WKWebView`.
    pub fn set_microphone_capture_state(
        &self,
        state: MediaCaptureState,
    ) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_set_microphone_capture_state(self.ptr, state.as_raw(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Mirrors the corresponding `WKWebView` API.
    #[must_use]
    pub fn fullscreen_state(&self) -> FullscreenState {
        FullscreenState::from_raw(unsafe { ffi::wk_webview_get_fullscreen_state(self.ptr) })
    }

    /// Evaluate JavaScript and return the result as a `String`.
    ///
    /// # Errors
    /// Returns an error if JavaScript evaluation fails.
    pub fn evaluate_javascript(&self, js: &str) -> Result<String, WebKitError> {
        let c_js = to_cstring(js);
        let mut out_result: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_evaluate_js(self.ptr, c_js.as_ptr(), &mut out_result, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_string(out_result) })
    }

    /// Call async JavaScript and return the result as a `String`.
    ///
    /// # Errors
    /// Returns an error if the JavaScript call fails.
    pub fn call_async_javascript(&self, js: &str) -> Result<String, WebKitError> {
        let c_js = to_cstring(js);
        let mut out_result: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_call_async_js(self.ptr, c_js.as_ptr(), &mut out_result, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_string(out_result) })
    }

    /// Search the current page using the default `WKFindConfiguration` values.
    ///
    /// # Errors
    /// Returns an error if the find-in-page request fails.
    pub fn find_string(&self, query: &str) -> Result<FindResult, WebKitError> {
        self.find_string_with_configuration(query, &FindConfiguration::default())
    }

    /// Search the current page using a custom find configuration.
    ///
    /// # Errors
    /// Returns an error if the find-in-page request fails.
    pub fn find_string_with_configuration(
        &self,
        query: &str,
        configuration: &FindConfiguration,
    ) -> Result<FindResult, WebKitError> {
        let c_query = to_cstring(query);
        let configuration_json = to_json_cstring(configuration);
        let mut out_result: *mut c_char = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_find_string(
                self.ptr,
                c_query.as_ptr(),
                configuration_json.as_ptr(),
                &mut out_result,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_json_or_default(out_result) })
    }

    /// Returns the corresponding value from `WKWebView`.
    #[must_use]
    pub fn can_perform_text_finder_action(&self, action: TextFinderAction) -> bool {
        unsafe { ffi::wk_webview_validate_text_finder_action(self.ptr, action.as_raw()) }
    }

    /// Calls the corresponding `WKWebView` API.
    pub fn perform_text_finder_action(&self, action: TextFinderAction) {
        unsafe { ffi::wk_webview_perform_text_finder_action(self.ptr, action.as_raw()) }
    }

    /// Take a snapshot of the current page and return PNG bytes.
    ///
    /// # Errors
    /// Returns an error if snapshotting fails.
    pub fn take_snapshot_png(&self) -> Result<Vec<u8>, WebKitError> {
        self.take_snapshot_png_with_configuration(&SnapshotConfiguration::default())
    }

    /// Take a snapshot using a custom configuration and return PNG bytes.
    ///
    /// # Errors
    /// Returns an error if snapshotting fails.
    pub fn take_snapshot_png_with_configuration(
        &self,
        configuration: &SnapshotConfiguration,
    ) -> Result<Vec<u8>, WebKitError> {
        let rect = configuration.rect.unwrap_or_default();
        let mut out_png: *mut u8 = ptr::null_mut();
        let mut out_len: usize = 0;
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_take_snapshot_png(
                self.ptr,
                configuration.rect.is_some(),
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                configuration.snapshot_width.is_some(),
                configuration.snapshot_width.unwrap_or_default(),
                configuration.after_screen_updates,
                &mut out_png,
                &mut out_len,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_bytes(out_png, out_len) })
    }

    /// Create a PDF representation of the current page.
    ///
    /// # Errors
    /// Returns an error if PDF generation fails.
    pub fn create_pdf(&self, configuration: &PDFConfiguration) -> Result<Vec<u8>, WebKitError> {
        let rect = configuration.rect.unwrap_or_default();
        let mut out_bytes: *mut u8 = ptr::null_mut();
        let mut out_len: usize = 0;
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_create_pdf(
                self.ptr,
                configuration.rect.is_some(),
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                configuration.allow_transparent_background,
                &mut out_bytes,
                &mut out_len,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_bytes(out_bytes, out_len) })
    }

    /// Calls the corresponding `WKWebView` API.
    pub fn fetch_data_of_types(&self, data_types: WebViewDataType) -> Result<Vec<u8>, WebKitError> {
        let mut out_bytes: *mut u8 = ptr::null_mut();
        let mut out_len: usize = 0;
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_fetch_data_of_types(
                self.ptr,
                data_types.bits(),
                &mut out_bytes,
                &mut out_len,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_bytes(out_bytes, out_len) })
    }

    /// Calls the corresponding `WKWebView` API.
    pub fn restore_data(&self, data: &[u8]) -> Result<(), WebKitError> {
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_restore_data(self.ptr, data.as_ptr(), data.len(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Start a download for the given request URL.
    ///
    /// # Errors
    /// Returns an error if the URL is invalid or the download cannot be started.
    pub fn start_download_using_request(
        &self,
        url: &str,
        destination_directory: impl AsRef<Path>,
    ) -> Result<Download, WebKitError> {
        let c_url = to_cstring(url);
        let c_destination_directory = to_cstring(&destination_directory.as_ref().to_string_lossy());
        let mut out_download: *mut c_void = ptr::null_mut();
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_start_download(
                self.ptr,
                c_url.as_ptr(),
                c_destination_directory.as_ptr(),
                &mut out_download,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Download::from_ptr(out_download).ok_or_else(|| {
            WebKitError::FrameworkError("start_download returned no download handle".to_owned())
        })
    }

    /// Pump the main run loop for the given duration to drain pending events.
    pub fn pump_run_loop(&self, duration: Duration) {
        let _ = self.ptr;
        crate::pump_run_loop(duration.as_secs_f64());
    }
}

impl Drop for WebView {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ffi::wk_webview_set_nav_callback(self.ptr, None, ptr::null_mut());
                ffi::wk_webview_set_msg_callback(self.ptr, None, ptr::null_mut());
                ffi::wk_webview_release(self.ptr);
            }
            self.ptr = ptr::null_mut();
        }
    }
}
