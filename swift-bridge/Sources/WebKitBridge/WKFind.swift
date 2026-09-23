import Foundation
import WebKit

func wkMakeFindConfiguration(from dictionary: [String: Any]?) -> WKFindConfiguration {
    let configuration = WKFindConfiguration()
    guard let dictionary else {
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
    let configurationDictionary = wkJSONObject(from: configurationJson) as? [String: Any]

    let (status, matchFound, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            let configuration = wkMakeFindConfiguration(from: configurationDictionary)
            box.webView.find(searchString, configuration: configuration) { result in
                completion(result.matchFound, nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    if let matchFound {
        outResult?.pointee = wkCString(wkJSONString(["matchFound": matchFound]))
    } else {
        outResult?.pointee = wkCString("{\"matchFound\":false}")
    }
    return status
}
