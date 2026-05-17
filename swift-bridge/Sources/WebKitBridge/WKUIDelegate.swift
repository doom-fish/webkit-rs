import Foundation
import WebKit

private func wkWindowFeaturesDictionary(_ windowFeatures: WKWindowFeatures) -> [String: Any] {
    [
        "menuBarVisibility": windowFeatures.menuBarVisibility ?? NSNull(),
        "statusBarVisibility": windowFeatures.statusBarVisibility ?? NSNull(),
        "toolbarsVisibility": windowFeatures.toolbarsVisibility ?? NSNull(),
        "allowsResizing": windowFeatures.allowsResizing ?? NSNull(),
        "x": windowFeatures.x ?? NSNull(),
        "y": windowFeatures.y ?? NSNull(),
        "width": windowFeatures.width ?? NSNull(),
        "height": windowFeatures.height ?? NSNull()
    ]
}

private func wkOpenPanelParametersDictionary(_ parameters: WKOpenPanelParameters) -> [String: Any] {
    [
        "allowsMultipleSelection": parameters.allowsMultipleSelection,
        "allowsDirectories": parameters.allowsDirectories
    ]
}

private func wkSecurityOriginDictionary(_ origin: WKSecurityOrigin) -> [String: Any] {
    [
        "protocol": origin.protocol,
        "host": origin.host,
        "port": origin.port
    ]
}

private func wkMediaCaptureTypeString(_ type: WKMediaCaptureType) -> String {
    switch type {
    case .camera:
        return "camera"
    case .microphone:
        return "microphone"
    case .cameraAndMicrophone:
        return "cameraAndMicrophone"
    @unknown default:
        return "camera"
    }
}

private func wkPermissionDecisionString(_ decision: WKPermissionDecision) -> String {
    switch decision {
    case .prompt:
        return "prompt"
    case .grant:
        return "grant"
    case .deny:
        return "deny"
    @unknown default:
        return "prompt"
    }
}

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
            "frameURL": navigationAction.request.url?.absoluteString ?? "",
            "windowFeatures": wkWindowFeaturesDictionary(windowFeatures)
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
            "allowsDirectories": parameters.allowsDirectories,
            "openPanelParameters": wkOpenPanelParametersDictionary(parameters)
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
        let decision: WKPermissionDecision = .deny
        emit([
            "kind": "mediaCapturePermission",
            "frameURL": frame.request.url?.absoluteString ?? "",
            "host": origin.host,
            "type": type.rawValue,
            "securityOrigin": wkSecurityOriginDictionary(origin),
            "mediaCaptureType": wkMediaCaptureTypeString(type),
            "permissionDecision": wkPermissionDecisionString(decision)
        ])
        decisionHandler(decision)
    }
}
