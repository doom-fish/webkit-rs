use core::ffi::{c_char, c_void, CStr};
use core::ptr;
use std::path::Path;
use std::time::Duration;

use crate::back_forward_list::BackForwardList;
use crate::config::WebViewConfiguration;
use crate::download::Download;
use crate::error::WebKitError;
use crate::ffi::{self, WKMsgCallback, WKNavCallback};
use crate::navigation::Navigation;
use crate::navigation_delegate::{NavigationDelegateConfig, NavigationEvent};
use crate::pdf_configuration::PDFConfiguration;
use crate::private::{maybe_take_error, take_bytes, take_json_or_default, take_string, to_cstring};
use crate::script_message_handler::ScriptMessage;
use crate::snapshot_configuration::SnapshotConfiguration;
use crate::ui_delegate::{UIDelegateConfig, UIDelegateEvent};

pub use crate::navigation_delegate::NavigationEventKind;

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
    let event = serde_json::from_str::<NavigationEvent>(&json)
        .unwrap_or_else(|_| NavigationEvent::unknown());
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

    pub fn set_navigation_delegate_config(&self, config: &NavigationDelegateConfig) {
        unsafe {
            ffi::wk_webview_set_navigation_delegate_config(
                self.ptr,
                config.action_policy.as_raw(),
                config.response_policy.as_raw(),
            );
        }
    }

    #[must_use]
    pub fn drain_navigation_events(&self) -> Vec<NavigationEvent> {
        unsafe { take_json_or_default(ffi::wk_webview_drain_navigation_events_json(self.ptr)) }
    }

    pub fn set_ui_delegate_config(&self, config: &UIDelegateConfig) {
        let prompt_response = config.prompt_response.as_deref().map(to_cstring);
        let prompt_response_ptr = prompt_response.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        unsafe {
            ffi::wk_webview_set_ui_delegate_config(
                self.ptr,
                config.confirm_response,
                prompt_response_ptr,
            );
        }
    }

    #[must_use]
    pub fn drain_ui_events(&self) -> Vec<UIDelegateEvent> {
        unsafe { take_json_or_default(ffi::wk_webview_drain_ui_events_json(self.ptr)) }
    }

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

    #[must_use]
    pub fn reload_from_origin(&self) -> Option<Navigation> {
        let mut out_navigation: *mut c_void = ptr::null_mut();
        let has_navigation = unsafe { ffi::wk_webview_reload_from_origin(self.ptr, &mut out_navigation) };
        if has_navigation {
            Navigation::from_ptr(out_navigation)
        } else {
            None
        }
    }

    pub fn stop_loading(&self) {
        unsafe { ffi::wk_webview_stop_loading(self.ptr) }
    }

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

    #[must_use]
    pub fn title(&self) -> String {
        unsafe { take_string(ffi::wk_webview_copy_title(self.ptr)) }
    }

    #[must_use]
    pub fn url(&self) -> String {
        unsafe { take_string(ffi::wk_webview_copy_url(self.ptr)) }
    }

    #[must_use]
    pub fn is_loading(&self) -> bool {
        unsafe { ffi::wk_webview_is_loading(self.ptr) }
    }

    #[must_use]
    pub fn estimated_progress(&self) -> f64 {
        unsafe { ffi::wk_webview_get_estimated_progress(self.ptr) }
    }

    #[must_use]
    pub fn has_only_secure_content(&self) -> bool {
        unsafe { ffi::wk_webview_get_has_only_secure_content(self.ptr) }
    }

    #[must_use]
    pub fn can_go_back(&self) -> bool {
        unsafe { ffi::wk_webview_get_can_go_back(self.ptr) }
    }

    #[must_use]
    pub fn can_go_forward(&self) -> bool {
        unsafe { ffi::wk_webview_get_can_go_forward(self.ptr) }
    }

    #[must_use]
    pub fn back_forward_list(&self) -> BackForwardList {
        unsafe { BackForwardList::from_json_ptr(ffi::wk_webview_copy_back_forward_list_json(self.ptr)) }
    }

    pub fn set_custom_user_agent(&self, value: Option<&str>) {
        let value = value.map(to_cstring);
        let value_ptr = value.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        unsafe { ffi::wk_webview_set_custom_user_agent(self.ptr, value_ptr) }
    }

    #[must_use]
    pub fn custom_user_agent(&self) -> String {
        unsafe { take_string(ffi::wk_webview_copy_custom_user_agent(self.ptr)) }
    }

    pub fn set_allows_link_preview(&self, value: bool) {
        unsafe { ffi::wk_webview_set_allows_link_preview(self.ptr, value) }
    }

    #[must_use]
    pub fn allows_link_preview(&self) -> bool {
        unsafe { ffi::wk_webview_get_allows_link_preview(self.ptr) }
    }

    pub fn set_page_zoom(&self, value: f64) {
        unsafe { ffi::wk_webview_set_page_zoom(self.ptr, value) }
    }

    #[must_use]
    pub fn page_zoom(&self) -> f64 {
        unsafe { ffi::wk_webview_get_page_zoom(self.ptr) }
    }

    pub fn set_media_type(&self, value: Option<&str>) {
        let value = value.map(to_cstring);
        let value_ptr = value.as_ref().map_or(ptr::null(), |value| value.as_ptr());
        unsafe { ffi::wk_webview_set_media_type(self.ptr, value_ptr) }
    }

    #[must_use]
    pub fn media_type(&self) -> String {
        unsafe { take_string(ffi::wk_webview_copy_media_type(self.ptr)) }
    }

    pub fn set_inspectable(&self, value: bool) {
        unsafe { ffi::wk_webview_set_inspectable(self.ptr, value) }
    }

    #[must_use]
    pub fn is_inspectable(&self) -> bool {
        unsafe { ffi::wk_webview_get_inspectable(self.ptr) }
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
