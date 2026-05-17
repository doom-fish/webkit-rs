import Foundation
import WebKit

public typealias WKMsgCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?
) -> Void

public typealias WKMsgReplyCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?,
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

private func wkScriptMessageDictionary(_ message: WKScriptMessage) -> [String: Any] {
    var dictionary: [String: Any] = [
        "name": message.name,
        "body": wkMessageBodyString(message.body),
        "frameURL": message.frameInfo.request.url?.absoluteString ?? "",
        "isMainFrame": message.frameInfo.isMainFrame
    ]
    if #available(macOS 11.0, *) {
        dictionary["world"] = message.world.name
    }
    return dictionary
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
    return (try? JSONSerialization.jsonObject(with: data, options: [])) ?? replyString
}

private func wkReplyError(from errorCString: UnsafeMutablePointer<CChar>?) -> String? {
    guard let errorCString else {
        return nil
    }
    defer { free(errorCString) }
    return String(cString: errorCString)
}

final class WKRustMessageHandler: NSObject, WKScriptMessageHandler {
    var callback: WKMsgCallback?
    var userInfo: UnsafeMutableRawPointer?
    var events: [[String: Any]] = []

    func drainEvents() -> UnsafeMutablePointer<CChar>? {
        wkDrainEvents(&events)
    }

    func userContentController(
        _ userContentController: WKUserContentController,
        didReceive message: WKScriptMessage
    ) {
        let event = wkScriptMessageDictionary(message)
        events.append(event)

        let name = message.name
        let body = wkMessageBodyString(message.body)
        name.withCString { nameCStr in
            body.withCString { bodyCStr in
                callback?(userInfo, nameCStr, bodyCStr)
            }
        }
    }
}

@available(macOS 11.0, *)
final class WKRustReplyMessageHandler: NSObject, WKScriptMessageHandlerWithReply {
    var callback: WKMsgReplyCallback?
    var userInfo: UnsafeMutableRawPointer?
    var events: [[String: Any]] = []

    func drainEvents() -> UnsafeMutablePointer<CChar>? {
        wkDrainEvents(&events)
    }

    func userContentController(
        _ userContentController: WKUserContentController,
        didReceive message: WKScriptMessage,
        replyHandler: @escaping (Any?, String?) -> Void
    ) {
        let event = wkScriptMessageDictionary(message)
        events.append(event)

        guard let callback else {
            replyHandler(nil, nil)
            return
        }

        let name = message.name
        let body = wkMessageBodyString(message.body)
        var replyCString: UnsafeMutablePointer<CChar>?
        var errorCString: UnsafeMutablePointer<CChar>?
        let status = name.withCString { nameCStr in
            body.withCString { bodyCStr in
                callback(userInfo, nameCStr, bodyCStr, &replyCString, &errorCString)
            }
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
