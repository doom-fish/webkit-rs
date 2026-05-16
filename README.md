# webkit-rs

Safe Rust bindings for Apple's `WKWebView` (WebKit) APIs on macOS.

> **Status:** v0.1.0 — headless/offscreen web view, HTML/URL loading, JavaScript evaluation, user scripts, script message handlers, navigation delegate, and PNG snapshot support.

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

    view.load_html("<html><body><script>window.webkit.messageHandlers.bridge.postMessage('hi');</script></body></html>", None)?;
    let title = view.evaluate_javascript("document.title")?;
    println!("title: {title}");

    webkit::pump_run_loop(0.2);
    Ok(())
}
```

## Features

- `WebViewConfiguration` — user content controller, data store, user agent, JavaScript toggle, AirPlay
- `WebView::new_offscreen()` / `WebView::with_config()` — headless `WKWebView`
- `load_url` / `load_html` — blocking navigation
- `evaluate_javascript` / `call_async_javascript` — blocking JS evaluation
- `set_navigation_handler` — `didFinish`, `didFail`, `decidePolicyForAction` events
- `set_message_handler` — `window.webkit.messageHandlers.<name>.postMessage(...)` callbacks
- `take_snapshot_png` — returns PNG `Vec<u8>`
- `pump_run_loop` — drain run-loop events

## Requirements

- macOS 13+
- Swift 5.9+ (Xcode)
- Rust 1.76+
