use core::ffi::c_void;
use core::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::WebKitError;
use crate::ffi;
use crate::private::{maybe_take_error, take_json_or_default, to_json_cstring};

/// Wraps `NSHTTPCookie.AcceptPolicy` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum CookiePolicy {
    /// Mirrors the `Allow` case used by `NSHTTPCookie.AcceptPolicy`.
    Allow = 0,
    /// Mirrors the `Disallow` case used by `NSHTTPCookie.AcceptPolicy`.
    Disallow = 1,
}

impl CookiePolicy {
    /// Returns the corresponding value from `NSHTTPCookie.AcceptPolicy`.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Creates a value for `NSHTTPCookie.AcceptPolicy`.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Disallow,
            _ => Self::Allow,
        }
    }
}

/// Wraps `NSHTTPCookie`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    /// Mirrors the `name` value exposed by `NSHTTPCookie`.
    pub name: String,
    /// Mirrors the `value` value exposed by `NSHTTPCookie`.
    pub value: String,
    /// Mirrors the `domain` value exposed by `NSHTTPCookie`.
    pub domain: String,
    /// Mirrors the `path` value exposed by `NSHTTPCookie`.
    pub path: String,
    /// Mirrors the `secure` value exposed by `NSHTTPCookie`.
    pub secure: bool,
    /// Mirrors the `http_only` value exposed by `NSHTTPCookie`.
    pub http_only: bool,
    /// Mirrors the `session_only` value exposed by `NSHTTPCookie`.
    pub session_only: bool,
    /// Mirrors the `expires` value exposed by `NSHTTPCookie`.
    pub expires: Option<i64>,
}

impl Cookie {
    /// Creates a value for `NSHTTPCookie`.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
        domain: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            domain: domain.into(),
            path: "/".to_owned(),
            secure: false,
            http_only: false,
            session_only: false,
            expires: None,
        }
    }

    /// Sets the corresponding option used by `NSHTTPCookie`.
    #[must_use]
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    /// Sets the corresponding option used by `NSHTTPCookie`.
    #[must_use]
    pub const fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Sets the corresponding option used by `NSHTTPCookie`.
    #[must_use]
    pub const fn with_http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }

    /// Sets the corresponding option used by `NSHTTPCookie`.
    #[must_use]
    pub const fn with_session_only(mut self, session_only: bool) -> Self {
        self.session_only = session_only;
        self
    }

    /// Sets the corresponding option used by `NSHTTPCookie`.
    #[must_use]
    pub fn with_expires(mut self, expires_at: SystemTime) -> Self {
        let seconds = expires_at
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.expires = Some(i64::try_from(seconds).unwrap_or(i64::MAX));
        self
    }
}

/// Captures data returned by `WKHTTPCookieStore`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookieStoreEvent {
    /// Mirrors the `kind` value exposed by `WKHTTPCookieStore`.
    pub kind: String,
}

/// Wraps `WKHTTPCookieStore`.
pub struct HttpCookieStore {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for HttpCookieStore {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for HttpCookieStore {}

impl HttpCookieStore {
    #[must_use]
    pub(crate) fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    #[cfg(feature = "async")]
    #[must_use]
    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Returns the corresponding value from `WKHTTPCookieStore`.
    pub fn all_cookies(&self) -> Result<Vec<Cookie>, WebKitError> {
        let mut out_json = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_http_cookie_store_copy_all_cookies_json(self.ptr, &mut out_json, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_json_or_default(out_json) })
    }

    /// Sets the corresponding value on `WKHTTPCookieStore`.
    pub fn set_cookie(&self, cookie: &Cookie) -> Result<(), WebKitError> {
        let cookie_json = to_json_cstring(cookie);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_http_cookie_store_set_cookie(self.ptr, cookie_json.as_ptr(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Sets the corresponding value on `WKHTTPCookieStore`.
    pub fn set_cookies(&self, cookies: &[Cookie]) -> Result<(), WebKitError> {
        let cookies_json = to_json_cstring(cookies);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_http_cookie_store_set_cookies(self.ptr, cookies_json.as_ptr(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Calls the corresponding `WKHTTPCookieStore` API.
    pub fn delete_cookie(&self, cookie: &Cookie) -> Result<(), WebKitError> {
        let cookie_json = to_json_cstring(cookie);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_http_cookie_store_delete_cookie(self.ptr, cookie_json.as_ptr(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Calls the corresponding `WKHTTPCookieStore` API.
    pub fn start_observing(&self) {
        unsafe { ffi::wk_http_cookie_store_set_observing(self.ptr, true) }
    }

    /// Calls the corresponding `WKHTTPCookieStore` API.
    pub fn stop_observing(&self) {
        unsafe { ffi::wk_http_cookie_store_set_observing(self.ptr, false) }
    }

    /// Returns the corresponding value from `WKHTTPCookieStore`.
    #[must_use]
    pub fn drain_events(&self) -> Vec<CookieStoreEvent> {
        unsafe { take_json_or_default(ffi::wk_http_cookie_store_drain_events_json(self.ptr)) }
    }

    /// Sets the corresponding value on `WKHTTPCookieStore`.
    pub fn set_cookie_policy(&self, policy: CookiePolicy) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_http_cookie_store_set_cookie_policy(self.ptr, policy.as_raw(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Returns the corresponding value from `WKHTTPCookieStore`.
    pub fn cookie_policy(&self) -> Result<CookiePolicy, WebKitError> {
        let mut out_policy = 0;
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_http_cookie_store_get_cookie_policy(self.ptr, &mut out_policy, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(CookiePolicy::from_raw(out_policy))
    }
}

impl Drop for HttpCookieStore {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_http_cookie_store_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}
