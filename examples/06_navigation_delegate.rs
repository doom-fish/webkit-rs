mod common;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let mut view = WebView::with_config(&config)?;
    view.set_navigation_delegate_config(&NavigationDelegateConfig::default());
    view.set_navigation_handler(|event| {
        println!("navigation event: {:?} {}", event.kind, event.url);
    });

    common::load_html(&view, "<p>navigation delegate</p>", "https://nav-delegate.test/")?;
    let events = view.drain_navigation_events();
    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind, NavigationEventKind::DidFinish))
    );

    let action_event = events
        .iter()
        .find(|event| matches!(event.kind, NavigationEventKind::DecidePolicyForAction))
        .expect("expected a navigation action policy event");
    let action = action_event
        .navigation_action
        .as_ref()
        .expect("expected typed navigation action details");
    assert!(action.source_frame.main_frame);
    assert!(matches!(action.navigation_type, NavigationType::Other | NavigationType::LinkActivated));

    if let Some(response_event) = events
        .iter()
        .find(|event| matches!(event.kind, NavigationEventKind::DecidePolicyForResponse))
    {
        let response = response_event
            .navigation_response
            .as_ref()
            .expect("expected typed navigation response details");
        assert!(response.for_main_frame);
        assert!(response.can_show_mime_type);
    }

    println!("captured {} navigation delegate events", events.len());
    Ok(())
}
