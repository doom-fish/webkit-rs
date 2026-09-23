import AppKit
import Foundation
import WebKit

private func wkJavaScriptResultString(_ value: Any?) -> String {
    if let string = value as? String {
        return string
    }
    if let number = value as? NSNumber {
        return number.stringValue
    }
    if let value,
       JSONSerialization.isValidJSONObject(value),
       let data = try? JSONSerialization.data(withJSONObject: value, options: []),
       let string = String(data: data, encoding: .utf8)
    {
        return string
    }
    if let value {
        return String(describing: value)
    }
    return ""
}

final class WKWebViewBox: NSObject {
    let webView: WKWebView
    let navDelegate: WKRustNavDelegate
    let uiDelegate: WKRustUIDelegate
    let scriptMessages = WKRustEventQueue()
    var messageCallback: WKRustCallback<WKMsgCallback>?
    var replyMessageCallback: WKRustCallback<WKMsgReplyCallback>?
    private let router: WKRustScriptMessageRouter

    init(configuration: WKWebViewConfiguration) {
        let navDelegate = WKRustNavDelegate()
        let uiDelegate = WKRustUIDelegate()
        self.navDelegate = navDelegate
        self.uiDelegate = uiDelegate
        self.webView = WKWebView(frame: CGRect(x: 0, y: 0, width: 800, height: 600), configuration: configuration)
        self.router = WKRustScriptMessageRouter.router(for: webView.configuration.userContentController)
        super.init()
        webView.navigationDelegate = navDelegate
        webView.uiDelegate = uiDelegate
        navDelegate.owner = self
        router.attach(self)
    }

    deinit {
        router.detachReleasedViews()
    }

    func resetLoadState() {
        wkOnMain {
            navDelegate.loadDone = false
            navDelegate.loadError = nil
        }
    }

    func waitForLoad(timeoutSeconds: Double = 30.0) -> (Int32, String?) {
        let deadline = Date(timeIntervalSinceNow: timeoutSeconds)
        while !wkOnMain({ navDelegate.loadDone }) {
            if Date() >= deadline {
                return (WK_TIMED_OUT, "load timed out after \(timeoutSeconds)s")
            }
            wkOnMain {
                wkRunLoopStep()
            }
        }
        if let error = wkOnMain({ navDelegate.loadError }) {
            return (WK_FRAMEWORK_ERROR, error)
        }
        return (WK_OK, nil)
    }

    func performNavigation(timeoutSeconds: Double = 30.0, _ start: () -> WKNavigation?) -> (Int32, UnsafeMutableRawPointer?, String?) {
        resetLoadState()
        let navigationPtr = wkOnMain {
            start().map { wkRetain(WKNavigationBox(navigation: $0)) }
        }
        let (status, error) = waitForLoad(timeoutSeconds: timeoutSeconds)
        return (status, navigationPtr, error)
    }

    func evaluateJS(_ js: String, timeoutSeconds: Double = 30.0) -> (Int32, String?, String?) {
        let (status, value, error): (Int32, String?, String?) = wkWaitForAsync(timeoutSeconds: timeoutSeconds) { completion in
            DispatchQueue.main.async {
                self.webView.evaluateJavaScript(js) { result, evalError in
                    if let evalError {
                        completion(nil, evalError.localizedDescription)
                        return
                    }
                    completion(wkJavaScriptResultString(result), nil)
                }
            }
        }
        return (status, value, error)
    }

    func callAsyncJS(
        _ functionBody: String,
        arguments: [String: Any],
        frame: WKFrameInfoBox?,
        worldKind: Int32,
        worldName: String?,
        timeoutSeconds: Double = 30.0
    ) -> (Int32, String?, String?) {
        let (status, value, error): (Int32, String?, String?) = wkWaitForAsync(timeoutSeconds: timeoutSeconds) { completion in
            DispatchQueue.main.async {
                let world = wkContentWorld(kind: worldKind, name: worldName)
                guard let world else {
                    completion(nil, "invalid content world")
                    return
                }
                self.webView.callAsyncJavaScript(functionBody, arguments: arguments, in: frame?.frameInfo, in: world) { result in
                    switch result {
                    case let .success(value):
                        completion(wkJavaScriptResultString(value), nil)
                    case let .failure(asyncError):
                        completion(nil, asyncError.localizedDescription)
                    }
                }
            }
        }
        return (status, value, error)
    }

    func takeSnapshotPNG(
        hasRect: Bool,
        x: Double,
        y: Double,
        width: Double,
        height: Double,
        hasSnapshotWidth: Bool,
        snapshotWidth: Double,
        afterScreenUpdates: Bool,
        timeoutSeconds: Double = 30.0
    ) -> (Int32, Data?, String?) {
        let (status, data, error): (Int32, Data?, String?) = wkWaitForAsync(timeoutSeconds: timeoutSeconds) { completion in
            DispatchQueue.main.async {
                let configuration = wkMakeSnapshotConfiguration(
                    hasRect: hasRect,
                    x: x,
                    y: y,
                    width: width,
                    height: height,
                    hasSnapshotWidth: hasSnapshotWidth,
                    snapshotWidth: snapshotWidth,
                    afterScreenUpdates: afterScreenUpdates
                )
                self.webView.takeSnapshot(with: configuration) { image, snapshotError in
                    if let snapshotError {
                        completion(nil, snapshotError.localizedDescription)
                        return
                    }
                    guard let image,
                          let tiffData = image.tiffRepresentation,
                          let bitmap = NSBitmapImageRep(data: tiffData),
                          let pngData = bitmap.representation(using: .png, properties: [:])
                    else {
                        completion(nil, "failed to convert snapshot to PNG")
                        return
                    }
                    completion(pngData, nil)
                }
            }
        }
        return (status, data, error)
    }

    func createPDF(
        hasRect: Bool,
        x: Double,
        y: Double,
        width: Double,
        height: Double,
        allowTransparentBackground: Bool,
        timeoutSeconds: Double = 30.0
    ) -> (Int32, Data?, String?) {
        let (status, data, error): (Int32, Data?, String?) = wkWaitForAsync(timeoutSeconds: timeoutSeconds) { completion in
            DispatchQueue.main.async {
                let configuration = wkMakePDFConfiguration(
                    hasRect: hasRect,
                    x: x,
                    y: y,
                    width: width,
                    height: height,
                    allowTransparentBackground: allowTransparentBackground
                )
                self.webView.createPDF(configuration: configuration) { result in
                    switch result {
                    case let .success(data):
                        completion(data, nil)
                    case let .failure(error):
                        completion(nil, error.localizedDescription)
                    }
                }
            }
        }
        return (status, data, error)
    }

    func startDownload(
        request: URLRequest,
        destinationDirectory: String,
        timeoutSeconds: Double = 30.0
    ) -> (Int32, UnsafeMutableRawPointer?, String?) {
        let (status, box, error): (Int32, WKDownloadBox?, String?) = wkWaitForAsync(timeoutSeconds: timeoutSeconds) { completion in
            DispatchQueue.main.async {
                self.webView.startDownload(using: request) { download in
                    completion(WKDownloadBox(download: download, destinationDirectory: destinationDirectory), nil)
                }
            }
        }
        if let error {
            return (status, nil, error)
        }
        return (status, box.map(wkRetain), nil)
    }
}

@_cdecl("wk_webview_new")
public func wk_webview_new(_ cfgPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer {
    let configBox: WKConfigBox? = cfgPtr.map { wkBorrow($0) }
    return wkOnMain {
        wkRetain(WKWebViewBox(configuration: configBox?.config ?? WKWebViewConfiguration()))
    }
}

@_cdecl("wk_webview_release")
public func wk_webview_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkReleaseOnMain(ptr)
}

@_cdecl("wk_webview_set_nav_callback")
public func wk_webview_set_nav_callback(
    _ ptr: UnsafeMutableRawPointer?,
    _ callback: WKNavCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ retain: WKContextRetainCallback?,
    _ release: WKContextReleaseCallback?
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.navDelegate.eventCallback = WKRustCallback(function: callback, userInfo: userInfo, retain: retain, release: release)
    }
}

@_cdecl("wk_webview_set_back_forward_list_nav_callback")
public func wk_webview_set_back_forward_list_nav_callback(
    _ ptr: UnsafeMutableRawPointer?,
    _ callback: WKNavCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ retain: WKContextRetainCallback?,
    _ release: WKContextReleaseCallback?
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.navDelegate.backForwardListCallback = WKRustCallback(
            function: callback,
            userInfo: userInfo,
            retain: retain,
            release: release
        )
    }
}

@_cdecl("wk_webview_set_navigation_action_decision_callback")
public func wk_webview_set_navigation_action_decision_callback(
    _ ptr: UnsafeMutableRawPointer?,
    _ callback: WKNavDecisionCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ retain: WKContextRetainCallback?,
    _ release: WKContextReleaseCallback?
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.navDelegate.actionDecisionCallback = WKRustCallback(
            function: callback,
            userInfo: userInfo,
            retain: retain,
            release: release
        )
    }
}

@_cdecl("wk_webview_set_navigation_response_decision_callback")
public func wk_webview_set_navigation_response_decision_callback(
    _ ptr: UnsafeMutableRawPointer?,
    _ callback: WKNavDecisionCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ retain: WKContextRetainCallback?,
    _ release: WKContextReleaseCallback?
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.navDelegate.responseDecisionCallback = WKRustCallback(
            function: callback,
            userInfo: userInfo,
            retain: retain,
            release: release
        )
    }
}

@_cdecl("wk_webview_set_msg_callback")
public func wk_webview_set_msg_callback(
    _ ptr: UnsafeMutableRawPointer?,
    _ callback: WKMsgCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ retain: WKContextRetainCallback?,
    _ release: WKContextReleaseCallback?
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.messageCallback = WKRustCallback(function: callback, userInfo: userInfo, retain: retain, release: release)
    }
}

@_cdecl("wk_webview_set_msg_reply_callback")
public func wk_webview_set_msg_reply_callback(
    _ ptr: UnsafeMutableRawPointer?,
    _ callback: WKMsgReplyCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ retain: WKContextRetainCallback?,
    _ release: WKContextReleaseCallback?
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.replyMessageCallback = WKRustCallback(
            function: callback,
            userInfo: userInfo,
            retain: retain,
            release: release
        )
    }
}

@_cdecl("wk_webview_set_navigation_delegate_config")
public func wk_webview_set_navigation_delegate_config(
    _ ptr: UnsafeMutableRawPointer?,
    _ actionPolicy: Int32,
    _ responsePolicy: Int32
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.navDelegate.actionPolicy = WKRustNavigationActionPolicy(rawValue: actionPolicy) ?? .cancel
        box.navDelegate.responsePolicy = WKRustNavigationResponsePolicy(rawValue: responsePolicy) ?? .cancel
    }
}

@_cdecl("wk_webview_set_back_forward_list_navigation_policy")
public func wk_webview_set_back_forward_list_navigation_policy(
    _ ptr: UnsafeMutableRawPointer?,
    _ policy: Int32
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.navDelegate.backForwardListPolicy =
            WKRustBackForwardListNavigationPolicy(rawValue: policy) ?? .allow
    }
}

@_cdecl("wk_webview_drain_navigation_events_json")
public func wk_webview_drain_navigation_events_json(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return WKRustEventQueue().drain() }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.navDelegate.events.drain() }
}

@_cdecl("wk_webview_drain_back_forward_list_navigation_events_json")
public func wk_webview_drain_back_forward_list_navigation_events_json(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return WKRustEventQueue().drain() }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.navDelegate.backForwardListEvents.drain() }
}

@_cdecl("wk_webview_set_ui_delegate_config")
public func wk_webview_set_ui_delegate_config(
    _ ptr: UnsafeMutableRawPointer?,
    _ confirmResponse: Bool,
    _ promptResponse: UnsafePointer<CChar>?
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    let prompt = promptResponse.map(String.init(cString:))
    wkOnMain {
        box.uiDelegate.confirmResponse = confirmResponse
        box.uiDelegate.promptResponse = prompt
    }
}

@_cdecl("wk_webview_drain_ui_events_json")
public func wk_webview_drain_ui_events_json(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return WKRustEventQueue().drain() }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.uiDelegate.events.drain() }
}

@_cdecl("wk_webview_drain_script_messages_json")
public func wk_webview_drain_script_messages_json(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return WKRustEventQueue().drain() }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.scriptMessages.drain() }
}

@_cdecl("wk_webview_load_url")
public func wk_webview_load_url(
    _ ptr: UnsafeMutableRawPointer?,
    _ urlStr: UnsafePointer<CChar>?,
    _ outNavigation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let urlStr else {
        outErr?.pointee = wkCString("missing webview or url")
        return WK_INVALID_ARGUMENT
    }
    guard let url = URL(string: String(cString: urlStr)) else {
        outErr?.pointee = wkCString("invalid URL")
        return WK_INVALID_ARGUMENT
    }

    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, navigation, error) = box.performNavigation {
        box.webView.load(URLRequest(url: url))
    }
    outNavigation?.pointee = navigation
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_webview_load_html")
public func wk_webview_load_html(
    _ ptr: UnsafeMutableRawPointer?,
    _ html: UnsafePointer<CChar>?,
    _ baseUrl: UnsafePointer<CChar>?,
    _ outNavigation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let html else {
        outErr?.pointee = wkCString("missing webview or html")
        return WK_INVALID_ARGUMENT
    }

    let box: WKWebViewBox = wkBorrow(ptr)
    let htmlString = String(cString: html)
    let baseURL = baseUrl.flatMap { URL(string: String(cString: $0)) }
    let (status, navigation, error) = box.performNavigation {
        box.webView.loadHTMLString(htmlString, baseURL: baseURL)
    }
    outNavigation?.pointee = navigation
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_webview_load_file_url")
public func wk_webview_load_file_url(
    _ ptr: UnsafeMutableRawPointer?,
    _ fileUrl: UnsafePointer<CChar>?,
    _ readAccessUrl: UnsafePointer<CChar>?,
    _ outNavigation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let fileUrl, let readAccessUrl else {
        outErr?.pointee = wkCString("missing webview or file URL")
        return WK_INVALID_ARGUMENT
    }
    let fileURL = URL(fileURLWithPath: String(cString: fileUrl))
    let readAccessURL = URL(fileURLWithPath: String(cString: readAccessUrl))
    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, navigation, error) = box.performNavigation {
        box.webView.loadFileURL(fileURL, allowingReadAccessTo: readAccessURL)
    }
    outNavigation?.pointee = navigation
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_webview_load_data")
public func wk_webview_load_data(
    _ ptr: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ mimeType: UnsafePointer<CChar>?,
    _ encodingName: UnsafePointer<CChar>?,
    _ baseUrl: UnsafePointer<CChar>?,
    _ outNavigation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let bytes, let mimeType, let encodingName, let baseUrl else {
        outErr?.pointee = wkCString("missing webview or data arguments")
        return WK_INVALID_ARGUMENT
    }
    let data = Data(bytes: bytes, count: len)
    let mime = String(cString: mimeType)
    let encoding = String(cString: encodingName)
    let url = URL(string: String(cString: baseUrl)) ?? URL(fileURLWithPath: "/")
    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, navigation, error) = box.performNavigation {
        box.webView.load(data, mimeType: mime, characterEncodingName: encoding, baseURL: url)
    }
    outNavigation?.pointee = navigation
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_webview_go_back")
public func wk_webview_go_back(
    _ ptr: UnsafeMutableRawPointer?,
    _ outNavigation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    let navigation = wkOnMain {
        box.webView.goBack().map { wkRetain(WKNavigationBox(navigation: $0)) }
    }
    outNavigation?.pointee = navigation
    return navigation != nil
}

@_cdecl("wk_webview_go_forward")
public func wk_webview_go_forward(
    _ ptr: UnsafeMutableRawPointer?,
    _ outNavigation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    let navigation = wkOnMain {
        box.webView.goForward().map { wkRetain(WKNavigationBox(navigation: $0)) }
    }
    outNavigation?.pointee = navigation
    return navigation != nil
}

@_cdecl("wk_webview_reload")
public func wk_webview_reload(
    _ ptr: UnsafeMutableRawPointer?,
    _ outNavigation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    let navigation = wkOnMain {
        box.webView.reload().map { wkRetain(WKNavigationBox(navigation: $0)) }
    }
    outNavigation?.pointee = navigation
    return navigation != nil
}

@_cdecl("wk_webview_reload_from_origin")
public func wk_webview_reload_from_origin(
    _ ptr: UnsafeMutableRawPointer?,
    _ outNavigation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    let navigation = wkOnMain {
        box.webView.reloadFromOrigin().map { wkRetain(WKNavigationBox(navigation: $0)) }
    }
    outNavigation?.pointee = navigation
    return navigation != nil
}

@_cdecl("wk_webview_stop_loading")
public func wk_webview_stop_loading(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.webView.stopLoading()
    }
}

@_cdecl("wk_webview_perform_go_back_action")
public func wk_webview_perform_go_back_action(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.webView.goBack(nil)
    }
}

@_cdecl("wk_webview_perform_go_forward_action")
public func wk_webview_perform_go_forward_action(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.webView.goForward(nil)
    }
}

@_cdecl("wk_webview_perform_reload_action")
public func wk_webview_perform_reload_action(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.webView.reload(nil)
    }
}

@_cdecl("wk_webview_perform_reload_from_origin_action")
public func wk_webview_perform_reload_from_origin_action(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.webView.reloadFromOrigin(nil)
    }
}

@_cdecl("wk_webview_perform_stop_loading_action")
public func wk_webview_perform_stop_loading_action(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.webView.stopLoading(nil)
    }
}

@_cdecl("wk_webview_validate_text_finder_action")
public func wk_webview_validate_text_finder_action(
    _ ptr: UnsafeMutableRawPointer?,
    _ action: Int32
) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain {
        let finder = NSTextFinder()
        finder.client = box.webView
        guard let action = NSTextFinder.Action(rawValue: Int(action)) else {
            return false
        }
        return finder.validateAction(action)
    }
}

@_cdecl("wk_webview_perform_text_finder_action")
public func wk_webview_perform_text_finder_action(
    _ ptr: UnsafeMutableRawPointer?,
    _ action: Int32
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        let finder = NSTextFinder()
        finder.client = box.webView
        if let action = NSTextFinder.Action(rawValue: Int(action)) {
            finder.performAction(action)
        }
    }
}

@_cdecl("wk_webview_go_to_back_forward_index")
public func wk_webview_go_to_back_forward_index(
    _ ptr: UnsafeMutableRawPointer?,
    _ index: Int,
    _ outNavigation: UnsafeMutablePointer<UnsafeMutableRawPointer?>?
) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    let navigation: UnsafeMutableRawPointer? = wkOnMain {
        guard let item = box.webView.backForwardList.item(at: index) else {
            return nil
        }
        return box.webView.go(to: item).map { wkRetain(WKNavigationBox(navigation: $0)) }
    }
    outNavigation?.pointee = navigation
    return navigation != nil
}

@_cdecl("wk_webview_copy_title")
public func wk_webview_copy_title(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkCString(wkOnMain { box.webView.title ?? "" })
}

@_cdecl("wk_webview_copy_url")
public func wk_webview_copy_url(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkCString(wkOnMain { box.webView.url?.absoluteString ?? "" })
}

@_cdecl("wk_webview_is_loading")
public func wk_webview_is_loading(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.webView.isLoading }
}

@_cdecl("wk_webview_get_estimated_progress")
public func wk_webview_get_estimated_progress(_ ptr: UnsafeMutableRawPointer?) -> Double {
    guard let ptr else { return 0 }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.webView.estimatedProgress }
}

@_cdecl("wk_webview_get_has_only_secure_content")
public func wk_webview_get_has_only_secure_content(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.webView.hasOnlySecureContent }
}

@_cdecl("wk_webview_get_can_go_back")
public func wk_webview_get_can_go_back(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.webView.canGoBack }
}

@_cdecl("wk_webview_get_can_go_forward")
public func wk_webview_get_can_go_forward(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.webView.canGoForward }
}

@_cdecl("wk_webview_copy_back_forward_list_json")
public func wk_webview_copy_back_forward_list_json(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return wkCString("[]") }
    let box: WKWebViewBox = wkBorrow(ptr)
    let snapshot = wkOnMain {
        wkBackForwardListSnapshot(box.webView.backForwardList)
    }
    return wkCString(wkJSONString(snapshot))
}

@_cdecl("wk_webview_set_custom_user_agent")
public func wk_webview_set_custom_user_agent(_ ptr: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    let string = value.map(String.init(cString:))
    wkOnMain {
        box.webView.customUserAgent = string
    }
}

@_cdecl("wk_webview_copy_custom_user_agent")
public func wk_webview_copy_custom_user_agent(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkCString(wkOnMain { box.webView.customUserAgent ?? "" })
}

@_cdecl("wk_webview_set_allows_link_preview")
public func wk_webview_set_allows_link_preview(_ ptr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.webView.allowsLinkPreview = value
    }
}

@_cdecl("wk_webview_get_allows_link_preview")
public func wk_webview_get_allows_link_preview(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.webView.allowsLinkPreview }
}

@_cdecl("wk_webview_set_page_zoom")
public func wk_webview_set_page_zoom(_ ptr: UnsafeMutableRawPointer?, _ value: Double) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        box.webView.pageZoom = value
    }
}

@_cdecl("wk_webview_get_page_zoom")
public func wk_webview_get_page_zoom(_ ptr: UnsafeMutableRawPointer?) -> Double {
    guard let ptr else { return 1.0 }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain { box.webView.pageZoom }
}

@_cdecl("wk_webview_set_media_type")
public func wk_webview_set_media_type(_ ptr: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    let string = value.map(String.init(cString:))
    wkOnMain {
        if #available(macOS 11.0, *) {
            box.webView.mediaType = string
        }
    }
}

@_cdecl("wk_webview_copy_media_type")
public func wk_webview_copy_media_type(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    let box: WKWebViewBox = wkBorrow(ptr)
    let mediaType = wkOnMain {
        if #available(macOS 11.0, *) {
            return box.webView.mediaType ?? ""
        }
        return ""
    }
    return wkCString(mediaType)
}

@_cdecl("wk_webview_set_inspectable")
public func wk_webview_set_inspectable(_ ptr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    wkOnMain {
        if #available(macOS 13.3, *) {
            box.webView.isInspectable = value
        }
    }
}

@_cdecl("wk_webview_get_inspectable")
public func wk_webview_get_inspectable(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return false }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain {
        if #available(macOS 13.3, *) {
            return box.webView.isInspectable
        }
        return false
    }
}

@_cdecl("wk_webview_request_media_playback_state")
public func wk_webview_request_media_playback_state(
    _ ptr: UnsafeMutableRawPointer?,
    _ outState: UnsafeMutablePointer<Int32>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing webview")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, state, error): (Int32, Int32?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            if #available(macOS 12.0, *) {
                box.webView.requestMediaPlaybackState { state in
                    completion(Int32(state.rawValue), nil)
                }
            } else {
                completion(nil, "media playback state requires macOS 12.0")
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    outState?.pointee = state ?? 0
    return status
}

@_cdecl("wk_webview_get_camera_capture_state")
public func wk_webview_get_camera_capture_state(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return 0 }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain {
        if #available(macOS 12.0, *) {
            return Int32(box.webView.cameraCaptureState.rawValue)
        }
        return 0
    }
}

@_cdecl("wk_webview_get_microphone_capture_state")
public func wk_webview_get_microphone_capture_state(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return 0 }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain {
        if #available(macOS 12.0, *) {
            return Int32(box.webView.microphoneCaptureState.rawValue)
        }
        return 0
    }
}

@_cdecl("wk_webview_set_camera_capture_state")
public func wk_webview_set_camera_capture_state(
    _ ptr: UnsafeMutableRawPointer?,
    _ rawValue: Int32,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing webview")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            if #available(macOS 12.0, *), let state = WKMediaCaptureState(rawValue: Int(rawValue)) {
                box.webView.setCameraCaptureState(state) {
                    completion(true, nil)
                }
            } else {
                completion(nil, "camera capture state requires macOS 12.0")
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_webview_set_microphone_capture_state")
public func wk_webview_set_microphone_capture_state(
    _ ptr: UnsafeMutableRawPointer?,
    _ rawValue: Int32,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing webview")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            if #available(macOS 12.0, *), let state = WKMediaCaptureState(rawValue: Int(rawValue)) {
                box.webView.setMicrophoneCaptureState(state) {
                    completion(true, nil)
                }
            } else {
                completion(nil, "microphone capture state requires macOS 12.0")
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_webview_get_fullscreen_state")
public func wk_webview_get_fullscreen_state(_ ptr: UnsafeMutableRawPointer?) -> Int32 {
    guard let ptr else { return 0 }
    let box: WKWebViewBox = wkBorrow(ptr)
    return wkOnMain {
        if #available(macOS 13.0, *) {
            return Int32(box.webView.fullscreenState.rawValue)
        }
        return 0
    }
}

@_cdecl("wk_webview_fetch_data_of_types")
public func wk_webview_fetch_data_of_types(
    _ ptr: UnsafeMutableRawPointer?,
    _ dataTypes: UInt64,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<Int>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing webview")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 26.0, *) else {
        outErr?.pointee = wkCString("web view data export requires macOS 26.0")
        return WK_UNSUPPORTED
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, data, error): (Int32, Data?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.webView.fetchData(of: WKWebViewDataType(rawValue: UInt(dataTypes))) { data, error in
                if let error {
                    completion(nil, error.localizedDescription)
                } else {
                    completion(data, nil)
                }
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    if let data {
        wkSetBytes(data, outBytes, outLen)
        return status
    }
    outErr?.pointee = wkCString("web view data export returned no data")
    return WK_UNKNOWN
}

@_cdecl("wk_webview_restore_data")
public func wk_webview_restore_data(
    _ ptr: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let bytes else {
        outErr?.pointee = wkCString("missing webview or data")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 26.0, *) else {
        outErr?.pointee = wkCString("web view data restore requires macOS 26.0")
        return WK_UNSUPPORTED
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let data = Data(bytes: bytes, count: len)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.webView.restoreData(data) { error in
                if let error {
                    completion(nil, error.localizedDescription)
                } else {
                    completion(true, nil)
                }
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_webview_evaluate_js")
public func wk_webview_evaluate_js(
    _ ptr: UnsafeMutableRawPointer?,
    _ js: UnsafePointer<CChar>?,
    _ outResult: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let js else {
        outErr?.pointee = wkCString("missing webview or js")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, result, error) = box.evaluateJS(String(cString: js))
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    outResult?.pointee = wkCString(result ?? "")
    return status
}

@_cdecl("wk_webview_call_async_js")
public func wk_webview_call_async_js(
    _ ptr: UnsafeMutableRawPointer?,
    _ functionBody: UnsafePointer<CChar>?,
    _ argumentsJson: UnsafePointer<CChar>?,
    _ framePtr: UnsafeMutableRawPointer?,
    _ worldKind: Int32,
    _ worldName: UnsafePointer<CChar>?,
    _ outResult: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let functionBody else {
        outErr?.pointee = wkCString("missing webview or function body")
        return WK_INVALID_ARGUMENT
    }
    guard let arguments = wkJavaScriptArguments(from: argumentsJson) else {
        outErr?.pointee = wkCString("callAsyncJavaScript arguments must be a JSON object")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let frame: WKFrameInfoBox? = framePtr.map { wkBorrow($0) }
    let (status, result, error) = box.callAsyncJS(
        String(cString: functionBody),
        arguments: arguments,
        frame: frame,
        worldKind: worldKind,
        worldName: worldName.map(String.init(cString:))
    )
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    outResult?.pointee = wkCString(result ?? "")
    return status
}

@_cdecl("wk_webview_take_snapshot_png")
public func wk_webview_take_snapshot_png(
    _ ptr: UnsafeMutableRawPointer?,
    _ hasRect: Bool,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ hasSnapshotWidth: Bool,
    _ snapshotWidth: Double,
    _ afterScreenUpdates: Bool,
    _ outPng: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<Int>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing webview")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, data, error) = box.takeSnapshotPNG(
        hasRect: hasRect,
        x: x,
        y: y,
        width: width,
        height: height,
        hasSnapshotWidth: hasSnapshotWidth,
        snapshotWidth: snapshotWidth,
        afterScreenUpdates: afterScreenUpdates
    )
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    guard let data else {
        outErr?.pointee = wkCString("snapshot returned nil data")
        return WK_UNKNOWN
    }
    wkSetBytes(data, outPng, outLen)
    return status
}

@_cdecl("wk_webview_create_pdf")
public func wk_webview_create_pdf(
    _ ptr: UnsafeMutableRawPointer?,
    _ hasRect: Bool,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ allowTransparentBackground: Bool,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<Int>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing webview")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, data, error) = box.createPDF(
        hasRect: hasRect,
        x: x,
        y: y,
        width: width,
        height: height,
        allowTransparentBackground: allowTransparentBackground
    )
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    guard let data else {
        outErr?.pointee = wkCString("pdf creation returned nil data")
        return WK_UNKNOWN
    }
    wkSetBytes(data, outBytes, outLen)
    return status
}

@_cdecl("wk_webview_start_download")
public func wk_webview_start_download(
    _ ptr: UnsafeMutableRawPointer?,
    _ urlStr: UnsafePointer<CChar>?,
    _ destinationDirectory: UnsafePointer<CChar>?,
    _ outDownload: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let urlStr, let destinationDirectory else {
        outErr?.pointee = wkCString("missing webview, url, or destination directory")
        return WK_INVALID_ARGUMENT
    }
    guard let url = URL(string: String(cString: urlStr)) else {
        outErr?.pointee = wkCString("invalid download URL")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, download, error) = box.startDownload(
        request: URLRequest(url: url),
        destinationDirectory: String(cString: destinationDirectory)
    )
    outDownload?.pointee = download
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}
