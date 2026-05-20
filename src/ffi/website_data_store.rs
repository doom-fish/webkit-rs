#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn wk_website_data_store_default() -> *mut c_void;
    pub fn wk_website_data_store_nonpersistent() -> *mut c_void;
    pub fn wk_website_data_store_for_identifier(
        identifier: *const c_char,
        out_store: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_website_data_store_release(ptr: *mut c_void);
    pub fn wk_website_data_store_is_persistent(ptr: *mut c_void) -> bool;
    pub fn wk_website_data_store_copy_identifier(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_website_data_store_copy_all_data_types_json() -> *mut c_char;
    pub fn wk_website_data_store_fetch_all_identifiers_json(
        out_json: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_website_data_store_remove_data_store_for_identifier(
        identifier: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_website_data_store_copy_http_cookie_store(ptr: *mut c_void) -> *mut c_void;
    pub fn wk_website_data_store_fetch_data_records_json(
        ptr: *mut c_void,
        data_types_json: *const c_char,
        out_json: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_website_data_store_remove_data_for_display_names(
        ptr: *mut c_void,
        data_types_json: *const c_char,
        display_names_json: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_website_data_store_remove_data_modified_since(
        ptr: *mut c_void,
        data_types_json: *const c_char,
        modified_since_unix_seconds: f64,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_website_data_store_copy_proxy_configurations(
        ptr: *mut c_void,
        out_proxy_configurations: *mut *mut *mut c_void,
        out_len: *mut usize,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_website_data_store_set_proxy_configurations(
        ptr: *mut c_void,
        proxy_configurations: *const *mut c_void,
        len: usize,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_website_data_store_clear_proxy_configurations(
        ptr: *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_website_data_store_fetch_data(
        ptr: *mut c_void,
        data_types_json: *const c_char,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_website_data_store_restore_data(
        ptr: *mut c_void,
        bytes: *const u8,
        len: usize,
        out_err: *mut *mut c_char,
    ) -> i32;
}
