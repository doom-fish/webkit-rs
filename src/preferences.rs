use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum InactiveSchedulingPolicy {
    #[default]
    Suspend = 0,
    Throttle = 1,
    None = 2,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum UpgradeToHTTPSPolicy {
    #[default]
    #[serde(rename = "keepAsRequested")]
    KeepAsRequested = 0,
    #[serde(rename = "automaticFallbackToHTTP")]
    AutomaticFallbackToHttp = 1,
    #[serde(rename = "userMediatedFallbackToHTTP")]
    UserMediatedFallbackToHttp = 2,
    #[serde(rename = "errorOnFailure")]
    ErrorOnFailure = 3,
}

impl UpgradeToHTTPSPolicy {
    #[must_use]
    pub const fn as_raw(self) -> i32 {
        self as i32
    }

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

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub minimum_font_size: f64,
    pub java_script_can_open_windows_automatically: bool,
    pub fraudulent_website_warning_enabled: bool,
    pub should_print_backgrounds: bool,
    pub tab_focuses_links: bool,
    pub text_interaction_enabled: bool,
    pub site_specific_quirks_mode_enabled: bool,
    pub element_fullscreen_enabled: bool,
    pub inactive_scheduling_policy: InactiveSchedulingPolicy,
    pub java_script_enabled: bool,
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
