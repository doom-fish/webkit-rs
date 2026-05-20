use core::ffi::c_void;
use core::ptr;

use serde::{Deserialize, Serialize};

use crate::error::WebKitError;
use crate::ffi;
use crate::private::{maybe_take_error, take_json_or_default, to_cstring};

/// Summarises a `nw_proxy_config_t` used by `WKWebsiteDataStore`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProxyConfigurationSummary {
    /// Human-readable description returned by Network.framework.
    pub description: String,
    /// Whether failover to a direct connection is allowed.
    pub failover_allowed: bool,
    /// Domain suffixes that should use the proxy.
    #[serde(default)]
    pub match_domains: Vec<String>,
    /// Domain suffixes that should bypass the proxy.
    #[serde(default)]
    pub excluded_domains: Vec<String>,
}

/// Wraps `nw_proxy_config_t` values used by `WKWebsiteDataStore`.
pub struct ProxyConfiguration {
    ptr: *mut c_void,
}

// SAFETY: The raw pointer is an opaque Network.framework object managed by the Swift bridge.
unsafe impl Send for ProxyConfiguration {}
// SAFETY: The Swift bridge serialises property access through main-thread WebKit APIs.
unsafe impl Sync for ProxyConfiguration {}

impl ProxyConfiguration {
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

    /// Creates a cleartext HTTP CONNECT proxy configuration.
    pub fn http_connect(host: &str, port: u16) -> Result<Self, WebKitError> {
        let c_host = to_cstring(host);
        let mut out_proxy_configuration = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_proxy_configuration_create_http_connect(
                c_host.as_ptr(),
                port,
                &mut out_proxy_configuration,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_proxy_configuration).ok_or_else(|| {
            WebKitError::FrameworkError("http_connect returned no proxy configuration".to_owned())
        })
    }

    /// Creates a SOCKSv5 proxy configuration.
    pub fn socks_v5(host: &str, port: u16) -> Result<Self, WebKitError> {
        let c_host = to_cstring(host);
        let mut out_proxy_configuration = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_proxy_configuration_create_socksv5(
                c_host.as_ptr(),
                port,
                &mut out_proxy_configuration,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Self::from_ptr(out_proxy_configuration).ok_or_else(|| {
            WebKitError::FrameworkError("socks_v5 returned no proxy configuration".to_owned())
        })
    }

    /// Configures proxy authentication credentials.
    pub fn set_username_and_password(
        &self,
        username: &str,
        password: Option<&str>,
    ) -> Result<(), WebKitError> {
        let c_username = to_cstring(username);
        let c_password = password.map(to_cstring);
        let password_ptr = c_password
            .as_ref()
            .map_or(ptr::null(), |value| value.as_ptr());
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_proxy_configuration_set_username_and_password(
                self.ptr,
                c_username.as_ptr(),
                password_ptr,
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Sets whether the proxy may fail over to a direct connection.
    pub fn set_failover_allowed(&self, allowed: bool) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_proxy_configuration_set_failover_allowed(self.ptr, allowed, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Adds a domain suffix that should use the proxy.
    pub fn add_match_domain(&self, domain: &str) -> Result<(), WebKitError> {
        let c_domain = to_cstring(domain);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_proxy_configuration_add_match_domain(self.ptr, c_domain.as_ptr(), &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Removes all match domains from the proxy configuration.
    pub fn clear_match_domains(&self) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status =
            unsafe { ffi::wk_proxy_configuration_clear_match_domains(self.ptr, &mut out_err) };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Adds a domain suffix that should bypass the proxy.
    pub fn add_excluded_domain(&self, domain: &str) -> Result<(), WebKitError> {
        let c_domain = to_cstring(domain);
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_proxy_configuration_add_excluded_domain(
                self.ptr,
                c_domain.as_ptr(),
                &mut out_err,
            )
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Removes all excluded domains from the proxy configuration.
    pub fn clear_excluded_domains(&self) -> Result<(), WebKitError> {
        let mut out_err = ptr::null_mut();
        let status =
            unsafe { ffi::wk_proxy_configuration_clear_excluded_domains(self.ptr, &mut out_err) };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(())
    }

    /// Returns an inspectable summary of the proxy configuration.
    pub fn summary(&self) -> Result<ProxyConfigurationSummary, WebKitError> {
        let mut out_json = ptr::null_mut();
        let mut out_err = ptr::null_mut();
        let status = unsafe {
            ffi::wk_proxy_configuration_copy_summary_json(self.ptr, &mut out_json, &mut out_err)
        };
        if let Some(error) = unsafe { maybe_take_error(status, out_err) } {
            return Err(error);
        }
        Ok(unsafe { take_json_or_default(out_json) })
    }

    /// Returns whether the proxy may fail over to a direct connection.
    pub fn failover_allowed(&self) -> Result<bool, WebKitError> {
        Ok(self.summary()?.failover_allowed)
    }

    /// Returns the configured match domains.
    pub fn match_domains(&self) -> Result<Vec<String>, WebKitError> {
        Ok(self.summary()?.match_domains)
    }

    /// Returns the configured excluded domains.
    pub fn excluded_domains(&self) -> Result<Vec<String>, WebKitError> {
        Ok(self.summary()?.excluded_domains)
    }
}

impl Drop for ProxyConfiguration {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_proxy_configuration_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProxyConfigurationSummary;

    #[test]
    fn proxy_configuration_summary_round_trips_through_json(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let summary = ProxyConfigurationSummary {
            description: "HTTP CONNECT proxy".to_owned(),
            failover_allowed: true,
            match_domains: vec!["example.test".to_owned()],
            excluded_domains: vec!["static.example.test".to_owned()],
        };

        let json = serde_json::to_string(&summary)?;
        let roundtrip: ProxyConfigurationSummary = serde_json::from_str(&json)?;

        assert_eq!(roundtrip, summary);
        Ok(())
    }
}
