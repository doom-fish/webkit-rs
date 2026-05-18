use core::ffi::c_void;
use core::ptr;

use serde::Deserialize;

use crate::error::WebKitError;
use crate::ffi;
use crate::private::{maybe_take_error, take_bytes, take_json_or_default, take_string};

/// Wraps `WKDownloadRedirectPolicy` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DownloadRedirectPolicy {
    /// Mirrors the `Cancel` case used by `WKDownloadRedirectPolicy`.
    Cancel = 0,
    /// Mirrors the `Allow` case used by `WKDownloadRedirectPolicy`.
    Allow = 1,
}

impl DownloadRedirectPolicy {
    /// Returns the corresponding value from `WKDownloadRedirectPolicy`.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Creates a value for `WKDownloadRedirectPolicy`.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Cancel,
            _ => Self::Allow,
        }
    }
}

/// Captures data returned by `WKDownload`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadEvent {
    /// Mirrors the `kind` value exposed by `WKDownload`.
    pub kind: String,
    /// Mirrors the `suggested_filename` value exposed by `WKDownload`.
    pub suggested_filename: Option<String>,
    /// Mirrors the `destination` value exposed by `WKDownload`.
    pub destination: Option<String>,
    /// Mirrors the `original_request_url` value exposed by `WKDownload`.
    pub original_request_url: Option<String>,
    /// Mirrors the `error` value exposed by `WKDownload`.
    pub error: Option<String>,
    /// Mirrors the `has_resume_data` value exposed by `WKDownload`.
    pub has_resume_data: Option<bool>,
    /// Mirrors the `status_code` value exposed by `WKDownload`.
    pub status_code: Option<i64>,
    /// Mirrors the `url` value exposed by `WKDownload`.
    pub url: Option<String>,
}

/// Wraps `WKDownload`.
pub struct Download {
    ptr: *mut c_void,
}

unsafe impl Send for Download {}

impl Download {
    #[must_use]
    pub(crate) fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    #[must_use]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Returns the corresponding value from `WKDownload`.
    #[must_use]
    pub fn original_request_url(&self) -> String {
        unsafe { take_string(ffi::wk_download_copy_original_request_url(self.ptr)) }
    }

    /// Returns the corresponding value from `WKDownload`.
    #[must_use]
    pub fn is_user_initiated(&self) -> bool {
        unsafe { ffi::wk_download_is_user_initiated(self.ptr) }
    }

    /// Sets the corresponding value on `WKDownload`.
    pub fn set_redirect_policy(&self, policy: DownloadRedirectPolicy) {
        unsafe { ffi::wk_download_set_redirect_policy(self.ptr, policy.as_raw()) }
    }

    /// Returns the corresponding value from `WKDownload`.
    #[must_use]
    pub fn redirect_policy(&self) -> DownloadRedirectPolicy {
        DownloadRedirectPolicy::from_raw(unsafe { ffi::wk_download_get_redirect_policy(self.ptr) })
    }

    /// Returns the corresponding value from `WKDownload`.
    #[must_use]
    pub fn drain_events(&self) -> Vec<DownloadEvent> {
        unsafe { take_json_or_default(ffi::wk_download_copy_events_json(self.ptr)) }
    }

    /// Calls the corresponding `WKDownload` API.
    pub fn cancel(&self) -> Result<Option<Vec<u8>>, WebKitError> {
        let mut out_resume_data: *mut u8 = ptr::null_mut();
        let mut out_resume_data_len: usize = 0;
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_download_cancel(
                self.ptr,
                &mut out_resume_data,
                &mut out_resume_data_len,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        if out_resume_data.is_null() {
            Ok(None)
        } else {
            Ok(Some(unsafe {
                take_bytes(out_resume_data, out_resume_data_len)
            }))
        }
    }
}

impl Drop for Download {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_download_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}
