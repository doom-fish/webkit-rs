use core::ffi::{c_char, c_void, CStr};
use core::ptr;
use std::time::Duration;

use crate::config::WebViewConfiguration;
use crate::error::WebKitError;
use crate::ffi::{self, WKMsgCallback, WKNavCallback};
use crate::private::{maybe_take_error, take_string, to_cstring};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NavigationEventKind {
    DidFinish,
    DidFail,
    DidFailProvisional,
    DecidePolicyForAction,
    Unknown(String),
}

#[derive(Debug, Clone)]
pub struct NavigationEvent {
    pub kind: NavigationEventKind,
    pub url: String,
    pub error: Option<String>,
    pub navigation_type: Option<i64>,
}

impl NavigationEvent {
    fn from_json(json: &str) -> Self {
        let event = extract_str(json, "event").unwrap_or_default();
        let url = extract_str(json, "url").unwrap_or_default();
        let error = extract_str(json, "error");
        let navigation_type = extract_i64(json, "navigationType");
        let kind = match event.as_str() {
            "didFinish" => NavigationEventKind::DidFinish,
            "didFail" => NavigationEventKind::DidFail,
            "didFailProvisional" => NavigationEventKind::DidFailProvisional,
            "decidePolicyForAction" => NavigationEventKind::DecidePolicyForAction,
            other => NavigationEventKind::Unknown(other.to_owned()),
        };
        Self {
            kind,
            url,
            error,
            navigation_type,
        }
    }
}

fn extract_str(json: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\":\"");
    let start = json.find(&needle)? + needle.len();
    let rest = &json[start..];
    let mut result = String::new();
    let mut chars = rest.chars();

    loop {
        match chars.next()? {
            '"' => break,
            '\\' => match chars.next()? {
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                other => {
                    result.push('\\');
                    result.push(other);
                }
            },
            ch => result.push(ch),
        }
    }

    Some(result)
}

fn extract_i64(json: &str, key: &str) -> Option<i64> {
    let needle = format!("\"{key}\":");
    let start = json.find(&needle)? + needle.len();
    let rest = json[start..].trim_start();
    let end = rest
        .find(|ch: char| !ch.is_ascii_digit() && ch != '-')
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}

type NavigationHandler = dyn Fn(NavigationEvent) + Send + 'static;
type MessageHandler = dyn Fn(&str, &str) + Send + 'static;

struct NavCallbackHolder {
    f: Box<NavigationHandler>,
}

struct MsgCallbackHolder {
    f: Box<MessageHandler>,
}

unsafe extern "C" fn nav_trampoline(user_info: *mut c_void, event_json: *const c_char) {
    if user_info.is_null() || event_json.is_null() {
        return;
    }

    let holder = &*(user_info.cast::<NavCallbackHolder>());
    let json = CStr::from_ptr(event_json).to_string_lossy();
    let event = NavigationEvent::from_json(&json);
    (holder.f)(event);
}

unsafe extern "C" fn msg_trampoline(
    user_info: *mut c_void,
    handler_name: *const c_char,
    body: *const c_char,
) {
    if user_info.is_null() {
        return;
    }

    let holder = &*(user_info.cast::<MsgCallbackHolder>());
    let name = if handler_name.is_null() {
        ""
    } else {
        CStr::from_ptr(handler_name).to_str().unwrap_or("")
    };
    let body_str = if body.is_null() {
        ""
    } else {
        CStr::from_ptr(body).to_str().unwrap_or("")
    };
    (holder.f)(name, body_str);
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
}

// SAFETY: The Swift bridge ensures all WebKit calls happen on the main thread.
unsafe impl Send for WebView {}

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
        })
    }

    /// Register a navigation-event callback.
    pub fn set_navigation_handler<F>(&mut self, f: F)
    where
        F: Fn(NavigationEvent) + Send + 'static,
    {
        let holder = Box::new(NavCallbackHolder { f: Box::new(f) });
        let user_info = std::ptr::from_ref(holder.as_ref()).cast_mut().cast::<c_void>();
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
        let user_info = std::ptr::from_ref(holder.as_ref()).cast_mut().cast::<c_void>();
        unsafe {
            ffi::wk_webview_set_msg_callback(
                self.ptr,
                Some(msg_trampoline as WKMsgCallback),
                user_info,
            );
        }
        self.msg_holder = Some(holder);
    }

    /// Load a URL and block until navigation finishes.
    ///
    /// # Errors
    /// Returns an error if the URL is invalid or navigation fails.
    pub fn load_url(&self, url: &str) -> Result<(), WebKitError> {
        let c_url = to_cstring(url);
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe { ffi::wk_webview_load_url(self.ptr, c_url.as_ptr(), &mut out_err) };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Load an HTML string and block until navigation finishes.
    ///
    /// # Errors
    /// Returns an error if navigation fails.
    pub fn load_html(&self, html: &str, base_url: Option<&str>) -> Result<(), WebKitError> {
        let c_html = to_cstring(html);
        let c_base = base_url.map(to_cstring);
        let base_ptr = c_base.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_load_html(self.ptr, c_html.as_ptr(), base_ptr, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
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
            ffi::wk_webview_evaluate_js(
                self.ptr,
                c_js.as_ptr(),
                &mut out_result,
                &mut out_err,
            )
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
            ffi::wk_webview_call_async_js(
                self.ptr,
                c_js.as_ptr(),
                &mut out_result,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_string(out_result) })
    }

    /// Take a snapshot of the current page and return PNG bytes.
    ///
    /// # Errors
    /// Returns an error if snapshotting fails.
    pub fn take_snapshot_png(&self) -> Result<Vec<u8>, WebKitError> {
        let mut out_png: *mut u8 = ptr::null_mut();
        let mut out_len: usize = 0;
        let mut out_err: *mut c_char = ptr::null_mut();
        let status = unsafe {
            ffi::wk_webview_take_snapshot_png(
                self.ptr,
                &mut out_png,
                &mut out_len,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        if out_png.is_null() {
            return Err(WebKitError::FrameworkError(
                "snapshot returned null".to_owned(),
            ));
        }

        let bytes = unsafe { std::slice::from_raw_parts(out_png, out_len).to_vec() };
        unsafe { ffi::wk_bytes_free(out_png, out_len) }
        Ok(bytes)
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
