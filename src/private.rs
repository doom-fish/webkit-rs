use core::ffi::{c_char, CStr};
use std::ffi::CString;

use serde::{de::DeserializeOwned, Serialize};

use crate::error::WebKitError;
use crate::ffi;

pub fn to_cstring(value: &str) -> CString {
    CString::new(value)
        .unwrap_or_else(|_| CString::new(value.replace('\0', " ")).unwrap_or_default())
}

pub fn to_json_cstring<T>(value: &T) -> CString
where
    T: Serialize + ?Sized,
{
    let json = serde_json::to_string(value).unwrap_or_else(|_| "null".to_owned());
    to_cstring(&json)
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

/// Take ownership of an optional C string returned by the bridge.
///
/// # Safety
/// `ptr` must have been allocated by the bridge.
pub unsafe fn take_optional_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(take_string(ptr))
    }
}

/// Decode a bridge-owned JSON C string into `T`.
///
/// # Safety
/// `ptr` must have been allocated by the bridge.
pub unsafe fn take_json_or_default<T>(ptr: *mut c_char) -> T
where
    T: DeserializeOwned + Default,
{
    let json = take_string(ptr);
    serde_json::from_str(&json).unwrap_or_default()
}

/// Take ownership of bridge-owned bytes and return a `Vec<u8>`.
///
/// # Safety
/// `ptr` and `len` must describe a buffer allocated by `wk_bytes_free`-compatible bridge code.
pub unsafe fn take_bytes(ptr: *mut u8, len: usize) -> Vec<u8> {
    if ptr.is_null() || len == 0 {
        if !ptr.is_null() {
            ffi::wk_bytes_free(ptr, len);
        }
        return Vec::new();
    }
    let bytes = std::slice::from_raw_parts(ptr, len).to_vec();
    ffi::wk_bytes_free(ptr, len);
    bytes
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
