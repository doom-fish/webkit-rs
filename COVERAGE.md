# WebKit coverage audit

Audited against the macOS 26.5 SDK headers for:

- `WKWebView.h`
- `WKWebViewConfiguration.h`
- `WKWebsiteDataStore.h`
- `WKWebsiteDataRecord.h`
- `WKUserScript.h`
- `WKNavigation.h`
- `WKNavigationDelegate.h`
- `WKUIDelegate.h`
- `WKScriptMessageHandler.h`
- `WKPreferences.h`
- `WKContentRuleListStore.h`
- `WKContentRuleList.h`
- `WKHTTPCookieStore.h`
- `WKDownload.h`
- `WKBackForwardList.h`
- `WKSnapshotConfiguration.h`
- `WKPDFConfiguration.h`
- `WKURLSchemeHandler.h`, `WKURLSchemeTask.h`
- `WKFrameInfo.h`, `WKSecurityOrigin.h`, `WKContentWorld.h`
- `WKWebExtension*.h`

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped (deprecated / iOS-only / unsuitable for this headless macOS crate)

## WKWebView

| API | Status | Notes |
| --- | --- | --- |
| `init(frame:configuration:)` | ✅ | `WebView::new_offscreen` / `WebView::with_config` create an offscreen `WKWebView`. |
| `load(_:)`, `loadHTMLString(_:baseURL:)`, `loadFileURL(_:allowingReadAccessTo:)`, `load(_:mimeType:characterEncodingName:baseURL:)` | ✅ | `load_url`, `load_html`, `load_file_url`, `load_data`. |
| `goBack()`, `goForward()`, `reload()`, `reloadFromOrigin()`, `stopLoading()`, `go(to:)` | ✅ | `go_back`, `go_forward`, `reload`, `reload_from_origin`, `stop_loading`, `go_to_back_forward_index`. |
| `title`, `url`, `isLoading`, `estimatedProgress`, `hasOnlySecureContent`, `canGoBack`, `canGoForward` | ✅ | Safe getters exposed on `WebView`. |
| `backForwardList` | 🟡 | Safe snapshot wrapper exists, but offscreen navigations can still report an empty item snapshot even when `canGoBack` is true. |
| `customUserAgent`, `allowsLinkPreview`, `pageZoom`, `mediaType`, `inspectable` | ✅ | Getters and setters exposed. |
| `evaluateJavaScript(_:completionHandler:)`, `callAsyncJavaScript(_:arguments:in:in:completionHandler:)` | ✅ | Blocking and async wrappers. `call_async_javascript` passes a JSON object as `arguments` and takes a `ContentWorld` and an optional `FrameHandle`. `evaluateJavaScript(_:in:in:)` (frame / world) isn't wrapped. |
| `takeSnapshot(with:completionHandler:)`, `createPDF(configuration:completionHandler:)`, `startDownload(using:completionHandler:)` | ✅ | Snapshot, PDF, and download wrappers exposed. |
| AppKit / platform extras (`scrollView`, magnification, theme color, media controls, find interactions, editable state, etc.) | 🟡 | IBAction navigation helpers, text-finder actions, media playback / capture / fullscreen state, and web-view data import/export are wrapped; magnification, theme colors, and other AppKit extras remain open. |

## WKWebViewConfiguration

| API | Status | Notes |
| --- | --- | --- |
| `preferences` | ✅ | `Preferences` round-trips through JSON bridge helpers. |
| `websiteDataStore` | ✅ | `set_website_data_store` and `website_data_store`. |
| `applicationNameForUserAgent`, `allowsAirPlayForMediaPlayback`, `showsSystemScreenTimeBlockingView` | ✅ | Direct getters / setters exposed, with runtime availability handling for `showsSystemScreenTimeBlockingView` on macOS 26+. |
| `defaultWebpagePreferences.allowsContentJavaScript` | ✅ | Exposed through `set_allows_content_javascript` / `allows_content_javascript` and `Preferences::java_script_enabled`. |
| `userContentController` user scripts / message handlers / content rule lists | ✅ | User scripts, one-way and reply-capable handler registration per content world (a duplicate name in a world returns an error; `remove_message_handler` unregisters), and content rule list install / removal are bridged. Handlers are registered once per user content controller and routed to the `WebView` that sent the message. |
| `processPool` | ⏭️ | Deprecated on macOS 12+. |
| macOS / iOS feature flags such as `suppressesIncrementalRendering`, `upgradeKnownHostsToHTTPS`, media playback requirements, inline predictions, writing tools, and URL scheme handlers | 🟡 | `mediaTypesRequiringUserActionForPlayback` and `userInterfaceDirectionPolicy` are wrapped; the remaining knobs are still open. |
| iOS-only members (`allowsInlineMediaPlayback`, data detectors, picture-in-picture, viewport behaviors) | ⏭️ | iOS-only for this macOS crate. |

## WKWebsiteDataStore / WKWebsiteDataRecord

| API | Status | Notes |
| --- | --- | --- |
| `defaultDataStore`, `nonPersistentDataStore`, `persistent` | ✅ | `default_data_store`, `non_persistent`, `is_persistent`. |
| `allWebsiteDataTypes` | ✅ | `WebsiteDataStore::all_website_data_types()`. |
| `fetchDataRecords(ofTypes:)` | ✅ | `data_records`. |
| `removeData(ofTypes:for:)`, `removeData(ofTypes:modifiedSince:)` | ✅ | `remove_data_for_records`, `remove_data_modified_since`. |
| `httpCookieStore` | ✅ | `http_cookie_store()`. |
| `identifier`, `init(forIdentifier:)`, `remove(forIdentifier:)`, `fetchAllDataStoreIdentifiers` | ✅ | Availability-gated and return `Unsupported` on older macOS versions. |
| `fetchData(of:)`, `restoreData(_:)` | ✅ | Availability-gated import / export helpers for macOS 26+. |
| `proxyConfigurations` | ✅ | `ProxyConfiguration` / `ProxyConfigurationSummary` plus `proxy_configurations`, `set_proxy_configurations`, and `clear_proxy_configurations` expose the Network.framework-backed override surface. |
| `WKWebsiteDataRecord.displayName`, `WKWebsiteDataRecord.dataTypes` | ✅ | Exposed as `WebsiteDataRecord`. |
| `WKWebsiteDataType*` constants in `WKWebsiteDataRecord.h` | ✅ | Wrapped as `WebsiteDataType` constructors. |

## WKUserScript

| API | Status | Notes |
| --- | --- | --- |
| `source`, `injectionTime`, `forMainFrameOnly` | ✅ | Exposed on `UserScript`. |
| `init(... in: WKContentWorld)` | ✅ | `with_content_world` is bridged for macOS 11+. |

## WKNavigation

| API | Status | Notes |
| --- | --- | --- |
| Opaque navigation handle lifetime | ✅ | `Navigation` owns the retained handle and releases it on drop. |
| Public inspection APIs | ✅ | No additional public inspection surface in the requested header beyond handle identity. |

## WKNavigationDelegate

| API | Status | Notes |
| --- | --- | --- |
| `didStartProvisionalNavigation`, `didReceiveServerRedirectForProvisionalNavigation`, `didCommit`, `didFinish`, `didFail`, `didFailProvisionalNavigation`, `webViewWebContentProcessDidTerminate` | ✅ | Recorded as `NavigationEvent`s and available through callback + event drain. |
| `decidePolicyForNavigationAction`, `decidePolicyForNavigationResponse` | ✅ | Per-request `set_navigation_action_handler` / `set_navigation_response_handler`; a panicking handler cancels. Without a handler the static `NavigationDelegateConfig` policy applies (default: allow). |
| `navigationActionDidBecomeDownload`, `navigationResponseDidBecomeDownload` | ✅ | Captured as navigation events. |
| `webView:shouldGoToBackForwardListItem:willUseInstantBack:completionHandler:` | ✅ | Exposed through `BackForwardListNavigationPolicy`, `set_back_forward_list_navigation_policy`, `set_back_forward_list_navigation_handler`, and `drain_back_forward_list_navigation_events`. |
| Remaining delegate hooks (authentication challenges, HTTPS upgrades, rendering process details, etc.) | 🟡 | Not yet exposed; authentication challenges get WebKit's default handling. |
| Event storage | ✅ | Navigation and back/forward list events are also kept in bounded queues (1024 events, 4 MiB); drains report dropped events. |

## WKUIDelegate

| API | Status | Notes |
| --- | --- | --- |
| JavaScript alert / confirm / prompt panels | ✅ | Headless-safe responses and event recording in a bounded queue (1024 events, 4 MiB). |
| `createWebViewWithConfiguration`, `runOpenPanel`, media-capture permission request | ✅ | Bridged with deterministic headless behavior and typed `WindowFeatures` / `OpenPanelParameters` / `SecurityOrigin` event details. |
| Remaining AppKit / iOS delegate hooks (fullscreen, focus updates, etc.) | 🟡 | Standalone `WKContextMenuElementInfo` / `WKPreviewElementInfo` / `WKPreviewActionItem` wrappers are exposed; the remaining delegate hooks are still not bridged. |

## WKScriptMessageHandler

| API | Status | Notes |
| --- | --- | --- |
| `userContentController(_:didReceive:)` | ✅ | Live callback plus a bounded event queue (1024 messages, 4 MiB) for draining. |
| `WKScriptMessage` name / body / frame info / security origin / world | ✅ | Exposed as `ScriptMessage` (`frame: FrameInfo`, `world: ContentWorld`); live callbacks also get a `FrameHandle` for the sending frame. |
| `WKScriptMessageHandlerWithReply` | ✅ | `WebViewConfiguration::add_message_handler_with_reply` plus `WebView::set_message_handler_with_reply`; an `Err` rejects the page's promise. |

## WKURLSchemeHandler / WKURLSchemeTask

| API | Status | Notes |
| --- | --- | --- |
| `webView(_:start:)`, `webView(_:stop:)` | ✅ | `UrlSchemeHandler::start` / `stop`, called on the main thread. |
| `didReceive(_: URLResponse)`, `didReceive(_: Data)`, `didFinish()`, `didFailWithError(_:)` | ✅ | `UrlSchemeTask` methods. Task state is tracked on the main thread; calls WebKit would reject with an exception (wrong order, repeated completion, any call after `stop`) return `WebKitError::InvalidState`. |
| `request` URL / method / headers / body | ✅ | `UrlSchemeRequest`; `body` is filled when WebKit supplies `httpBody` (streamed bodies aren't read). |

## WKFrameInfo / WKSecurityOrigin / WKContentWorld

| API | Status | Notes |
| --- | --- | --- |
| `WKFrameInfo.isMainFrame`, `request`, `securityOrigin`, `webView` | ✅ | `FrameInfo` value snapshot (`webview_url` stands in for `webView`); `FrameHandle` keeps a frame for `call_async_javascript`. |
| `WKSecurityOrigin.protocol`, `host`, `port` | ✅ | `SecurityOrigin`. |
| `WKContentWorld.pageWorld`, `defaultClientWorld`, `world(name:)`, `name` | ✅ | `ContentWorld::{Page, DefaultClient, Named}`. |

## WKPreferences

| API | Status | Notes |
| --- | --- | --- |
| `minimumFontSize`, `javaScriptCanOpenWindowsAutomatically`, `fraudulentWebsiteWarningEnabled`, `shouldPrintBackgrounds`, `tabFocusesLinks`, `textInteractionEnabled`, `siteSpecificQuirksModeEnabled`, `elementFullscreenEnabled`, `inactiveSchedulingPolicy` | ✅ | Bridged through the `Preferences` value type with availability handling. |
| Deprecated `javaScriptEnabled` | ✅ | Mapped to `defaultWebpagePreferences.allowsContentJavaScript`. |
| Deprecated Java / plug-in knobs | ⏭️ | Deprecated by Apple and intentionally omitted. |
| visionOS-only `isLookToScrollEnabled` | ⏭️ | Not available on macOS. |

## WKContentRuleListStore / WKContentRuleList

| API | Status | Notes |
| --- | --- | --- |
| `defaultStore`, `storeWithURL:` | ✅ | `default_store`, `with_path`. |
| `compileContentRuleListForIdentifier`, `lookUpContentRuleListForIdentifier`, `removeContentRuleListForIdentifier`, `getAvailableContentRuleListIdentifiers` | ✅ | Fully bridged. |
| `WKContentRuleList.identifier` | ✅ | `ContentRuleList::identifier()`. |
| `WKUserContentController` add / remove / clear content rule lists | ✅ | Bridged through `WebViewConfiguration`. |

## WKHTTPCookieStore

| API | Status | Notes |
| --- | --- | --- |
| `getAllCookies`, `setCookie`, `deleteCookie` | ✅ | Bridged as blocking operations. |
| `setCookies` | ✅ | Availability-gated for macOS 26+. |
| `addObserver`, `removeObserver` | ✅ | Exposed as `start_observing`, `stop_observing`, and `drain_events` (bounded queue). |
| `setCookiePolicy`, `getCookiePolicy` | ✅ | Availability-gated for macOS 14+. |

## WKDownload

| API | Status | Notes |
| --- | --- | --- |
| `startDownload(using:)` on `WKWebView` | ✅ | `start_download_using_request`. |
| `cancel(_:)` / resume data | ✅ | `Download::cancel()`. |
| Original request URL, user-initiated state | ✅ | Bridged getters exposed. |
| Download delegate events (`decideDestination`, redirect, finish, fail`) | ✅ | Recorded as `DownloadEvent`s in a bounded queue. Suggested file names are sanitized and must resolve inside the destination directory. |
| Resume download / advanced session management | 🟡 | Not yet exposed. |

## WKWebExtension*

| API | Status | Notes |
| --- | --- | --- |
| `WKWebExtension`, `WKWebExtensionContext`, `WKWebExtensionController`, match patterns, permissions, and data records | ✅ | Safe wrappers and typed value models (macOS 15.4+; constructors return `WebKitError::Unsupported` on older systems). |
| `WKWebExtensionMessagePort` | 🟡 | `WebExtensionMessagePort` has methods but no constructor; ports only come from the controller delegate, which isn't bridged, so no value can be obtained. |
| `WKWebExtensionControllerDelegate`, `WKWebExtensionTab`, `WKWebExtensionWindow` | 🟡 | Rust traits with default methods and helper types only. The crate doesn't install a `WKWebExtensionControllerDelegate` or bridge tab/window objects, so WebKit never calls these traits. |

## WKBackForwardList / WKBackForwardListItem

| API | Status | Notes |
| --- | --- | --- |
| Snapshotting back / current / forward items | 🟡 | Snapshot wrapper exists, but some offscreen navigations still produce empty item lists. |
| `WKBackForwardListItem.url`, `title`, `initialURL` | ✅ | Exposed as `BackForwardListItem` fields when snapshot data is available. |
| Direct live item handles | 🟡 | Not yet exposed; current API is snapshot-based. |

## WKSnapshotConfiguration

| API | Status | Notes |
| --- | --- | --- |
| `rect`, `snapshotWidth`, `afterScreenUpdates` | ✅ | Fully wrapped by `SnapshotConfiguration`. |
| `WKWebView.takeSnapshot(with:)` integration | ✅ | `take_snapshot_png_with_configuration`. |

## WKPDFConfiguration

| API | Status | Notes |
| --- | --- | --- |
| `rect`, `allowTransparentBackground` | ✅ | Fully wrapped by `PDFConfiguration`. |
| `WKWebView.createPDF(configuration:)` integration | ✅ | `create_pdf`. |
