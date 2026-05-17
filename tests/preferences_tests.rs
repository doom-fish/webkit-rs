use webkit::prelude::*;

#[test]
fn preferences_defaults_are_sensible() {
    let preferences = Preferences::default();
    assert!(preferences.minimum_font_size.abs() < f64::EPSILON);
    assert!(preferences.java_script_enabled);
    assert!(matches!(
        preferences.inactive_scheduling_policy,
        InactiveSchedulingPolicy::Suspend
    ));
    assert!(matches!(
        preferences.upgrade_to_https_policy,
        UpgradeToHTTPSPolicy::KeepAsRequested
    ));
}

#[test]
fn upgrade_to_https_policy_maps_raw_values() {
    assert_eq!(
        UpgradeToHTTPSPolicy::from_raw(0),
        UpgradeToHTTPSPolicy::KeepAsRequested
    );
    assert_eq!(
        UpgradeToHTTPSPolicy::from_raw(1),
        UpgradeToHTTPSPolicy::AutomaticFallbackToHttp
    );
    assert_eq!(
        UpgradeToHTTPSPolicy::from_raw(2),
        UpgradeToHTTPSPolicy::UserMediatedFallbackToHttp
    );
    assert_eq!(
        UpgradeToHTTPSPolicy::from_raw(3),
        UpgradeToHTTPSPolicy::ErrorOnFailure
    );
    assert_eq!(UpgradeToHTTPSPolicy::ErrorOnFailure.as_raw(), 3);
}
