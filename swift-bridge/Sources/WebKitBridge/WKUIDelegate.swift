import Foundation
import WebKit

final class WKRustUIDelegate: NSObject, WKUIDelegate {
    var confirmResponse = false
    var promptResponse: String?
    var events: [[String: Any]] = []

    func drainEvents() -> UnsafeMutablePointer<CChar>? {
        wkDrainEvents(&events)
    }

    private func emit(_ payload: [String: Any]) {
        events.append(payload)
    }

    func webView(
        _ webView: WKWebView,
        createWebViewWith configuration: WKWebViewConfiguration,
        for navigationAction: WKNavigationAction,
        windowFeatures: WKWindowFeatures
    ) -> WKWebView? {
        emit([
            "kind": "createWebView",
            "url": navigationAction.request.url?.absoluteString ?? ""
        ])
        return nil
    }

    func webViewDidClose(_ webView: WKWebView) {
        emit([
            "kind": "close",
            "url": webView.url?.absoluteString ?? ""
        ])
    }

    func webView(
        _ webView: WKWebView,
        runJavaScriptAlertPanelWithMessage message: String,
        initiatedByFrame frame: WKFrameInfo,
        completionHandler: @escaping () -> Void
    ) {
        emit([
            "kind": "alert",
            "message": message,
            "frameURL": frame.request.url?.absoluteString ?? ""
        ])
        completionHandler()
    }

    func webView(
        _ webView: WKWebView,
        runJavaScriptConfirmPanelWithMessage message: String,
        initiatedByFrame frame: WKFrameInfo,
        completionHandler: @escaping (Bool) -> Void
    ) {
        emit([
            "kind": "confirm",
            "message": message,
            "frameURL": frame.request.url?.absoluteString ?? "",
            "response": confirmResponse
        ])
        completionHandler(confirmResponse)
    }

    func webView(
        _ webView: WKWebView,
        runJavaScriptTextInputPanelWithPrompt prompt: String,
        defaultText: String?,
        initiatedByFrame frame: WKFrameInfo,
        completionHandler: @escaping (String?) -> Void
    ) {
        emit([
            "kind": "prompt",
            "prompt": prompt,
            "defaultText": defaultText ?? NSNull(),
            "frameURL": frame.request.url?.absoluteString ?? "",
            "response": promptResponse ?? NSNull()
        ])
        completionHandler(promptResponse)
    }

    func webView(
        _ webView: WKWebView,
        runOpenPanelWith parameters: WKOpenPanelParameters,
        initiatedByFrame frame: WKFrameInfo,
        completionHandler: @escaping ([URL]?) -> Void
    ) {
        emit([
            "kind": "openPanel",
            "frameURL": frame.request.url?.absoluteString ?? "",
            "allowsMultipleSelection": parameters.allowsMultipleSelection,
            "allowsDirectories": parameters.allowsDirectories
        ])
        completionHandler(nil)
    }

    func webView(
        _ webView: WKWebView,
        requestMediaCapturePermissionFor origin: WKSecurityOrigin,
        initiatedByFrame frame: WKFrameInfo,
        type: WKMediaCaptureType,
        decisionHandler: @escaping (WKPermissionDecision) -> Void
    ) {
        emit([
            "kind": "mediaCapturePermission",
            "frameURL": frame.request.url?.absoluteString ?? "",
            "host": origin.host,
            "type": type.rawValue
        ])
        decisionHandler(.deny)
    }
}
