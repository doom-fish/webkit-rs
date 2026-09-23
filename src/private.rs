use core::ffi::c_char;
use std::ffi::CString;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

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

pub fn javascript_arguments_json<A>(arguments: &A) -> Result<CString, WebKitError>
where
    A: Serialize + ?Sized,
{
    match serde_json::to_value(arguments) {
        Ok(Value::Object(map)) => serde_json::to_string(&map)
            .map(|json| to_cstring(&json))
            .map_err(|error| WebKitError::InvalidArgument(error.to_string())),
        Ok(Value::Null) => Ok(to_cstring("{}")),
        Ok(_) => Err(WebKitError::InvalidArgument(
            "callAsyncJavaScript arguments must serialize to a JSON object".to_owned(),
        )),
        Err(error) => Err(WebKitError::InvalidArgument(format!(
            "callAsyncJavaScript arguments are not serializable: {error}"
        ))),
    }
}

/// Take ownership of a C string returned by the bridge and convert it to a
/// Rust `String`, freeing the original buffer.
///
/// # Safety
/// `ptr` must have been allocated by `wk_string_free`-compatible bridge code.
pub unsafe fn take_string(ptr: *mut c_char) -> String {
    doom_fish_utils::ffi_string::take_owned_cstring_c(ptr, |p| ffi::wk_string_free(p))
        .unwrap_or_default()
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::javascript_arguments_json;
    use crate::error::WebKitError;

    #[test]
    fn javascript_arguments_must_be_a_json_object() {
        let arguments = javascript_arguments_json(&json!({"name": "</script><img src=x>", "n": 2}))
            .expect("object arguments serialize");
        let text = arguments.to_str().expect("utf-8 arguments");
        let parsed: serde_json::Value = serde_json::from_str(text).expect("valid JSON");
        assert_eq!(parsed["name"], "</script><img src=x>");
        assert_eq!(parsed["n"], 2);

        let mut map = BTreeMap::new();
        map.insert("nul", "a\0b");
        let with_nul = javascript_arguments_json(&map).expect("map arguments serialize");
        assert!(with_nul.to_str().expect("utf-8").contains("\\u0000"));

        assert_eq!(
            javascript_arguments_json(&json!(null))
                .expect("null means no arguments")
                .to_str()
                .expect("utf-8"),
            "{}"
        );
        for invalid in [json!([1, 2]), json!("text"), json!(3)] {
            assert!(matches!(
                javascript_arguments_json(&invalid),
                Err(WebKitError::InvalidArgument(_))
            ));
        }
    }
}
