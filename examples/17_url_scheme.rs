mod common;

use webkit::prelude::*;

struct ExampleSchemeHandler;

impl UrlSchemeHandler for ExampleSchemeHandler {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    config.set_url_scheme_handler("app", ExampleSchemeHandler)?;

    let view = WebView::with_config(&config)?;
    view.load_url("app://hello")?;

    assert_eq!(
        view.evaluate_javascript("document.getElementById('custom').textContent")?,
        "hello from custom scheme"
    );

    println!("custom URL scheme handled successfully");
    Ok(())
}
