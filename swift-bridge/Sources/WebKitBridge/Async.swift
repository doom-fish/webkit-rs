import AppKit
import Foundation
import WebKit

private func wkAsyncBytes(
    _ data: Data,
    _ cb: @convention(c) (UnsafePointer<UInt8>?, Int, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    _ ctx: UnsafeMutableRawPointer
) {
    data.withUnsafeBytes { buf in
        cb(buf.baseAddress?.assumingMemoryBound(to: UInt8.self), data.count, nil, ctx)
    }
}

@_cdecl("wk_webview_evaluate_js_async")
public func wk_webview_evaluate_js_async(
    _ ptr: UnsafeMutableRawPointer?,
    _ js: UnsafePointer<CChar>?,
    cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr, let js else {
        "missing webview or js".withCString { cb(nil, $0, ctx) }
        return
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let script = String(cString: js)
    DispatchQueue.main.async {
        box.webView.evaluateJavaScript(script) { result, error in
            if let error {
                error.localizedDescription.withCString { cb(nil, $0, ctx) }
                return
            }
            let str: String
            if let s = result as? String {
                str = s
            } else if let n = result as? NSNumber {
                str = n.stringValue
            } else if let result,
                      JSONSerialization.isValidJSONObject(result),
                      let data = try? JSONSerialization.data(withJSONObject: result),
                      let s = String(data: data, encoding: .utf8)
            {
                str = s
            } else {
                str = ""
            }
            str.withCString { cb($0, nil, ctx) }
        }
    }
}

@_cdecl("wk_webview_call_async_js_async")
public func wk_webview_call_async_js_async(
    _ ptr: UnsafeMutableRawPointer?,
    _ js: UnsafePointer<CChar>?,
    cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr, let js else {
        "missing webview or js".withCString { cb(nil, $0, ctx) }
        return
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let script = String(cString: js)
    DispatchQueue.main.async {
        box.webView.callAsyncJavaScript(script, arguments: [:], in: nil, in: .page) { result in
            switch result {
            case let .success(value):
                let str: String
                if let s = value as? String {
                    str = s
                } else if let n = value as? NSNumber {
                    str = n.stringValue
                } else if JSONSerialization.isValidJSONObject(value),
                          let data = try? JSONSerialization.data(withJSONObject: value),
                          let s = String(data: data, encoding: .utf8)
                {
                    str = s
                } else {
                    str = ""
                }
                str.withCString { cb($0, nil, ctx) }
            case let .failure(error):
                error.localizedDescription.withCString { cb(nil, $0, ctx) }
            }
        }
    }
}

@_cdecl("wk_webview_take_snapshot_async")
public func wk_webview_take_snapshot_async(
    _ ptr: UnsafeMutableRawPointer?,
    _ hasRect: Bool,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ hasSnapshotWidth: Bool,
    _ snapshotWidth: Double,
    _ afterScreenUpdates: Bool,
    cb: @convention(c) (UnsafePointer<UInt8>?, Int, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr else {
        "missing webview".withCString { cb(nil, 0, $0, ctx) }
        return
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let configuration = wkMakeSnapshotConfiguration(
        hasRect: hasRect,
        x: x,
        y: y,
        width: width,
        height: height,
        hasSnapshotWidth: hasSnapshotWidth,
        snapshotWidth: snapshotWidth,
        afterScreenUpdates: afterScreenUpdates
    )
    DispatchQueue.main.async {
        box.webView.takeSnapshot(with: configuration) { image, error in
            if let error {
                error.localizedDescription.withCString { cb(nil, 0, $0, ctx) }
                return
            }
            guard let image,
                  let tiffData = image.tiffRepresentation,
                  let bitmap = NSBitmapImageRep(data: tiffData),
                  let pngData = bitmap.representation(using: .png, properties: [:])
            else {
                "failed to encode snapshot as PNG".withCString { cb(nil, 0, $0, ctx) }
                return
            }
            wkAsyncBytes(pngData, cb, ctx)
        }
    }
}

@_cdecl("wk_webview_create_pdf_async")
public func wk_webview_create_pdf_async(
    _ ptr: UnsafeMutableRawPointer?,
    _ hasRect: Bool,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ allowTransparentBackground: Bool,
    cb: @convention(c) (UnsafePointer<UInt8>?, Int, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr else {
        "missing webview".withCString { cb(nil, 0, $0, ctx) }
        return
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let configuration = wkMakePDFConfiguration(
        hasRect: hasRect,
        x: x,
        y: y,
        width: width,
        height: height,
        allowTransparentBackground: allowTransparentBackground
    )
    DispatchQueue.main.async {
        box.webView.createPDF(configuration: configuration) { result in
            switch result {
            case let .success(data):
                wkAsyncBytes(data, cb, ctx)
            case let .failure(error):
                error.localizedDescription.withCString { cb(nil, 0, $0, ctx) }
            }
        }
    }
}

@_cdecl("wk_webview_create_web_archive_async")
public func wk_webview_create_web_archive_async(
    _ ptr: UnsafeMutableRawPointer?,
    cb: @convention(c) (UnsafePointer<UInt8>?, Int, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr else {
        "missing webview".withCString { cb(nil, 0, $0, ctx) }
        return
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    DispatchQueue.main.async {
        box.webView.createWebArchiveData { result in
            switch result {
            case let .success(data):
                wkAsyncBytes(data, cb, ctx)
            case let .failure(error):
                error.localizedDescription.withCString { cb(nil, 0, $0, ctx) }
            }
        }
    }
}

@_cdecl("wk_webview_find_string_async")
public func wk_webview_find_string_async(
    _ ptr: UnsafeMutableRawPointer?,
    _ query: UnsafePointer<CChar>?,
    _ configurationJson: UnsafePointer<CChar>?,
    cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr, let query else {
        "missing webview or query".withCString { cb(nil, $0, ctx) }
        return
    }
    let box: WKWebViewBox = wkBorrow(ptr)
    let queryString = String(cString: query)
    let configuration = wkMakeFindConfiguration(from: configurationJson)
    DispatchQueue.main.async {
        box.webView.find(queryString, configuration: configuration) { result in
            let payload: [String: Any] = [
                "matchFound": result.matchFound,
                "matchesFound": result.matchFound
            ]
            wkJSONString(payload).withCString { cb($0, nil, ctx) }
        }
    }
}

@_cdecl("wk_website_data_store_fetch_data_records_async")
public func wk_website_data_store_fetch_data_records_async(
    _ ptr: UnsafeMutableRawPointer?,
    _ dataTypesJson: UnsafePointer<CChar>?,
    cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr else {
        "missing data store".withCString { cb(nil, $0, ctx) }
        return
    }
    let box: WKWebsiteDataStoreBox = wkBorrow(ptr)
    let dataTypes = Set(wkStringArray(from: dataTypesJson))
    DispatchQueue.main.async {
        box.dataStore.fetchDataRecords(ofTypes: dataTypes) { records in
            let payload = records.map { record in
                [
                    "displayName": record.displayName,
                    "dataTypes": Array(record.dataTypes).sorted()
                ] as [String: Any]
            }
            wkJSONString(payload).withCString { cb($0, nil, ctx) }
        }
    }
}

@_cdecl("wk_website_data_store_remove_data_async")
public func wk_website_data_store_remove_data_async(
    _ ptr: UnsafeMutableRawPointer?,
    _ dataTypesJson: UnsafePointer<CChar>?,
    _ displayNamesJson: UnsafePointer<CChar>?,
    cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr else {
        "missing data store".withCString { cb(nil, $0, ctx) }
        return
    }
    let box: WKWebsiteDataStoreBox = wkBorrow(ptr)
    let dataTypes = Set(wkStringArray(from: dataTypesJson))
    let displayNames = Set(wkStringArray(from: displayNamesJson))
    DispatchQueue.main.async {
        box.dataStore.fetchDataRecords(ofTypes: dataTypes) { records in
            let toRemove = records.filter { displayNames.contains($0.displayName) }
            box.dataStore.removeData(ofTypes: dataTypes, for: toRemove) {
                "".withCString { cb($0, nil, ctx) }
            }
        }
    }
}

@_cdecl("wk_http_cookie_store_get_all_cookies_async")
public func wk_http_cookie_store_get_all_cookies_async(
    _ ptr: UnsafeMutableRawPointer?,
    cb: @convention(c) (UnsafePointer<CChar>?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr else {
        "missing cookie store".withCString { cb(nil, $0, ctx) }
        return
    }
    let box: WKHTTPCookieStoreBox = wkBorrow(ptr)
    DispatchQueue.main.async {
        box.cookieStore.getAllCookies { cookies in
            let payload = cookies.map(wkCookieDictionary)
            wkJSONString(payload).withCString { cb($0, nil, ctx) }
        }
    }
}

@_cdecl("wk_content_rule_list_store_compile_async")
public func wk_content_rule_list_store_compile_async(
    _ ptr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ encodedRuleList: UnsafePointer<CChar>?,
    cb: @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr, let identifier, let encodedRuleList else {
        "missing arguments".withCString { cb(nil, $0, ctx) }
        return
    }
    let box: WKContentRuleListStoreBox = wkBorrow(ptr)
    let identifierString = String(cString: identifier)
    let ruleListString = String(cString: encodedRuleList)
    DispatchQueue.main.async {
        box.store.compileContentRuleList(
            forIdentifier: identifierString,
            encodedContentRuleList: ruleListString
        ) { ruleList, error in
            if let error {
                error.localizedDescription.withCString { cb(nil, $0, ctx) }
                return
            }
            guard let ruleList else {
                "compile returned no rule list".withCString { cb(nil, $0, ctx) }
                return
            }
            cb(wkRetain(WKContentRuleListBox(ruleList: ruleList)), nil, ctx)
        }
    }
}

@_cdecl("wk_download_cancel_async")
public func wk_download_cancel_async(
    _ ptr: UnsafeMutableRawPointer?,
    cb: @convention(c) (UnsafePointer<UInt8>?, Int, UnsafePointer<CChar>?, UnsafeMutableRawPointer) -> Void,
    ctx: UnsafeMutableRawPointer
) {
    guard let ptr else {
        "missing download".withCString { cb(nil, 0, $0, ctx) }
        return
    }
    let box: WKDownloadBox = wkBorrow(ptr)
    DispatchQueue.main.async {
        box.download.cancel { resumeData in
            wkAsyncBytes(resumeData ?? Data(), cb, ctx)
        }
    }
}
