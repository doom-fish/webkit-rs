#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub type WKURLSchemeTaskCallback =
    unsafe extern "C" fn(user_info: *mut c_void, task: *mut c_void, request_json: *const c_char);

unsafe extern "C" {
    pub fn wk_config_set_url_scheme_handler(
        ptr: *mut c_void,
        scheme: *const c_char,
        start_callback: Option<WKURLSchemeTaskCallback>,
        stop_callback: Option<WKURLSchemeTaskCallback>,
        user_info: *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_url_scheme_task_release(ptr: *mut c_void);
    pub fn wk_url_scheme_task_send_response_json(
        ptr: *mut c_void,
        response_json: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_url_scheme_task_send_data(
        ptr: *mut c_void,
        bytes: *const u8,
        len: usize,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_url_scheme_task_finish(ptr: *mut c_void, out_err: *mut *mut c_char) -> i32;
    pub fn wk_url_scheme_task_fail(
        ptr: *mut c_void,
        message: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
}
