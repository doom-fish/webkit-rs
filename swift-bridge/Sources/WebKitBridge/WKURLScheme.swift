import Foundation
import WebKit

public typealias WKURLSchemeTaskCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> Void

@_silgen_name("wk_rust_release_url_scheme_handler")
func wkRustReleaseURLSchemeHandler(_ userInfo: UnsafeMutableRawPointer?)

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

final class WKRustURLSchemeTaskBox: NSObject {
    let task: any WKURLSchemeTask

    init(task: any WKURLSchemeTask) {
        self.task = task
        super.init()
    }
}

@available(macOS 10.13, *)
final class WKRustURLSchemeHandler: NSObject, WKURLSchemeHandler {
    let startCallback: WKURLSchemeTaskCallback?
    let stopCallback: WKURLSchemeTaskCallback?
    let userInfo: UnsafeMutableRawPointer?
    var activeTasks: [ObjectIdentifier: WKRustURLSchemeTaskBox] = [:]

    init(
        startCallback: WKURLSchemeTaskCallback?,
        stopCallback: WKURLSchemeTaskCallback?,
        userInfo: UnsafeMutableRawPointer?
    ) {
        self.startCallback = startCallback
        self.stopCallback = stopCallback
        self.userInfo = userInfo
        super.init()
    }

    deinit {
        wkRustReleaseURLSchemeHandler(userInfo)
    }

    private func retainedTaskPointer(for task: any WKURLSchemeTask) -> UnsafeMutableRawPointer {
        let key = ObjectIdentifier(task as AnyObject)
        let box: WKRustURLSchemeTaskBox
        if let existing = activeTasks[key] {
            box = existing
        } else {
            box = WKRustURLSchemeTaskBox(task: task)
            activeTasks[key] = box
        }
        return wkRetain(box)
    }

    func webView(_ webView: WKWebView, start urlSchemeTask: any WKURLSchemeTask) {
        let pointer = retainedTaskPointer(for: urlSchemeTask)
        let json = wkJSONString(wkURLSchemeRequestDictionary(urlSchemeTask.request))
        json.withCString { startCallback?(userInfo, pointer, $0) }
    }

    func webView(_ webView: WKWebView, stop urlSchemeTask: any WKURLSchemeTask) {
        let key = ObjectIdentifier(urlSchemeTask as AnyObject)
        let box = activeTasks.removeValue(forKey: key) ?? WKRustURLSchemeTaskBox(task: urlSchemeTask)
        let json = wkJSONString(wkURLSchemeRequestDictionary(urlSchemeTask.request))
        let pointer = wkRetain(box)
        json.withCString { stopCallback?(userInfo, pointer, $0) }
    }
}

@_cdecl("wk_config_set_url_scheme_handler")
public func wk_config_set_url_scheme_handler(
    _ ptr: UnsafeMutableRawPointer?,
    _ scheme: UnsafePointer<CChar>?,
    _ startCallback: WKURLSchemeTaskCallback?,
    _ stopCallback: WKURLSchemeTaskCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let scheme, let startCallback, let userInfo else {
        outErr?.pointee = wkCString("missing configuration, scheme, or callback")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 10.13, *) else {
        outErr?.pointee = wkCString("custom URL scheme handlers require macOS 10.13")
        return WK_UNSUPPORTED
    }

    let box: WKConfigBox = wkBorrow(ptr)
    let rawScheme = String(cString: scheme).lowercased()
    guard wkIsValidCustomURLScheme(rawScheme) else {
        outErr?.pointee = wkCString("invalid URL scheme")
        return WK_INVALID_ARGUMENT
    }
    guard !WKWebView.handlesURLScheme(rawScheme) else {
        outErr?.pointee = wkCString("WebKit already handles this URL scheme")
        return WK_INVALID_ARGUMENT
    }
    guard box.registeredURLSchemeHandlers[rawScheme] == nil else {
        outErr?.pointee = wkCString("URL scheme already registered")
        return WK_INVALID_ARGUMENT
    }

    let handler = WKRustURLSchemeHandler(
        startCallback: startCallback,
        stopCallback: stopCallback,
        userInfo: userInfo
    )
    box.config.setURLSchemeHandler(handler, forURLScheme: rawScheme)
    box.registeredURLSchemeHandlers[rawScheme] = handler
    return WK_OK
}

@_cdecl("wk_url_scheme_task_release")
public func wk_url_scheme_task_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
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

    wkOnMain {
        box.task.didReceive(urlResponse)
    }
    return WK_OK
}

@_cdecl("wk_url_scheme_task_send_data")
public func wk_url_scheme_task_send_data(
    _ ptr: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let bytes else {
        outErr?.pointee = wkCString("missing URL scheme task or data")
        return WK_INVALID_ARGUMENT
    }
    let data = Data(bytes: bytes, count: len)
    let box: WKRustURLSchemeTaskBox = wkBorrow(ptr)
    wkOnMain {
        box.task.didReceive(data)
    }
    return WK_OK
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
    wkOnMain {
        box.task.didFinish()
    }
    return WK_OK
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
    wkOnMain {
        box.task.didFailWithError(error)
    }
    return WK_OK
}
