mod common;

use webkit::prelude::*;

fn main() {
    let config = common::base_config();
    let preferences = Preferences {
        minimum_font_size: 16.0,
        java_script_can_open_windows_automatically: false,
        ..Preferences::default()
    };
    config.set_preferences(&preferences);
    config.set_application_name_for_user_agent("webkit-rs/0.2.0");
    config.set_allows_airplay_for_media_playback(false);
    config.set_website_data_store(&WebsiteDataStore::non_persistent());

    assert_eq!(config.application_name_for_user_agent(), "webkit-rs/0.2.0");
    assert!((config.preferences().minimum_font_size - 16.0).abs() < f64::EPSILON);
    assert!(config.website_data_store().is_some());

    println!("configured WebViewConfiguration successfully");
}
