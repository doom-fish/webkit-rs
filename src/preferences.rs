use serde::{Deserialize, Serialize};

/// Wraps `WKInactiveSchedulingPolicy` values.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum InactiveSchedulingPolicy {
    /// Mirrors the `Suspend` case used by `WKInactiveSchedulingPolicy`.
    #[default]
    Suspend = 0,
    /// Mirrors the `Throttle` case used by `WKInactiveSchedulingPolicy`.
    Throttle = 1,
    /// Mirrors the `None` case used by `WKInactiveSchedulingPolicy`.
    None = 2,
}

/// Wraps `WKWebpagePreferences.UpgradeToHTTPSPolicy` values.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum UpgradeToHTTPSPolicy {
    /// Mirrors the `KeepAsRequested` case used by `WKWebpagePreferences.UpgradeToHTTPSPolicy`.
    #[default]
    #[serde(rename = "keepAsRequested")]
    KeepAsRequested = 0,
    /// Mirrors the `AutomaticFallbackToHttp` case used by `WKWebpagePreferences.UpgradeToHTTPSPolicy`.
    #[serde(rename = "automaticFallbackToHTTP")]
    AutomaticFallbackToHttp = 1,
    /// Mirrors the `UserMediatedFallbackToHttp` case used by `WKWebpagePreferences.UpgradeToHTTPSPolicy`.
    #[serde(rename = "userMediatedFallbackToHTTP")]
    UserMediatedFallbackToHttp = 2,
    /// Mirrors the `ErrorOnFailure` case used by `WKWebpagePreferences.UpgradeToHTTPSPolicy`.
    #[serde(rename = "errorOnFailure")]
    ErrorOnFailure = 3,
}

impl UpgradeToHTTPSPolicy {
    /// Returns the corresponding value from `WKWebpagePreferences.UpgradeToHTTPSPolicy`.
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

    /// Creates a value for `WKWebpagePreferences.UpgradeToHTTPSPolicy`.
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::AutomaticFallbackToHttp,
            2 => Self::UserMediatedFallbackToHttp,
            3 => Self::ErrorOnFailure,
            _ => Self::KeepAsRequested,
        }
    }
}

/// Wraps `WKPreferences`.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    /// Mirrors the `minimum_font_size` value exposed by `WKPreferences`.
    pub minimum_font_size: f64,
    /// Mirrors the `java_script_can_open_windows_automatically` value exposed by `WKPreferences`.
    pub java_script_can_open_windows_automatically: bool,
    /// Mirrors the `fraudulent_website_warning_enabled` value exposed by `WKPreferences`.
    pub fraudulent_website_warning_enabled: bool,
    /// Mirrors the `should_print_backgrounds` value exposed by `WKPreferences`.
    pub should_print_backgrounds: bool,
    /// Mirrors the `tab_focuses_links` value exposed by `WKPreferences`.
    pub tab_focuses_links: bool,
    /// Mirrors the `text_interaction_enabled` value exposed by `WKPreferences`.
    pub text_interaction_enabled: bool,
    /// Mirrors the `site_specific_quirks_mode_enabled` value exposed by `WKPreferences`.
    pub site_specific_quirks_mode_enabled: bool,
    /// Mirrors the `element_fullscreen_enabled` value exposed by `WKPreferences`.
    pub element_fullscreen_enabled: bool,
    /// Mirrors the `inactive_scheduling_policy` value exposed by `WKPreferences`.
    pub inactive_scheduling_policy: InactiveSchedulingPolicy,
    /// Mirrors the `java_script_enabled` value exposed by `WKPreferences`.
    pub java_script_enabled: bool,
    /// Mirrors the `upgrade_to_https_policy` value exposed by `WKPreferences`.
    #[serde(default, rename = "upgradeToHTTPSPolicy")]
    pub upgrade_to_https_policy: UpgradeToHTTPSPolicy,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            minimum_font_size: 0.0,
            java_script_can_open_windows_automatically: true,
            fraudulent_website_warning_enabled: true,
            should_print_backgrounds: false,
            tab_focuses_links: false,
            text_interaction_enabled: true,
            site_specific_quirks_mode_enabled: true,
            element_fullscreen_enabled: false,
            inactive_scheduling_policy: InactiveSchedulingPolicy::Suspend,
            java_script_enabled: true,
            upgrade_to_https_policy: UpgradeToHTTPSPolicy::KeepAsRequested,
        }
    }
}

impl Preferences {
    /// Creates a value for `WKPreferences`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
