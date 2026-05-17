mod common;

use std::fs;
use std::sync::Arc;

use webkit::prelude::*;

struct StubControllerDelegate;
impl WebExtensionControllerDelegate for StubControllerDelegate {}

struct StubTab;
impl WebExtensionTab for StubTab {}

struct StubWindow;
impl WebExtensionWindow for StubWindow {}

type OpenNewWindowFn = fn(
    &StubControllerDelegate,
    &WebExtensionController,
    &WebExtensionWindowConfiguration,
    &WebExtensionContext,
) -> Result<Option<WebExtensionWindowHandle>, WebKitError>;
type OpenNewTabFn = fn(
    &StubControllerDelegate,
    &WebExtensionController,
    &WebExtensionTabConfiguration,
    &WebExtensionContext,
) -> Result<Option<WebExtensionTabHandle>, WebKitError>;
type PromptForPermissionsFn = fn(
    &StubControllerDelegate,
    &WebExtensionController,
    &[WebExtensionPermission],
    Option<&dyn WebExtensionTab>,
    &WebExtensionContext,
) -> WebExtensionPermissionGrant;
type PromptForUrlsFn = fn(
    &StubControllerDelegate,
    &WebExtensionController,
    &[String],
    Option<&dyn WebExtensionTab>,
    &WebExtensionContext,
) -> WebExtensionUrlGrant;
type PromptForMatchPatternsFn = fn(
    &StubControllerDelegate,
    &WebExtensionController,
    &[WebExtensionMatchPattern],
    Option<&dyn WebExtensionTab>,
    &WebExtensionContext,
) -> WebExtensionMatchPatternGrant;
type SendMessageFn = fn(
    &StubControllerDelegate,
    &WebExtensionController,
    &serde_json::Value,
    Option<&str>,
    &WebExtensionContext,
) -> Result<Option<serde_json::Value>, WebKitError>;

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

    let permission_grant = WebExtensionPermissionGrant {
        allowed: vec![WebExtensionPermission::storage()],
        expiration_date: Some("2026-05-17T00:00:00Z".to_owned()),
    };
    assert_eq!(permission_grant.allowed[0].as_str(), "storage");
    assert_eq!(WebExtensionUrlGrant::default().allowed.len(), 0);
    assert_eq!(WebExtensionMatchPatternGrant::default().allowed.len(), 0);

    let size = WebExtensionSize::new(1280.0, 720.0);
    assert!((size.width - 1280.0).abs() < f64::EPSILON);
    assert!((size.height - 720.0).abs() < f64::EPSILON);

    let snapshot = WebExtensionTabSnapshot {
        png_data: vec![0x89, b'P', b'N', b'G'],
    };
    assert_eq!(snapshot.png_data.len(), 4);

    let tab: WebExtensionTabHandle = Arc::new(StubTab);
    let window: WebExtensionWindowHandle = Arc::new(StubWindow);
    assert_eq!(Arc::strong_count(&tab), 1);
    assert_eq!(Arc::strong_count(&window), 1);
}

#[test]
fn web_extension_delegate_traits_expose_controller_surface() {
    let _: fn(
        &StubControllerDelegate,
        &WebExtensionController,
        &WebExtensionContext,
    ) -> Vec<WebExtensionWindowHandle> = StubControllerDelegate::open_windows_for_context;
    let _: fn(
        &StubControllerDelegate,
        &WebExtensionController,
        &WebExtensionContext,
    ) -> Option<WebExtensionWindowHandle> = StubControllerDelegate::focused_window_for_context;
    let _: OpenNewWindowFn = StubControllerDelegate::open_new_window;
    let _: OpenNewTabFn = StubControllerDelegate::open_new_tab;
    let _: fn(
        &StubControllerDelegate,
        &WebExtensionController,
        &WebExtensionContext,
    ) -> Result<(), WebKitError> = StubControllerDelegate::open_options_page;
    let _: PromptForPermissionsFn = StubControllerDelegate::prompt_for_permissions;
    let _: PromptForUrlsFn = StubControllerDelegate::prompt_for_permission_to_access_urls;
    let _: PromptForMatchPatternsFn = StubControllerDelegate::prompt_for_permission_match_patterns;
    let _: fn(
        &StubControllerDelegate,
        &WebExtensionController,
        &WebExtensionAction,
        &WebExtensionContext,
    ) = StubControllerDelegate::did_update_action;
    let _: fn(
        &StubControllerDelegate,
        &WebExtensionController,
        &WebExtensionAction,
        &WebExtensionContext,
    ) -> Result<(), WebKitError> = StubControllerDelegate::present_popup_for_action;
    let _: SendMessageFn = StubControllerDelegate::send_message;
    let _: fn(
        &StubControllerDelegate,
        &WebExtensionController,
        &WebExtensionMessagePort,
        &WebExtensionContext,
    ) -> Result<(), WebKitError> = StubControllerDelegate::connect_using_message_port;
}

#[test]
fn web_extension_delegate_traits_expose_tab_surface() {
    let _: fn(&StubTab, &WebExtensionContext) -> Option<WebExtensionWindowHandle> = StubTab::window;
    let _: fn(&StubTab, &WebExtensionContext) -> usize = StubTab::index_in_window;
    let _: fn(&StubTab, &WebExtensionContext) -> Option<WebExtensionTabHandle> =
        StubTab::parent_tab;
    let _: fn(
        &StubTab,
        Option<WebExtensionTabHandle>,
        &WebExtensionContext,
    ) -> Result<(), WebKitError> = StubTab::set_parent_tab;
    let _: fn(&StubTab, &WebExtensionContext) -> Option<WebExtensionWebViewHandle> =
        StubTab::webview;
    let _: fn(&StubTab, &WebExtensionContext) -> Option<String> = StubTab::title;
    let _: fn(&StubTab, &WebExtensionContext) -> bool = StubTab::is_pinned;
    let _: fn(&StubTab, bool, &WebExtensionContext) -> Result<(), WebKitError> =
        StubTab::set_pinned;
    let _: fn(&StubTab, &WebExtensionContext) -> bool = StubTab::is_reader_mode_available;
    let _: fn(&StubTab, &WebExtensionContext) -> bool = StubTab::is_reader_mode_active;
    let _: fn(&StubTab, bool, &WebExtensionContext) -> Result<(), WebKitError> =
        StubTab::set_reader_mode_active;
    let _: fn(&StubTab, &WebExtensionContext) -> bool = StubTab::is_playing_audio;
    let _: fn(&StubTab, &WebExtensionContext) -> bool = StubTab::is_muted;
    let _: fn(&StubTab, bool, &WebExtensionContext) -> Result<(), WebKitError> = StubTab::set_muted;
    let _: fn(&StubTab, &WebExtensionContext) -> WebExtensionSize = StubTab::size;
    let _: fn(&StubTab, &WebExtensionContext) -> f64 = StubTab::zoom_factor;
    let _: fn(&StubTab, f64, &WebExtensionContext) -> Result<(), WebKitError> =
        StubTab::set_zoom_factor;
    let _: fn(&StubTab, &WebExtensionContext) -> Option<String> = StubTab::url;
    let _: fn(&StubTab, &WebExtensionContext) -> Option<String> = StubTab::pending_url;
    let _: fn(&StubTab, &WebExtensionContext) -> bool = StubTab::is_loading_complete;
    let _: fn(&StubTab, &WebExtensionContext) -> Result<Option<String>, WebKitError> =
        StubTab::detect_webpage_locale;
    let _: fn(
        &StubTab,
        &SnapshotConfiguration,
        &WebExtensionContext,
    ) -> Result<Option<WebExtensionTabSnapshot>, WebKitError> = StubTab::take_snapshot;
    let _: fn(&StubTab, &str, &WebExtensionContext) -> Result<(), WebKitError> = StubTab::load_url;
    let _: fn(&StubTab, bool, &WebExtensionContext) -> Result<(), WebKitError> = StubTab::reload;
    let _: fn(&StubTab, &WebExtensionContext) -> Result<(), WebKitError> = StubTab::go_back;
    let _: fn(&StubTab, &WebExtensionContext) -> Result<(), WebKitError> = StubTab::go_forward;
    let _: fn(&StubTab, &WebExtensionContext) -> Result<(), WebKitError> = StubTab::activate;
    let _: fn(&StubTab, &WebExtensionContext) -> bool = StubTab::is_selected;
    let _: fn(&StubTab, bool, &WebExtensionContext) -> Result<(), WebKitError> =
        StubTab::set_selected;
    let _: fn(
        &StubTab,
        &WebExtensionTabConfiguration,
        &WebExtensionContext,
    ) -> Result<Option<WebExtensionTabHandle>, WebKitError> = StubTab::duplicate;
    let _: fn(&StubTab, &WebExtensionContext) -> Result<(), WebKitError> = StubTab::close;
    let _: fn(&StubTab, &WebExtensionContext) -> bool =
        StubTab::should_grant_permissions_on_user_gesture;
    let _: fn(&StubTab, &WebExtensionContext) -> bool = StubTab::should_bypass_permissions;
}

#[test]
fn web_extension_delegate_traits_expose_window_surface() {
    let _: fn(&StubWindow, &WebExtensionContext) -> Vec<WebExtensionTabHandle> = StubWindow::tabs;
    let _: fn(&StubWindow, &WebExtensionContext) -> Option<WebExtensionTabHandle> =
        StubWindow::active_tab;
    let _: fn(&StubWindow, &WebExtensionContext) -> WebExtensionWindowType =
        StubWindow::window_type;
    let _: fn(&StubWindow, &WebExtensionContext) -> WebExtensionWindowState =
        StubWindow::window_state;
    let _: fn(
        &StubWindow,
        WebExtensionWindowState,
        &WebExtensionContext,
    ) -> Result<(), WebKitError> = StubWindow::set_window_state;
    let _: fn(&StubWindow, &WebExtensionContext) -> bool = StubWindow::is_private;
    let _: fn(&StubWindow, &WebExtensionContext) -> Rect = StubWindow::screen_frame;
    let _: fn(&StubWindow, &WebExtensionContext) -> Rect = StubWindow::frame;
    let _: fn(&StubWindow, Rect, &WebExtensionContext) -> Result<(), WebKitError> =
        StubWindow::set_frame;
    let _: fn(&StubWindow, &WebExtensionContext) -> Result<(), WebKitError> = StubWindow::focus;
    let _: fn(&StubWindow, &WebExtensionContext) -> Result<(), WebKitError> = StubWindow::close;
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
