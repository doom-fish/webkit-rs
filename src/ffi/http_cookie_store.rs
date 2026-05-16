#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn wk_http_cookie_store_release(ptr: *mut c_void);
    pub fn wk_http_cookie_store_copy_all_cookies_json(
        ptr: *mut c_void,
        out_json: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_http_cookie_store_set_cookie(
        ptr: *mut c_void,
        cookie_json: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_http_cookie_store_set_cookies(
        ptr: *mut c_void,
        cookies_json: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_http_cookie_store_delete_cookie(
        ptr: *mut c_void,
        cookie_json: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_http_cookie_store_set_observing(ptr: *mut c_void, observing: bool);
    pub fn wk_http_cookie_store_drain_events_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_http_cookie_store_set_cookie_policy(
        ptr: *mut c_void,
        policy: i32,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_http_cookie_store_get_cookie_policy(
        ptr: *mut c_void,
        out_policy: *mut i32,
        out_err: *mut *mut c_char,
    ) -> i32;
}
