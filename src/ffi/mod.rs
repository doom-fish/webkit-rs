#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub type WKNavCallback =
    unsafe extern "C" fn(user_info: *mut c_void, event_json: *const c_char);

pub type WKMsgCallback = unsafe extern "C" fn(
    user_info: *mut c_void,
    handler_name: *const c_char,
    body: *const c_char,
);

unsafe extern "C" {
    // Lifecycle
    pub fn wk_string_free(s: *mut c_char);
    pub fn wk_bytes_free(ptr: *mut u8, len: usize);
    pub fn wk_run_loop_pump(seconds: f64);
    pub fn wk_init_app();

    // Configuration
    pub fn wk_config_new() -> *mut c_void;
    pub fn wk_config_release(ptr: *mut c_void);
    pub fn wk_config_set_application_name(ptr: *mut c_void, name: *const c_char);
    pub fn wk_config_set_allows_airplay(ptr: *mut c_void, v: bool);
    pub fn wk_config_set_allows_content_javascript(ptr: *mut c_void, v: bool);
    pub fn wk_config_use_nonpersistent_data_store(ptr: *mut c_void);
    pub fn wk_config_add_user_script(
        ptr: *mut c_void,
        source: *const c_char,
        injection_time: i32,
        main_frame_only: bool,
    );
    pub fn wk_config_add_message_handler_name(ptr: *mut c_void, name: *const c_char);

    // WebView
    pub fn wk_webview_new(cfg: *mut c_void) -> *mut c_void;
    pub fn wk_webview_release(ptr: *mut c_void);
    pub fn wk_webview_set_nav_callback(
        ptr: *mut c_void,
        callback: Option<WKNavCallback>,
        user_info: *mut c_void,
    );
    pub fn wk_webview_set_msg_callback(
        ptr: *mut c_void,
        callback: Option<WKMsgCallback>,
        user_info: *mut c_void,
    );
    pub fn wk_webview_load_url(
        ptr: *mut c_void,
        url: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_load_html(
        ptr: *mut c_void,
        html: *const c_char,
        base_url: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_evaluate_js(
        ptr: *mut c_void,
        js: *const c_char,
        out_result: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_call_async_js(
        ptr: *mut c_void,
        js: *const c_char,
        out_result: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_take_snapshot_png(
        ptr: *mut c_void,
        out_png: *mut *mut u8,
        out_len: *mut usize,
        out_err: *mut *mut c_char,
    ) -> i32;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const TIMED_OUT: i32 = -3;
    pub const FRAMEWORK_ERROR: i32 = -5;
    pub const UNKNOWN: i32 = -99;
}
