mod common;

use std::fs;

use webkit::prelude::*;

#[test]
fn web_extension_types_are_structured() {
    let permission = WebExtensionPermission::new("storage");
    let data_type = WebExtensionDataType::new("local");

    assert_eq!(permission.as_str(), "storage");
    assert_eq!(data_type.as_str(), "local");

    let mut options = WebExtensionMatchPatternOptions::IGNORE_SCHEMES;
    options |= WebExtensionMatchPatternOptions::IGNORE_PATHS;
    assert!(options.contains(WebExtensionMatchPatternOptions::IGNORE_SCHEMES));
    assert!(options.contains(WebExtensionMatchPatternOptions::IGNORE_PATHS));

    let mut changed = WebExtensionTabChangedProperties::LOADING;
    changed |= WebExtensionTabChangedProperties::TITLE;
    assert!(changed.contains(WebExtensionTabChangedProperties::TITLE));
}

#[test]
#[ignore = "WKWebExtension smoke tests require macOS 15.4+ runtime support; example covers live validation"]
fn web_extension_loads_minimal_manifest() -> Result<(), Box<dyn std::error::Error>> {
    let root = common::artifact_dir("web-extension-tests")?.join("sample-extension");
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
    Ok(())
}
