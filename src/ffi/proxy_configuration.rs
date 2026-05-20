#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn wk_proxy_configuration_create_http_connect(
        host: *const c_char,
        port: u16,
        out_proxy_configuration: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_proxy_configuration_create_socksv5(
        host: *const c_char,
        port: u16,
        out_proxy_configuration: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_proxy_configuration_release(ptr: *mut c_void);
    pub fn wk_proxy_configuration_copy_summary_json(
        ptr: *mut c_void,
        out_json: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_proxy_configuration_set_username_and_password(
        ptr: *mut c_void,
        username: *const c_char,
        password: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_proxy_configuration_set_failover_allowed(
        ptr: *mut c_void,
        allowed: bool,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_proxy_configuration_add_match_domain(
        ptr: *mut c_void,
        domain: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_proxy_configuration_clear_match_domains(
        ptr: *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_proxy_configuration_add_excluded_domain(
        ptr: *mut c_void,
        domain: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_proxy_configuration_clear_excluded_domains(
        ptr: *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_pointer_array_free(ptr: *mut *mut c_void);
}
