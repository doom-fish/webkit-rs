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

    println!("captured {} navigation delegate events", events.len());
    Ok(())
}
