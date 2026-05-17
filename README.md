# webkit-rs

Safe Rust bindings for Apple's `WKWebView` APIs on macOS.

> **Status:** v0.3.0 — the audited macOS WebKit surface now includes Tier-1 async wrappers for WebKit completion-handler APIs alongside the existing synchronous coverage. See [`COVERAGE.md`](COVERAGE.md) for the audited SDK matrix.

## Quick start

```rust,no_run
use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WebViewConfiguration::new();
    config.use_nonpersistent_data_store();
    config.add_message_handler("bridge");

    let mut view = WebView::with_config(&config)?;
    view.set_message_handler(|name, body| {
        println!("message [{name}]: {body}");
    });

    view.load_html(
        "<html><body><script>window.webkit.messageHandlers.bridge.postMessage('hi');</script></body></html>",
        None,
    )?;
    let title = view.evaluate_javascript("document.title")?;
    println!("title: {title}");

    webkit::pump_run_loop(0.2);
    Ok(())
}
```

## Covered areas

- `WKWebView`
  - offscreen construction
  - HTML / URL / file / raw-data loading
  - synchronous JavaScript evaluation, async JavaScript calls, and find-in-page
  - navigation / reload / stop / progress / history queries
  - custom user agent, page zoom, media type, inspectable state
  - PNG snapshots, PDF output, and downloads
- `WKWebViewConfiguration`
  - application name, AirPlay, content JavaScript toggle, and custom URL scheme handlers
  - `WKPreferences` round-tripping, including upgrade-to-HTTPS policy
  - `WKWebsiteDataStore` assignment and retrieval
  - user scripts, script-message handler registration, content rule lists
- `WKWebsiteDataStore`
  - persistent / non-persistent stores
  - data-record fetch and removal
  - import / export helpers for macOS 26+
  - data-store identifier APIs for macOS 14+
  - cookie-store access
- `WKHTTPCookieStore`
  - get / set / delete cookies
  - observer-style event draining
  - cookie policy APIs for macOS 14+
- `WKContentRuleListStore`
  - default and custom stores
  - compile / look up / remove rule lists
  - list available identifiers
- Delegates and events
  - `WKNavigationDelegate` with typed action / response / frame summaries
  - `WKUIDelegate`
  - `WKScriptMessageHandler`
- `WKURLSchemeHandler` / `WKURLSchemeTask`
  - Rust trait callbacks for custom schemes
  - safe task request / response helpers
- `WKWebExtension*`
  - extensions, contexts, controllers, match patterns, permissions, and data records
  - `WKWebExtensionControllerDelegate`, `WKWebExtensionTab`, and `WKWebExtensionWindow` mirrored as Rust traits with forward-compatible defaults
  - notification names, permission / data-type constants, message-port helpers, handle aliases, and related value types
- Supporting types
  - `WKUserScript`
  - `WKNavigation`
  - `WKDownload`
  - `WKBackForwardList`
  - `WKSnapshotConfiguration`
  - `WKPDFConfiguration`

## Async API

Enable the `async` feature for `Future`-based wrappers over WebKit's completion-handler APIs:

```toml
[dependencies]
webkit = { version = "0.3", features = ["async"] }
```

| Type | Description |
|------|-------------|
| `AsyncWebView` | `evaluateJavaScript`, `callAsyncJavaScript`, `takeSnapshot`, `createPDF`, `createWebArchiveData`, `find` |
| `AsyncWebsiteDataStore` | `fetchDataRecords`, `removeData` |
| `AsyncHttpCookieStore` | `getAllCookies` |
| `AsyncContentRuleListStore` | `compileContentRuleList` |
| `AsyncDownload` | `cancel` |

The async API is executor-agnostic and pumps the main run loop while awaiting WebKit completions, so it works in headless CLI examples with `pollster::block_on` as well as other runtimes.

## Examples and tests

- `examples/` contains 21 numbered, headless-safe examples covering every requested logical area.
- `tests/` contains 21 area-specific test files.
- Live `WKWebView` smoke tests that require the process main thread are represented as runnable examples and as `#[ignore]` integration tests with notes.

## Requirements

- macOS 13+
- Swift 5.9+ (Xcode)
- Rust 1.76+
