import AppKit
import Foundation
import WebKit

public typealias WKNavCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> Void

public typealias WKMsgCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?
) -> Void

private func wkJSONString(_ object: [String: Any]) -> String {
    guard JSONSerialization.isValidJSONObject(object),
          let data = try? JSONSerialization.data(withJSONObject: object, options: []),
          let string = String(data: data, encoding: .utf8)
    else {
        return #"{"event":"unknown"}"#
    }
    return string
}

private func wkMessageBodyString(_ body: Any) -> String {
    if let string = body as? String {
        return string
    }
    if let number = body as? NSNumber {
        return number.stringValue
    }
    if JSONSerialization.isValidJSONObject(body),
       let data = try? JSONSerialization.data(withJSONObject: body, options: []),
       let string = String(data: data, encoding: .utf8)
    {
        return string
    }
    return String(describing: body)
}

final class WKRustNavDelegate: NSObject, WKNavigationDelegate {
    var callback: WKNavCallback?
    var userInfo: UnsafeMutableRawPointer?
    var loadDone = false
    var loadError: String?

    private func emit(_ payload: [String: Any]) {
        let json = wkJSONString(payload)
        json.withCString { callback?(userInfo, $0) }
    }

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        loadDone = true
        loadError = nil
        emit([
            "event": "didFinish",
            "url": webView.url?.absoluteString ?? ""
        ])
    }

    func webView(_ webView: WKWebView, didFail navigation: WKNavigation!, withError error: Error) {
        loadDone = true
        loadError = error.localizedDescription
        emit([
            "event": "didFail",
            "url": webView.url?.absoluteString ?? "",
            "error": error.localizedDescription
        ])
    }

    func webView(
        _ webView: WKWebView,
        didFailProvisionalNavigation navigation: WKNavigation!,
        withError error: Error
    ) {
        loadDone = true
        loadError = error.localizedDescription
        emit([
            "event": "didFailProvisional",
            "url": webView.url?.absoluteString ?? "",
            "error": error.localizedDescription
        ])
    }

    func webView(
        _ webView: WKWebView,
        decidePolicyFor navigationAction: WKNavigationAction,
        decisionHandler: @escaping (WKNavigationActionPolicy) -> Void
    ) {
        emit([
            "event": "decidePolicyForAction",
            "url": navigationAction.request.url?.absoluteString ?? "",
            "navigationType": navigationAction.navigationType.rawValue
        ])
        decisionHandler(.allow)
    }
}

final class WKRustMessageHandler: NSObject, WKScriptMessageHandler {
    var callback: WKMsgCallback?
    var userInfo: UnsafeMutableRawPointer?

    func userContentController(
        _ userContentController: WKUserContentController,
        didReceive message: WKScriptMessage
    ) {
        let name = message.name
        let body = wkMessageBodyString(message.body)
        name.withCString { nameCStr in
            body.withCString { bodyCStr in
                callback?(userInfo, nameCStr, bodyCStr)
            }
        }
    }
}

final class WKRustUIDelegate: NSObject, WKUIDelegate {}

final class WKWebViewBox: NSObject {
    let webView: WKWebView
    let navDelegate: WKRustNavDelegate
    let msgHandler: WKRustMessageHandler
    let uiDelegate: WKRustUIDelegate
    let handlerNames: [String]

    init(configuration: WKWebViewConfiguration, handlerNames: [String]) {
        self.handlerNames = handlerNames
        let navDelegate = WKRustNavDelegate()
        let msgHandler = WKRustMessageHandler()
        let uiDelegate = WKRustUIDelegate()
        self.navDelegate = navDelegate
        self.msgHandler = msgHandler
        self.uiDelegate = uiDelegate

        for handlerName in handlerNames {
            configuration.userContentController.add(msgHandler, name: handlerName)
        }

        let initialFrame = CGRect(x: 0, y: 0, width: 800, height: 600)
        let createdWebView: WKWebView = wkOnMain {
            WKWebView(frame: initialFrame, configuration: configuration)
        }
        self.webView = createdWebView
        super.init()
        self.webView.navigationDelegate = navDelegate
        self.webView.uiDelegate = uiDelegate
    }

    deinit {
        for handlerName in handlerNames {
            webView.configuration.userContentController.removeScriptMessageHandler(forName: handlerName)
        }
    }

    func resetLoadState() {
        navDelegate.loadDone = false
        navDelegate.loadError = nil
    }

    func waitForLoad(timeoutSeconds: Double = 30.0) -> (Int32, String?) {
        let completed = wkWait(timeoutSeconds: timeoutSeconds) { [weak self] in
            self?.navDelegate.loadDone ?? true
        }
        if !completed {
            return (WK_TIMED_OUT, "load timed out after \(timeoutSeconds)s")
        }
        if let error = navDelegate.loadError {
            return (WK_FRAMEWORK_ERROR, error)
        }
        return (WK_OK, nil)
    }

    func evaluateJS(_ js: String, timeoutSeconds: Double = 30.0) -> (Int32, String?, String?) {
        var done = false
        var result: String?
        var error: String?

        DispatchQueue.main.async { [weak self] in
            guard let self else {
                error = "webview released"
                done = true
                return
            }
            self.webView.evaluateJavaScript(js) { value, evalError in
                if let evalError {
                    error = evalError.localizedDescription
                } else if let string = value as? String {
                    result = string
                } else if let number = value as? NSNumber {
                    result = number.stringValue
                } else if let value,
                          JSONSerialization.isValidJSONObject(value),
                          let data = try? JSONSerialization.data(withJSONObject: value, options: []),
                          let string = String(data: data, encoding: .utf8)
                {
                    result = string
                } else if let value {
                    result = String(describing: value)
                } else {
                    result = ""
                }
                done = true
            }
        }

        if !wkWait(timeoutSeconds: timeoutSeconds, isDone: { done }) {
            return (WK_TIMED_OUT, nil, "evaluateJavaScript timed out")
        }
        if let error {
            return (WK_FRAMEWORK_ERROR, nil, error)
        }
        return (WK_OK, result ?? "", nil)
    }

    func callAsyncJS(_ js: String, timeoutSeconds: Double = 30.0) -> (Int32, String?, String?) {
        var done = false
        var result: String?
        var error: String?

        DispatchQueue.main.async { [weak self] in
            guard let self else {
                error = "webview released"
                done = true
                return
            }
            self.webView.callAsyncJavaScript(
                js,
                arguments: [:],
                in: nil,
                in: .page
            ) { asyncResult in
                switch asyncResult {
                case let .success(value):
                    if let string = value as? String {
                        result = string
                    } else if let number = value as? NSNumber {
                        result = number.stringValue
                    } else if JSONSerialization.isValidJSONObject(value),
                              let data = try? JSONSerialization.data(withJSONObject: value, options: []),
                              let string = String(data: data, encoding: .utf8)
                    {
                        result = string
                    } else {
                        result = String(describing: value)
                    }
                case let .failure(asyncError):
                    error = asyncError.localizedDescription
                }
                done = true
            }
        }

        if !wkWait(timeoutSeconds: timeoutSeconds, isDone: { done }) {
            return (WK_TIMED_OUT, nil, "callAsyncJavaScript timed out")
        }
        if let error {
            return (WK_FRAMEWORK_ERROR, nil, error)
        }
        return (WK_OK, result ?? "", nil)
    }

    func takeSnapshotPNG(timeoutSeconds: Double = 30.0) -> (Int32, Data?, String?) {
        var done = false
        var imageData: Data?
        var error: String?

        DispatchQueue.main.async { [weak self] in
            guard let self else {
                error = "webview released"
                done = true
                return
            }
            let configuration = WKSnapshotConfiguration()
            configuration.rect = self.webView.bounds.isEmpty
                ? CGRect(x: 0, y: 0, width: 800, height: 600)
                : self.webView.bounds
            self.webView.takeSnapshot(with: configuration) { image, snapshotError in
                if let snapshotError {
                    error = snapshotError.localizedDescription
                } else if let image,
                          let tiffData = image.tiffRepresentation,
                          let bitmap = NSBitmapImageRep(data: tiffData),
                          let pngData = bitmap.representation(using: .png, properties: [:])
                {
                    imageData = pngData
                } else {
                    error = "failed to convert snapshot to PNG"
                }
                done = true
            }
        }

        if !wkWait(timeoutSeconds: timeoutSeconds, isDone: { done }) {
            return (WK_TIMED_OUT, nil, "takeSnapshot timed out")
        }
        if let error {
            return (WK_FRAMEWORK_ERROR, nil, error)
        }
        return (WK_OK, imageData, nil)
    }
}

@_cdecl("wk_webview_new")
public func wk_webview_new(_ cfgPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer {
    let configuration: WKWebViewConfiguration
    let handlerNames: [String]

    if let cfgPtr {
        let box: WKConfigBox = wkBorrow(cfgPtr)
        configuration = box.config
        handlerNames = box.registeredHandlerNames
    } else {
        configuration = WKWebViewConfiguration()
        handlerNames = []
    }

    let box = WKWebViewBox(configuration: configuration, handlerNames: handlerNames)
    return wkRetain(box)
}

@_cdecl("wk_webview_release")
public func wk_webview_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_webview_set_nav_callback")
public func wk_webview_set_nav_callback(
    _ ptr: UnsafeMutableRawPointer?,
    _ callback: WKNavCallback?,
    _ userInfo: UnsafeMutableRawPointer?
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    box.navDelegate.callback = callback
    box.navDelegate.userInfo = userInfo
}

@_cdecl("wk_webview_set_msg_callback")
public func wk_webview_set_msg_callback(
    _ ptr: UnsafeMutableRawPointer?,
    _ callback: WKMsgCallback?,
    _ userInfo: UnsafeMutableRawPointer?
) {
    guard let ptr else { return }
    let box: WKWebViewBox = wkBorrow(ptr)
    box.msgHandler.callback = callback
    box.msgHandler.userInfo = userInfo
}

@_cdecl("wk_webview_load_url")
public func wk_webview_load_url(
    _ ptr: UnsafeMutableRawPointer?,
    _ urlStr: UnsafePointer<CChar>?,
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
    box.resetLoadState()
    DispatchQueue.main.async {
        box.webView.load(URLRequest(url: url))
    }

    let (status, error) = box.waitForLoad()
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
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let html else {
        outErr?.pointee = wkCString("missing webview or html")
        return WK_INVALID_ARGUMENT
    }

    let box: WKWebViewBox = wkBorrow(ptr)
    let htmlString = String(cString: html)
    let baseURL = baseUrl.flatMap { URL(string: String(cString: $0)) }
    box.resetLoadState()
    DispatchQueue.main.async {
        box.webView.loadHTMLString(htmlString, baseURL: baseURL)
    }

    let (status, error) = box.waitForLoad()
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
    _ js: UnsafePointer<CChar>?,
    _ outResult: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let js else {
        outErr?.pointee = wkCString("missing webview or js")
        return WK_INVALID_ARGUMENT
    }

    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, result, error) = box.callAsyncJS(String(cString: js))
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
    _ outPng: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<Int>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing webview")
        return WK_INVALID_ARGUMENT
    }

    let box: WKWebViewBox = wkBorrow(ptr)
    let (status, data, error) = box.takeSnapshotPNG()
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    guard let data else {
        outErr?.pointee = wkCString("snapshot returned nil data")
        return WK_UNKNOWN
    }

    let count = data.count
    let buffer = UnsafeMutablePointer<UInt8>.allocate(capacity: count)
    data.copyBytes(to: buffer, count: count)
    outPng?.pointee = buffer
    outLen?.pointee = count
    return status
}
