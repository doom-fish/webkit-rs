use core::ffi::c_void;
use core::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::WebKitError;
use crate::ffi;
use crate::private::{maybe_take_error, take_json_or_default, to_json_cstring};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum CookiePolicy {
    Allow = 0,
    Disallow = 1,
}

impl CookiePolicy {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Disallow,
            _ => Self::Allow,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub session_only: bool,
    pub expires: Option<i64>,
}

impl Cookie {
    #[must_use]
    pub fn new(name: impl Into<String>, value: impl Into<String>, domain: impl Into<String>) -> Self {
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

    #[must_use]
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    #[must_use]
    pub const fn with_secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    #[must_use]
    pub const fn with_http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }

    #[must_use]
    pub const fn with_session_only(mut self, session_only: bool) -> Self {
        self.session_only = session_only;
        self
    }

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookieStoreEvent {
    pub kind: String,
}

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

    pub fn set_cookie(&self, cookie: &Cookie) -> Result<(), WebKitError> {
        let cookie_json = to_json_cstring(cookie);
        let mut out_err = ptr::null_mut();
        let status =
            unsafe { ffi::wk_http_cookie_store_set_cookie(self.ptr, cookie_json.as_ptr(), &mut out_err) };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

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

    pub fn start_observing(&self) {
        unsafe { ffi::wk_http_cookie_store_set_observing(self.ptr, true) }
    }

    pub fn stop_observing(&self) {
        unsafe { ffi::wk_http_cookie_store_set_observing(self.ptr, false) }
    }

    #[must_use]
    pub fn drain_events(&self) -> Vec<CookieStoreEvent> {
        unsafe { take_json_or_default(ffi::wk_http_cookie_store_drain_events_json(self.ptr)) }
    }

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
