# Changelog

## [0.2.1] - 2026-05-16

### Added

- Added `WKFindConfiguration` / `WKFindResult` wrappers and `WebView::find_string{,_with_configuration}`.
- Added typed `WKNavigationAction`, `WKNavigationResponse`, `WKFrameInfo`, and `WKNavigationType` models to navigation delegate events.
- Added `WKURLSchemeHandler` / `WKURLSchemeTask` support via `WebViewConfiguration::set_url_scheme_handler` and safe Rust task helpers.
- Added the requested `WKWebExtension*` surface wrappers for extensions, contexts, controllers, match patterns, permissions, data records, message ports, notifications, and related value types.
- Added `WKWebpagePreferencesUpgradeToHTTPSPolicy` support through `Preferences::upgrade_to_https_policy`.
- Added numbered examples `16_find`, `17_url_scheme`, and `18_web_extension`, plus focused tests for the new logical areas.

### Changed

- Bumped the crate version from `0.2.0` to `0.2.1`.
- Refreshed the README and coverage audit to reflect the widened WebKit surface.

## [0.2.0] - 2026-05-16

### Added

- Expanded the Swift bridge into logical-area files mirroring the `screencapturekit-rs` pattern.
- Added `WKWebsiteDataStore` wrappers with persistent / non-persistent stores, record fetch / removal, cookie-store access, and macOS 14+/26+ identifier/import/export helpers.
- Added `WKHTTPCookieStore` wrappers for fetching, setting, deleting, observing, and policy management.
- Added `WKContentRuleListStore` and `WKContentRuleList` wrappers plus configuration integration for compiled rule lists.
- Added `WKPreferences` round-tripping through `WebViewConfiguration`.
- Added `WKNavigation`, `WKDownload`, `WKBackForwardList`, `WKSnapshotConfiguration`, and `WKPDFConfiguration` support to the safe Rust API.
- Added typed navigation, UI-delegate, and script-message event models.
- Added 14 new numbered examples (15 total) covering every requested logical area.
- Added 15 area-specific integration test files.
- Added `COVERAGE.md` with the audited SDK matrix for the requested WebKit headers.

### Changed

- Bumped the crate version from `0.1.0` to `0.2.0`.
- Refreshed the README to document the widened WebKit surface and validation strategy.

## [0.1.0] - 2026-05-16

### Added

- `WebViewConfiguration` wrapping `WKWebViewConfiguration` with user content controller, non-persistent data store, application user agent, AirPlay, and content JavaScript enable/disable knobs.
- `WebView::new_offscreen()` and `WebView::with_config()` headless constructors.
- `WebView::load_url` and `WebView::load_html` blocking navigation wrappers.
- `WebView::evaluate_javascript` and `WebView::call_async_javascript` blocking JS evaluation.
- `WebView::set_navigation_handler` for `didFinish`, `didFail`, `didFailProvisional`, and `decidePolicyForAction` navigation events.
- `WebView::set_message_handler` for `WKScriptMessageHandler` callbacks.
- User script injection via `WebViewConfiguration::add_user_script` with `InjectionTime` enum.
- `WebView::take_snapshot_png` returning PNG bytes via `WKSnapshotConfiguration`.
- `pump_run_loop` helper for draining macOS main run-loop events.
- Swift bridge (`WebKitBridge`) backed by `WebKit.framework`, `AppKit.framework`, and `Foundation.framework`.
- End-to-end smoke example `examples/01_load_eval.rs`.
