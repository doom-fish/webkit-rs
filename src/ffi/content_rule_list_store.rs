#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn wk_content_rule_list_store_default() -> *mut c_void;
    pub fn wk_content_rule_list_store_with_path(path: *const c_char) -> *mut c_void;
    pub fn wk_content_rule_list_store_release(ptr: *mut c_void);
    pub fn wk_content_rule_list_release(ptr: *mut c_void);
    pub fn wk_content_rule_list_copy_identifier(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_content_rule_list_store_compile(
        ptr: *mut c_void,
        identifier: *const c_char,
        encoded_rule_list: *const c_char,
        out_rule_list: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_content_rule_list_store_lookup(
        ptr: *mut c_void,
        identifier: *const c_char,
        out_rule_list: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_content_rule_list_store_remove(
        ptr: *mut c_void,
        identifier: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_content_rule_list_store_copy_available_identifiers_json(
        ptr: *mut c_void,
        out_json: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
}
