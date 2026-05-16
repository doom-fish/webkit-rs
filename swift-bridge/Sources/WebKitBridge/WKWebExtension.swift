import Foundation
import WebKit

private func wkNSErrorDictionary(_ error: NSError) -> [String: Any] {
    [
        "domain": error.domain,
        "code": error.code,
        "description": error.localizedDescription
    ]
}

@available(macOS 15.4, *)
private func wkWebExtensionPermissionArray(_ permissions: Set<WKWebExtension.Permission>) -> [String] {
    permissions.map(\.rawValue).sorted()
}

@available(macOS 15.4, *)
private func wkWebExtensionDataTypeArray(_ dataTypes: Set<WKWebExtension.DataType>) -> [String] {
    dataTypes.map(\.rawValue).sorted()
}

@available(macOS 15.4, *)
private func wkWebExtensionMatchPatternArray(_ patterns: Set<WKWebExtension.MatchPattern>) -> [String] {
    patterns.map(\ .string).sorted()
}

@available(macOS 15.4, *)
private func wkWebExtensionMatchPatternOptions(_ rawValue: UInt64) -> WKWebExtension.MatchPattern.Options {
    WKWebExtension.MatchPattern.Options(rawValue: UInt(rawValue))
}

@available(macOS 15.4, *)
private func wkWebExtensionPermissionStatus(_ rawValue: Int64) -> WKWebExtensionContext.PermissionStatus {
    WKWebExtensionContext.PermissionStatus(rawValue: Int(rawValue)) ?? .unknown
}

@available(macOS 15.4, *)
private func wkWebExtensionPermission(_ rawValue: String) -> WKWebExtension.Permission {
    WKWebExtension.Permission(rawValue: rawValue)
}

@available(macOS 15.4, *)
private func wkWebExtensionDataType(_ rawValue: String) -> WKWebExtension.DataType {
    WKWebExtension.DataType(rawValue: rawValue)
}

@available(macOS 15.4, *)
private func wkWebExtensionDataTypeSet(from jsonCString: UnsafePointer<CChar>?) -> Set<WKWebExtension.DataType> {
    Set(wkStringArray(from: jsonCString).map(wkWebExtensionDataType))
}

@available(macOS 15.4, *)
private func wkWebExtensionActionDictionary(_ action: WKWebExtension.Action) -> [String: Any] {
    [
        "label": action.label,
        "badgeText": action.badgeText,
        "hasUnreadBadgeText": action.hasUnreadBadgeText,
        "inspectionName": action.inspectionName ?? NSNull(),
        "enabled": action.isEnabled,
        "presentsPopup": action.presentsPopup,
        "associatedTabAvailable": action.associatedTab != nil,
        "popupWebViewAvailable": action.popupWebView != nil
    ]
}

@available(macOS 15.4, *)
private func wkWebExtensionCommandDictionary(_ command: WKWebExtension.Command) -> [String: Any] {
    [
        "identifier": command.id,
        "title": command.title,
        "activationKey": command.activationKey ?? NSNull(),
        "modifierFlags": UInt64(command.modifierFlags.rawValue)
    ]
}

@available(macOS 15.4, *)
private func wkWebExtensionDataRecordDictionary(_ record: WKWebExtension.DataRecord) -> [String: Any] {
    var sizeInBytesByType: [String: UInt64] = [:]
    for dataType in record.containedDataTypes {
        sizeInBytesByType[dataType.rawValue] = UInt64(record.sizeInBytes(ofTypes: Set([dataType])))
    }
    return [
        "displayName": record.displayName,
        "uniqueIdentifier": record.uniqueIdentifier,
        "containedDataTypes": wkWebExtensionDataTypeArray(record.containedDataTypes),
        "errors": record.errors.map { wkNSErrorDictionary($0 as NSError) },
        "totalSizeInBytes": UInt64(record.totalSizeInBytes),
        "sizeInBytesByType": sizeInBytesByType
    ]
}

@available(macOS 15.4, *)
private func wkWebExtensionSummaryDictionary(_ webExtension: WKWebExtension) -> [String: Any] {
    [
        "errors": webExtension.errors.map { wkNSErrorDictionary($0 as NSError) },
        "manifest": webExtension.manifest,
        "manifestVersion": webExtension.manifestVersion,
        "defaultLocaleIdentifier": webExtension.defaultLocale?.identifier ?? NSNull(),
        "displayName": webExtension.displayName ?? NSNull(),
        "displayShortName": webExtension.displayShortName ?? NSNull(),
        "displayVersion": webExtension.displayVersion ?? NSNull(),
        "displayDescription": webExtension.displayDescription ?? NSNull(),
        "displayActionLabel": webExtension.displayActionLabel ?? NSNull(),
        "version": webExtension.version ?? NSNull(),
        "requestedPermissions": wkWebExtensionPermissionArray(webExtension.requestedPermissions),
        "optionalPermissions": wkWebExtensionPermissionArray(webExtension.optionalPermissions),
        "requestedPermissionMatchPatterns": wkWebExtensionMatchPatternArray(webExtension.requestedPermissionMatchPatterns),
        "optionalPermissionMatchPatterns": wkWebExtensionMatchPatternArray(webExtension.optionalPermissionMatchPatterns),
        "allRequestedMatchPatterns": wkWebExtensionMatchPatternArray(webExtension.allRequestedMatchPatterns),
        "hasBackgroundContent": webExtension.hasBackgroundContent,
        "hasPersistentBackgroundContent": webExtension.hasPersistentBackgroundContent,
        "hasInjectedContent": webExtension.hasInjectedContent,
        "hasOptionsPage": webExtension.hasOptionsPage,
        "hasOverrideNewTabPage": webExtension.hasOverrideNewTabPage,
        "hasCommands": webExtension.hasCommands,
        "hasContentModificationRules": webExtension.hasContentModificationRules
    ]
}

@available(macOS 15.4, *)
private func wkWebExtensionMatchPatternDictionary(_ pattern: WKWebExtension.MatchPattern) -> [String: Any] {
    [
        "string": pattern.string,
        "scheme": pattern.scheme ?? NSNull(),
        "host": pattern.host ?? NSNull(),
        "path": pattern.path ?? NSNull(),
        "matchesAllURLs": pattern.matchesAllURLs,
        "matchesAllHosts": pattern.matchesAllHosts
    ]
}

@available(macOS 15.4, *)
private func wkWebExtensionContextSummaryDictionary(_ context: WKWebExtensionContext) -> [String: Any] {
    [
        "errors": context.errors.map { wkNSErrorDictionary($0 as NSError) },
        "loaded": context.isLoaded,
        "baseURL": context.baseURL.absoluteString,
        "uniqueIdentifier": context.uniqueIdentifier,
        "inspectable": context.isInspectable,
        "inspectionName": context.inspectionName ?? NSNull(),
        "unsupportedAPIs": Array(context.unsupportedAPIs).sorted(),
        "optionsPageURL": context.optionsPageURL?.absoluteString ?? NSNull(),
        "overrideNewTabPageURL": context.overrideNewTabPageURL?.absoluteString ?? NSNull(),
        "hasRequestedOptionalAccessToAllHosts": context.hasRequestedOptionalAccessToAllHosts,
        "hasAccessToPrivateData": context.hasAccessToPrivateData,
        "currentPermissions": wkWebExtensionPermissionArray(context.currentPermissions),
        "currentPermissionMatchPatterns": wkWebExtensionMatchPatternArray(context.currentPermissionMatchPatterns),
        "hasAccessToAllURLs": context.hasAccessToAllURLs,
        "hasAccessToAllHosts": context.hasAccessToAllHosts,
        "hasInjectedContent": context.hasInjectedContent,
        "hasContentModificationRules": context.hasContentModificationRules,
        "webviewConfigurationAvailable": context.webViewConfiguration != nil
    ]
}

@available(macOS 15.4, *)
private func wkWebExtensionConstantsDictionary() -> [String: Any] {
    [
        "webExtensionErrorDomain": WKWebExtension.errorDomain,
        "webExtensionContextErrorDomain": WKWebExtensionContext.errorDomain,
        "webExtensionDataRecordErrorDomain": WKWebExtension.DataRecord.errorDomain,
        "webExtensionMatchPatternErrorDomain": WKWebExtension.MatchPattern.errorDomain,
        "webExtensionMessagePortErrorDomain": WKWebExtension.MessagePort.errorDomain,
        "errorsDidUpdateNotification": WKWebExtensionContext.errorsDidUpdateNotification.rawValue,
        "permissionsWereGrantedNotification": WKWebExtensionContext.permissionsWereGrantedNotification.rawValue,
        "permissionsWereDeniedNotification": WKWebExtensionContext.permissionsWereDeniedNotification.rawValue,
        "grantedPermissionsWereRemovedNotification": WKWebExtensionContext.grantedPermissionsWereRemovedNotification.rawValue,
        "deniedPermissionsWereRemovedNotification": WKWebExtensionContext.deniedPermissionsWereRemovedNotification.rawValue,
        "permissionMatchPatternsWereGrantedNotification": WKWebExtensionContext.permissionMatchPatternsWereGrantedNotification.rawValue,
        "permissionMatchPatternsWereDeniedNotification": WKWebExtensionContext.permissionMatchPatternsWereDeniedNotification.rawValue,
        "grantedPermissionMatchPatternsWereRemovedNotification": WKWebExtensionContext.grantedPermissionMatchPatternsWereRemovedNotification.rawValue,
        "deniedPermissionMatchPatternsWereRemovedNotification": WKWebExtensionContext.deniedPermissionMatchPatternsWereRemovedNotification.rawValue,
        "notificationUserInfoKeyPermissions": WKWebExtensionContext.NotificationUserInfoKey.permissions.rawValue,
        "notificationUserInfoKeyMatchPatterns": WKWebExtensionContext.NotificationUserInfoKey.matchPatterns.rawValue,
        "permissionActiveTab": WKWebExtension.Permission.activeTab.rawValue,
        "permissionAlarms": WKWebExtension.Permission.alarms.rawValue,
        "permissionClipboardWrite": WKWebExtension.Permission.clipboardWrite.rawValue,
        "permissionContextMenus": WKWebExtension.Permission.contextMenus.rawValue,
        "permissionCookies": WKWebExtension.Permission.cookies.rawValue,
        "permissionDeclarativeNetRequest": WKWebExtension.Permission.declarativeNetRequest.rawValue,
        "permissionDeclarativeNetRequestFeedback": WKWebExtension.Permission.declarativeNetRequestFeedback.rawValue,
        "permissionDeclarativeNetRequestWithHostAccess": WKWebExtension.Permission.declarativeNetRequestWithHostAccess.rawValue,
        "permissionMenus": WKWebExtension.Permission.menus.rawValue,
        "permissionNativeMessaging": WKWebExtension.Permission.nativeMessaging.rawValue,
        "permissionScripting": WKWebExtension.Permission.scripting.rawValue,
        "permissionStorage": WKWebExtension.Permission.storage.rawValue,
        "permissionTabs": WKWebExtension.Permission.tabs.rawValue,
        "permissionUnlimitedStorage": WKWebExtension.Permission.unlimitedStorage.rawValue,
        "permissionWebNavigation": WKWebExtension.Permission.webNavigation.rawValue,
        "permissionWebRequest": WKWebExtension.Permission.webRequest.rawValue,
        "dataTypeLocal": WKWebExtension.DataType.local.rawValue,
        "dataTypeSession": WKWebExtension.DataType.session.rawValue,
        "dataTypeSynchronized": WKWebExtension.DataType.synchronized.rawValue
    ]
}

@available(macOS 15.4, *)
final class WKWebExtensionBox: NSObject {
    let webExtension: WKWebExtension

    init(webExtension: WKWebExtension) {
        self.webExtension = webExtension
        super.init()
    }
}

@available(macOS 15.4, *)
final class WKWebExtensionMatchPatternBox: NSObject {
    let matchPattern: WKWebExtension.MatchPattern

    init(matchPattern: WKWebExtension.MatchPattern) {
        self.matchPattern = matchPattern
        super.init()
    }
}

@available(macOS 15.4, *)
final class WKWebExtensionControllerConfigurationBox: NSObject {
    let configuration: WKWebExtensionController.Configuration

    init(configuration: WKWebExtensionController.Configuration) {
        self.configuration = configuration
        super.init()
    }
}

@available(macOS 15.4, *)
final class WKWebExtensionControllerBox: NSObject {
    let controller: WKWebExtensionController

    init(controller: WKWebExtensionController) {
        self.controller = controller
        super.init()
    }
}

@available(macOS 15.4, *)
final class WKWebExtensionContextBox: NSObject {
    let context: WKWebExtensionContext

    init(context: WKWebExtensionContext) {
        self.context = context
        super.init()
    }
}

@available(macOS 15.4, *)
final class WKWebExtensionMessagePortBox: NSObject {
    let port: WKWebExtension.MessagePort

    init(port: WKWebExtension.MessagePort) {
        self.port = port
        super.init()
    }
}

@_cdecl("wk_web_extension_copy_constants_json")
public func wk_web_extension_copy_constants_json() -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.4, *) else { return wkCString("{}") }
    return wkCString(wkJSONString(wkWebExtensionConstantsDictionary()))
}

@_cdecl("wk_web_extension_create_with_resource_base_url")
public func wk_web_extension_create_with_resource_base_url(
    _ path: UnsafePointer<CChar>?,
    _ outExtension: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let path else {
        outErr?.pointee = wkCString("missing extension resource path")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("web extensions require macOS 15.4")
        return WK_UNSUPPORTED
    }

    let url = URL(fileURLWithPath: String(cString: path))
    let (status, webExtension, error): (Int32, WKWebExtension?, String?) = wkWaitForAsync { completion in
        Task { @MainActor in
            do {
                let webExtension = try await WKWebExtension(resourceBaseURL: url)
                completion(webExtension, nil)
            } catch {
                completion(nil, error.localizedDescription)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    outExtension?.pointee = webExtension.map { wkRetain(WKWebExtensionBox(webExtension: $0)) }
    return status
}

@_cdecl("wk_web_extension_create_with_app_extension_bundle")
public func wk_web_extension_create_with_app_extension_bundle(
    _ path: UnsafePointer<CChar>?,
    _ outExtension: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let path else {
        outErr?.pointee = wkCString("missing app extension bundle path")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("web extensions require macOS 15.4")
        return WK_UNSUPPORTED
    }

    let bundleURL = URL(fileURLWithPath: String(cString: path))
    guard let bundle = Bundle(url: bundleURL) else {
        outErr?.pointee = wkCString("invalid app extension bundle")
        return WK_INVALID_ARGUMENT
    }
    let (status, webExtension, error): (Int32, WKWebExtension?, String?) = wkWaitForAsync { completion in
        Task { @MainActor in
            do {
                let webExtension = try await WKWebExtension(appExtensionBundle: bundle)
                completion(webExtension, nil)
            } catch {
                completion(nil, error.localizedDescription)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    outExtension?.pointee = webExtension.map { wkRetain(WKWebExtensionBox(webExtension: $0)) }
    return status
}

@_cdecl("wk_web_extension_release")
public func wk_web_extension_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_web_extension_copy_summary_json")
public func wk_web_extension_copy_summary_json(_ ptr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let ptr, #available(macOS 15.4, *) else { return wkCString("{}") }
    let box: WKWebExtensionBox = wkBorrow(ptr)
    return wkCString(wkJSONString(wkWebExtensionSummaryDictionary(box.webExtension)))
}

@_cdecl("wk_web_extension_supports_manifest_version")
public func wk_web_extension_supports_manifest_version(
    _ ptr: UnsafeMutableRawPointer?,
    _ manifestVersion: Double
) -> Bool {
    guard let ptr, #available(macOS 15.4, *) else { return false }
    let box: WKWebExtensionBox = wkBorrow(ptr)
    return box.webExtension.supportsManifestVersion(manifestVersion)
}

@_cdecl("wk_web_extension_match_pattern_register_custom_url_scheme")
public func wk_web_extension_match_pattern_register_custom_url_scheme(
    _ scheme: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let scheme else {
        outErr?.pointee = wkCString("missing custom URL scheme")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("web extension match patterns require macOS 15.4")
        return WK_UNSUPPORTED
    }
    WKWebExtension.MatchPattern.registerCustomURLScheme(String(cString: scheme))
    return WK_OK
}

@_cdecl("wk_web_extension_match_pattern_all_urls")
public func wk_web_extension_match_pattern_all_urls() -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.4, *) else { return nil }
    return wkRetain(WKWebExtensionMatchPatternBox(matchPattern: WKWebExtension.MatchPattern.allURLs()))
}

@_cdecl("wk_web_extension_match_pattern_all_hosts_and_schemes")
public func wk_web_extension_match_pattern_all_hosts_and_schemes() -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.4, *) else { return nil }
    return wkRetain(
        WKWebExtensionMatchPatternBox(matchPattern: WKWebExtension.MatchPattern.allHostsAndSchemes())
    )
}

@_cdecl("wk_web_extension_match_pattern_with_string")
public func wk_web_extension_match_pattern_with_string(
    _ pattern: UnsafePointer<CChar>?,
    _ outPattern: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let pattern else {
        outErr?.pointee = wkCString("missing match pattern string")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("web extension match patterns require macOS 15.4")
        return WK_UNSUPPORTED
    }
    do {
        let matchPattern = try WKWebExtension.MatchPattern(string: String(cString: pattern))
        outPattern?.pointee = wkRetain(WKWebExtensionMatchPatternBox(matchPattern: matchPattern))
        return WK_OK
    } catch {
        outErr?.pointee = wkCString(error.localizedDescription)
        return WK_FRAMEWORK_ERROR
    }
}

@_cdecl("wk_web_extension_match_pattern_with_components")
public func wk_web_extension_match_pattern_with_components(
    _ scheme: UnsafePointer<CChar>?,
    _ host: UnsafePointer<CChar>?,
    _ path: UnsafePointer<CChar>?,
    _ outPattern: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let scheme, let host, let path else {
        outErr?.pointee = wkCString("missing match pattern components")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("web extension match patterns require macOS 15.4")
        return WK_UNSUPPORTED
    }
    do {
        let matchPattern = try WKWebExtension.MatchPattern(
            scheme: String(cString: scheme),
            host: String(cString: host),
            path: String(cString: path)
        )
        outPattern?.pointee = wkRetain(WKWebExtensionMatchPatternBox(matchPattern: matchPattern))
        return WK_OK
    } catch {
        outErr?.pointee = wkCString(error.localizedDescription)
        return WK_FRAMEWORK_ERROR
    }
}

@_cdecl("wk_web_extension_match_pattern_release")
public func wk_web_extension_match_pattern_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_web_extension_match_pattern_copy_summary_json")
public func wk_web_extension_match_pattern_copy_summary_json(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let ptr, #available(macOS 15.4, *) else { return wkCString("{}") }
    let box: WKWebExtensionMatchPatternBox = wkBorrow(ptr)
    return wkCString(wkJSONString(wkWebExtensionMatchPatternDictionary(box.matchPattern)))
}

@_cdecl("wk_web_extension_match_pattern_matches_url")
public func wk_web_extension_match_pattern_matches_url(
    _ ptr: UnsafeMutableRawPointer?,
    _ url: UnsafePointer<CChar>?,
    _ options: UInt64
) -> Bool {
    guard let ptr,
          let url,
          let parsedURL = URL(string: String(cString: url)),
          #available(macOS 15.4, *)
    else {
        return false
    }
    let box: WKWebExtensionMatchPatternBox = wkBorrow(ptr)
    return box.matchPattern.matches(parsedURL, options: wkWebExtensionMatchPatternOptions(options))
}

@_cdecl("wk_web_extension_match_pattern_matches_pattern")
public func wk_web_extension_match_pattern_matches_pattern(
    _ ptr: UnsafeMutableRawPointer?,
    _ otherPtr: UnsafeMutableRawPointer?,
    _ options: UInt64
) -> Bool {
    guard let ptr, let otherPtr, #available(macOS 15.4, *) else { return false }
    let box: WKWebExtensionMatchPatternBox = wkBorrow(ptr)
    let otherBox: WKWebExtensionMatchPatternBox = wkBorrow(otherPtr)
    return box.matchPattern.matches(otherBox.matchPattern, options: wkWebExtensionMatchPatternOptions(options))
}

@_cdecl("wk_web_extension_controller_configuration_default")
public func wk_web_extension_controller_configuration_default() -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.4, *) else { return nil }
    return wkRetain(
        WKWebExtensionControllerConfigurationBox(
            configuration: WKWebExtensionController.Configuration.default()
        )
    )
}

@_cdecl("wk_web_extension_controller_configuration_nonpersistent")
public func wk_web_extension_controller_configuration_nonpersistent() -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.4, *) else { return nil }
    return wkRetain(
        WKWebExtensionControllerConfigurationBox(
            configuration: WKWebExtensionController.Configuration.nonPersistent()
        )
    )
}

@_cdecl("wk_web_extension_controller_configuration_with_identifier")
public func wk_web_extension_controller_configuration_with_identifier(
    _ identifier: UnsafePointer<CChar>?,
    _ outConfiguration: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let identifier else {
        outErr?.pointee = wkCString("missing controller configuration identifier")
        return WK_INVALID_ARGUMENT
    }
    guard #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("web extension controller configuration requires macOS 15.4")
        return WK_UNSUPPORTED
    }
    guard let uuid = UUID(uuidString: String(cString: identifier)) else {
        outErr?.pointee = wkCString("invalid UUID string")
        return WK_INVALID_ARGUMENT
    }
    outConfiguration?.pointee = wkRetain(
        WKWebExtensionControllerConfigurationBox(
            configuration: WKWebExtensionController.Configuration(identifier: uuid)
        )
    )
    return WK_OK
}

@_cdecl("wk_web_extension_controller_configuration_release")
public func wk_web_extension_controller_configuration_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_web_extension_controller_configuration_copy_summary_json")
public func wk_web_extension_controller_configuration_copy_summary_json(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let ptr, #available(macOS 15.4, *) else { return wkCString("{}") }
    let box: WKWebExtensionControllerConfigurationBox = wkBorrow(ptr)
    let dictionary: [String: Any] = [
        "persistent": box.configuration.isPersistent,
        "identifier": box.configuration.identifier?.uuidString ?? NSNull()
    ]
    return wkCString(wkJSONString(dictionary))
}

@_cdecl("wk_web_extension_controller_configuration_set_webview_configuration")
public func wk_web_extension_controller_configuration_set_webview_configuration(
    _ ptr: UnsafeMutableRawPointer?,
    _ configPtr: UnsafeMutableRawPointer?
) {
    guard let ptr, let configPtr, #available(macOS 15.4, *) else { return }
    let box: WKWebExtensionControllerConfigurationBox = wkBorrow(ptr)
    let configBox: WKConfigBox = wkBorrow(configPtr)
    box.configuration.webViewConfiguration = (configBox.config.copy() as! WKWebViewConfiguration)
}

@_cdecl("wk_web_extension_controller_configuration_copy_webview_configuration")
public func wk_web_extension_controller_configuration_copy_webview_configuration(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let ptr, #available(macOS 15.4, *) else { return nil }
    let box: WKWebExtensionControllerConfigurationBox = wkBorrow(ptr)
    return wkRetain(WKConfigBox(configuration: box.configuration.webViewConfiguration))
}

@_cdecl("wk_web_extension_controller_configuration_set_default_website_data_store")
public func wk_web_extension_controller_configuration_set_default_website_data_store(
    _ ptr: UnsafeMutableRawPointer?,
    _ storePtr: UnsafeMutableRawPointer?
) {
    guard let ptr, let storePtr, #available(macOS 15.4, *) else { return }
    let box: WKWebExtensionControllerConfigurationBox = wkBorrow(ptr)
    let storeBox: WKWebsiteDataStoreBox = wkBorrow(storePtr)
    box.configuration.defaultWebsiteDataStore = storeBox.dataStore
}

@_cdecl("wk_web_extension_controller_configuration_copy_default_website_data_store")
public func wk_web_extension_controller_configuration_copy_default_website_data_store(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let ptr, #available(macOS 15.4, *) else { return nil }
    let box: WKWebExtensionControllerConfigurationBox = wkBorrow(ptr)
    return wkRetain(WKWebsiteDataStoreBox(dataStore: box.configuration.defaultWebsiteDataStore))
}

@_cdecl("wk_web_extension_controller_new")
public func wk_web_extension_controller_new() -> UnsafeMutableRawPointer? {
    guard #available(macOS 15.4, *) else { return nil }
    return wkRetain(WKWebExtensionControllerBox(controller: WKWebExtensionController()))
}

@_cdecl("wk_web_extension_controller_with_configuration")
public func wk_web_extension_controller_with_configuration(
    _ configurationPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let configurationPtr, #available(macOS 15.4, *) else { return nil }
    let configurationBox: WKWebExtensionControllerConfigurationBox = wkBorrow(configurationPtr)
    return wkRetain(
        WKWebExtensionControllerBox(
            controller: WKWebExtensionController(configuration: configurationBox.configuration)
        )
    )
}

@_cdecl("wk_web_extension_controller_release")
public func wk_web_extension_controller_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_web_extension_controller_copy_configuration")
public func wk_web_extension_controller_copy_configuration(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let ptr, #available(macOS 15.4, *) else { return nil }
    let box: WKWebExtensionControllerBox = wkBorrow(ptr)
    return wkRetain(
        WKWebExtensionControllerConfigurationBox(configuration: box.controller.configuration)
    )
}

@_cdecl("wk_web_extension_controller_load_context")
public func wk_web_extension_controller_load_context(
    _ ptr: UnsafeMutableRawPointer?,
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let contextPtr, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing web extension controller or context")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionControllerBox = wkBorrow(ptr)
    let contextBox: WKWebExtensionContextBox = wkBorrow(contextPtr)
    do {
        try box.controller.load(contextBox.context)
        return WK_OK
    } catch {
        outErr?.pointee = wkCString(error.localizedDescription)
        return WK_FRAMEWORK_ERROR
    }
}

@_cdecl("wk_web_extension_controller_unload_context")
public func wk_web_extension_controller_unload_context(
    _ ptr: UnsafeMutableRawPointer?,
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let contextPtr, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing web extension controller or context")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionControllerBox = wkBorrow(ptr)
    let contextBox: WKWebExtensionContextBox = wkBorrow(contextPtr)
    do {
        try box.controller.unload(contextBox.context)
        return WK_OK
    } catch {
        outErr?.pointee = wkCString(error.localizedDescription)
        return WK_FRAMEWORK_ERROR
    }
}

@_cdecl("wk_web_extension_controller_copy_context_for_extension")
public func wk_web_extension_controller_copy_context_for_extension(
    _ ptr: UnsafeMutableRawPointer?,
    _ extensionPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let ptr, let extensionPtr, #available(macOS 15.4, *) else { return nil }
    let box: WKWebExtensionControllerBox = wkBorrow(ptr)
    let extensionBox: WKWebExtensionBox = wkBorrow(extensionPtr)
    guard let context = box.controller.extensionContext(for: extensionBox.webExtension) else {
        return nil
    }
    return wkRetain(WKWebExtensionContextBox(context: context))
}

@_cdecl("wk_web_extension_controller_copy_context_for_url")
public func wk_web_extension_controller_copy_context_for_url(
    _ ptr: UnsafeMutableRawPointer?,
    _ url: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let ptr,
          let url,
          let parsedURL = URL(string: String(cString: url)),
          #available(macOS 15.4, *)
    else {
        return nil
    }
    let box: WKWebExtensionControllerBox = wkBorrow(ptr)
    guard let context = box.controller.extensionContext(for: parsedURL) else {
        return nil
    }
    return wkRetain(WKWebExtensionContextBox(context: context))
}

@_cdecl("wk_web_extension_controller_copy_all_data_types_json")
public func wk_web_extension_controller_copy_all_data_types_json() -> UnsafeMutablePointer<CChar>? {
    guard #available(macOS 15.4, *) else { return wkCString("[]") }
    return wkCString(wkJSONString(wkWebExtensionDataTypeArray(WKWebExtensionController.allExtensionDataTypes)))
}

@_cdecl("wk_web_extension_controller_copy_data_records_json")
public func wk_web_extension_controller_copy_data_records_json(
    _ ptr: UnsafeMutableRawPointer?,
    _ dataTypesJson: UnsafePointer<CChar>?,
    _ outJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing web extension controller")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionControllerBox = wkBorrow(ptr)
    let dataTypes = wkWebExtensionDataTypeSet(from: dataTypesJson)
    let (status, records, error): (Int32, [WKWebExtension.DataRecord]?, String?) = wkWaitForAsync { completion in
        Task { @MainActor in
            let records = await box.controller.dataRecords(ofTypes: dataTypes)
            completion(records, nil)
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    outJson?.pointee = wkCString(
        wkJSONString(records?.map(wkWebExtensionDataRecordDictionary) ?? [])
    )
    return status
}

@_cdecl("wk_web_extension_controller_copy_data_record_json_for_context")
public func wk_web_extension_controller_copy_data_record_json_for_context(
    _ ptr: UnsafeMutableRawPointer?,
    _ dataTypesJson: UnsafePointer<CChar>?,
    _ contextPtr: UnsafeMutableRawPointer?,
    _ outJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let contextPtr, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing web extension controller or context")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionControllerBox = wkBorrow(ptr)
    let contextBox: WKWebExtensionContextBox = wkBorrow(contextPtr)
    let dataTypes = wkWebExtensionDataTypeSet(from: dataTypesJson)
    let (status, record, error): (Int32, WKWebExtension.DataRecord?, String?) = wkWaitForAsync { completion in
        Task { @MainActor in
            let record = await box.controller.dataRecord(ofTypes: dataTypes, for: contextBox.context)
            completion(record, nil)
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    outJson?.pointee = wkCString(
        wkJSONString(record.map(wkWebExtensionDataRecordDictionary) ?? NSNull())
    )
    return status
}

@_cdecl("wk_web_extension_controller_remove_data_for_identifiers")
public func wk_web_extension_controller_remove_data_for_identifiers(
    _ ptr: UnsafeMutableRawPointer?,
    _ dataTypesJson: UnsafePointer<CChar>?,
    _ identifiersJson: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing web extension controller")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionControllerBox = wkBorrow(ptr)
    let dataTypes = wkWebExtensionDataTypeSet(from: dataTypesJson)
    let identifiers = Set(wkStringArray(from: identifiersJson))
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        Task { @MainActor in
            let records = await box.controller.dataRecords(ofTypes: dataTypes)
            let filtered = records.filter { identifiers.contains($0.uniqueIdentifier) }
            await box.controller.removeData(ofTypes: dataTypes, from: filtered)
            completion(true, nil)
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_web_extension_context_new_for_extension")
public func wk_web_extension_context_new_for_extension(
    _ extensionPtr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let extensionPtr, #available(macOS 15.4, *) else { return nil }
    let extensionBox: WKWebExtensionBox = wkBorrow(extensionPtr)
    return wkRetain(WKWebExtensionContextBox(context: WKWebExtensionContext(for: extensionBox.webExtension)))
}

@_cdecl("wk_web_extension_context_release")
public func wk_web_extension_context_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_web_extension_context_copy_summary_json")
public func wk_web_extension_context_copy_summary_json(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let ptr, #available(macOS 15.4, *) else { return wkCString("{}") }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    return wkCString(wkJSONString(wkWebExtensionContextSummaryDictionary(box.context)))
}

@_cdecl("wk_web_extension_context_set_base_url")
public func wk_web_extension_context_set_base_url(
    _ ptr: UnsafeMutableRawPointer?,
    _ url: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let url, let parsedURL = URL(string: String(cString: url)), #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("invalid base URL")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    box.context.baseURL = parsedURL
    return WK_OK
}

@_cdecl("wk_web_extension_context_set_unique_identifier")
public func wk_web_extension_context_set_unique_identifier(
    _ ptr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let identifier, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing web extension unique identifier")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    box.context.uniqueIdentifier = String(cString: identifier)
    return WK_OK
}

@_cdecl("wk_web_extension_context_set_inspectable")
public func wk_web_extension_context_set_inspectable(
    _ ptr: UnsafeMutableRawPointer?,
    _ value: Bool
) {
    guard let ptr, #available(macOS 15.4, *) else { return }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    box.context.isInspectable = value
}

@_cdecl("wk_web_extension_context_set_inspection_name")
public func wk_web_extension_context_set_inspection_name(
    _ ptr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) {
    guard let ptr, #available(macOS 15.4, *) else { return }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    box.context.inspectionName = name.map(String.init(cString:))
}

@_cdecl("wk_web_extension_context_set_unsupported_apis_json")
public func wk_web_extension_context_set_unsupported_apis_json(
    _ ptr: UnsafeMutableRawPointer?,
    _ apisJson: UnsafePointer<CChar>?
) {
    guard let ptr, #available(macOS 15.4, *) else { return }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    box.context.unsupportedAPIs = Set(wkStringArray(from: apisJson))
}

@_cdecl("wk_web_extension_context_set_requested_optional_access_to_all_hosts")
public func wk_web_extension_context_set_requested_optional_access_to_all_hosts(
    _ ptr: UnsafeMutableRawPointer?,
    _ value: Bool
) {
    guard let ptr, #available(macOS 15.4, *) else { return }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    box.context.hasRequestedOptionalAccessToAllHosts = value
}

@_cdecl("wk_web_extension_context_set_access_to_private_data")
public func wk_web_extension_context_set_access_to_private_data(
    _ ptr: UnsafeMutableRawPointer?,
    _ value: Bool
) {
    guard let ptr, #available(macOS 15.4, *) else { return }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    box.context.hasAccessToPrivateData = value
}

@_cdecl("wk_web_extension_context_copy_webview_configuration")
public func wk_web_extension_context_copy_webview_configuration(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let ptr, #available(macOS 15.4, *) else { return nil }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    guard let configuration = box.context.webViewConfiguration else {
        return nil
    }
    return wkRetain(WKConfigBox(configuration: configuration))
}

@_cdecl("wk_web_extension_context_has_permission")
public func wk_web_extension_context_has_permission(
    _ ptr: UnsafeMutableRawPointer?,
    _ permission: UnsafePointer<CChar>?
) -> Bool {
    guard let ptr, let permission, #available(macOS 15.4, *) else { return false }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    return box.context.hasPermission(wkWebExtensionPermission(String(cString: permission)))
}

@_cdecl("wk_web_extension_context_has_access_to_url")
public func wk_web_extension_context_has_access_to_url(
    _ ptr: UnsafeMutableRawPointer?,
    _ url: UnsafePointer<CChar>?
) -> Bool {
    guard let ptr,
          let url,
          let parsedURL = URL(string: String(cString: url)),
          #available(macOS 15.4, *)
    else {
        return false
    }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    return box.context.hasAccess(to: parsedURL)
}

@_cdecl("wk_web_extension_context_permission_status_for_permission")
public func wk_web_extension_context_permission_status_for_permission(
    _ ptr: UnsafeMutableRawPointer?,
    _ permission: UnsafePointer<CChar>?
) -> Int64 {
    guard let ptr, let permission, #available(macOS 15.4, *) else { return 0 }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    return Int64(box.context.permissionStatus(for: wkWebExtensionPermission(String(cString: permission))).rawValue)
}

@_cdecl("wk_web_extension_context_set_permission_status_for_permission")
public func wk_web_extension_context_set_permission_status_for_permission(
    _ ptr: UnsafeMutableRawPointer?,
    _ status: Int64,
    _ permission: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let permission, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing web extension context permission")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    box.context.setPermissionStatus(
        wkWebExtensionPermissionStatus(status),
        for: wkWebExtensionPermission(String(cString: permission))
    )
    return WK_OK
}

@_cdecl("wk_web_extension_context_permission_status_for_url")
public func wk_web_extension_context_permission_status_for_url(
    _ ptr: UnsafeMutableRawPointer?,
    _ url: UnsafePointer<CChar>?
) -> Int64 {
    guard let ptr,
          let url,
          let parsedURL = URL(string: String(cString: url)),
          #available(macOS 15.4, *)
    else {
        return 0
    }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    return Int64(box.context.permissionStatus(for: parsedURL).rawValue)
}

@_cdecl("wk_web_extension_context_set_permission_status_for_url")
public func wk_web_extension_context_set_permission_status_for_url(
    _ ptr: UnsafeMutableRawPointer?,
    _ status: Int64,
    _ url: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr,
          let url,
          let parsedURL = URL(string: String(cString: url)),
          #available(macOS 15.4, *)
    else {
        outErr?.pointee = wkCString("invalid URL permission target")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    box.context.setPermissionStatus(wkWebExtensionPermissionStatus(status), for: parsedURL)
    return WK_OK
}

@_cdecl("wk_web_extension_context_permission_status_for_match_pattern")
public func wk_web_extension_context_permission_status_for_match_pattern(
    _ ptr: UnsafeMutableRawPointer?,
    _ patternPtr: UnsafeMutableRawPointer?
) -> Int64 {
    guard let ptr, let patternPtr, #available(macOS 15.4, *) else { return 0 }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    let patternBox: WKWebExtensionMatchPatternBox = wkBorrow(patternPtr)
    return Int64(box.context.permissionStatus(for: patternBox.matchPattern).rawValue)
}

@_cdecl("wk_web_extension_context_set_permission_status_for_match_pattern")
public func wk_web_extension_context_set_permission_status_for_match_pattern(
    _ ptr: UnsafeMutableRawPointer?,
    _ status: Int64,
    _ patternPtr: UnsafeMutableRawPointer?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let patternPtr, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing match pattern permission target")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    let patternBox: WKWebExtensionMatchPatternBox = wkBorrow(patternPtr)
    box.context.setPermissionStatus(wkWebExtensionPermissionStatus(status), for: patternBox.matchPattern)
    return WK_OK
}

@_cdecl("wk_web_extension_context_load_background_content")
public func wk_web_extension_context_load_background_content(
    _ ptr: UnsafeMutableRawPointer?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing web extension context")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        Task { @MainActor in
            do {
                try await box.context.loadBackgroundContent()
                completion(true, nil)
            } catch {
                completion(nil, error.localizedDescription)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_web_extension_context_copy_default_action_json")
public func wk_web_extension_context_copy_default_action_json(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let ptr, #available(macOS 15.4, *) else { return wkCString("null") }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    guard let action = box.context.action(for: nil) else {
        return wkCString("null")
    }
    return wkCString(wkJSONString(wkWebExtensionActionDictionary(action)))
}

@_cdecl("wk_web_extension_context_perform_default_action")
public func wk_web_extension_context_perform_default_action(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr, #available(macOS 15.4, *) else { return }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    box.context.performAction(for: nil)
}

@_cdecl("wk_web_extension_context_copy_commands_json")
public func wk_web_extension_context_copy_commands_json(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let ptr, #available(macOS 15.4, *) else { return wkCString("[]") }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    return wkCString(wkJSONString(box.context.commands.map(wkWebExtensionCommandDictionary)))
}

@_cdecl("wk_web_extension_context_perform_command_for_identifier")
public func wk_web_extension_context_perform_command_for_identifier(
    _ ptr: UnsafeMutableRawPointer?,
    _ identifier: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, let identifier, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing web extension command identifier")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionContextBox = wkBorrow(ptr)
    let identifierString = String(cString: identifier)
    guard let command = box.context.commands.first(where: { $0.id == identifierString }) else {
        outErr?.pointee = wkCString("unknown web extension command identifier")
        return WK_INVALID_ARGUMENT
    }
    box.context.performCommand(command)
    return WK_OK
}

@_cdecl("wk_web_extension_message_port_release")
public func wk_web_extension_message_port_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    wkRelease(ptr)
}

@_cdecl("wk_web_extension_message_port_copy_application_identifier")
public func wk_web_extension_message_port_copy_application_identifier(
    _ ptr: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let ptr, #available(macOS 15.4, *) else { return nil }
    let box: WKWebExtensionMessagePortBox = wkBorrow(ptr)
    return box.port.applicationIdentifier.flatMap(wkCString)
}

@_cdecl("wk_web_extension_message_port_is_disconnected")
public func wk_web_extension_message_port_is_disconnected(_ ptr: UnsafeMutableRawPointer?) -> Bool {
    guard let ptr, #available(macOS 15.4, *) else { return true }
    let box: WKWebExtensionMessagePortBox = wkBorrow(ptr)
    return box.port.isDisconnected
}

@_cdecl("wk_web_extension_message_port_send_message_json")
public func wk_web_extension_message_port_send_message_json(
    _ ptr: UnsafeMutableRawPointer?,
    _ messageJson: UnsafePointer<CChar>?,
    _ outErr: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let ptr, #available(macOS 15.4, *) else {
        outErr?.pointee = wkCString("missing web extension message port")
        return WK_INVALID_ARGUMENT
    }
    let box: WKWebExtensionMessagePortBox = wkBorrow(ptr)
    let message = wkJSONObject(from: messageJson)
    let (status, _, error): (Int32, Bool?, String?) = wkWaitForAsync { completion in
        Task { @MainActor in
            do {
                try await box.port.sendMessage(message)
                completion(true, nil)
            } catch {
                completion(nil, error.localizedDescription)
            }
        }
    }
    if let error {
        outErr?.pointee = wkCString(error)
    }
    return status
}

@_cdecl("wk_web_extension_message_port_disconnect")
public func wk_web_extension_message_port_disconnect(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr, #available(macOS 15.4, *) else { return }
    let box: WKWebExtensionMessagePortBox = wkBorrow(ptr)
    box.port.disconnect()
}

@_cdecl("wk_web_extension_message_port_disconnect_with_error")
public func wk_web_extension_message_port_disconnect_with_error(
    _ ptr: UnsafeMutableRawPointer?,
    _ message: UnsafePointer<CChar>?
) {
    guard let ptr, #available(macOS 15.4, *) else { return }
    let box: WKWebExtensionMessagePortBox = wkBorrow(ptr)
    let error = NSError(
        domain: WKWebExtension.MessagePort.errorDomain,
        code: WKWebExtension.MessagePort.Error.unknown.rawValue,
        userInfo: [NSLocalizedDescriptionKey: message.map(String.init(cString:)) ?? "Disconnected"]
    )
    box.port.disconnect(throwing: error)
}
