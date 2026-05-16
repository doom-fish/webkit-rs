# Changelog

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
