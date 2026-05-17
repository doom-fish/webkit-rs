//! Async `HTTPCookieStore` example.
#[cfg(feature = "async")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use webkit::async_api::AsyncHttpCookieStore;
    use webkit::{WebView, WebViewConfiguration, WebsiteDataStore};

    webkit::init_app();
    let cfg = WebViewConfiguration::new();
    cfg.use_nonpersistent_data_store();
    let _view = WebView::with_config(&cfg)?;

    let store = WebsiteDataStore::non_persistent();
    let cookie_store = store.http_cookie_store()?;

    pollster::block_on(async {
        let cookies_json = AsyncHttpCookieStore::get_all_cookies(&cookie_store).await?;
        println!("cookies: {cookies_json}");
        Ok::<(), Box<dyn std::error::Error>>(())
    })
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("compile with --features async");
}
