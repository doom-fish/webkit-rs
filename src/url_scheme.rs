use core::ffi::{c_char, c_void, CStr};
use core::ptr;
use std::collections::BTreeMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::config::WebViewConfiguration;
use crate::error::WebKitError;
use crate::ffi::{self, WKURLSchemeTaskCallback};
use crate::private::{maybe_take_error, to_cstring, to_json_cstring};

/// Request metadata for a `WKURLSchemeTask`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UrlSchemeRequest {
    pub url: String,
    pub method: String,
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
}

/// Response metadata sent back to a `WKURLSchemeTask`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlSchemeResponse {
    pub url: String,
    pub mime_type: String,
    pub text_encoding_name: Option<String>,
    pub status_code: i64,
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
}

impl UrlSchemeResponse {
    #[must_use]
    pub fn new(url: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            mime_type: mime_type.into(),
            text_encoding_name: None,
            status_code: 200,
            headers: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_text_encoding_name(mut self, text_encoding_name: impl Into<String>) -> Self {
        self.text_encoding_name = Some(text_encoding_name.into());
        self
    }

    #[must_use]
    pub fn with_status_code(mut self, status_code: i64) -> Self {
        self.status_code = status_code;
        self
    }

    #[must_use]
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }
}

/// Safe wrapper around `WKURLSchemeTask` callbacks.
pub struct UrlSchemeTask {
    ptr: *mut c_void,
    request: UrlSchemeRequest,
}

// SAFETY: The Swift bridge serialises WebKit operations onto the main thread.
unsafe impl Send for UrlSchemeTask {}
// SAFETY: The Swift bridge serialises WebKit operations onto the main thread.
unsafe impl Sync for UrlSchemeTask {}

impl UrlSchemeTask {
    fn from_ptr_and_json(ptr: *mut c_void, request_json: *const c_char) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }

        let request = if request_json.is_null() {
            UrlSchemeRequest::default()
        } else {
            let json = unsafe { CStr::from_ptr(request_json) }.to_string_lossy();
            serde_json::from_str(&json).unwrap_or_default()
        };

        Some(Self { ptr, request })
    }

    #[must_use]
    pub fn request(&self) -> &UrlSchemeRequest {
        &self.request
    }

    pub fn did_receive_response(&self, response: &UrlSchemeResponse) -> Result<(), WebKitError> {
        let response_json = to_json_cstring(response);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_url_scheme_task_send_response_json(self.ptr, response_json.as_ptr(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn did_receive_data(&self, data: &[u8]) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_url_scheme_task_send_data(self.ptr, data.as_ptr(), data.len(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn did_finish(&self) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe { ffi::wk_url_scheme_task_finish(self.ptr, &mut out_err) };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn did_fail(&self, message: &str) -> Result<(), WebKitError> {
        let c_message = to_cstring(message);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_url_scheme_task_fail(self.ptr, c_message.as_ptr(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn respond(&self, response: &UrlSchemeResponse, body: &[u8]) -> Result<(), WebKitError> {
        self.did_receive_response(response)?;
        if !body.is_empty() {
            self.did_receive_data(body)?;
        }
        self.did_finish()
    }
}

impl Drop for UrlSchemeTask {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_url_scheme_task_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}

/// Trait mirroring the `WKURLSchemeHandler` protocol.
pub trait UrlSchemeHandler: Send + Sync + 'static {
    fn start(&self, task: UrlSchemeTask);
    fn stop(&self, task: UrlSchemeTask);
}

struct UrlSchemeHandlerHolder {
    handler: Box<dyn UrlSchemeHandler>,
}

unsafe extern "C" fn url_scheme_start_trampoline(
    user_info: *mut c_void,
    task: *mut c_void,
    request_json: *const c_char,
) {
    if user_info.is_null() || task.is_null() {
        return;
    }

    let holder = unsafe { Arc::<UrlSchemeHandlerHolder>::from_raw(user_info.cast()) };
    let cloned = Arc::clone(&holder);
    let _ = Arc::into_raw(holder);
    if let Some(task) = UrlSchemeTask::from_ptr_and_json(task, request_json) {
        cloned.handler.start(task);
    }
}

unsafe extern "C" fn url_scheme_stop_trampoline(
    user_info: *mut c_void,
    task: *mut c_void,
    request_json: *const c_char,
) {
    if user_info.is_null() || task.is_null() {
        return;
    }

    let holder = unsafe { Arc::<UrlSchemeHandlerHolder>::from_raw(user_info.cast()) };
    let cloned = Arc::clone(&holder);
    let _ = Arc::into_raw(holder);
    if let Some(task) = UrlSchemeTask::from_ptr_and_json(task, request_json) {
        cloned.handler.stop(task);
    }
}

/// Called by the Swift bridge when a registered URL scheme handler is released.
#[no_mangle]
pub extern "C" fn wk_rust_release_url_scheme_handler(user_info: *mut c_void) {
    if user_info.is_null() {
        return;
    }
    unsafe {
        drop(Arc::<UrlSchemeHandlerHolder>::from_raw(user_info.cast()));
    }
}

impl WebViewConfiguration {
    /// Register a custom URL scheme handler on this configuration.
    ///
    /// # Errors
    /// Returns an error if the scheme is invalid, already registered, or unsupported on the current macOS version.
    pub fn set_url_scheme_handler<H>(&self, scheme: &str, handler: H) -> Result<(), WebKitError>
    where
        H: UrlSchemeHandler,
    {
        let c_scheme = to_cstring(scheme);
        let holder = Arc::new(UrlSchemeHandlerHolder {
            handler: Box::new(handler),
        });
        let user_info = Arc::into_raw(holder).cast_mut().cast::<c_void>();

        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_config_set_url_scheme_handler(
                self.as_ptr(),
                c_scheme.as_ptr(),
                Some(url_scheme_start_trampoline as WKURLSchemeTaskCallback),
                Some(url_scheme_stop_trampoline as WKURLSchemeTaskCallback),
                user_info,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            wk_rust_release_url_scheme_handler(user_info);
            return Err(error);
        }
        Ok(())
    }
}
