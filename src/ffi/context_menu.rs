#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn wk_context_menu_element_info_release(ptr: *mut c_void);
    pub fn wk_context_menu_element_info_copy_link_url(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_preview_element_info_release(ptr: *mut c_void);
    pub fn wk_preview_element_info_copy_link_url(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_preview_action_item_release(ptr: *mut c_void);
    pub fn wk_preview_action_item_copy_identifier(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_preview_action_item_copy_title(ptr: *mut c_void) -> *mut c_char;
}
