//! Async `WebView` operations example.
#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use webkit::async_api::AsyncWebView;
    use webkit::{WebView, WebViewConfiguration};

    webkit::init_app();
    let cfg = WebViewConfiguration::new();
    cfg.use_nonpersistent_data_store();
    let view = WebView::with_config(&cfg)?;
    view.load_html("<html><body><h1>hello</h1></body></html>", None)?;
    webkit::pump_run_loop(0.5);

    pollster::block_on(async {
        let result = AsyncWebView::evaluate_javascript(&view, "1 + 1").await?;
        println!("evaluate_javascript result: {result}");

        let archive = AsyncWebView::create_web_archive(&view).await?;
        println!("web_archive bytes: {}", archive.len());

        Ok::<(), Box<dyn std::error::Error>>(())
    })
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("compile with --features async");
}
