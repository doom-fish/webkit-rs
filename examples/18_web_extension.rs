mod common;

use std::fs;

use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = common::artifact_dir("web-extension-example")?.join("sample-extension");
    fs::create_dir_all(&root)?;
    fs::write(
        root.join("manifest.json"),
        r#"{
            "manifest_version": 3,
            "name": "webkit-rs Example Extension",
            "version": "1.0.0",
            "description": "Minimal manifest for bridge validation"
        }"#,
    )?;

    let extension = WebExtension::from_resource_base_url(&root)?;
    let summary = extension.summary();

    assert_eq!(
        summary.display_name.as_deref(),
        Some("webkit-rs Example Extension")
    );
    assert!(extension.supports_manifest_version(3.0));

    println!("loaded web extension: {}", summary.display_name.unwrap_or_default());
    Ok(())
}
