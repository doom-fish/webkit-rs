import Foundation
import WebKit

public typealias WKMsgCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?,
    UnsafePointer<CChar>?
) -> Void

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
