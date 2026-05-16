mod common;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = common::base_config();
    let view = WebView::with_config(&config)?;
    common::load_html(
        &view,
        r#"<div style="width:400px;height:200px;background:#00ff88">snapshot</div>"#,
        "https://snapshot.test/",
    )?;

    let png = view.take_snapshot_png_with_configuration(
        &SnapshotConfiguration::new()
            .with_rect(Rect::new(0.0, 0.0, 400.0, 200.0))
            .with_snapshot_width(200.0),
    )?;
    assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));

    println!("snapshot bytes: {}", png.len());
    Ok(())
}
