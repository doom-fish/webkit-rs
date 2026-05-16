# webkit-rs

Safe Rust bindings for Apple's `WKWebView` APIs on macOS.

> **Status:** v0.2.0 — expanded Swift-bridge coverage for `WKWebView`, `WKWebViewConfiguration`, `WKWebsiteDataStore`, `WKHTTPCookieStore`, `WKContentRuleListStore`, `WKPreferences`, delegates, downloads, snapshots, and PDF generation. See [`COVERAGE.md`](COVERAGE.md) for the audited SDK matrix.

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
  - synchronous JavaScript evaluation and async JavaScript calls
  - navigation / reload / stop / progress / history queries
  - custom user agent, page zoom, media type, inspectable state
  - PNG snapshots, PDF output, and downloads
- `WKWebViewConfiguration`
  - application name, AirPlay, content JavaScript toggle
  - `WKPreferences` round-tripping
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
  - `WKNavigationDelegate`
  - `WKUIDelegate`
  - `WKScriptMessageHandler`
- Supporting types
  - `WKUserScript`
  - `WKNavigation`
  - `WKDownload`
  - `WKBackForwardList`
  - `WKSnapshotConfiguration`
  - `WKPDFConfiguration`

## Examples and tests

- `examples/` contains 15 numbered, headless-safe examples covering every requested logical area.
- `tests/` contains 15 area-specific test files.
- Live `WKWebView` smoke tests that require the process main thread are represented as runnable examples and as `#[ignore]` integration tests with notes.

## Requirements

- macOS 13+
- Swift 5.9+ (Xcode)
- Rust 1.76+
