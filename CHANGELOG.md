# Changelog

All notable changes to `webkit` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-09-24

### Security

- Out-of-order `WKURLSchemeTask` calls no longer abort the process. Task state
  (response sent, data sent, finished or failed, stopped by WebKit) is tracked
  on the main thread, and calls that WebKit would answer with
  `NSInternalInconsistencyException` return `WebKitError::InvalidState`
  instead. That includes every call made after WebKit stopped the task, which
  happens whenever a page navigates away while a handler is still responding.
  Finished and failed tasks are removed from the handler's task table, which
  used to keep every completed task alive.
- Page JavaScript can no longer grow host memory without limit. Script
  messages, JavaScript dialogs, navigation and back/forward list events, cookie
  changes and download events are kept for the `drain_*` methods in bounded
  queues (1024 events and 4 MiB per queue). The oldest events are dropped first
  and every drain reports how many were dropped. Live handlers still receive
  every event.
- Script-message handlers receive the sending frame (main-frame flag, request
  URL), its security origin (protocol, host, port) and the content world, so
  they can reject messages from cross-origin iframes. Handlers can be
  registered in an isolated content world that page scripts cannot reach, and
  a reply handler's error rejects the page's promise.
- Navigation policy can be decided per request. `set_navigation_action_handler`
  sees the request URL, navigation type, source frame and origin, and target
  frame; `set_navigation_response_handler` sees each response. A handler that
  panics cancels the navigation. Without a handler the static
  `NavigationDelegateConfig` policy applies, which still allows every
  navigation by default, matching WebKit.
- `call_async_javascript` passes a JSON object as WebKit `arguments` and takes
  a content world and an optional target frame, so untrusted data no longer
  has to be formatted into JavaScript source.
- Download file names suggested by the server are sanitized: path separators,
  `:` and control characters become `_`, leading dots are removed, names are
  capped at 240 bytes, and the destination must stay inside the requested
  directory.
- `Cookie`'s `Debug` output redacts the cookie value.

### Fixed

- WebKit's classes are main-actor only, but most configuration, website data
  store, cookie store, download, content rule list and web extension exports
  ran on the caller's thread. Every export now runs on the main thread, the
  web view box is created there, and every wrapper releases its Swift object on
  the main thread (later, when dropped on another thread). Event drains no
  longer race with the delegates that fill them.
- `set_*_handler` and `WebView`'s `Drop` swap the Swift callback and its
  context in one step on the main thread. The handler lives in a
  `doom_fish_utils::callback_context::CallbackContext` that the Swift callback
  slot retains, so a callback that is already running never sees a freed
  handler, and a replaced or dropped handler is never called again.
- Registering a script-message handler name that already exists in a content
  world of a shared `WKUserContentController` returns an error instead of
  raising `NSInvalidArgumentException`. Plain and reply handlers share one
  namespace, as in WebKit. Handlers are registered once per user content
  controller and routed to the web view that sent the message, so several
  `WebView`s created from one configuration work and each gets its own
  messages.
- `set_url_scheme_handler` asks the configuration whether the scheme already
  has a handler, so a copied configuration can no longer register a second one
  and abort.
- Exports that need a newer macOS than 13 (data-store identifiers and proxy
  configurations on 14, web extensions on 15.4, data export / restore and
  `showsSystemScreenTimeBlockingView` on 26) were declared `@available`, which
  compiled their runtime checks away, so older systems would have called
  missing symbols. They now return `WebKitError::Unsupported` there.
- Reply-handler results are decoded as JSON fragments, so a reply such as
  `"pong"` or `42` reaches JavaScript as a string or number rather than as
  quoted text.
- `ProxyConfiguration` setters raced when called from several threads (the
  type is `Sync`); the Swift box now serializes access with a lock.
- Cookie expiry dates outside the `i64` range no longer trap in Swift, and
  copying a configuration for a web extension controller no longer uses a
  force cast.

### Changed

- **Breaking:** `WebViewConfiguration::add_message_handler` and
  `add_message_handler_with_reply` take a `ContentWorld` and return
  `Result<(), WebKitError>`.
- **Breaking:** message handlers are `Fn(&ScriptMessage)` and
  `Fn(&ScriptMessage) -> Result<Option<Value>, WebKitError>`. `ScriptMessage`
  has `frame: FrameInfo`, `world: ContentWorld` and
  `frame_handle: Option<FrameHandle>` in place of `frame_url`, `is_main_frame`
  and `world: Option<String>`.
- **Breaking:** every `WebView` handler must be `Send + Sync`.
- **Breaking:** `FrameInfo` has `security_origin: SecurityOrigin` in place of
  the three optional `security_origin_*` fields.
- **Breaking:** `WebView::drain_navigation_events`,
  `drain_back_forward_list_navigation_events`, `drain_ui_events`,
  `drain_ui_event_details`, `drain_script_messages`,
  `HttpCookieStore::drain_events` and `Download::drain_events` return
  `DrainedEvents<T>` (the events plus a `dropped` count).
- **Breaking:** `WebView::call_async_javascript(function_body, arguments,
  frame, content_world)`. `AsyncWebView::call_async_javascript` takes the same
  arguments and returns `Result<CallAsyncJavaScriptFuture, WebKitError>`.
- **Breaking:** `WebExtensionController::new` and `with_configuration`,
  `WebExtensionControllerConfiguration::default_configuration` and
  `non_persistent_configuration`, `WebExtensionMatchPattern::all_urls` and
  `all_hosts_and_schemes`, and `WebExtensionContext::for_extension` return
  `Result` instead of panicking; `WebExtensionController` no longer implements
  `Default`.
- **Breaking:** `WebKitError` has an `InvalidState` variant.
- A call on any wrapper from a thread other than the main thread blocks until
  the main thread runs its run loop or dispatch queue.
- `rust-version` is 1.82 (was 1.76). Requires `apple-cf` 0.11 and
  `doom-fish-utils` 0.4.1.

### Added

- `ContentWorld`, `FrameHandle` and `DrainedEvents`.
- `WebView::set_navigation_action_handler` and
  `set_navigation_response_handler`.
- `WebViewConfiguration::remove_message_handler`.
- `UrlSchemeRequest::body`, filled when WebKit supplies the request body as
  data.
- `tests/main_thread.rs`, a custom-harness test that runs live web views on
  the process main thread through the fixed paths using `loadHTMLString`,
  custom schemes and a loopback server.

### Removed

- **Breaking:** the public `wk_rust_release_url_scheme_handler` export, a safe
  `extern "C"` function that called `Arc::from_raw` on any pointer. The Swift
  handler now receives the release function as a parameter.

## [0.3.11] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.3.10] - 2026-05-20

### Added

- Added `WebViewConfiguration::{set_shows_system_screen_time_blocking_view, shows_system_screen_time_blocking_view}` for the macOS 26.0+ `showsSystemScreenTimeBlockingView` property.
- Added self-contained `ProxyConfiguration` / `ProxyConfigurationSummary` support plus `WebsiteDataStore::{proxy_configurations, set_proxy_configurations, clear_proxy_configurations}` for the macOS 14.0+ `proxyConfigurations` surface.
- Added `BackForwardListNavigationPolicy`, `BackForwardListNavigationEvent`, and corresponding `WebView` bridge methods for `webView:shouldGoToBackForwardListItem:willUseInstantBack:completionHandler:`.
- Added compile/serde coverage and ignored smoke tests for the MacOSX26.5 completeness sweep.

### Notes

- Phase 32 completeness + async sweep.
- Bumped the crate version from `0.3.9` to `0.3.10` and refreshed the coverage docs for the MacOSX26.5 SDK.

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
