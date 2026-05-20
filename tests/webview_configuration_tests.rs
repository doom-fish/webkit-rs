use webkit::prelude::*;

#[test]
fn webview_configuration_enum_helpers_are_typed() {
    let mut media_types = AudiovisualMediaTypes::AUDIO;
    media_types |= AudiovisualMediaTypes::VIDEO;

    assert!(media_types.contains(AudiovisualMediaTypes::AUDIO));
    assert!(media_types.contains(AudiovisualMediaTypes::VIDEO));
    assert_eq!(UserInterfaceDirectionPolicy::System.as_raw(), 1);
    assert_eq!(
        UserInterfaceDirectionPolicy::from_raw(0),
        UserInterfaceDirectionPolicy::Content
    );

    let _: fn(&WebViewConfiguration, AudiovisualMediaTypes) =
        WebViewConfiguration::set_media_types_requiring_user_action_for_playback;
    let _: fn(&WebViewConfiguration) -> AudiovisualMediaTypes =
        WebViewConfiguration::media_types_requiring_user_action_for_playback;
    let _: fn(&WebViewConfiguration, UserInterfaceDirectionPolicy) =
        WebViewConfiguration::set_user_interface_direction_policy;
    let _: fn(&WebViewConfiguration) -> UserInterfaceDirectionPolicy =
        WebViewConfiguration::user_interface_direction_policy;
    let _: fn(&WebViewConfiguration, bool) -> Result<(), WebKitError> =
        WebViewConfiguration::set_shows_system_screen_time_blocking_view;
    let _: fn(&WebViewConfiguration) -> Result<bool, WebKitError> =
        WebViewConfiguration::shows_system_screen_time_blocking_view;
}

#[test]
#[ignore = "WKWebViewConfiguration bridge tests must run on the process main thread; examples cover live validation"]
fn webview_configuration_roundtrips_basic_settings() {
    let config = WebViewConfiguration::new();
    let preferences = Preferences {
        minimum_font_size: 14.0,
        java_script_can_open_windows_automatically: false,
        upgrade_to_https_policy: UpgradeToHTTPSPolicy::AutomaticFallbackToHttp,
        ..Preferences::default()
    };
    config.set_preferences(&preferences);
    config.set_application_name_for_user_agent("webkit-rs/test");
    config.set_allows_airplay_for_media_playback(false);
    config.set_allows_content_javascript(true);

    assert_eq!(config.application_name_for_user_agent(), "webkit-rs/test");
    assert!(!config.allows_airplay_for_media_playback());
    assert!(config.allows_content_javascript());
    let roundtrip = config.preferences();
    assert!((roundtrip.minimum_font_size - 14.0).abs() < f64::EPSILON);
    assert_eq!(
        roundtrip.upgrade_to_https_policy,
        UpgradeToHTTPSPolicy::AutomaticFallbackToHttp
    );
}

#[test]
#[ignore = "WKWebViewConfiguration bridge tests must run on the process main thread; examples cover live validation"]
fn webview_configuration_roundtrips_screen_time_blocking_view_when_supported(
) -> Result<(), Box<dyn std::error::Error>> {
    let config = WebViewConfiguration::new();
    match config.shows_system_screen_time_blocking_view() {
        Ok(initial) => {
            config.set_shows_system_screen_time_blocking_view(!initial)?;
            assert_eq!(config.shows_system_screen_time_blocking_view()?, !initial);
        }
        Err(WebKitError::Unsupported(_)) => {}
        Err(error) => return Err(Box::new(error)),
    }
    Ok(())
}
