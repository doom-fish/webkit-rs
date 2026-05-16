import AppKit
import Foundation
import WebKit

let WK_OK: Int32 = 0
let WK_INVALID_ARGUMENT: Int32 = -1
let WK_UNSUPPORTED: Int32 = -2
let WK_TIMED_OUT: Int32 = -3
let WK_FRAMEWORK_ERROR: Int32 = -5
let WK_UNKNOWN: Int32 = -99

@inline(__always)
func wkCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
func wkRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func wkBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as type: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
func wkRelease(_ ptr: UnsafeMutableRawPointer) {
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@inline(__always)
func wkOnMain<T>(_ work: () -> T) -> T {
    if Thread.isMainThread {
        return work()
    }
    return DispatchQueue.main.sync(execute: work)
}

func wkJSONString(_ object: Any) -> String {
    guard JSONSerialization.isValidJSONObject(object),
          let data = try? JSONSerialization.data(withJSONObject: object, options: []),
          let string = String(data: data, encoding: .utf8)
    else {
        return "null"
    }
    return string
}

func wkJSONObject(from jsonCString: UnsafePointer<CChar>?) -> Any? {
    guard let jsonCString else {
        return nil
    }
    let json = String(cString: jsonCString)
    guard let data = json.data(using: .utf8) else {
        return nil
    }
    return try? JSONSerialization.jsonObject(with: data, options: [])
}

func wkDictionaryArray(from jsonCString: UnsafePointer<CChar>?) -> [[String: Any]] {
    wkJSONObject(from: jsonCString) as? [[String: Any]] ?? []
}

func wkDrainEvents(_ events: inout [[String: Any]]) -> UnsafeMutablePointer<CChar>? {
    let drained = events
    events.removeAll()
    return wkCString(wkJSONString(drained))
}

func wkRunLoopStep() {
    _ = RunLoop.main.run(mode: .default, before: Date(timeIntervalSinceNow: 0.02))
}

func wkWaitForSemaphore(_ semaphore: DispatchSemaphore, timeoutSeconds: Double) -> Bool {
    let deadline = Date(timeIntervalSinceNow: max(0.001, timeoutSeconds))
    while true {
        if semaphore.wait(timeout: .now()) == .success {
            return true
        }
        if Date() >= deadline {
            return false
        }
        wkOnMain {
            wkRunLoopStep()
        }
    }
}

func wkWaitForAsync<T>(timeoutSeconds: Double = 30.0, _ start: (@escaping (T?, String?) -> Void) -> Void) -> (Int32, T?, String?) {
    let semaphore = DispatchSemaphore(value: 0)
    var result: T?
    var error: String?

    start { value, message in
        result = value
        error = message
        semaphore.signal()
    }

    guard wkWaitForSemaphore(semaphore, timeoutSeconds: timeoutSeconds) else {
        return (WK_TIMED_OUT, nil, "timed out after \(timeoutSeconds)s")
    }
    if let error {
        return (WK_FRAMEWORK_ERROR, nil, error)
    }
    return (WK_OK, result, nil)
}

func wkSetBytes(
    _ data: Data,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<Int>?
) {
    let buffer = UnsafeMutablePointer<UInt8>.allocate(capacity: data.count)
    data.copyBytes(to: buffer, count: data.count)
    outBytes?.pointee = buffer
    outLen?.pointee = data.count
}

func wkStringArray(from jsonCString: UnsafePointer<CChar>?) -> [String] {
    guard let jsonCString else {
        return []
    }
    let json = String(cString: jsonCString)
    guard let data = json.data(using: .utf8),
          let array = try? JSONSerialization.jsonObject(with: data, options: []) as? [String]
    else {
        return []
    }
    return array
}

func wkCookieDictionary(_ cookie: HTTPCookie) -> [String: Any] {
    var dictionary: [String: Any] = [
        "name": cookie.name,
        "value": cookie.value,
        "domain": cookie.domain,
        "path": cookie.path,
        "secure": cookie.isSecure,
        "httpOnly": cookie.isHTTPOnly,
        "sessionOnly": cookie.isSessionOnly
    ]
    if let expiresDate = cookie.expiresDate {
        dictionary["expires"] = Int(expiresDate.timeIntervalSince1970)
    } else {
        dictionary["expires"] = NSNull()
    }
    return dictionary
}

func wkBackForwardListItemDictionary(_ item: WKBackForwardListItem, relativeIndex: Int) -> [String: Any] {
    [
        "relativeIndex": relativeIndex,
        "url": item.url.absoluteString,
        "title": item.title ?? NSNull(),
        "initialURL": item.initialURL.absoluteString
    ]
}

@_cdecl("wk_string_free")
public func wk_string_free(_ ptr: UnsafeMutablePointer<CChar>?) {
    guard let ptr else { return }
    free(ptr)
}

@_cdecl("wk_bytes_free")
public func wk_bytes_free(_ ptr: UnsafeMutablePointer<UInt8>?, _ len: Int) {
    guard let ptr else { return }
    ptr.deallocate()
}

@_cdecl("wk_run_loop_pump")
public func wk_run_loop_pump(_ seconds: Double) {
    let interval = max(0.001, seconds)
    wkOnMain {
        _ = RunLoop.main.run(mode: .default, before: Date(timeIntervalSinceNow: interval))
    }
}

@_cdecl("wk_init_app")
public func wk_init_app() {
    wkOnMain {
        _ = NSApplication.shared
        NSApp.setActivationPolicy(.prohibited)
    }
}
