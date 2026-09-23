import AppKit
import Foundation
import WebKit

let WK_OK: Int32 = 0
let WK_INVALID_ARGUMENT: Int32 = -1
let WK_UNSUPPORTED: Int32 = -2
let WK_TIMED_OUT: Int32 = -3
let WK_INVALID_STATE: Int32 = -4
let WK_FRAMEWORK_ERROR: Int32 = -5
let WK_UNKNOWN: Int32 = -99

let WK_EVENT_QUEUE_MAX_EVENTS = 1024
let WK_EVENT_QUEUE_MAX_BYTES = 4 * 1024 * 1024

public typealias WKContextRetainCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias WKContextReleaseCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

final class WKRustCallback<Function> {
    let function: Function
    let userInfo: UnsafeMutableRawPointer
    private let release: WKContextReleaseCallback

    init?(
        function: Function?,
        userInfo: UnsafeMutableRawPointer?,
        retain: WKContextRetainCallback?,
        release: WKContextReleaseCallback?
    ) {
        guard let function, let userInfo, let retain, let release else {
            return nil
        }
        self.function = function
        self.userInfo = userInfo
        self.release = release
        retain(userInfo)
    }

    deinit {
        release(userInfo)
    }
}

final class WKRustEventQueue {
    private let maxEvents: Int
    private let maxBytes: Int
    private var entries: [String] = []
    private var sizes: [Int] = []
    private var start = 0
    private var bytes = 0
    private var dropped: UInt64 = 0

    init(maxEvents: Int = WK_EVENT_QUEUE_MAX_EVENTS, maxBytes: Int = WK_EVENT_QUEUE_MAX_BYTES) {
        self.maxEvents = max(1, maxEvents)
        self.maxBytes = max(1, maxBytes)
    }

    private var count: Int {
        entries.count - start
    }

    func append(_ payload: [String: Any]) {
        append(json: wkJSONString(payload))
    }

    func append(json: String) {
        let size = json.utf8.count
        guard size <= maxBytes else {
            dropped &+= 1
            return
        }
        while count > 0, count >= maxEvents || bytes + size > maxBytes {
            dropOldest()
        }
        entries.append(json)
        sizes.append(size)
        bytes += size
    }

    private func dropOldest() {
        bytes -= sizes[start]
        entries[start] = ""
        start += 1
        dropped &+= 1
        if start >= 64, start * 2 >= entries.count {
            entries.removeFirst(start)
            sizes.removeFirst(start)
            start = 0
        }
    }

    func drain() -> UnsafeMutablePointer<CChar>? {
        let events = entries[start...].joined(separator: ",")
        let json = "{\"dropped\":\(dropped),\"events\":[\(events)]}"
        entries.removeAll()
        sizes.removeAll()
        start = 0
        bytes = 0
        dropped = 0
        return wkCString(json)
    }
}

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

func wkReleaseOnMain(_ ptr: UnsafeMutableRawPointer) {
    if Thread.isMainThread {
        wkRelease(ptr)
        return
    }
    let address = UInt(bitPattern: ptr)
    DispatchQueue.main.async {
        if let pointer = UnsafeMutableRawPointer(bitPattern: address) {
            wkRelease(pointer)
        }
    }
}

func wkSecurityOriginDictionary(_ origin: WKSecurityOrigin) -> [String: Any] {
    [
        "protocol": origin.protocol,
        "host": origin.host,
        "port": origin.port
    ]
}

func wkFrameInfoDictionary(_ frameInfo: WKFrameInfo) -> [String: Any] {
    [
        "mainFrame": frameInfo.isMainFrame,
        "requestUrl": frameInfo.request.url?.absoluteString ?? "",
        "requestMethod": frameInfo.request.httpMethod ?? "GET",
        "securityOrigin": wkSecurityOriginDictionary(frameInfo.securityOrigin),
        "webviewUrl": frameInfo.webView?.url?.absoluteString ?? NSNull()
    ]
}

func wkContentWorldDictionary(_ world: WKContentWorld) -> [String: Any] {
    if world === WKContentWorld.page {
        return ["kind": "page"]
    }
    if world === WKContentWorld.defaultClient {
        return ["kind": "defaultClient"]
    }
    return ["kind": "named", "name": world.name ?? ""]
}

func wkContentWorldKey(_ world: WKContentWorld) -> String {
    if world === WKContentWorld.page {
        return "page"
    }
    if world === WKContentWorld.defaultClient {
        return "defaultClient"
    }
    return "named:" + (world.name ?? "")
}

func wkContentWorld(kind: Int32, name: String?) -> WKContentWorld? {
    switch kind {
    case 0:
        return .page
    case 1:
        return .defaultClient
    case 2:
        guard let name, !name.isEmpty else {
            return nil
        }
        return .world(name: name)
    default:
        return nil
    }
}

final class WKFrameInfoBox: NSObject {
    let frameInfo: WKFrameInfo

    init(frameInfo: WKFrameInfo) {
        self.frameInfo = frameInfo
        super.init()
    }
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

func wkJavaScriptArguments(from jsonCString: UnsafePointer<CChar>?) -> [String: Any]? {
    guard let jsonCString else {
        return [:]
    }
    return wkJSONObject(from: jsonCString) as? [String: Any]
}

func wkDictionaryArray(from jsonCString: UnsafePointer<CChar>?) -> [[String: Any]] {
    wkJSONObject(from: jsonCString) as? [[String: Any]] ?? []
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
    if let expiresDate = cookie.expiresDate, expiresDate.timeIntervalSince1970.isFinite {
        let seconds = expiresDate.timeIntervalSince1970.rounded(.down)
        dictionary["expires"] = seconds >= 9.2e18 ? Int64.max : (seconds <= -9.2e18 ? Int64.min : Int64(seconds))
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

@_cdecl("wk_pointer_array_free")
public func wk_pointer_array_free(_ ptr: UnsafeMutablePointer<UnsafeMutableRawPointer?>?) {
    guard let ptr else { return }
    ptr.deallocate()
}

@_cdecl("wk_frame_info_retain")
public func wk_frame_info_retain(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let ptr else { return nil }
    _ = Unmanaged<AnyObject>.fromOpaque(ptr).retain()
    return ptr
}

@_cdecl("wk_frame_info_release")
public func wk_frame_info_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkReleaseOnMain(ptr)
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
