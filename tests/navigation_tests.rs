mod common;

use std::sync::{Arc, Mutex};
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
    assert!(common::wait_for(Duration::from_secs(2), || view
        .url()
        .contains("first-navigation.test")));
    assert!(view.go_forward().is_some());
    assert!(common::wait_for(Duration::from_secs(2), || view
        .url()
        .contains("second-navigation.test")));
    Ok(())
}

#[test]
#[ignore = "WKNavigationDelegate back/forward tests must run on the process main thread; examples cover live validation"]
fn back_forward_list_navigation_handler_receives_events() -> Result<(), Box<dyn std::error::Error>>
{
    let config = common::base_config();
    let mut view = WebView::with_config(&config)?;
    let events = Arc::new(Mutex::new(Vec::<BackForwardListNavigationEvent>::new()));
    let seen_events = Arc::clone(&events);

    view.set_back_forward_list_navigation_handler(move |event| {
        seen_events
            .lock()
            .expect("back/forward event mutex poisoned")
            .push(event);
    });
    view.set_back_forward_list_navigation_policy(BackForwardListNavigationPolicy::Allow);

    view.load_html(
        "<!doctype html><html><head><title>first</title></head><body>first</body></html>",
        Some("https://first-history.test/"),
    )?;
    view.load_html(
        "<!doctype html><html><head><title>second</title></head><body>second</body></html>",
        Some("https://second-history.test/"),
    )?;

    let _result = view.evaluate_javascript("history.back()");
    assert!(common::wait_for(Duration::from_secs(2), || !events
        .lock()
        .expect("back/forward event mutex poisoned")
        .is_empty()));

    let first_event = {
        let events = events.lock().expect("back/forward event mutex poisoned");
        events[0].clone()
    };
    assert_eq!(first_event.item.url, "https://first-history.test/");
    assert_eq!(first_event.item.relative_index, -1);
    Ok(())
}
