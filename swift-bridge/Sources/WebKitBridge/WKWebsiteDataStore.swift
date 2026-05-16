import Foundation
import WebKit

private func wkWebsiteDataRecordDictionary(_ record: WKWebsiteDataRecord) -> [String: Any] {
    [
        "displayName": record.displayName,
        "dataTypes": Array(record.dataTypes).sorted()
    ]
}

private func wkWebsiteDataTypes(from jsonCString: UnsafePointer<CChar>?) -> Set<String> {
    Set(wkStringArray(from: jsonCString))
}

final class WKWebsiteDataStoreBox: NSObject {
    let dataStore: WKWebsiteDataStore

    init(dataStore: WKWebsiteDataStore) {
        self.dataStore = dataStore
        super.init()
    }
}

@_cdecl("wk_website_data_store_default")
public func wk_website_data_store_default() -> UnsafeMutableRawPointer {
    wkRetain(WKWebsiteDataStoreBox(dataStore: WKWebsiteDataStore.default()))
}

@_cdecl("wk_website_data_store_nonpersistent")
public func wk_website_data_store_nonpersistent() -> UnsafeMutableRawPointer {
    wkRetain(WKWebsiteDataStoreBox(dataStore: WKWebsiteDataStore.nonPersistent()))
}

@_cdecl("wk_website_data_store_for_identifier")
public func wk_website_data_store_for_identifier(
    _ identifier: UnsafePointer<CChar>?,
    _ outStore: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let identifier else {
        outErr?.pointee = wkCString("missing data store identifier")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("dataStore(forIdentifier:) requires macOS 14.0+")
        return WK_UNSUPPORTED
    }
    guard let uuid = UUID(uuidString: String(cString: identifier)) else {
        outErr?.pointee = wkCString("invalid data store identifier")
        return WK_INVALID_ARGUMENT
    }
    outStore?.pointee = wkRetain(WKWebsiteDataStoreBox(dataStore: WKWebsiteDataStore(forIdentifier: uuid)))
    return WK_OK
}

@_cdecl("wk_website_data_store_release")
public func wk_website_data_store_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_website_data_store_is_persistent")
public func wk_website_data_store_is_persistent(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return false }
    let box: WKWebsiteDataStoreBox = wkBorrow(ptr)
    return box.dataStore.isPersistent
}

@_cdecl("wk_website_data_store_copy_identifier")
public func wk_website_data_store_copy_identifier(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    let box: WKWebsiteDataStoreBox = wkBorrow(ptr)
    if #available(macOS 14.0, *), let identifier = box.dataStore.identifier {
        return wkCString(identifier.uuidString)
    }
    return nil
}

@_cdecl("wk_website_data_store_copy_all_data_types_json")
public func wk_website_data_store_copy_all_data_types_json() -> UnsafeMutablePointer<CChar>? {
    wkCString(wkJSONString(Array(WKWebsiteDataStore.allWebsiteDataTypes()).sorted()))
}

@_cdecl("wk_website_data_store_fetch_all_identifiers_json")
public func wk_website_data_store_fetch_all_identifiers_json(
    _ outJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("fetchAllDataStoreIdentifiers requires macOS 14.0+")
        return WK_UNSUPPORTED
    }
    let (status, identifiers, error): (Int32, [String]?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            WKWebsiteDataStore.fetchAllDataStoreIdentifiers { identifiers in
                completion(identifiers.map(\.uuidString), nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    outJson?.pointee = wkCString(wkJSONString(identifiers ?? []))
    return status
}

@_cdecl("wk_website_data_store_remove_data_store_for_identifier")
public func wk_website_data_store_remove_data_store_for_identifier(
    _ identifier: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let identifier else {
        outErr?.pointee = wkCString("missing data store identifier")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 14.0, *) else {
        outErr?.pointee = wkCString("remove(forIdentifier:) requires macOS 14.0+")
        return WK_UNSUPPORTED
    }
    guard let uuid = UUID(uuidString: String(cString: identifier)) else {
        outErr?.pointee = wkCString("invalid data store identifier")
        return WK_INVALID_ARGUMENT
    }
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            WKWebsiteDataStore.remove(forIdentifier: uuid) { maybeError in
                completion(maybeError == nil, maybeError?.localizedDescription)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_website_data_store_copy_http_cookie_store")
public func wk_website_data_store_copy_http_cookie_store(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let ptr else { return nil }
    let box: WKWebsiteDataStoreBox = wkBorrow(ptr)
    return wkRetain(WKHTTPCookieStoreBox(cookieStore: box.dataStore.httpCookieStore))
}

@_cdecl("wk_website_data_store_fetch_data_records_json")
public func wk_website_data_store_fetch_data_records_json(
    _ ptr: UnsafeMutableRawPointer?,
    _ dataTypesJSON: UnsafePointer<CChar>?,
    _ outJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing website data store")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebsiteDataStoreBox = wkBorrow(ptr)
    let dataTypes = wkWebsiteDataTypes(from: dataTypesJSON)
    let (status, records, error): (Int32, [[String: Any]]?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.dataStore.fetchDataRecords(ofTypes: dataTypes) { records in
                completion(records.map(wkWebsiteDataRecordDictionary), nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    outJson?.pointee = wkCString(wkJSONString(records ?? []))
    return status
}

@_cdecl("wk_website_data_store_remove_data_for_display_names")
public func wk_website_data_store_remove_data_for_display_names(
    _ ptr: UnsafeMutableRawPointer?,
    _ dataTypesJSON: UnsafePointer<CChar>?,
    _ displayNamesJSON: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing website data store")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebsiteDataStoreBox = wkBorrow(ptr)
    let dataTypes = wkWebsiteDataTypes(from: dataTypesJSON)
    let displayNames = Set(wkStringArray(from: displayNamesJSON))
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.dataStore.fetchDataRecords(ofTypes: dataTypes) { records in
                let recordsToRemove = records.filter { displayNames.contains($0.displayName) }
                guard !recordsToRemove.isEmpty else {
                    completion(true, nil)
                    return
                }
                box.dataStore.removeData(ofTypes: dataTypes, for: recordsToRemove) {
                    completion(true, nil)
                }
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_website_data_store_remove_data_modified_since")
public func wk_website_data_store_remove_data_modified_since(
    _ ptr: UnsafeMutableRawPointer?,
    _ dataTypesJSON: UnsafePointer<CChar>?,
    _ modifiedSinceUnixSeconds: Double,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing website data store")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebsiteDataStoreBox = wkBorrow(ptr)
    let dataTypes = wkWebsiteDataTypes(from: dataTypesJSON)
    let modifiedSince = Date(timeIntervalSince1970: modifiedSinceUnixSeconds)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.dataStore.removeData(ofTypes: dataTypes, modifiedSince: modifiedSince) {
                completion(true, nil)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_website_data_store_fetch_data")
public func wk_website_data_store_fetch_data(
    _ ptr: UnsafeMutableRawPointer?,
    _ dataTypesJSON: UnsafePointer<CChar>?,
    _ outBytes: UnsafeMutablePointer<UnsafeMutablePointer<UInt8>?>?,
    _ outLen: UnsafeMutablePointer<Int>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing website data store")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 26.0, *) else {
        outErr?.pointee = wkCString("fetchData(of:) requires macOS 26.0+")
        return WK_UNSUPPORTED
    }
    let box: WKWebsiteDataStoreBox = wkBorrow(ptr)
    let dataTypes = wkWebsiteDataTypes(from: dataTypesJSON)
    let (status, data, error): (Int32, Data?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.dataStore.fetchData(of: dataTypes) { data, maybeError in
                completion(data, maybeError?.localizedDescription)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    if let data {
        wkSetBytes(data, outBytes, outLen)
    }
    return status
}

@_cdecl("wk_website_data_store_restore_data")
public func wk_website_data_store_restore_data(
    _ ptr: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let bytes else {
        outErr?.pointee = wkCString("missing website data store or data")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 26.0, *) else {
        outErr?.pointee = wkCString("restoreData(_:) requires macOS 26.0+")
        return WK_UNSUPPORTED
    }
    let box: WKWebsiteDataStoreBox = wkBorrow(ptr)
    let data = Data(bytes: bytes, count: len)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.dataStore.restoreData(data) { maybeError in
                completion(maybeError == nil, maybeError?.localizedDescription)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}
