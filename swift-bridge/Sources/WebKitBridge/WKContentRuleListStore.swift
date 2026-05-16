import Foundation
import WebKit

final class WKContentRuleListBox: NSObject {
    let ruleList: WKContentRuleList

    init(ruleList: WKContentRuleList) {
        self.ruleList = ruleList
        super.init()
    }
}

final class WKContentRuleListStoreBox: NSObject {
    let store: WKContentRuleListStore

    init(store: WKContentRuleListStore) {
        self.store = store
        super.init()
    }
}

@_cdecl("wk_content_rule_list_store_default")
public func wk_content_rule_list_store_default() -> UnsafeMutableRawPointer? {
    guard let store = WKContentRuleListStore.default() else {
        return nil
    }
    return wkRetain(WKContentRuleListStoreBox(store: store))
}

@_cdecl("wk_content_rule_list_store_with_path")
public func wk_content_rule_list_store_with_path(_ path: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let path else { return nil }
    guard let store = WKContentRuleListStore(url: URL(fileURLWithPath: String(cString: path))) else {
        return nil
    }
    return wkRetain(WKContentRuleListStoreBox(store: store))
}

@_cdecl("wk_content_rule_list_store_release")
public func wk_content_rule_list_store_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_content_rule_list_release")
public func wk_content_rule_list_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_content_rule_list_copy_identifier")
public func wk_content_rule_list_copy_identifier(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    let box: WKContentRuleListBox = wkBorrow(ptr)
    return wkCString(box.ruleList.identifier)
}

@_cdecl("wk_content_rule_list_store_compile")
public func wk_content_rule_list_store_compile(
    _ ptr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ encodedRuleList: UnsafePointer<CChar>?,
    _ outRuleList: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let identifier, let encodedRuleList else {
        outErr?.pointee = wkCString("missing content rule list store arguments")
        return WK_INVALID_ARGUMENT
    }
    let box: WKContentRuleListStoreBox = wkBorrow(ptr)
    let (status, ruleList, error): (Int32, WKContentRuleList?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.store.compileContentRuleList(
                forIdentifier: String(cString: identifier),
                encodedContentRuleList: String(cString: encodedRuleList)
            ) { ruleList, maybeError in
                completion(ruleList, maybeError?.localizedDescription)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    outRuleList?.pointee = ruleList.map { wkRetain(WKContentRuleListBox(ruleList: $0)) }
    return status
}

@_cdecl("wk_content_rule_list_store_lookup")
public func wk_content_rule_list_store_lookup(
    _ ptr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outRuleList: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let identifier else {
        outErr?.pointee = wkCString("missing content rule list store or identifier")
        return WK_INVALID_ARGUMENT
    }
    let box: WKContentRuleListStoreBox = wkBorrow(ptr)
    let (status, ruleList, error): (Int32, WKContentRuleList?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.store.lookUpContentRuleList(forIdentifier: String(cString: identifier)) { ruleList, maybeError in
                completion(ruleList, maybeError?.localizedDescription)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
        return status
    }
    outRuleList?.pointee = ruleList.map { wkRetain(WKContentRuleListBox(ruleList: $0)) }
    return status
}

@_cdecl("wk_content_rule_list_store_remove")
public func wk_content_rule_list_store_remove(
    _ ptr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let identifier else {
        outErr?.pointee = wkCString("missing content rule list store or identifier")
        return WK_INVALID_ARGUMENT
    }
    let box: WKContentRuleListStoreBox = wkBorrow(ptr)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.store.removeContentRuleList(forIdentifier: String(cString: identifier)) { maybeError in
                completion(maybeError == nil, maybeError?.localizedDescription)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_content_rule_list_store_copy_available_identifiers_json")
public func wk_content_rule_list_store_copy_available_identifiers_json(
    _ ptr: UnsafeMutableRawPointer?,
    _ outJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr else {
        outErr?.pointee = wkCString("missing content rule list store")
        return WK_INVALID_ARGUMENT
    }
    let box: WKContentRuleListStoreBox = wkBorrow(ptr)
    let (status, identifiers, error): (Int32, [String]?, String?) = wkWaitForAsync { completion in
        DispatchQueue.main.async {
            box.store.getAvailableContentRuleListIdentifiers { identifiers in
                completion(identifiers, nil)
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
