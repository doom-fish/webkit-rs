mod common;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    common::load_html(
        &view,
        r#"<article style="width:500px">PDF content from webkit-rs</article>"#,
        "https://pdf.test/",
    )?;

    let pdf = view.create_pdf(
        &PDFConfiguration::new()
            .with_rect(Rect::new(0.0, 0.0, 500.0, 300.0))
            .with_allow_transparent_background(true),
    )?;
    assert!(pdf.starts_with(b"%PDF-"));

    println!("pdf bytes: {}", pdf.len());
    Ok(())
}
