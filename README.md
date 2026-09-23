# webkit-rs

Safe Rust bindings for Apple's `WKWebView` APIs on macOS.

> **Status:** v0.3.0 — the audited macOS WebKit surface now includes Tier-1 async wrappers for WebKit completion-handler APIs alongside the existing synchronous coverage. See [`COVERAGE.md`](COVERAGE.md) for the audited SDK matrix.

## Quick start

```rust,no_run
use webkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WebViewConfiguration::new();
    config.use_nonpersistent_data_store();
    config.add_message_handler("bridge", &ContentWorld::Page)?;

    let mut view = WebView::with_config(&config)?;
    view.set_message_handler(|message| {
        if message.frame.main_frame {
            println!("message [{}]: {}", message.name, message.body);
        }
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
  - synchronous JavaScript evaluation, `callAsyncJavaScript` with arguments, content world and target frame, and find-in-page
  - navigation / reload / stop / progress / history queries
  - custom user agent, page zoom, media type, inspectable state
  - PNG snapshots, PDF output, and downloads
- `WKWebViewConfiguration`
  - application name, AirPlay, content JavaScript toggle, and custom URL scheme handlers
  - `WKPreferences` round-tripping, including upgrade-to-HTTPS policy
  - `WKWebsiteDataStore` assignment and retrieval
  - user scripts, script-message handler registration per content world (duplicates return an error), content rule lists
- `WKWebsiteDataStore`
  - persistent / non-persistent stores
  - data-record fetch and removal
  - import / export helpers for macOS 26+
  - data-store identifier APIs for macOS 14+
  - cookie-store access
- `WKHTTPCookieStore`
  - get / set / delete cookies
  - observer-style event draining (bounded queue)
  - cookie policy APIs for macOS 14+
- `WKContentRuleListStore`
  - default and custom stores
  - compile / look up / remove rule lists
  - list available identifiers
- Delegates and events
  - `WKNavigationDelegate` with typed action / response / frame summaries and per-request policy handlers
  - `WKUIDelegate`
  - `WKScriptMessageHandler` / `WKScriptMessageHandlerWithReply` with frame, security origin and content world
- `WKURLSchemeHandler` / `WKURLSchemeTask`
  - Rust trait callbacks for custom schemes
  - safe task request / response helpers; out-of-order calls return `WebKitError::InvalidState`
  - request bodies when WebKit supplies them as data
- `WKWebExtension*` (macOS 15.4+; constructors return `WebKitError::Unsupported` on older systems)
  - extensions, contexts, controllers, match patterns, permissions, and data records
  - `WebExtensionControllerDelegate`, `WebExtensionTab`, and `WebExtensionWindow` are Rust models only: the crate doesn't install a controller delegate yet, so WebKit never calls them, and there is no way to obtain a `WebExtensionMessagePort`
  - notification names, permission / data-type constants, handle aliases, and related value types
- Supporting types
  - `WKUserScript`
  - `WKNavigation`
  - `WKDownload` (suggested file names are sanitized)
  - `WKBackForwardList`
  - `WKSnapshotConfiguration`
  - `WKPDFConfiguration`
  - `WKFrameInfo`, `WKSecurityOrigin` and `WKContentWorld` as `FrameInfo`, `SecurityOrigin`, `ContentWorld` and `FrameHandle`

## Threading

WebKit's classes are main-thread only. Every wrapper is `Send + Sync` and performs its WebKit calls on the main thread: directly when called there, otherwise through `DispatchQueue.main.sync`. A call from another thread blocks until the main thread runs its run loop or dispatch queue, so it deadlocks if the main thread is waiting for that thread. Handlers and `UrlSchemeHandler` methods run on the main thread. Dropping a wrapper on another thread releases the WebKit object on the main thread later.

## Handling untrusted web content

- **Script messages.** Handlers receive a `ScriptMessage` with the sending frame (`frame.main_frame`, `frame.request_url`), its `frame.security_origin` (protocol, host, port) and the content `world`. Check them before acting on a message; a reply handler that returns `Err` rejects the page's promise. Register a handler in `ContentWorld::Named(..)` to hide it from page scripts: only user scripts injected into the same world can post to it. Registering a name twice in one world returns an error, for plain and reply handlers alike.
- **Navigation policy.** Without a handler every navigation is allowed, which is WebKit's own default (`set_navigation_delegate_config` can switch the static policy to cancel). `set_navigation_action_handler` decides each request from its URL, navigation type, source frame and origin, and target frame; `set_navigation_response_handler` decides each response. A handler that panics cancels the navigation.

  ```rust,no_run
  # use webkit::prelude::*;
  # fn allowlist(view: &mut WebView) {
  view.set_navigation_action_handler(|action| {
      if action.request_url.starts_with("https://app.example/") {
          NavigationActionPolicy::Allow
      } else {
          NavigationActionPolicy::Cancel
      }
  });
  # }
  ```
- **JavaScript arguments.** Pass untrusted values as `call_async_javascript` arguments instead of formatting them into JavaScript source. The arguments must serialize to a JSON object; WebKit exposes each key as a local variable of the function body.

  ```rust,no_run
  # use webkit::prelude::*;
  # fn greet(view: &WebView, untrusted_name: &str) -> Result<String, WebKitError> {
  view.call_async_javascript(
      "document.getElementById('name').textContent = name; return name.length",
      &serde_json::json!({ "name": untrusted_name }),
      None,
      &ContentWorld::Page,
  )
  # }
  ```
- **Event queues.** Navigation, back/forward list, UI dialog, script-message, cookie-observer and download events are also kept for the `drain_*` methods in bounded queues of 1024 events and 4 MiB each. When a queue is full the oldest events are dropped, and `DrainedEvents::dropped` reports how many. Live handlers still see every event.
- **URL scheme tasks.** Calls that break WebKit's ordering rules (data before a response, a response after data, finishing twice) and every call after WebKit stopped a task return `WebKitError::InvalidState` instead of aborting the process.
- **Downloads.** Server-suggested file names lose path separators, control characters and leading dots before they are joined to the destination directory.

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
- `tests/` contains 21 area-specific test files plus `tests/main_thread.rs`, a custom-harness test that runs live web views on the process main thread (non-persistent data store, `loadHTMLString`, custom schemes and a loopback server only; no remote URLs). It skips itself when WebKit can't load a page in the current session.
- The other live `WKWebView` smoke tests need the process main thread, which libtest doesn't provide, so they stay `#[ignore]` and are covered by the examples.

## Requirements

- macOS 13+. Some APIs need newer systems and return `WebKitError::Unsupported` below them: macOS 14 (data-store identifiers, proxy configurations, cookie policy), macOS 15.4 (web extensions), macOS 26 (web view and data-store data export / restore, `setCookies`, `showsSystemScreenTimeBlockingView`).
- Swift 5.9+ (Xcode)
- Rust 1.82+
