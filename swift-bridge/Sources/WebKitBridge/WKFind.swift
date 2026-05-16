import Foundation
import WebKit

private func wkFindConfiguration(from jsonCString: UnsafePointer<CChar>?) -> WKFindConfiguration {
    let configuration = WKFindConfiguration()
    guard let dictionary = wkJSONObject(from: jsonCString) as? [String: Any] else {
        return configuration
    }
    if let backwards = dictionary["backwards"] as? Bool {
        configuration.backwards = backwards
    }
    if let caseSensitive = dictionary["caseSensitive"] as? Bool {
        configuration.caseSensitive = caseSensitive
    }
    if let wraps = dictionary["wraps"] as? Bool {
        configuration.wraps = wraps
    }
    return configuration
}

@_cdecl("wk_webview_find_string")
public func wk_webview_find_string(
    _ ptr: UnsafeMutableRawPointer?,
    _ query: UnsafePointer<CChar>?,
    _ configurationJson: UnsafePointer<CChar>?,
    _ outResult: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let query else {
        outErr?.pointee = wkCString("missing webview or query")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let searchString = String(cString: query)
    let configuration = wkFindConfiguration(from: configurationJson)

    let (status, result, error): (Int32, WKFindResult?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.webView.find(searchString, configuration: configuration) { result in
                completion(result, nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    if let result {
        outResult?.pointee = wkCString(wkJSONString(["matchFound": result.matchFound]))
    } else {
        outResult?.pointee = wkCString("{\"matchFound\":false}")
    }
    return status
}
