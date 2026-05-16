import Foundation
import WebKit

private func wkPreferencesDictionary(from configuration: WKWebViewConfiguration) -> [String: Any] {
    let preferences = configuration.preferences
    var dictionary: [String: Any] = [
        "minimumFontSize": preferences.minimumFontSize,
        "javaScriptCanOpenWindowsAutomatically": preferences.javaScriptCanOpenWindowsAutomatically,
        "javaScriptEnabled": configuration.defaultWebpagePreferences.allowsContentJavaScript,
        "fraudulentWebsiteWarningEnabled": preferences.isFraudulentWebsiteWarningEnabled,
        "shouldPrintBackgrounds": false,
        "tabFocusesLinks": false,
        "textInteractionEnabled": true,
        "siteSpecificQuirksModeEnabled": true,
        "elementFullscreenEnabled": false,
        "inactiveSchedulingPolicy": "Suspend"
    ]
    if #available(macOS 13.3, *) {
        dictionary["shouldPrintBackgrounds"] = preferences.shouldPrintBackgrounds
    }
    if #available(macOS 10.12.4, *) {
        dictionary["tabFocusesLinks"] = preferences.tabFocusesLinks
    }
    if #available(macOS 11.3, *) {
        dictionary["textInteractionEnabled"] = preferences.isTextInteractionEnabled
    }
    if #available(macOS 12.3, *) {
        dictionary["siteSpecificQuirksModeEnabled"] = preferences.isSiteSpecificQuirksModeEnabled
        dictionary["elementFullscreenEnabled"] = preferences.isElementFullscreenEnabled
    }
    if #available(macOS 14.0, *) {
        dictionary["inactiveSchedulingPolicy"] = switch preferences.inactiveSchedulingPolicy {
        case .suspend:
            "Suspend"
        case .throttle:
            "Throttle"
        case .none:
            "None"
        @unknown default:
            "Suspend"
        }
    }
    return dictionary
}

private func wkApplyPreferences(_ dictionary: [String: Any], to configuration: WKWebViewConfiguration) {
    let preferences = configuration.preferences

    if let value = dictionary["minimumFontSize"] as? NSNumber {
        preferences.minimumFontSize = CGFloat(value.doubleValue)
    }
    if let value = dictionary["javaScriptCanOpenWindowsAutomatically"] as? Bool {
        preferences.javaScriptCanOpenWindowsAutomatically = value
    }
    if let value = dictionary["fraudulentWebsiteWarningEnabled"] as? Bool {
        preferences.isFraudulentWebsiteWarningEnabled = value
    }
    if #available(macOS 13.3, *), let value = dictionary["shouldPrintBackgrounds"] as? Bool {
        preferences.shouldPrintBackgrounds = value
    }
    if #available(macOS 10.12.4, *), let value = dictionary["tabFocusesLinks"] as? Bool {
        preferences.tabFocusesLinks = value
    }
    if #available(macOS 11.3, *), let value = dictionary["textInteractionEnabled"] as? Bool {
        preferences.isTextInteractionEnabled = value
    }
    if #available(macOS 12.3, *), let value = dictionary["siteSpecificQuirksModeEnabled"] as? Bool {
        preferences.isSiteSpecificQuirksModeEnabled = value
    }
    if #available(macOS 12.3, *), let value = dictionary["elementFullscreenEnabled"] as? Bool {
        preferences.isElementFullscreenEnabled = value
    }
    if #available(macOS 14.0, *) {
        if let value = dictionary["inactiveSchedulingPolicy"] as? String {
            switch value {
            case "Throttle":
                preferences.inactiveSchedulingPolicy = .throttle
            case "None":
                preferences.inactiveSchedulingPolicy = .none
            default:
                preferences.inactiveSchedulingPolicy = .suspend
            }
        } else if let value = dictionary["inactiveSchedulingPolicy"] as? NSNumber,
                  let policy = WKPreferences.InactiveSchedulingPolicy(rawValue: value.intValue)
        {
            preferences.inactiveSchedulingPolicy = policy
        }
    }
    if let value = dictionary["javaScriptEnabled"] as? Bool {
        configuration.defaultWebpagePreferences.allowsContentJavaScript = value
    }
}

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

@_cdecl("wk_config_copy_application_name")
public func wk_config_copy_application_name(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return nil }
    let box: WKConfigBox = wkBorrow(ptr)
    return wkCString(box.config.applicationNameForUserAgent ?? "")
}

@_cdecl("wk_config_set_allows_airplay")
public func wk_config_set_allows_airplay(_ ptr: UnsafeMutableRawPointer?, _ value: Bool) {
    guard let ptr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    box.config.allowsAirPlayForMediaPlayback = value
}

@_cdecl("wk_config_get_allows_airplay")
public func wk_config_get_allows_airplay(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return false }
    let box: WKConfigBox = wkBorrow(ptr)
    return box.config.allowsAirPlayForMediaPlayback
}

@_cdecl("wk_config_set_allows_content_javascript")
public func wk_config_set_allows_content_javascript(
    _ ptr: UnsafeMutableRawPointer?,
    _ value: Bool
) {
    guard let ptr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    box.config.defaultWebpagePreferences.allowsContentJavaScript = value
}

@_cdecl("wk_config_get_allows_content_javascript")
public func wk_config_get_allows_content_javascript(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr else { return true }
    let box: WKConfigBox = wkBorrow(ptr)
    return box.config.defaultWebpagePreferences.allowsContentJavaScript
}

@_cdecl("wk_config_set_preferences_json")
public func wk_config_set_preferences_json(
    _ ptr: UnsafeMutableRawPointer?,
    _ json: UnsafePointer<CChar>?
) {
    guard let ptr,
          let dictionary = wkJSONObject(from: json) as? [String: Any]
    else {
        return
    }
    let box: WKConfigBox = wkBorrow(ptr)
    wkApplyPreferences(dictionary, to: box.config)
}

@_cdecl("wk_config_copy_preferences_json")
public func wk_config_copy_preferences_json(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr else { return wkCString("{}") }
    let box: WKConfigBox = wkBorrow(ptr)
    return wkCString(wkJSONString(wkPreferencesDictionary(from: box.config)))
}

@_cdecl("wk_config_set_website_data_store")
public func wk_config_set_website_data_store(
    _ ptr: UnsafeMutableRawPointer?,
    _ storePtr: UnsafeMutableRawPointer?
) {
    guard let ptr, let storePtr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    let storeBox: WKWebsiteDataStoreBox = wkBorrow(storePtr)
    box.config.websiteDataStore = storeBox.dataStore
}

@_cdecl("wk_config_copy_website_data_store")
public func wk_config_copy_website_data_store(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let ptr else { return nil }
    let box: WKConfigBox = wkBorrow(ptr)
    return wkRetain(WKWebsiteDataStoreBox(dataStore: box.config.websiteDataStore))
}

@_cdecl("wk_config_use_nonpersistent_data_store")
public func wk_config_use_nonpersistent_data_store(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    box.config.websiteDataStore = .nonPersistent()
}

@_cdecl("wk_config_add_content_rule_list")
public func wk_config_add_content_rule_list(
    _ ptr: UnsafeMutableRawPointer?,
    _ ruleListPtr: UnsafeMutableRawPointer?
) {
    guard let ptr, let ruleListPtr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    let ruleListBox: WKContentRuleListBox = wkBorrow(ruleListPtr)
    box.config.userContentController.add(ruleListBox.ruleList)
}

@_cdecl("wk_config_remove_content_rule_list")
public func wk_config_remove_content_rule_list(
    _ ptr: UnsafeMutableRawPointer?,
    _ ruleListPtr: UnsafeMutableRawPointer?
) {
    guard let ptr, let ruleListPtr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    let ruleListBox: WKContentRuleListBox = wkBorrow(ruleListPtr)
    box.config.userContentController.remove(ruleListBox.ruleList)
}

@_cdecl("wk_config_remove_all_content_rule_lists")
public func wk_config_remove_all_content_rule_lists(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    box.config.userContentController.removeAllContentRuleLists()
}

@_cdecl("wk_config_add_user_script")
public func wk_config_add_user_script(
    _ ptr: UnsafeMutableRawPointer?,
    _ source: UnsafePointer<CChar>?,
    _ injectionTime: Int32,
    _ mainFrameOnly: Bool,
    _ contentWorldName: UnsafePointer<CChar>?
) {
    guard let ptr, let source else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    let script = wkMakeUserScript(
        source: String(cString: source),
        injectionTime: injectionTime,
        mainFrameOnly: mainFrameOnly,
        contentWorldName: contentWorldName.map(String.init(cString:))
    )
    box.config.userContentController.addUserScript(script)
}

@_cdecl("wk_config_remove_all_user_scripts")
public func wk_config_remove_all_user_scripts(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    let box: WKConfigBox = wkBorrow(ptr)
    box.config.userContentController.removeAllUserScripts()
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
