mod common;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let preferences = Preferences {
        minimum_font_size: 18.0,
        java_script_can_open_windows_automatically: false,
        should_print_backgrounds: true,
        upgrade_to_https_policy: UpgradeToHTTPSPolicy::AutomaticFallbackToHttp,
        ..Preferences::default()
    };
    config.set_preferences(&preferences);

    let roundtrip = config.preferences();
    assert!((roundtrip.minimum_font_size - 18.0).abs() < f64::EPSILON);
    assert!(!roundtrip.java_script_can_open_windows_automatically);
    assert_eq!(
        roundtrip.upgrade_to_https_policy,
        UpgradeToHTTPSPolicy::AutomaticFallbackToHttp
    );

    let view = WebView::with_config(&config)?;
    common::load_html(&view, "<p>preferences</p>", "https://preferences.test/")?;
    assert_eq!(
        view.evaluate_javascript("document.body.textContent.trim()")?,
        "preferences"
    );

    println!("preferences round-trip succeeded");
    Ok(())
}
