#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn wk_attributed_string_release(ptr: *mut c_void);
    pub fn wk_attributed_string_copy_string(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_attributed_string_copy_document_attributes_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_attributed_string_load_html_request(
        request_url: *const c_char,
        options_json: *const c_char,
        out_attributed_string: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_attributed_string_load_html_file(
        file_url: *const c_char,
        options_json: *const c_char,
        out_attributed_string: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_attributed_string_load_html_string(
        string: *const c_char,
        options_json: *const c_char,
        out_attributed_string: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_attributed_string_load_html_data(
        bytes: *const u8,
        len: usize,
        options_json: *const c_char,
        out_attributed_string: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
}
