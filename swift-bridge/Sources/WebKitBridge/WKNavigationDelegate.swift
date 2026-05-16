import Foundation
import WebKit

public typealias WKNavCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> Void

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

private func wkEmitNavCallback(_ callback: WKNavCallback?, _ userInfo: UnsafeMutableRawPointer?, _ payload: [String: Any]) {
    let json = wkJSONString(payload)
    json.withCString { callback?(userInfo, $0) }
}

final class WKRustNavDelegate: NSObject, WKNavigationDelegate {
    weak var owner: WKWebViewBox?
    var callback: WKNavCallback?
    var userInfo: UnsafeMutableRawPointer?
    var events: [[String: Any]] = []
    var loadDone = false
    var loadError: String?
    var actionPolicy: WKRustNavigationActionPolicy = .allow
    var responsePolicy: WKRustNavigationResponsePolicy = .allow

    func drainEvents() -> UnsafeMutablePointer<CChar>? {
        wkDrainEvents(&events)
    }

    private func emit(_ payload: [String: Any]) {
        events.append(payload)
        wkEmitNavCallback(callback, userInfo, payload)
    }

    func webView(_ webView: WKWebView, didStartProvisionalNavigation navigation: WKNavigation!) {
        emit([
            "kind": "didStartProvisional",
            "url": webView.url?.absoluteString ?? ""
        ])
    }

    func webView(_ webView: WKWebView, didReceiveServerRedirectForProvisionalNavigation navigation: WKNavigation!) {
        emit([
            "kind": "didReceiveServerRedirect",
            "url": webView.url?.absoluteString ?? ""
        ])
    }

    func webView(_ webView: WKWebView, didCommit navigation: WKNavigation!) {
        emit([
            "kind": "didCommit",
            "url": webView.url?.absoluteString ?? ""
        ])
    }

    func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
        loadDone = true
        loadError = nil
        emit([
            "kind": "didFinish",
            "url": webView.url?.absoluteString ?? ""
        ])
    }

    func webView(_ webView: WKWebView, didFail navigation: WKNavigation!, withError error: Error) {
        loadDone = true
        loadError = error.localizedDescription
        emit([
            "kind": "didFail",
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
            "kind": "didFailProvisional",
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
            "kind": "decidePolicyForAction",
            "url": navigationAction.request.url?.absoluteString ?? "",
            "navigationType": navigationAction.navigationType.rawValue
        ])
        switch actionPolicy {
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
        emit([
            "kind": "decidePolicyForResponse",
            "url": navigationResponse.response.url?.absoluteString ?? "",
            "statusCode": (navigationResponse.response as? HTTPURLResponse)?.statusCode ?? 0
        ])
        switch responsePolicy {
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
        emit([
            "kind": "processDidTerminate",
            "url": webView.url?.absoluteString ?? ""
        ])
    }

    func webView(
        _ webView: WKWebView,
        navigationAction: WKNavigationAction,
        didBecome download: WKDownload
    ) {
        emit([
            "kind": "navigationActionDidBecomeDownload",
            "url": navigationAction.request.url?.absoluteString ?? ""
        ])
    }

    func webView(
        _ webView: WKWebView,
        navigationResponse: WKNavigationResponse,
        didBecome download: WKDownload
    ) {
        emit([
            "kind": "navigationResponseDidBecomeDownload",
            "url": navigationResponse.response.url?.absoluteString ?? ""
        ])
    }
}
