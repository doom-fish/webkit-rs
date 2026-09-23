import Foundation
import WebKit

public typealias WKMsgCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafeMutableRawPointer?
) -> Void

public typealias WKMsgReplyCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafeMutableRawPointer?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32

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

func wkScriptMessageJSON(_ message: WKScriptMessage) -> String {
    wkJSONString([
        "name": message.name,
        "body": wkMessageBodyString(message.body),
        "frame": wkFrameInfoDictionary(message.frameInfo),
        "world": wkContentWorldDictionary(message.world)
    ])
}

private func wkReplyValue(from replyCString: UnsafeMutablePointer<CChar>?) -> Any? {
    guard let replyCString else {
        return nil
    }
    defer { free(replyCString) }
    let replyString = String(cString: replyCString)
    guard let data = replyString.data(using: .utf8) else {
        return replyString
    }
    return (try? JSONSerialization.jsonObject(with: data, options: [.fragmentsAllowed])) ?? replyString
}

private func wkReplyError(from errorCString: UnsafeMutablePointer<CChar>?) -> String? {
    guard let errorCString else {
        return nil
    }
    defer { free(errorCString) }
    return String(cString: errorCString)
}

private struct WKRustScriptMessageKey: Hashable {
    let world: String
    let name: String
}

private var wkScriptMessageRouterKey: UInt8 = 0

final class WKRustScriptMessageRouter: NSObject, WKScriptMessageHandler, WKScriptMessageHandlerWithReply {
    private final class WeakWebViewBox {
        weak var box: WKWebViewBox?

        init(_ box: WKWebViewBox) {
            self.box = box
        }
    }

    private var views: [WeakWebViewBox] = []
    private var registrations: [WKRustScriptMessageKey: Bool] = [:]

    static func router(for controller: WKUserContentController) -> WKRustScriptMessageRouter {
        if let existing = objc_getAssociatedObject(controller, &wkScriptMessageRouterKey) as? WKRustScriptMessageRouter {
            return existing
        }
        let router = WKRustScriptMessageRouter()
        objc_setAssociatedObject(controller, &wkScriptMessageRouterKey, router, .OBJC_ASSOCIATION_RETAIN_NONATOMIC)
        return router
    }

    func attach(_ box: WKWebViewBox) {
        views.removeAll { $0.box == nil }
        views.append(WeakWebViewBox(box))
    }

    func detachReleasedViews() {
        views.removeAll { $0.box == nil }
    }

    private func box(for webView: WKWebView?) -> WKWebViewBox? {
        guard let webView else {
            return nil
        }
        return views.first { $0.box?.webView === webView }?.box
    }

    func register(
        name: String,
        world: WKContentWorld,
        reply: Bool,
        on controller: WKUserContentController
    ) -> String? {
        let key = WKRustScriptMessageKey(world: wkContentWorldKey(world), name: name)
        if let existingIsReply = registrations[key] {
            let kind = existingIsReply ? "reply" : "plain"
            return "a \(kind) script message handler named '\(name)' is already registered in this content world"
        }
        if reply {
            controller.addScriptMessageHandler(self, contentWorld: world, name: name)
        } else {
            controller.add(self, contentWorld: world, name: name)
        }
        registrations[key] = reply
        return nil
    }

    func unregister(name: String, world: WKContentWorld, on controller: WKUserContentController) -> String? {
        let key = WKRustScriptMessageKey(world: wkContentWorldKey(world), name: name)
        guard registrations.removeValue(forKey: key) != nil else {
            return "no script message handler named '\(name)' is registered in this content world"
        }
        controller.removeScriptMessageHandler(forName: name, contentWorld: world)
        return nil
    }

    func userContentController(
        _ userContentController: WKUserContentController,
        didReceive message: WKScriptMessage
    ) {
        box(for: message.webView)?.deliverScriptMessage(message)
    }

    func userContentController(
        _ userContentController: WKUserContentController,
        didReceive message: WKScriptMessage,
        replyHandler: @escaping (Any?, String?) -> Void
    ) {
        guard let box = box(for: message.webView) else {
            replyHandler(nil, "no web view is attached to this script message handler")
            return
        }
        box.deliverReplyScriptMessage(message, replyHandler: replyHandler)
    }
}

extension WKWebViewBox {
    func deliverScriptMessage(_ message: WKScriptMessage) {
        let payload = wkScriptMessageJSON(message)
        scriptMessages.append(json: payload)
        guard let callback = messageCallback else {
            return
        }
        let frame = wkRetain(WKFrameInfoBox(frameInfo: message.frameInfo))
        payload.withCString { callback.function(callback.userInfo, $0, frame) }
    }

    func deliverReplyScriptMessage(_ message: WKScriptMessage, replyHandler: @escaping (Any?, String?) -> Void) {
        let payload = wkScriptMessageJSON(message)
        scriptMessages.append(json: payload)
        guard let callback = replyMessageCallback else {
            replyHandler(nil, nil)
            return
        }

        let frame = wkRetain(WKFrameInfoBox(frameInfo: message.frameInfo))
        var replyCString: UnsafeMutablePointer<CChar>?
        var errorCString: UnsafeMutablePointer<CChar>?
        let status = payload.withCString { payloadCStr in
            callback.function(callback.userInfo, payloadCStr, frame, &replyCString, &errorCString)
        }

        let errorMessage = wkReplyError(from: errorCString)
        if status == WK_OK, errorMessage == nil {
            replyHandler(wkReplyValue(from: replyCString), nil)
        } else {
            if let replyCString {
                free(replyCString)
            }
            replyHandler(nil, errorMessage ?? "script message reply failed with status \(status)")
        }
    }
}

private func wkUpdateScriptMessageHandler(
    _ ptr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ worldKind: Int32,
    _ worldName: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ update: (WKRustScriptMessageRouter, WKUserContentController, String, WKContentWorld) -> String?
) -> Int32 {
    guard let ptr, let name else {
        outErr?.pointee = wkCString("missing configuration or handler name")
        return WK_INVALID_ARGUMENT
    }
    let handlerName = String(cString: name)
    guard !handlerName.isEmpty else {
        outErr?.pointee = wkCString("script message handler names must not be empty")
        return WK_INVALID_ARGUMENT
    }
    let box: WKConfigBox = wkBorrow(ptr)
    let worldNameString = worldName.map(String.init(cString:))
    let error: String? = wkOnMain {
        guard let world = wkContentWorld(kind: worldKind, name: worldNameString) else {
            return "invalid content world"
        }
        let controller = box.config.userContentController
        return update(WKRustScriptMessageRouter.router(for: controller), controller, handlerName, world)
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return WK_INVALID_ARGUMENT
    }
    return WK_OK
}

@_cdecl("wk_config_add_script_message_handler")
public func wk_config_add_script_message_handler(
    _ ptr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ worldKind: Int32,
    _ worldName: UnsafePointer<CChar>?,
    _ reply: Bool,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkUpdateScriptMessageHandler(ptr, name, worldKind, worldName, outErr) { router, controller, handlerName, world in
        router.register(name: handlerName, world: world, reply: reply, on: controller)
    }
}

@_cdecl("wk_config_remove_script_message_handler")
public func wk_config_remove_script_message_handler(
    _ ptr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ worldKind: Int32,
    _ worldName: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    wkUpdateScriptMessageHandler(ptr, name, worldKind, worldName, outErr) { router, controller, handlerName, world in
        router.unregister(name: handlerName, world: world, on: controller)
    }
}
