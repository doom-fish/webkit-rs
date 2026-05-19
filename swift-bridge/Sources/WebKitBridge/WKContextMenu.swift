import Foundation
import WebKit

private func wkObjectValue<T: AnyObject>(
    _ ptr: UnsafeMutableRawPointer?,
    selectorName: String,
    as type: T.Type
) -> T? {
    guard let ptr else { return nil }
    let object: NSObject = wkBorrow(ptr)
    return wkOnMain {
        let selector = NSSelectorFromString(selectorName)
        guard object.responds(to: selector),
              let value = object.perform(selector)?.takeUnretainedValue()
        else {
            return nil
        }
        return value as? T
    }
}

private func wkStringProperty(
    _ ptr: UnsafeMutableRawPointer?,
    selectorName: String
) -> String? {
    guard let value = wkObjectValue(ptr, selectorName: selectorName, as: NSString.self) else {
        return nil
    }
    return value as String
}

private func wkURLProperty(
    _ ptr: UnsafeMutableRawPointer?,
    selectorName: String
) -> String? {
    guard let value = wkObjectValue(ptr, selectorName: selectorName, as: NSURL.self) else {
        return nil
    }
    return value.absoluteString
}

@_cdecl("wk_context_menu_element_info_release")
public func wk_context_menu_element_info_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_context_menu_element_info_copy_link_url")
public func wk_context_menu_element_info_copy_link_url(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    wkURLProperty(ptr, selectorName: "linkURL").flatMap(wkCString)
}

@_cdecl("wk_preview_element_info_release")
public func wk_preview_element_info_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_preview_element_info_copy_link_url")
public func wk_preview_element_info_copy_link_url(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    wkURLProperty(ptr, selectorName: "linkURL").flatMap(wkCString)
}

@_cdecl("wk_preview_action_item_release")
public func wk_preview_action_item_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_preview_action_item_copy_identifier")
public func wk_preview_action_item_copy_identifier(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    wkStringProperty(ptr, selectorName: "identifier").flatMap(wkCString)
}

@_cdecl("wk_preview_action_item_copy_title")
public func wk_preview_action_item_copy_title(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    wkStringProperty(ptr, selectorName: "title").flatMap(wkCString)
}
