import Foundation
import WebKit

public typealias WKURLSchemeTaskCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> Void

private func wkURLSchemeRequestDictionary(_ request: URLRequest) -> [String: Any] {
    [
        "url": request.url?.absoluteString ?? "",
        "method": request.httpMethod ?? "GET",
        "headers": request.allHTTPHeaderFields ?? [:]
    ]
}

private func wkIsValidCustomURLScheme(_ scheme: String) -> Bool {
    let characters = CharacterSet(charactersIn: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+.-")
    guard let first = scheme.unicodeScalars.first,
          CharacterSet.letters.contains(first),
          scheme.unicodeScalars.allSatisfy({ characters.contains($0) })
    else {
        return false
    }
    return true
}

enum WKRustURLSchemeTaskOperation {
    case response
    case data
    case finish
    case fail
}

final class WKRustURLSchemeTaskBox: NSObject {
    let task: any WKURLSchemeTask
    weak var handler: WKRustURLSchemeHandler?
    var stopped = false
    var responseSent = false
    var dataSent = false
    var completed = false

    init(task: any WKURLSchemeTask, handler: WKRustURLSchemeHandler?) {
        self.task = task
        self.handler = handler
        super.init()
    }

    func violation(for operation: WKRustURLSchemeTaskOperation) -> String? {
        if stopped {
            return "WebKit stopped this URL scheme task"
        }
        if completed {
            return "this URL scheme task has already finished or failed"
        }
        switch operation {
        case .response:
            return dataSent ? "a response can't be sent after data has been sent" : nil
        case .data:
            return responseSent ? nil : "data can't be sent before a response"
        case .finish:
            return responseSent ? nil : "a URL scheme task can't finish before a response is sent"
        case .fail:
            return nil
        }
    }

    func markCompleted() {
        completed = true
        handler?.taskDidComplete(self)
    }
}

final class WKRustURLSchemeHandler: NSObject, WKURLSchemeHandler {
    private let startCallback: WKURLSchemeTaskCallback
    private let stopCallback: WKURLSchemeTaskCallback?
    private let userInfo: UnsafeMutableRawPointer
    private let release: WKContextReleaseCallback
    private var activeTasks: [ObjectIdentifier: WKRustURLSchemeTaskBox] = [:]

    init(
        startCallback: WKURLSchemeTaskCallback,
        stopCallback: WKURLSchemeTaskCallback?,
        userInfo: UnsafeMutableRawPointer,
        release: @escaping WKContextReleaseCallback
    ) {
        self.startCallback = startCallback
        self.stopCallback = stopCallback
        self.userInfo = userInfo
        self.release = release
        super.init()
    }

    deinit {
        release(userInfo)
    }

    func taskDidComplete(_ box: WKRustURLSchemeTaskBox) {
        let key = ObjectIdentifier(box.task as AnyObject)
        if activeTasks[key] === box {
            activeTasks.removeValue(forKey: key)
        }
    }

    func webView(_ webView: WKWebView, start urlSchemeTask: any WKURLSchemeTask) {
        let box = WKRustURLSchemeTaskBox(task: urlSchemeTask, handler: self)
        activeTasks[ObjectIdentifier(urlSchemeTask as AnyObject)] = box
        let json = wkJSONString(wkURLSchemeRequestDictionary(urlSchemeTask.request))
        let pointer = wkRetain(box)
        json.withCString { startCallback(userInfo, pointer, $0) }
    }

    func webView(_ webView: WKWebView, stop urlSchemeTask: any WKURLSchemeTask) {
        let key = ObjectIdentifier(urlSchemeTask as AnyObject)
        let box = activeTasks.removeValue(forKey: key) ?? WKRustURLSchemeTaskBox(task: urlSchemeTask, handler: nil)
        box.stopped = true
        guard let stopCallback else { return }
        let json = wkJSONString(wkURLSchemeRequestDictionary(urlSchemeTask.request))
        let pointer = wkRetain(box)
        json.withCString { stopCallback(userInfo, pointer, $0) }
    }
}

@_cdecl("wk_config_set_url_scheme_handler")
public func wk_config_set_url_scheme_handler(
    _ ptr: UnsafeMutableRawPointer?,
    _ scheme: UnsafePointer<CChar>?,
    _ startCallback: WKURLSchemeTaskCallback?,
    _ stopCallback: WKURLSchemeTaskCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ release: WKContextReleaseCallback?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let scheme, let startCallback, let userInfo, let release else {
        outErr?.pointee = wkCString("missing configuration, scheme, or callback")
        return WK_INVALID_ARGUMENT
    }

    let box: WKConfigBox = wkBorrow(ptr)
    let rawScheme = String(cString: scheme).lowercased()
    let error: String? = wkOnMain {
        guard wkIsValidCustomURLScheme(rawScheme) else {
            return "invalid URL scheme"
        }
        guard !WKWebView.handlesURLScheme(rawScheme) else {
            return "WebKit already handles this URL scheme"
        }
        guard box.config.urlSchemeHandler(forURLScheme: rawScheme) == nil else {
            return "URL scheme already registered"
        }
        let handler = WKRustURLSchemeHandler(
            startCallback: startCallback,
            stopCallback: stopCallback,
            userInfo: userInfo,
            release: release
        )
        box.config.setURLSchemeHandler(handler, forURLScheme: rawScheme)
        return nil
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return WK_INVALID_ARGUMENT
    }
    return WK_OK
}

@_cdecl("wk_url_scheme_task_release")
public func wk_url_scheme_task_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkReleaseOnMain(ptr)
}

@_cdecl("wk_url_scheme_task_copy_request_body")
public func wk_url_scheme_task_copy_request_body(
    _ ptr: UnsafeMutableRawPointer?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<Int>?
) -> Bool {
    guard let ptr else { return false }
    let box: WKRustURLSchemeTaskBox = wkBorrow(ptr)
    let body: Data? = wkOnMain { box.task.request.httpBody }
    guard let body else { return false }
    wkSetBytes(body, outBytes, outLen)
    return true
}

private func wkURLSchemeTaskPerform(
    _ box: WKRustURLSchemeTaskBox,
    _ operation: WKRustURLSchemeTaskOperation,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ work: () -> Void
) -> Int32 {
    let error: String? = wkOnMain {
        if let violation = box.violation(for: operation) {
            return violation
        }
        work()
        return nil
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return WK_INVALID_STATE
    }
    return WK_OK
}

@_cdecl("wk_url_scheme_task_send_response_json")
public func wk_url_scheme_task_send_response_json(
    _ ptr: UnsafeMutableRawPointer?,
    _ responseJson: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr,
          let response = wkJSONObject(from: responseJson) as? [String: Any],
          let urlString = response["url"] as? String,
          let url = URL(string: urlString),
          let mimeType = response["mimeType"] as? String
    else {
        outErr?.pointee = wkCString("invalid URL scheme response")
        return WK_INVALID_ARGUMENT
    }

    let box: WKRustURLSchemeTaskBox = wkBorrow(ptr)
    let textEncodingName = response["textEncodingName"] as? String
    let statusCode = (response["statusCode"] as? NSNumber)?.intValue ?? 200
    let headers = response["headers"] as? [String: String] ?? [:]
    let urlResponse: URLResponse
    if let httpResponse = HTTPURLResponse(
        url: url,
        statusCode: statusCode,
        httpVersion: "HTTP/1.1",
        headerFields: headers
    ) {
        urlResponse = httpResponse
    } else {
        urlResponse = URLResponse(
            url: url,
            mimeType: mimeType,
            expectedContentLength: -1,
            textEncodingName: textEncodingName
        )
    }

    return wkURLSchemeTaskPerform(box, .response, outErr) {
        box.task.didReceive(urlResponse)
        box.responseSent = true
    }
}

@_cdecl("wk_url_scheme_task_send_data")
public func wk_url_scheme_task_send_data(
    _ ptr: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, len >= 0 else {
        outErr?.pointee = wkCString("missing URL scheme task or data")
        return WK_INVALID_ARGUMENT
    }
    let data: Data
    if len == 0 {
        data = Data()
    } else if let bytes {
        data = Data(bytes: bytes, count: len)
    } else {
        outErr?.pointee = wkCString("missing URL scheme task or data")
        return WK_INVALID_ARGUMENT
    }
    let box: WKRustURLSchemeTaskBox = wkBorrow(ptr)
    return wkURLSchemeTaskPerform(box, .data, outErr) {
        box.task.didReceive(data)
        box.dataSent = true
    }
}

@_cdecl("wk_url_scheme_task_finish")
public func wk_url_scheme_task_finish(
    _ ptr: UnsafeMutableRawPointer?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing URL scheme task")
        return WK_INVALID_ARGUMENT
    }
    let box: WKRustURLSchemeTaskBox = wkBorrow(ptr)
    return wkURLSchemeTaskPerform(box, .finish, outErr) {
        box.task.didFinish()
        box.markCompleted()
    }
}

@_cdecl("wk_url_scheme_task_fail")
public func wk_url_scheme_task_fail(
    _ ptr: UnsafeMutableRawPointer?,
    _ message: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let message else {
        outErr?.pointee = wkCString("missing URL scheme task or error message")
        return WK_INVALID_ARGUMENT
    }
    let error = NSError(
        domain: NSURLErrorDomain,
        code: NSURLErrorUnknown,
        userInfo: [NSLocalizedDescriptionKey: String(cString: message)]
    )
    let box: WKRustURLSchemeTaskBox = wkBorrow(ptr)
    return wkURLSchemeTaskPerform(box, .fail, outErr) {
        box.task.didFailWithError(error)
        box.markCompleted()
    }
}
