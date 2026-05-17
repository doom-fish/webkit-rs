#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub type AsyncCb =
    unsafe extern "C" fn(result: *const c_char, error: *const c_char, ctx: *mut c_void);

pub type AsyncBytesCb =
    unsafe extern "C" fn(bytes: *const u8, len: usize, error: *const c_char, ctx: *mut c_void);

pub type AsyncPtrCb =
    unsafe extern "C" fn(result: *mut c_void, error: *const c_char, ctx: *mut c_void);

unsafe extern "C" {
    pub fn wk_webview_evaluate_js_async(
        ptr: *mut c_void,
        js: *const c_char,
        cb: AsyncCb,
        ctx: *mut c_void,
    );
    pub fn wk_webview_call_async_js_async(
        ptr: *mut c_void,
        js: *const c_char,
        cb: AsyncCb,
        ctx: *mut c_void,
    );
    pub fn wk_webview_take_snapshot_async(
        ptr: *mut c_void,
        has_rect: bool,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        has_snapshot_width: bool,
        snapshot_width: f64,
        after_screen_updates: bool,
        cb: AsyncBytesCb,
        ctx: *mut c_void,
    );
    pub fn wk_webview_create_pdf_async(
        ptr: *mut c_void,
        has_rect: bool,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        allow_transparent_background: bool,
        cb: AsyncBytesCb,
        ctx: *mut c_void,
    );
    pub fn wk_webview_create_web_archive_async(
        ptr: *mut c_void,
        cb: AsyncBytesCb,
        ctx: *mut c_void,
    );
    pub fn wk_webview_find_string_async(
        ptr: *mut c_void,
        query: *const c_char,
        configuration_json: *const c_char,
        cb: AsyncCb,
        ctx: *mut c_void,
    );
    pub fn wk_website_data_store_fetch_data_records_async(
        ptr: *mut c_void,
        data_types_json: *const c_char,
        cb: AsyncCb,
        ctx: *mut c_void,
    );
    pub fn wk_website_data_store_remove_data_async(
        ptr: *mut c_void,
        data_types_json: *const c_char,
        display_names_json: *const c_char,
        cb: AsyncCb,
        ctx: *mut c_void,
    );
    pub fn wk_http_cookie_store_get_all_cookies_async(
        ptr: *mut c_void,
        cb: AsyncCb,
        ctx: *mut c_void,
    );
    pub fn wk_content_rule_list_store_compile_async(
        ptr: *mut c_void,
        identifier: *const c_char,
        encoded_rule_list: *const c_char,
        cb: AsyncPtrCb,
        ctx: *mut c_void,
    );
    pub fn wk_download_cancel_async(ptr: *mut c_void, cb: AsyncBytesCb, ctx: *mut c_void);
}
