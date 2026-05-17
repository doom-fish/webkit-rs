mod common;

use std::path::Path;

use webkit::prelude::*;

#[test]
fn attributed_string_api_is_available() {
    let request = HtmlLoadRequest::new("https://example.test/");
    let options = AttributedStringLoadOptions::new()
        .with_base_url("https://example.test/")
        .with_read_access_url(Path::new("."))
        .with_timeout_seconds(2.0)
        .with_text_size_multiplier(1.25)
        .with_text_encoding_name("utf-8")
        .with_character_encoding(4);
    let _handler: AttributedStringCompletionHandler = Box::new(|result| {
        let _ = result.is_ok();
    });

    assert_eq!(request.url, "https://example.test/");
    assert_eq!(options.text_encoding_name.as_deref(), Some("utf-8"));
    assert_eq!(
        READ_ACCESS_URL_DOCUMENT_OPTION,
        "NSReadAccessURLDocumentOption"
    );

    let _ = AttributedString::load_from_html_request;
    let _ = AttributedString::load_from_html_string;
    let _ = AttributedString::load_from_html_data;
    let _ = AttributedString::string;
    let _ = AttributedString::document_attributes;

    let load_file = |path: &Path, opts: &AttributedStringLoadOptions| {
        AttributedString::load_from_html_file(path, opts)
    };
    let _ = load_file;
}

#[test]
#[ignore = "NSAttributedString HTML loading smoke tests must run on the process main thread; examples cover live validation"]
fn attributed_string_loads_html_string() -> Result<(), Box<dyn std::error::Error>> {
    let _ = common::base_config();
    let attributed = AttributedString::load_from_html_string(
        "<main><strong>Hello</strong> attributed world</main>",
        &AttributedStringLoadOptions::new(),
    )?;
    assert!(attributed.string().contains("Hello attributed world"));
    Ok(())
}
