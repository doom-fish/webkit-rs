use core::ffi::{c_char, CStr};
use std::ffi::CString;

use crate::error::WebKitError;
use crate::ffi;

pub fn to_cstring(value: &str) -> CString {
    CString::new(value).unwrap_or_else(|_| {
        CString::new(value.replace('\0', " ")).unwrap_or_default()
    })
}

/// Take ownership of a C string returned by the bridge and convert it to a
/// Rust `String`, freeing the original buffer.
///
/// # Safety
/// `ptr` must have been allocated by `wk_string_free`-compatible bridge code.
pub unsafe fn take_string(ptr: *mut c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::wk_string_free(ptr);
    string
}

/// Read an error pointer written by the bridge. If non-null, takes ownership
/// and converts it to a [`WebKitError`].
///
/// # Safety
/// `ptr` must have been allocated by the bridge.
pub unsafe fn maybe_take_error(status: i32, ptr: *mut c_char) -> Option<WebKitError> {
    if ptr.is_null() {
        return if status == ffi::status::OK {
            None
        } else {
            Some(crate::error::error_from_status(
                status,
                format!("bridge returned status {status}"),
            ))
        };
    }

    let message = take_string(ptr);
    Some(crate::error::error_from_status(status, message))
}
