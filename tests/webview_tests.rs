mod common;

use webkit::prelude::*;

#[test]
fn webview_extra_surfaces_are_available() {
    let mut data_types = WebViewDataType::SESSION_STORAGE;
    data_types |= WebViewDataType::NONE;

    assert!(data_types.contains(WebViewDataType::SESSION_STORAGE));
    assert_eq!(MediaCaptureState::Muted.as_raw(), 2);
    assert_eq!(MediaCaptureState::from_raw(1), MediaCaptureState::Active);
    assert_eq!(
        MediaPlaybackState::from_raw(3),
        MediaPlaybackState::Suspended
    );
    assert_eq!(FullscreenState::from_raw(2), FullscreenState::InFullscreen);

    let _: fn(&WebView) -> Vec<UIDelegateEventDetail> = WebView::drain_ui_event_details;
    let _: fn(&WebView) -> Result<MediaPlaybackState, WebKitError> =
        WebView::request_media_playback_state;
    let _: fn(&WebView) -> MediaCaptureState = WebView::camera_capture_state;
    let _: fn(&WebView) -> MediaCaptureState = WebView::microphone_capture_state;
    let _: fn(&WebView, MediaCaptureState) -> Result<(), WebKitError> =
        WebView::set_camera_capture_state;
    let _: fn(&WebView, MediaCaptureState) -> Result<(), WebKitError> =
        WebView::set_microphone_capture_state;
    let _: fn(&WebView) -> FullscreenState = WebView::fullscreen_state;
    let _: fn(&WebView) = WebView::perform_go_back_action;
    let _: fn(&WebView) = WebView::perform_go_forward_action;
    let _: fn(&WebView) = WebView::perform_reload_action;
    let _: fn(&WebView) = WebView::perform_reload_from_origin_action;
    let _: fn(&WebView) = WebView::perform_stop_loading_action;
    let _: fn(&WebView, TextFinderAction) -> bool = WebView::can_perform_text_finder_action;
    let _: fn(&WebView, TextFinderAction) = WebView::perform_text_finder_action;
    let _: fn(&WebView, WebViewDataType) -> Result<Vec<u8>, WebKitError> =
        WebView::fetch_data_of_types;
    let _: fn(&WebView, &[u8]) -> Result<(), WebKitError> = WebView::restore_data;
}

#[test]
#[ignore = "WKWebView smoke tests must run on the process main thread; examples cover live validation"]
fn webview_loads_and_evaluates_html() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    common::load_html(
        &view,
        "<main id='root'>hello</main>",
        "https://webview.test/",
    )?;
    assert_eq!(
        view.evaluate_javascript("document.getElementById('root').textContent")?,
        "hello"
    );
    Ok(())
}
