use webkit::prelude::*;

#[test]
fn preferences_defaults_are_sensible() {
    let preferences = Preferences::default();
    assert!(preferences.minimum_font_size.abs() < f64::EPSILON);
    assert!(preferences.java_script_enabled);
    assert!(matches!(preferences.inactive_scheduling_policy, InactiveSchedulingPolicy::Suspend));
}
