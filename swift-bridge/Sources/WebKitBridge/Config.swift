import Foundation
import WebKit

final class WKConfigBox: NSObject {
    let config: WKWebViewConfiguration
    var registeredHandlerNames: [String] = []

    init(configuration: WKWebViewConfiguration) {
        self.config = configuration
        super.init()
    }
}

@_cdecl("wk_config_new")
public func wk_config_new() -> UnsafeMutableRawPointer {
    wkRetain(WKConfigBox(configuration: WKWebViewConfiguration()))
}

@_cdecl("wk_config_release")
public func wk_config_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_config_set_application_name")
public func wk_config_set_application_name(
    _ ptr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) {
    guard let ptr, let name else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    box.config.applicationNameForUserAgent = String(cString: name)
}

@_cdecl("wk_config_set_allows_airplay")
public func wk_config_set_allows_airplay(_ ptr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let ptr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    box.config.allowsAirPlayForMediaPlayback = value
}

@_cdecl("wk_config_set_allows_content_javascript")
public func wk_config_set_allows_content_javascript(
    _ ptr: UnsafeMutableRawPointer?,
    _ value: Bool
) {
    guard let ptr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    if #available(macOS 11.0, *) {
        box.config.defaultWebpagePreferences.allowsContentJavaScript = value
    } else {
        box.config.preferences.javaScriptEnabled = value
    }
}

@_cdecl("wk_config_use_nonpersistent_data_store")
public func wk_config_use_nonpersistent_data_store(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    box.config.websiteDataStore = .nonPersistent()
}

@_cdecl("wk_config_add_user_script")
public func wk_config_add_user_script(
    _ ptr: UnsafeMutableRawPointer?,
    _ source: UnsafePointer<CChar>?,
    _ injectionTime: Int32,
    _ mainFrameOnly: Bool
) {
    guard let ptr, let source else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    let time: WKUserScriptInjectionTime = injectionTime == 0 ? .atDocumentStart : .atDocumentEnd
    let script = WKUserScript(
        source: String(cString: source),
        injectionTime: time,
        forMainFrameOnly: mainFrameOnly
    )
    box.config.userContentController.addUserScript(script)
}

@_cdecl("wk_config_add_message_handler_name")
public func wk_config_add_message_handler_name(
    _ ptr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) {
    guard let ptr, let name else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    let handlerName = String(cString: name)
    if !box.registeredHandlerNames.contains(handlerName) {
        box.registeredHandlerNames.append(handlerName)
    }
}
