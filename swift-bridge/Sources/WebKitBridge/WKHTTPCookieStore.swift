import Foundation
import WebKit

private func wkMakeCookie(from dictionary: [String: Any]) -> HTTPCookie? {
    guard let name = dictionary["name"] as? String,
          let value = dictionary["value"] as? String,
          let domain = dictionary["domain"] as? String
    else {
        return nil
    }

    var properties: [HTTPCookiePropertyKey: Any] = [
        .name: name,
        .value: value,
        .domain: domain,
        .path: dictionary["path"] as? String ?? "/",
        .version: "0"
    ]
    if (dictionary["secure"] as? Bool) == true {
        properties[.secure] = "TRUE"
    }
    if (dictionary["httpOnly"] as? Bool) == true {
        properties[HTTPCookiePropertyKey(rawValue: "HttpOnly")] = "TRUE"
    }
    if (dictionary["sessionOnly"] as? Bool) == true {
        properties[.discard] = "TRUE"
    }
    if let expires = dictionary["expires"] as? NSNumber {
        properties[.expires] = Date(timeIntervalSince1970: expires.doubleValue)
    }
    return HTTPCookie(properties: properties)
}

final class WKRustCookieStoreObserver: NSObject, WKHTTPCookieStoreObserver {
    var events: [[String: Any]] = []

    func cookiesDidChange(in cookieStore: WKHTTPCookieStore) {
        events.append(["kind": "cookiesDidChange"])
    }

    func drainEvents() -> UnsafeMutablePointer<CChar>? {
        wkDrainEvents(&events)
    }
}

final class WKHTTPCookieStoreBox: NSObject {
    let cookieStore: WKHTTPCookieStore
    let observer = WKRustCookieStoreObserver()
    private(set) var observing = false

    init(cookieStore: WKHTTPCookieStore) {
        self.cookieStore = cookieStore
        super.init()
    }

    func setObserving(_ observing: Bool) {
        guard observing != self.observing else {
            return
        }
        if observing {
            cookieStore.add(observer)
        } else {
            cookieStore.remove(observer)
        }
        self.observing = observing
    }

    func drainEvents() -> UnsafeMutablePointer<CChar>? {
        observer.drainEvents()
    }

    deinit {
        if observing {
            cookieStore.remove(observer)
        }
    }
}

@_cdecl("wk_http_cookie_store_release")
public func wk_http_cookie_store_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_http_cookie_store_copy_all_cookies_json")
public func wk_http_cookie_store_copy_all_cookies_json(
    _ ptr: UnsafeMutableRawPointer?,
    _ outJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing cookie store")
        return WK_INVALID_ARGUMENT
    }
    let box: WKHTTPCookieStoreBox = wkBorrow(ptr)
    let (status, cookies, error): (Int32, [[String: Any]]?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.cookieStore.getAllCookies { cookies in
                completion(cookies.map(wkCookieDictionary), nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    outJson?.pointee = wkCString(wkJSONString(cookies ?? []))
    return status
}

@_cdecl("wk_http_cookie_store_set_cookie")
public func wk_http_cookie_store_set_cookie(
    _ ptr: UnsafeMutableRawPointer?,
    _ cookieJSON: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing cookie store")
        return WK_INVALID_ARGUMENT
    }
    guard let dictionary = wkJSONObject(from: cookieJSON) as? [String: Any],
          let cookie = wkMakeCookie(from: dictionary)
    else {
        outErr?.pointee = wkCString("invalid cookie payload")
        return WK_INVALID_ARGUMENT
    }
    let box: WKHTTPCookieStoreBox = wkBorrow(ptr)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.cookieStore.setCookie(cookie) {
                completion(true, nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_http_cookie_store_set_cookies")
public func wk_http_cookie_store_set_cookies(
    _ ptr: UnsafeMutableRawPointer?,
    _ cookiesJSON: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing cookie store")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 26.0, *) else {
        outErr?.pointee = wkCString("setCookies(_:) requires macOS 26.0+")
        return WK_UNSUPPORTED
    }
    let cookies = wkDictionaryArray(from: cookiesJSON).compactMap(wkMakeCookie)
    let box: WKHTTPCookieStoreBox = wkBorrow(ptr)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.cookieStore.setCookies(cookies) {
                completion(true, nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_http_cookie_store_delete_cookie")
public func wk_http_cookie_store_delete_cookie(
    _ ptr: UnsafeMutableRawPointer?,
    _ cookieJSON: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing cookie store")
        return WK_INVALID_ARGUMENT
    }
    guard let dictionary = wkJSONObject(from: cookieJSON) as? [String: Any],
          let cookie = wkMakeCookie(from: dictionary)
    else {
        outErr?.pointee = wkCString("invalid cookie payload")
        return WK_INVALID_ARGUMENT
    }
    let box: WKHTTPCookieStoreBox = wkBorrow(ptr)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.cookieStore.delete(cookie) {
                completion(true, nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_http_cookie_store_set_observing")
public func wk_http_cookie_store_set_observing(_ ptr: UnsafeMutableRawPointer?, _ observing: Bool) {
    guard let ptr else { return }
    let box: WKHTTPCookieStoreBox = wkBorrow(ptr)
    box.setObserving(observing)
}

@_cdecl("wk_http_cookie_store_drain_events_json")
public func wk_http_cookie_store_drain_events_json(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return wkCString("[]") }
    let box: WKHTTPCookieStoreBox = wkBorrow(ptr)
    return box.drainEvents()
}

@_cdecl("wk_http_cookie_store_set_cookie_policy")
public func wk_http_cookie_store_set_cookie_policy(
    _ ptr: UnsafeMutableRawPointer?,
    _ policyRawValue: Int32,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing cookie store")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("setCookiePolicy requires macOS 14.0+")
        return WK_UNSUPPORTED
    }
    guard let policy = WKHTTPCookieStore.CookiePolicy(rawValue: Int(policyRawValue)) else {
        outErr?.pointee = wkCString("invalid cookie policy")
        return WK_INVALID_ARGUMENT
    }
    let box: WKHTTPCookieStoreBox = wkBorrow(ptr)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.cookieStore.setCookiePolicy(policy) {
                completion(true, nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_http_cookie_store_get_cookie_policy")
public func wk_http_cookie_store_get_cookie_policy(
    _ ptr: UnsafeMutableRawPointer?,
    _ outPolicy: UnsafeMutablePointer<Int32>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing cookie store")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("getCookiePolicy requires macOS 14.0+")
        return WK_UNSUPPORTED
    }
    let box: WKHTTPCookieStoreBox = wkBorrow(ptr)
    let (status, policy, error): (Int32, Int32?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.cookieStore.getCookiePolicy { policy in
                completion(Int32(policy.rawValue), nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    outPolicy?.pointee = policy ?? Int32(WKHTTPCookieStore.CookiePolicy.allow.rawValue)
    return status
}
