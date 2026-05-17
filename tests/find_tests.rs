mod common;

use webkit::prelude::*;

#[test]
fn find_configuration_defaults_are_sensible() {
    let configuration = FindConfiguration::default();
    assert!(!configuration.backwards);
    assert!(!configuration.case_sensitive);
    assert!(configuration.wraps);
}

#[test]
fn find_helpers_cover_text_finder_surface() {
    let configuration = FindConfiguration::new()
        .with_backwards(true)
        .with_case_sensitive(true)
        .with_wraps(false);
    let result = FindResult { match_found: true };

    assert!(configuration.backwards);
    assert!(configuration.case_sensitive);
    assert!(!configuration.wraps);
    assert!(result.found());
    assert_eq!(TextFinderAction::NextMatch.as_raw(), 2);
}

#[test]
fn find_result_deserializes_from_json() -> Result<(), Box<dyn std::error::Error>> {
    let result: FindResult = serde_json::from_str(r#"{"matchFound":true}"#)?;
    assert!(result.match_found);
    Ok(())
}

#[test]
#[ignore = "WKFind smoke tests must run on the process main thread; examples cover live validation"]
fn webview_find_string_matches_loaded_text() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    common::load_html(
        &view,
        "<main>Needle in a haystack</main>",
        "https://find.test/",
    )?;

    let result = view.find_string_with_configuration(
        "Needle",
        &FindConfiguration {
            case_sensitive: true,
            ..FindConfiguration::default()
        },
    )?;

    assert!(result.match_found);
    Ok(())
}
