mod common;

use webkit::prelude::*;

struct TestSchemeHandler;

impl UrlSchemeHandler for TestSchemeHandler {
    fn start(&self, task: UrlSchemeTask) {
        let response = UrlSchemeResponse::new(task.request().url.clone(), "text/html")
            .with_text_encoding_name("utf-8")
            .with_header("Cache-Control", "no-store");
        let body =
            br"<!doctype html><html><body id='custom'>hello from custom scheme</body></html>";
        task.respond(&response, body)
            .expect("custom URL scheme handler should respond successfully");
    }

    fn stop(&self, _task: UrlSchemeTask) {}
}

#[test]
fn url_scheme_response_builder_preserves_fields() {
    let response = UrlSchemeResponse::new("app://hello", "text/plain")
        .with_text_encoding_name("utf-8")
        .with_status_code(201)
        .with_header("X-Test", "yes");

    assert_eq!(response.url, "app://hello");
    assert_eq!(response.mime_type, "text/plain");
    assert_eq!(response.text_encoding_name.as_deref(), Some("utf-8"));
    assert_eq!(response.status_code, 201);
    assert_eq!(
        response.headers.get("X-Test").map(String::as_str),
        Some("yes")
    );
}

#[test]
#[ignore = "WKURLSchemeHandler smoke tests must run on the process main thread; examples cover live validation"]
fn custom_url_scheme_handler_serves_html() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    config.set_url_scheme_handler("app", TestSchemeHandler)?;

    let view = WebView::with_config(&config)?;
    view.load_url("app://hello")?;

    assert_eq!(
        view.evaluate_javascript("document.getElementById('custom').textContent")?,
        "hello from custom scheme"
    );
    Ok(())
}
