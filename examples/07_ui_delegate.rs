mod common;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    view.set_ui_delegate_config(&UIDelegateConfig {
        confirm_response: true,
        prompt_response: Some("Rust".to_owned()),
    });

    common::load_html(&view, "<p>ui delegate</p>", "https://ui-delegate.test/")?;
    let result = view.evaluate_javascript(
        "(() => { alert('hello'); return String(confirm('continue?')) + '|' + prompt('name?', 'fallback'); })()",
    )?;
    assert_eq!(result, "true|Rust");
    let events = view.drain_ui_events();
    assert!(events.len() >= 3, "expected alert/confirm/prompt events");

    println!("UI delegate returned {result}");
    Ok(())
}
