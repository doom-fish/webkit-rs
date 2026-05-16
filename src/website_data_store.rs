use core::ffi::c_void;
use core::ptr;
use std::borrow::Cow;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::WebKitError;
use crate::ffi;
use crate::http_cookie_store::HttpCookieStore;
use crate::private::{
    maybe_take_error, take_bytes, take_json_or_default, take_optional_string, to_cstring,
    to_json_cstring,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WebsiteDataType(Cow<'static, str>);

impl WebsiteDataType {
    #[must_use]
    pub const fn from_static(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }

    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(Cow::Owned(value.into()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    #[must_use]
    pub const fn fetch_cache() -> Self {
        Self::from_static("WKWebsiteDataTypeFetchCache")
    }

    #[must_use]
    pub const fn disk_cache() -> Self {
        Self::from_static("WKWebsiteDataTypeDiskCache")
    }

    #[must_use]
    pub const fn memory_cache() -> Self {
        Self::from_static("WKWebsiteDataTypeMemoryCache")
    }

    #[must_use]
    pub const fn offline_web_application_cache() -> Self {
        Self::from_static("WKWebsiteDataTypeOfflineWebApplicationCache")
    }

    #[must_use]
    pub const fn cookies() -> Self {
        Self::from_static("WKWebsiteDataTypeCookies")
    }

    #[must_use]
    pub const fn session_storage() -> Self {
        Self::from_static("WKWebsiteDataTypeSessionStorage")
    }

    #[must_use]
    pub const fn local_storage() -> Self {
        Self::from_static("WKWebsiteDataTypeLocalStorage")
    }

    #[must_use]
    pub const fn web_sql_databases() -> Self {
        Self::from_static("WKWebsiteDataTypeWebSQLDatabases")
    }

    #[must_use]
    pub const fn indexed_db_databases() -> Self {
        Self::from_static("WKWebsiteDataTypeIndexedDBDatabases")
    }

    #[must_use]
    pub const fn service_worker_registrations() -> Self {
        Self::from_static("WKWebsiteDataTypeServiceWorkerRegistrations")
    }

    #[must_use]
    pub const fn file_system() -> Self {
        Self::from_static("WKWebsiteDataTypeFileSystem")
    }

    #[must_use]
    pub const fn search_field_recent_searches() -> Self {
        Self::from_static("WKWebsiteDataTypeSearchFieldRecentSearches")
    }

    #[must_use]
    pub const fn media_keys() -> Self {
        Self::from_static("WKWebsiteDataTypeMediaKeys")
    }

    #[must_use]
    pub const fn hash_salt() -> Self {
        Self::from_static("WKWebsiteDataTypeHashSalt")
    }

    #[must_use]
    pub const fn screen_time() -> Self {
        Self::from_static("WKWebsiteDataTypeScreenTime")
    }
}

impl AsRef<str> for WebsiteDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebsiteDataRecord {
    pub display_name: String,
    pub data_types: Vec<WebsiteDataType>,
}

pub struct WebsiteDataStore {
    ptr: *mut c_void,
}

// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Send for WebsiteDataStore {}
// SAFETY: The Swift bridge serialises all WebKit interactions onto the main thread.
unsafe impl Sync for WebsiteDataStore {}

impl Default for WebsiteDataStore {
    fn default() -> Self {
        Self::default_data_store()
    }
}

impl WebsiteDataStore {
    #[must_use]
    pub(crate) fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Create the shared persistent website data store.
    ///
    /// # Panics
    /// Panics if the Swift bridge fails to construct the store.
    #[must_use]
    pub fn default_data_store() -> Self {
        let ptr = unsafe { ffi::wk_website_data_store_default() };
        assert!(!ptr.is_null(), "wk_website_data_store_default returned null");
        Self { ptr }
    }

    /// Create an ephemeral website data store.
    ///
    /// # Panics
    /// Panics if the Swift bridge fails to construct the store.
    #[must_use]
    pub fn non_persistent() -> Self {
        let ptr = unsafe { ffi::wk_website_data_store_nonpersistent() };
        assert!(
            !ptr.is_null(),
            "wk_website_data_store_nonpersistent returned null"
        );
        Self { ptr }
    }

    pub fn data_store_for_identifier(identifier: &str) -> Result<Self, WebKitError> {
        let c_identifier = to_cstring(identifier);
        let mut out_store = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_website_data_store_for_identifier(
                c_identifier.as_ptr(),
                &mut out_store,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_store).ok_or_else(|| {
            WebKitError::FrameworkError("data_store_for_identifier returned null".to_owned())
        })
    }

    pub fn all_data_store_identifiers() -> Result<Vec<String>, WebKitError> {
        let mut out_json = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_website_data_store_fetch_all_identifiers_json(&mut out_json, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_json_or_default(out_json) })
    }

    pub fn remove_data_store_for_identifier(identifier: &str) -> Result<(), WebKitError> {
        let c_identifier = to_cstring(identifier);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_website_data_store_remove_data_store_for_identifier(
                c_identifier.as_ptr(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    #[must_use]
    pub fn all_website_data_types() -> Vec<WebsiteDataType> {
        unsafe { take_json_or_default(ffi::wk_website_data_store_copy_all_data_types_json()) }
    }

    #[must_use]
    pub fn is_persistent(&self) -> bool {
        unsafe { ffi::wk_website_data_store_is_persistent(self.ptr) }
    }

    #[must_use]
    pub fn identifier(&self) -> Option<String> {
        unsafe { take_optional_string(ffi::wk_website_data_store_copy_identifier(self.ptr)) }
    }

    pub fn http_cookie_store(&self) -> Result<HttpCookieStore, WebKitError> {
        let ptr = unsafe { ffi::wk_website_data_store_copy_http_cookie_store(self.ptr) };
        HttpCookieStore::from_ptr(ptr).ok_or_else(|| {
            WebKitError::FrameworkError("http_cookie_store returned null".to_owned())
        })
    }

    pub fn data_records(
        &self,
        data_types: &[WebsiteDataType],
    ) -> Result<Vec<WebsiteDataRecord>, WebKitError> {
        let data_types_json = to_json_cstring(data_types);
        let mut out_json = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_website_data_store_fetch_data_records_json(
                self.ptr,
                data_types_json.as_ptr(),
                &mut out_json,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_json_or_default(out_json) })
    }

    pub fn remove_data_for_records(
        &self,
        data_types: &[WebsiteDataType],
        records: &[WebsiteDataRecord],
    ) -> Result<(), WebKitError> {
        let data_types_json = to_json_cstring(data_types);
        let display_names = records
            .iter()
            .map(|record| record.display_name.clone())
            .collect::<Vec<_>>();
        let display_names_json = to_json_cstring(&display_names);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_website_data_store_remove_data_for_display_names(
                self.ptr,
                data_types_json.as_ptr(),
                display_names_json.as_ptr(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn remove_data_modified_since(
        &self,
        data_types: &[WebsiteDataType],
        modified_since: SystemTime,
    ) -> Result<(), WebKitError> {
        let data_types_json = to_json_cstring(data_types);
        let modified_since = modified_since
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_website_data_store_remove_data_modified_since(
                self.ptr,
                data_types_json.as_ptr(),
                modified_since,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    pub fn fetch_data(&self, data_types: &[WebsiteDataType]) -> Result<Vec<u8>, WebKitError> {
        let data_types_json = to_json_cstring(data_types);
        let mut out_bytes = ptr::null_mut();
        let mut out_len = 0;
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_website_data_store_fetch_data(
                self.ptr,
                data_types_json.as_ptr(),
                &mut out_bytes,
                &mut out_len,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_bytes(out_bytes, out_len) })
    }

    pub fn restore_data(&self, data: &[u8]) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_website_data_store_restore_data(
                self.ptr,
                data.as_ptr(),
                data.len(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }
}

impl Drop for WebsiteDataStore {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_website_data_store_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}
