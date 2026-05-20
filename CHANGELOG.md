# Changelog

## [0.3.9] - 2026-05-20

- Added in-`src/` unit tests across config, error, find, navigation_delegate, and webview (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.3.8] - 2026-05-20

- Clippy hygiene sweep: cleared all `-D warnings` lints across the crate. No public API change.

## [0.3.7] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.3.6] - 2026-05-19

### Added

- Added standalone `ContextMenuElementInfo`, `PreviewElementInfo`, and `PreviewActionItem` wrappers with Swift bridge accessors for `linkURL`, `title`, and `identifier`.
- Added focused `context_menu_tests` coverage for the new context-menu / preview metadata surface.

### Changed

- Bumped the crate version from `0.3.5` to `0.3.6`.

## [0.3.5] - 2026-05-18

### Changed

- Added `///` docs across the public `src/` API surface (excluding `src/ffi/` internals), referencing the corresponding WebKit framework counterparts and bringing `cargo rustdoc --lib -- -W missing-docs` to zero warnings.
- Bumped the crate version from `0.3.4` to `0.3.5`.

## [0.3.4] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.3.3] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

## [0.3.2] - 2026-06-05

### Fixed

- `webview.rs`: Added `std::panic::catch_unwind` to `nav_trampoline`,
  `msg_trampoline`, and `msg_reply_trampoline`. User-supplied navigation,
  message, and reply-message closures could previously panic across the C ABI
  boundary (undefined behaviour); panics are now caught and swallowed with a
  `FRAMEWORK_ERROR` status returned to the bridge for the reply trampoline.
- `url_scheme.rs`: Added `std::panic::catch_unwind` to
  `url_scheme_start_trampoline` and `url_scheme_stop_trampoline`. User-supplied
  `UrlSchemeHandler` implementations could previously panic across the C ABI.
- Added `// SAFETY:` comments to all explicit `unsafe { }` blocks in
  `async_api.rs` (`string_cb`, `bytes_cb`, `unit_cb`, `rule_list_cb`,
  `bytes_cb_discard`) and in `url_scheme.rs`
  (`url_scheme_{start,stop}_trampoline`, `wk_rust_release_url_scheme_handler`).
  Also added function-level SAFETY headers and inner-block annotations to all
  three trampolines in `webview.rs`.
- `Cargo.toml`: Tightened the `doom-fish-utils` version range from `"0.1"` to
  `">=0.1, <0.3"` to allow the next minor release while blocking breaking
  changes.

## [0.3.1] - 2026-06-05

### Changed

- Added `@available(macOS 26.0, *)` attribute to the `wk_webview_fetch_data_of_types`,
  `wk_webview_restore_data`, `wk_website_data_store_fetch_data`, and
  `wk_website_data_store_restore_data` Swift bridge thunks, which use
  `WKWebViewDataType` / `WKWebView.fetchData` / `WKWebView.restoreData` /
  `WKWebsiteDataStore.fetchData` / `WKWebsiteDataStore.restoreData` — APIs
  first available in macOS 26.0.
- Added `@available(macOS 15.4, *)` attribute to all 66 `@_cdecl` thunks in
  `WKWebExtension.swift`, covering the entire `WKWebExtension` /
  `WKWebExtensionContext` / `WKWebExtensionController` family (macOS 15.4+).
  The bridge can now compile against older macOS SDKs (macOS 15 / Xcode 16)
  without SDK-version pinning.

## [0.3.0] - 2026-05-18

### Added

- Added `async_api` module (Tier 1) gated behind the `async` feature flag.
- `AsyncWebView`: async wrappers for `evaluateJavaScript`, `callAsyncJavaScript`, `takeSnapshot`, `createPDF`, `createWebArchiveData`, and `find`.
- `AsyncWebsiteDataStore`: async wrappers for `fetchDataRecords` and `removeData`.
- `AsyncHttpCookieStore`: async wrapper for `getAllCookies`.
- `AsyncContentRuleListStore`: async wrapper for `compileContentRuleList`.
- `AsyncDownload`: async wrapper for `cancel` (with and without resume data).
- Added `doom-fish-utils` as an optional dependency (activated by `async` feature).
- Added `pollster = "0.3"` as a dev-dependency for running async examples/tests synchronously.
- Added examples `20_async_webview`, `21_async_cookie_store`, `22_async_data_store`.
- Added `tests/async_api_tests.rs` with happy-path and error-path coverage for the new async wrappers.
- `WKWebExtensionContext` async methods deferred to Tier 2 (Stream pattern).

### Changed

- Bumped version from `0.2.3` to `0.3.0`.

## [0.2.3] - 2026-05-17

### Added

- Added `AttributedString` HTML-loading helpers, `AttributedStringLoadOptions`, and the `READ_ACCESS_URL_DOCUMENT_OPTION` / `AttributedStringCompletionHandler` surface for the `NSAttributedString` WebKit additions.
- Added typed `WebKitErrorCode`, `DownloadRedirectPolicy`, `OpenPanelParameters`, `SecurityOrigin`, `WindowFeatures`, `MediaCaptureType`, `PermissionDecision`, `MediaPlaybackState`, `MediaCaptureState`, `FullscreenState`, `WebViewDataType`, `AudiovisualMediaTypes`, and `UserInterfaceDirectionPolicy` wrappers.
- Added reply-capable script message handling via `WebViewConfiguration::add_message_handler_with_reply` and `WebView::set_message_handler_with_reply`.
- Added WebView helpers for AppKit IBAction navigation, text-finder actions, media playback / capture state, and macOS 26 web-view data import / export.
- Added focused tests for the newly wrapped symbol families and pushed `COVERAGE_AUDIT.md` to 100%.

### Changed

- Bumped the crate version from `0.2.2` to `0.2.3`.
- Refreshed the coverage docs to reflect the closed gaps and typed delegate event details.

## [0.2.2] - 2026-05-17

### Added

- Replaced the `WKWebExtensionControllerDelegate`, `WKWebExtensionTab`, and `WKWebExtensionWindow` marker traits with exhaustive Rust trait surfaces that mirror the macOS SDK protocols while keeping default method bodies for semver compatibility.
- Added `WebExtensionGrant`, `WebExtension{Permission,Url,MatchPattern}Grant`, `WebExtensionSize`, `WebExtensionTabSnapshot`, and handle aliases for web-extension tab/window/webview references.
- Added focused tests covering the new delegate trait signatures and helper types.

### Changed

- Bumped the crate version from `0.2.1` to `0.2.2`.
- Refreshed the README to call out the completed `WKWebExtension*` delegate trait coverage.

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
