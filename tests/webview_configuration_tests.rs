use webkit::prelude::*;

#[test]
#[ignore = "WKWebViewConfiguration bridge tests must run on the process main thread; examples cover live validation"]
fn webview_configuration_roundtrips_basic_settings() {
    let config = WebViewConfiguration::new();
    let preferences = Preferences {
        minimum_font_size: 14.0,
        java_script_can_open_windows_automatically: false,
        ..Preferences::default()
    };
    config.set_preferences(&preferences);
    config.set_application_name_for_user_agent("webkit-rs/test");
    config.set_allows_airplay_for_media_playback(false);
    config.set_allows_content_javascript(true);

    assert_eq!(config.application_name_for_user_agent(), "webkit-rs/test");
    assert!(!config.allows_airplay_for_media_playback());
    assert!(config.allows_content_javascript());
    assert!((config.preferences().minimum_font_size - 14.0).abs() < f64::EPSILON);
}
