use core::ffi::c_void;
use core::ptr;
use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::WebKitError;
use crate::ffi;
use crate::private::{
    maybe_take_error, take_json_or_default, take_string, to_cstring, to_json_cstring,
};

/// Mirrors the `READ_ACCESS_URL_DOCUMENT_OPTION` constant used by `NSAttributedString`.
pub const READ_ACCESS_URL_DOCUMENT_OPTION: &str = "NSReadAccessURLDocumentOption";

/// Completion handler used by `NSAttributedString` APIs.
pub type AttributedStringCompletionHandler =
    Box<dyn FnOnce(Result<AttributedString, WebKitError>) + Send + 'static>;

/// Wraps `NSAttributedString`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HtmlLoadRequest {
    /// Mirrors the `url` value exposed by `NSAttributedString`.
    pub url: String,
}

impl HtmlLoadRequest {
    /// Creates a value for `NSAttributedString`.
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

/// Configures `NSAttributedString`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttributedStringLoadOptions {
    /// Mirrors the `read_access_url` value exposed by `NSAttributedString`.
    pub read_access_url: Option<String>,
    /// Mirrors the `base_url` value exposed by `NSAttributedString`.
    pub base_url: Option<String>,
    /// Mirrors the `timeout_seconds` value exposed by `NSAttributedString`.
    pub timeout_seconds: Option<f64>,
    /// Mirrors the `text_size_multiplier` value exposed by `NSAttributedString`.
    pub text_size_multiplier: Option<f64>,
    /// Mirrors the `text_encoding_name` value exposed by `NSAttributedString`.
    pub text_encoding_name: Option<String>,
    /// Mirrors the `character_encoding` value exposed by `NSAttributedString`.
    pub character_encoding: Option<u64>,
}

impl AttributedStringLoadOptions {
    /// Creates a value for `NSAttributedString`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the corresponding option used by `NSAttributedString`.
    #[must_use]
    pub fn with_read_access_url(mut self, read_access_url: impl AsRef<Path>) -> Self {
        self.read_access_url = Some(read_access_url.as_ref().to_string_lossy().into_owned());
        self
    }

    /// Sets the corresponding option used by `NSAttributedString`.
    #[must_use]
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the corresponding option used by `NSAttributedString`.
    #[must_use]
    pub fn with_timeout_seconds(mut self, timeout_seconds: f64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    /// Sets the corresponding option used by `NSAttributedString`.
    #[must_use]
    pub fn with_text_size_multiplier(mut self, text_size_multiplier: f64) -> Self {
        self.text_size_multiplier = Some(text_size_multiplier);
        self
    }

    /// Sets the corresponding option used by `NSAttributedString`.
    #[must_use]
    pub fn with_text_encoding_name(mut self, text_encoding_name: impl Into<String>) -> Self {
        self.text_encoding_name = Some(text_encoding_name.into());
        self
    }

    /// Sets the corresponding option used by `NSAttributedString`.
    #[must_use]
    pub const fn with_character_encoding(mut self, character_encoding: u64) -> Self {
        self.character_encoding = Some(character_encoding);
        self
    }
}

/// Wraps `NSAttributedString`.
pub struct AttributedString {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for AttributedString {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for AttributedString {}

impl AttributedString {
    #[must_use]
    fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Calls the corresponding `NSAttributedString` API.
    pub fn load_from_html_request(
        request: &HtmlLoadRequest,
        options: &AttributedStringLoadOptions,
    ) -> Result<Self, WebKitError> {
        let request_url = to_cstring(&request.url);
        let options_json = to_json_cstring(options);
        let mut out_attributed_string = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_attributed_string_load_html_request(
                request_url.as_ptr(),
                options_json.as_ptr(),
                &mut out_attributed_string,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_attributed_string).ok_or_else(|| {
            WebKitError::FrameworkError(
                "wk_attributed_string_load_html_request returned null".to_owned(),
            )
        })
    }

    /// Calls the corresponding `NSAttributedString` API.
    pub fn load_from_html_file(
        file_url: impl AsRef<Path>,
        options: &AttributedStringLoadOptions,
    ) -> Result<Self, WebKitError> {
        let file_url = to_cstring(&file_url.as_ref().to_string_lossy());
        let options_json = to_json_cstring(options);
        let mut out_attributed_string = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_attributed_string_load_html_file(
                file_url.as_ptr(),
                options_json.as_ptr(),
                &mut out_attributed_string,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_attributed_string).ok_or_else(|| {
            WebKitError::FrameworkError(
                "wk_attributed_string_load_html_file returned null".to_owned(),
            )
        })
    }

    /// Calls the corresponding `NSAttributedString` API.
    pub fn load_from_html_string(
        html: &str,
        options: &AttributedStringLoadOptions,
    ) -> Result<Self, WebKitError> {
        let html = to_cstring(html);
        let options_json = to_json_cstring(options);
        let mut out_attributed_string = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_attributed_string_load_html_string(
                html.as_ptr(),
                options_json.as_ptr(),
                &mut out_attributed_string,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_attributed_string).ok_or_else(|| {
            WebKitError::FrameworkError(
                "wk_attributed_string_load_html_string returned null".to_owned(),
            )
        })
    }

    /// Calls the corresponding `NSAttributedString` API.
    pub fn load_from_html_data(
        html: &[u8],
        options: &AttributedStringLoadOptions,
    ) -> Result<Self, WebKitError> {
        let options_json = to_json_cstring(options);
        let mut out_attributed_string = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_attributed_string_load_html_data(
                html.as_ptr(),
                html.len(),
                options_json.as_ptr(),
                &mut out_attributed_string,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_attributed_string).ok_or_else(|| {
            WebKitError::FrameworkError(
                "wk_attributed_string_load_html_data returned null".to_owned(),
            )
        })
    }

    /// Returns the corresponding value from `NSAttributedString`.
    #[must_use]
    pub fn string(&self) -> String {
        unsafe { take_string(ffi::wk_attributed_string_copy_string(self.ptr)) }
    }

    /// Returns the corresponding value from `NSAttributedString`.
    #[must_use]
    pub fn document_attributes(&self) -> BTreeMap<String, Value> {
        unsafe {
            take_json_or_default(ffi::wk_attributed_string_copy_document_attributes_json(
                self.ptr,
            ))
        }
    }
}

impl Drop for AttributedString {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_attributed_string_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}
