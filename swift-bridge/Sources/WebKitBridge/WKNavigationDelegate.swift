import Foundation
import WebKit

public typealias WKNavCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> Void

public typealias WKNavDecisionCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> Int32

enum WKRustNavigationActionPolicy: Int32 {
    case cancel = 0
    case allow = 1
    case download = 2
}

enum WKRustNavigationResponsePolicy: Int32 {
    case cancel = 0
    case allow = 1
    case download = 2
}

enum WKRustBackForwardListNavigationPolicy: Int32 {
    case cancel = 0
    case allow = 1
}

private func wkHeaderDictionary(_ headers: [AnyHashable: Any]) -> [String: String] {
    var dictionary: [String: String] = [:]
    for (key, value) in headers {
        dictionary[String(describing: key)] = String(describing: value)
    }
    return dictionary
}

private func wkNavigationTypeString(_ navigationType: WKNavigationType) -> String {
    switch navigationType {
    case .linkActivated:
        return "linkActivated"
    case .formSubmitted:
        return "formSubmitted"
    case .backForward:
        return "backForward"
    case .reload:
        return "reload"
    case .formResubmitted:
        return "formResubmitted"
    case .other:
        return "other"
    @unknown default:
        return "other"
    }
}

private func wkNavigationActionDictionary(_ navigationAction: WKNavigationAction) -> [String: Any] {
    var dictionary: [String: Any] = [
        "navigationType": wkNavigationTypeString(navigationAction.navigationType),
        "navigationTypeRawValue": navigationAction.navigationType.rawValue,
        "requestUrl": navigationAction.request.url?.absoluteString ?? "",
        "requestMethod": navigationAction.request.httpMethod ?? "GET",
        "requestHeaders": navigationAction.request.allHTTPHeaderFields ?? [:],
        "sourceFrame": wkFrameInfoDictionary(navigationAction.sourceFrame),
        "targetFrame": navigationAction.targetFrame.map(wkFrameInfoDictionary) ?? NSNull(),
        "shouldPerformDownload": false,
        "modifierFlags": UInt64(navigationAction.modifierFlags.rawValue),
        "buttonNumber": Int64(navigationAction.buttonNumber),
        "contentRuleListRedirect": NSNull()
    ]
    if #available(macOS 11.3, *) {
        dictionary["shouldPerformDownload"] = navigationAction.shouldPerformDownload
    }
    if #available(macOS 26.0, *) {
        dictionary["contentRuleListRedirect"] = navigationAction.isContentRuleListRedirect
    }
    return dictionary
}

private func wkNavigationResponseDictionary(_ navigationResponse: WKNavigationResponse) -> [String: Any] {
    let response = navigationResponse.response
    var dictionary: [String: Any] = [
        "forMainFrame": navigationResponse.isForMainFrame,
        "url": response.url?.absoluteString ?? "",
        "mimeType": response.mimeType ?? NSNull(),
        "expectedContentLength": Int64(response.expectedContentLength),
        "textEncodingName": response.textEncodingName ?? NSNull(),
        "statusCode": NSNull(),
        "headers": [:],
        "canShowMimeType": navigationResponse.canShowMIMEType
    ]
    if let httpResponse = response as? HTTPURLResponse {
        dictionary["statusCode"] = httpResponse.statusCode
        dictionary["headers"] = wkHeaderDictionary(httpResponse.allHeaderFields)
    }
    return dictionary
}

private func wkRelativeIndex(of item: WKBackForwardListItem, in list: WKBackForwardList) -> Int {
    var backIndex = -1
    while let candidate = list.item(at: backIndex) {
        if candidate === item {
            return backIndex
        }
        backIndex -= 1
    }

    if let currentItem = list.item(at: 0), currentItem === item {
        return 0
    }

    var forwardIndex = 1
    while let candidate = list.item(at: forwardIndex) {
        if candidate === item {
            return forwardIndex
        }
        forwardIndex += 1
    }
    return 0
}

private func wkBackForwardListNavigationEvent(
    item: WKBackForwardListItem,
    in webView: WKWebView,
    willUseInstantBack: Bool
) -> [String: Any] {
    [
        "item": wkBackForwardListItemDictionary(
            item,
            relativeIndex: wkRelativeIndex(of: item, in: webView.backForwardList)
        ),
        "willUseInstantBack": willUseInstantBack
    ]
}

private func wkNavigationEvent(
    kind: String,
    url: String,
    error: String? = nil,
    navigationType: Int? = nil,
    statusCode: Int? = nil,
    navigationAction: [String: Any]? = nil,
    navigationResponse: [String: Any]? = nil
) -> [String: Any] {
    [
        "kind": kind,
        "url": url,
        "error": error ?? NSNull(),
        "navigationType": navigationType ?? NSNull(),
        "statusCode": statusCode ?? NSNull(),
        "navigationAction": navigationAction ?? NSNull(),
        "navigationResponse": navigationResponse ?? NSNull()
    ]
}

final class WKRustNavDelegate: NSObject, WKNavigationDelegate {
    weak var owner: WKWebViewBox?
    var eventCallback: WKRustCallback<WKNavCallback>?
    var backForwardListCallback: WKRustCallback<WKNavCallback>?
    var actionDecisionCallback: WKRustCallback<WKNavDecisionCallback>?
    var responseDecisionCallback: WKRustCallback<WKNavDecisionCallback>?
    let events = WKRustEventQueue()
    let backForwardListEvents = WKRustEventQueue()
    var loadDone = false
    var loadError: String?
    var actionPolicy: WKRustNavigationActionPolicy = .allow
    var responsePolicy: WKRustNavigationResponsePolicy = .allow
    var backForwardListPolicy: WKRustBackForwardListNavigationPolicy = .allow

    private func emit(_ payload: [String: Any]) {
        let json = wkJSONString(payload)
        events.append(json: json)
        if let callback = eventCallback {
            json.withCString { callback.function(callback.userInfo, $0) }
        }
    }

    private func emitBackForwardList(_ payload: [String: Any]) {
        let json = wkJSONString(payload)
        backForwardListEvents.append(json: json)
        if let callback = backForwardListCallback {
            json.withCString { callback.function(callback.userInfo, $0) }
        }
    }

    private func decide(_ callback: WKRustCallback<WKNavDecisionCallback>, _ payload: [String: Any]) -> Int32 {
        wkJSONString(payload).withCString { callback.function(callback.userInfo, $0) }
    }

    func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
        emit(wkNavigationEvent(kind: "didStartProvisional", url: webView.url?.absoluteString ?? ""))
    }

    func webView(_ webView: WKWebView, didReceiveServerRedirectForProvisionalNavigation navigation: WKNavigation!) {
        emit(wkNavigationEvent(kind: "didReceiveServerRedirect", url: webView.url?.absoluteString ?? ""))
    }

    func webView(_ webView: WKWebView, didCommit navigation: WKNavigation!) {
        emit(wkNavigationEvent(kind: "didCommit", url: webView.url?.absoluteString ?? ""))
    }

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        loadDone = true
        loadError = nil
        emit(wkNavigationEvent(kind: "didFinish", url: webView.url?.absoluteString ?? ""))
    }

    func webView(_ webView: WKWebView, didFail navigation: WKNavigation!, withError error: Error) {
        loadDone = true
        loadError = error.localizedDescription
        emit(
            wkNavigationEvent(
                kind: "didFail",
                url: webView.url?.absoluteString ?? "",
                error: error.localizedDescription
            )
        )
    }

    func webView(
        _ webView: WKWebView,
        didFailProvisionalNavigation navigation: WKNavigation!,
        withError error: Error
    ) {
        loadDone = true
        loadError = error.localizedDescription
        emit(
            wkNavigationEvent(
                kind: "didFailProvisional",
                url: webView.url?.absoluteString ?? "",
                error: error.localizedDescription
            )
        )
    }

    func webView(
        _ webView: WKWebView,
        decidePolicyFor navigationAction: WKNavigationAction,
        decisionHandler: @escaping (WKNavigationActionPolicy) -> Void
    ) {
        let action = wkNavigationActionDictionary(navigationAction)
        emit(
            wkNavigationEvent(
                kind: "decidePolicyForAction",
                url: navigationAction.request.url?.absoluteString ?? "",
                navigationType: navigationAction.navigationType.rawValue,
                navigationAction: action
            )
        )
        let policy: WKRustNavigationActionPolicy
        if let callback = actionDecisionCallback {
            policy = WKRustNavigationActionPolicy(rawValue: decide(callback, action)) ?? .cancel
        } else {
            policy = actionPolicy
        }
        switch policy {
        case .cancel:
            decisionHandler(.cancel)
        case .allow:
            decisionHandler(.allow)
        case .download:
            if #available(macOS 11.3, *) {
                decisionHandler(.download)
            } else {
                decisionHandler(.allow)
            }
        }
    }

    func webView(
        _ webView: WKWebView,
        decidePolicyFor navigationResponse: WKNavigationResponse,
        decisionHandler: @escaping (WKNavigationResponsePolicy) -> Void
    ) {
        let response = wkNavigationResponseDictionary(navigationResponse)
        emit(
            wkNavigationEvent(
                kind: "decidePolicyForResponse",
                url: navigationResponse.response.url?.absoluteString ?? "",
                statusCode: (navigationResponse.response as? HTTPURLResponse)?.statusCode,
                navigationResponse: response
            )
        )
        let policy: WKRustNavigationResponsePolicy
        if let callback = responseDecisionCallback {
            policy = WKRustNavigationResponsePolicy(rawValue: decide(callback, response)) ?? .cancel
        } else {
            policy = responsePolicy
        }
        switch policy {
        case .cancel:
            decisionHandler(.cancel)
        case .allow:
            decisionHandler(.allow)
        case .download:
            if #available(macOS 11.3, *) {
                decisionHandler(.download)
            } else {
                decisionHandler(.allow)
            }
        }
    }

    func webViewWebContentProcessDidTerminate(_ webView: WKWebView) {
        emit(wkNavigationEvent(kind: "processDidTerminate", url: webView.url?.absoluteString ?? ""))
    }

    func webView(
        _ webView: WKWebView,
        navigationAction: WKNavigationAction,
        didBecome download: WKDownload
    ) {
        emit(
            wkNavigationEvent(
                kind: "navigationActionDidBecomeDownload",
                url: navigationAction.request.url?.absoluteString ?? "",
                navigationType: navigationAction.navigationType.rawValue,
                navigationAction: wkNavigationActionDictionary(navigationAction)
            )
        )
    }

    func webView(
        _ webView: WKWebView,
        navigationResponse: WKNavigationResponse,
        didBecome download: WKDownload
    ) {
        emit(
            wkNavigationEvent(
                kind: "navigationResponseDidBecomeDownload",
                url: navigationResponse.response.url?.absoluteString ?? "",
                statusCode: (navigationResponse.response as? HTTPURLResponse)?.statusCode,
                navigationResponse: wkNavigationResponseDictionary(navigationResponse)
            )
        )
    }

    @available(macOS 26.0, *)
    func webView(
        _ webView: WKWebView,
        shouldGoTo backForwardListItem: WKBackForwardListItem,
        willUseInstantBack: Bool,
        completionHandler: @escaping (Bool) -> Void
    ) {
        emitBackForwardList(
            wkBackForwardListNavigationEvent(
                item: backForwardListItem,
                in: webView,
                willUseInstantBack: willUseInstantBack
            )
        )
        completionHandler(backForwardListPolicy == .allow)
    }
}
