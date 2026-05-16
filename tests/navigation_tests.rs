mod common;

use std::time::Duration;

use webkit::prelude::*;

#[test]
#[ignore = "WKWebView navigation smoke tests must run on the process main thread; examples cover live validation"]
fn navigation_handles_back_and_forward() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    let first = view.load_html_with_navigation(
        "<!doctype html><html><head><title>first</title></head><body>first</body></html>",
        Some("https://first-navigation.test/"),
    )?;
    let second = view.load_html_with_navigation(
        "<!doctype html><html><head><title>second</title></head><body>second</body></html>",
        Some("https://second-navigation.test/"),
    )?;

    assert_ne!(first.id(), 0);
    assert_ne!(second.id(), 0);
    assert!(view.go_back().is_some());
    assert!(common::wait_for(Duration::from_secs(2), || view.url().contains("first-navigation.test")));
    assert!(view.go_forward().is_some());
    assert!(common::wait_for(Duration::from_secs(2), || view.url().contains("second-navigation.test")));
    Ok(())
}
