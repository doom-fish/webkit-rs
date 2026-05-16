mod common;

use webkit::prelude::*;

#[test]
#[ignore = "WKBackForwardList smoke tests must run on the process main thread; examples cover live validation"]
fn back_forward_list_reports_current_and_back_items() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    view.load_html(
        "<!doctype html><html><head><title>first</title></head><body>first</body></html>",
        Some("https://first-list.test/"),
    )?;
    view.load_html(
        "<!doctype html><html><head><title>second</title></head><body>second</body></html>",
        Some("https://second-list.test/"),
    )?;

    let list = view.back_forward_list();
    assert_eq!(list.back_item().map(|item| item.url.as_str()), Some("https://first-list.test/"));
    assert_eq!(list.current_item().map(|item| item.url.as_str()), Some("https://second-list.test/"));
    Ok(())
}
