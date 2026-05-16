mod common;

use webkit::prelude::*;

#[test]
#[ignore = "WKWebView smoke tests must run on the process main thread; examples cover live validation"]
fn webview_loads_and_evaluates_html() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    common::load_html(&view, "<main id='root'>hello</main>", "https://webview.test/")?;
    assert_eq!(
        view.evaluate_javascript("document.getElementById('root').textContent")?,
        "hello"
    );
    Ok(())
}
