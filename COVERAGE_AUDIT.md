# webkit coverage audit (vs MacOSX26.5.sdk)

Full top-level symbol audit of WebKit.framework headers after filtering out declarations unavailable on macOS. Legacy DOM* / Web* APIs remain listed as EXEMPT because Apple deprecated them on macOS and this crate intentionally targets the modern WK* surface. Member-level MacOSX26.5 completeness closures are tracked in `COVERAGE.md`.

SDK_PUBLIC_SYMBOLS: 367
VERIFIED: 134
GAPS: 0
EXEMPT: 233
COVERAGE_PCT: 100.0%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| WKBackForwardList | interface | WKBackForwardList.h | BackForwardList |
| WKBackForwardListItem | interface | WKBackForwardListItem.h | BackForwardListItem |
| WKContentRuleList | interface | WKContentRuleList.h | ContentRuleList |
| WKContentRuleListStore | interface | WKContentRuleListStore.h | ContentRuleListStore |
| WKContentWorld | interface | WKContentWorld.h | UserScript::with_content_world |
| WKDownload | interface | WKDownload.h | Download |
| WKDownloadDelegate | protocol | WKDownloadDelegate.h | DownloadEvent |
| WKCookiePolicy | enum | WKHTTPCookieStore.h | CookiePolicy |
| WKHTTPCookieStore | interface | WKHTTPCookieStore.h | HttpCookieStore |
| WKHTTPCookieStoreObserver | protocol | WKHTTPCookieStore.h | HttpCookieStore::{start_observing, drain_events} |
| WKFindConfiguration | interface | WKFindConfiguration.h | FindConfiguration, WebView::find_string_with_configuration |
| WKFindResult | interface | WKFindResult.h | FindResult, WebView::find_string |
| WKFrameInfo | interface | WKFrameInfo.h | FrameInfo |
| WKNavigation | interface | WKNavigation.h | Navigation |
| WKNavigationAction | interface | WKNavigationAction.h | NavigationAction |
| WKNavigationActionPolicy | enum | WKNavigationDelegate.h | NavigationActionPolicy |
| WKNavigationDelegate | protocol | WKNavigationDelegate.h | NavigationDelegateConfig, NavigationEvent |
| WKNavigationResponse | interface | WKNavigationResponse.h | NavigationResponse |
| WKNavigationResponsePolicy | enum | WKNavigationDelegate.h | NavigationResponsePolicy |
| WKNavigationType | enum | WKNavigationAction.h | NavigationType |
| WKPDFConfiguration | interface | WKPDFConfiguration.h | PDFConfiguration |
| WKInactiveSchedulingPolicy | enum | WKPreferences.h | InactiveSchedulingPolicy |
| WKPreferences | interface | WKPreferences.h | Preferences |
| WKScriptMessage | interface | WKScriptMessage.h | ScriptMessage |
| WKScriptMessageHandler | protocol | WKScriptMessageHandler.h | WebView::set_message_handler, ScriptMessage |
| WKSnapshotConfiguration | interface | WKSnapshotConfiguration.h | SnapshotConfiguration |
| WKUIDelegate | protocol | WKUIDelegate.h | UIDelegateConfig, UIDelegateEvent |
| WKURLSchemeHandler | protocol | WKURLSchemeHandler.h | UrlSchemeHandler, WebViewConfiguration::set_url_scheme_handler |
| WKURLSchemeTask | protocol | WKURLSchemeTask.h | UrlSchemeTask |
| WKUserContentController | interface | WKUserContentController.h | WebViewConfiguration::{add_user_script, add_content_rule_list, add_message_handler} |
| WKUserScript | interface | WKUserScript.h | UserScript |
| WKUserScriptInjectionTime | enum | WKUserScript.h | InjectionTime |
| WKWebExtension | interface | WKWebExtension.h | WebExtension |
| WKWebExtensionAction | interface | WKWebExtensionAction.h | WebExtensionAction, WebExtensionContext::action |
| WKWebExtensionCommand | interface | WKWebExtensionCommand.h | WebExtensionCommand, WebExtensionContext::commands |
| WKWebExtensionContext | interface | WKWebExtensionContext.h | WebExtensionContext |
| WKWebExtensionContextDeniedPermissionMatchPatternsWereRemovedNotification | constant | WKWebExtensionContext.h | WebExtensionContextNotifications::denied_permission_match_patterns_were_removed() |
| WKWebExtensionContextDeniedPermissionsWereRemovedNotification | constant | WKWebExtensionContext.h | WebExtensionContextNotifications::denied_permissions_were_removed() |
| WKWebExtensionContextError | enum | WKWebExtensionContext.h | WebExtensionContextError |
| WKWebExtensionContextErrorDomain | constant | WKWebExtensionContext.h | WebExtensionContextError::domain() |
| WKWebExtensionContextErrorsDidUpdateNotification | constant | WKWebExtensionContext.h | WebExtensionContextNotifications::errors_did_update() |
| WKWebExtensionContextGrantedPermissionMatchPatternsWereRemovedNotification | constant | WKWebExtensionContext.h | WebExtensionContextNotifications::granted_permission_match_patterns_were_removed() |
| WKWebExtensionContextGrantedPermissionsWereRemovedNotification | constant | WKWebExtensionContext.h | WebExtensionContextNotifications::granted_permissions_were_removed() |
| WKWebExtensionContextNotificationUserInfoKey | typealias | WKWebExtensionContext.h | WebExtensionContextNotificationUserInfoKey |
| WKWebExtensionContextNotificationUserInfoKeyMatchPatterns | constant | WKWebExtensionContext.h | WebExtensionContextNotificationUserInfoKey::match_patterns() |
| WKWebExtensionContextNotificationUserInfoKeyPermissions | constant | WKWebExtensionContext.h | WebExtensionContextNotificationUserInfoKey::permissions() |
| WKWebExtensionContextPermissionMatchPatternsWereDeniedNotification | constant | WKWebExtensionContext.h | WebExtensionContextNotifications::permission_match_patterns_were_denied() |
| WKWebExtensionContextPermissionMatchPatternsWereGrantedNotification | constant | WKWebExtensionContext.h | WebExtensionContextNotifications::permission_match_patterns_were_granted() |
| WKWebExtensionContextPermissionStatus | enum | WKWebExtensionContext.h | WebExtensionContextPermissionStatus |
| WKWebExtensionContextPermissionsWereDeniedNotification | constant | WKWebExtensionContext.h | WebExtensionContextNotifications::permissions_were_denied() |
| WKWebExtensionContextPermissionsWereGrantedNotification | constant | WKWebExtensionContext.h | WebExtensionContextNotifications::permissions_were_granted() |
| WKWebExtensionController | interface | WKWebExtensionController.h | WebExtensionController |
| WKWebExtensionControllerConfiguration | interface | WKWebExtensionControllerConfiguration.h | WebExtensionControllerConfiguration |
| WKWebExtensionControllerDelegate | protocol | WKWebExtensionControllerDelegate.h | WebExtensionControllerDelegate |
| WKWebExtensionDataRecord | interface | WKWebExtensionDataRecord.h | WebExtensionDataRecord |
| WKWebExtensionDataRecordError | enum | WKWebExtensionDataRecord.h | WebExtensionDataRecordError |
| WKWebExtensionDataRecordErrorDomain | constant | WKWebExtensionDataRecord.h | WebExtensionDataRecordError::domain() |
| WKWebExtensionDataType | typealias | WKWebExtensionDataType.h | WebExtensionDataType |
| WKWebExtensionDataTypeLocal | constant | WKWebExtensionDataType.h | WebExtensionDataType::local() |
| WKWebExtensionDataTypeSession | constant | WKWebExtensionDataType.h | WebExtensionDataType::session() |
| WKWebExtensionDataTypeSynchronized | constant | WKWebExtensionDataType.h | WebExtensionDataType::synchronized() |
| WKWebExtensionError | enum | WKWebExtension.h | WebExtensionError |
| WKWebExtensionErrorDomain | constant | WKWebExtension.h | WebExtensionError::domain() |
| WKWebExtensionMatchPattern | interface | WKWebExtensionMatchPattern.h | WebExtensionMatchPattern |
| WKWebExtensionMatchPatternError | enum | WKWebExtensionMatchPattern.h | WebExtensionMatchPatternError |
| WKWebExtensionMatchPatternErrorDomain | constant | WKWebExtensionMatchPattern.h | WebExtensionMatchPatternError::domain() |
| WKWebExtensionMatchPatternOptions | enum | WKWebExtensionMatchPattern.h | WebExtensionMatchPatternOptions |
| WKWebExtensionMessagePort | interface | WKWebExtensionMessagePort.h | WebExtensionMessagePort |
| WKWebExtensionMessagePortError | enum | WKWebExtensionMessagePort.h | WebExtensionMessagePortError |
| WKWebExtensionMessagePortErrorDomain | constant | WKWebExtensionMessagePort.h | WebExtensionMessagePortError::domain() |
| WKWebExtensionPermission | typealias | WKWebExtensionPermission.h | WebExtensionPermission |
| WKWebExtensionPermissionActiveTab | constant | WKWebExtensionPermission.h | WebExtensionPermission::active_tab() |
| WKWebExtensionPermissionAlarms | constant | WKWebExtensionPermission.h | WebExtensionPermission::alarms() |
| WKWebExtensionPermissionClipboardWrite | constant | WKWebExtensionPermission.h | WebExtensionPermission::clipboard_write() |
| WKWebExtensionPermissionContextMenus | constant | WKWebExtensionPermission.h | WebExtensionPermission::context_menus() |
| WKWebExtensionPermissionCookies | constant | WKWebExtensionPermission.h | WebExtensionPermission::cookies() |
| WKWebExtensionPermissionDeclarativeNetRequest | constant | WKWebExtensionPermission.h | WebExtensionPermission::declarative_net_request() |
| WKWebExtensionPermissionDeclarativeNetRequestFeedback | constant | WKWebExtensionPermission.h | WebExtensionPermission::declarative_net_request_feedback() |
| WKWebExtensionPermissionDeclarativeNetRequestWithHostAccess | constant | WKWebExtensionPermission.h | WebExtensionPermission::declarative_net_request_with_host_access() |
| WKWebExtensionPermissionMenus | constant | WKWebExtensionPermission.h | WebExtensionPermission::menus() |
| WKWebExtensionPermissionNativeMessaging | constant | WKWebExtensionPermission.h | WebExtensionPermission::native_messaging() |
| WKWebExtensionPermissionScripting | constant | WKWebExtensionPermission.h | WebExtensionPermission::scripting() |
| WKWebExtensionPermissionStorage | constant | WKWebExtensionPermission.h | WebExtensionPermission::storage() |
| WKWebExtensionPermissionTabs | constant | WKWebExtensionPermission.h | WebExtensionPermission::tabs() |
| WKWebExtensionPermissionUnlimitedStorage | constant | WKWebExtensionPermission.h | WebExtensionPermission::unlimited_storage() |
| WKWebExtensionPermissionWebNavigation | constant | WKWebExtensionPermission.h | WebExtensionPermission::web_navigation() |
| WKWebExtensionPermissionWebRequest | constant | WKWebExtensionPermission.h | WebExtensionPermission::web_request() |
| WKWebExtensionTab | protocol | WKWebExtensionTab.h | WebExtensionTab |
| WKWebExtensionTabChangedProperties | enum | WKWebExtensionTab.h | WebExtensionTabChangedProperties |
| WKWebExtensionTabConfiguration | interface | WKWebExtensionTabConfiguration.h | WebExtensionTabConfiguration |
| WKWebExtensionWindow | protocol | WKWebExtensionWindow.h | WebExtensionWindow |
| WKWebExtensionWindowConfiguration | interface | WKWebExtensionWindowConfiguration.h | WebExtensionWindowConfiguration |
| WKWebExtensionWindowState | enum | WKWebExtensionWindow.h | WebExtensionWindowState |
| WKWebExtensionWindowType | enum | WKWebExtensionWindow.h | WebExtensionWindowType |
| WKWebView | interface | WKWebView.h | WebView |
| WKWebViewConfiguration | interface | WKWebViewConfiguration.h | WebViewConfiguration |
| WKWebpagePreferences | interface | WKWebpagePreferences.h | Preferences, UpgradeToHTTPSPolicy |
| WKWebpagePreferencesUpgradeToHTTPSPolicy | enum | WKWebpagePreferences.h | UpgradeToHTTPSPolicy |
| WKWebsiteDataRecord | interface | WKWebsiteDataRecord.h | WebsiteDataRecord |
| WKWebsiteDataTypeCookies | constant | WKWebsiteDataRecord.h | WebsiteDataType::cookies() |
| WKWebsiteDataTypeDiskCache | constant | WKWebsiteDataRecord.h | WebsiteDataType::disk_cache() |
| WKWebsiteDataTypeFetchCache | constant | WKWebsiteDataRecord.h | WebsiteDataType::fetch_cache() |
| WKWebsiteDataTypeFileSystem | constant | WKWebsiteDataRecord.h | WebsiteDataType::file_system() |
| WKWebsiteDataTypeHashSalt | constant | WKWebsiteDataRecord.h | WebsiteDataType::hash_salt() |
| WKWebsiteDataTypeIndexedDBDatabases | constant | WKWebsiteDataRecord.h | WebsiteDataType::indexed_db_databases() |
| WKWebsiteDataTypeLocalStorage | constant | WKWebsiteDataRecord.h | WebsiteDataType::local_storage() |
| WKWebsiteDataTypeMediaKeys | constant | WKWebsiteDataRecord.h | WebsiteDataType::media_keys() |
| WKWebsiteDataTypeMemoryCache | constant | WKWebsiteDataRecord.h | WebsiteDataType::memory_cache() |
| WKWebsiteDataTypeScreenTime | constant | WKWebsiteDataRecord.h | WebsiteDataType::screen_time() |
| WKWebsiteDataTypeSearchFieldRecentSearches | constant | WKWebsiteDataRecord.h | WebsiteDataType::search_field_recent_searches() |
| WKWebsiteDataTypeServiceWorkerRegistrations | constant | WKWebsiteDataRecord.h | WebsiteDataType::service_worker_registrations() |
| WKWebsiteDataTypeSessionStorage | constant | WKWebsiteDataRecord.h | WebsiteDataType::session_storage() |
| WKWebsiteDataTypeWebSQLDatabases | constant | WKWebsiteDataRecord.h | WebsiteDataType::web_sql_databases() |
| WKWebsiteDataStore | interface | WKWebsiteDataStore.h | WebsiteDataStore |
| NSAttributedString (NSAttributedStringWebKitAdditions) | category | NSAttributedString.h | AttributedString::{load_from_html_request, load_from_html_file, load_from_html_string, load_from_html_data} |
| NSAttributedStringCompletionHandler | typealias | NSAttributedString.h | AttributedStringCompletionHandler |
| NSReadAccessURLDocumentOption | constant | NSAttributedString.h | READ_ACCESS_URL_DOCUMENT_OPTION, AttributedStringLoadOptions::with_read_access_url |
| WKDownloadRedirectPolicy | enum | WKDownloadDelegate.h | DownloadRedirectPolicy, Download::{set_redirect_policy, redirect_policy} |
| WKErrorCode | enum | WKError.h | WebKitErrorCode |
| WKErrorDomain | constant | WKError.h | WEBKIT_ERROR_DOMAIN, WebKitErrorCode::domain() |
| WKOpenPanelParameters | interface | WKOpenPanelParameters.h | OpenPanelParameters, UIDelegateEventDetail::open_panel_parameters |
| WKScriptMessageHandlerWithReply | protocol | WKScriptMessageHandlerWithReply.h | WebViewConfiguration::add_message_handler_with_reply, WebView::set_message_handler_with_reply |
| WKSecurityOrigin | interface | WKSecurityOrigin.h | SecurityOrigin, UIDelegateEventDetail::security_origin |
| WKMediaCaptureType | enum | WKUIDelegate.h | MediaCaptureType, UIDelegateEventDetail::media_capture_type |
| WKPermissionDecision | enum | WKUIDelegate.h | PermissionDecision, UIDelegateEventDetail::permission_decision |
| WKFullscreenState | enum | WKWebView.h | FullscreenState, WebView::fullscreen_state |
| WKMediaCaptureState | enum | WKWebView.h | MediaCaptureState, WebView::{camera_capture_state, microphone_capture_state, set_camera_capture_state, set_microphone_capture_state} |
| WKMediaPlaybackState | enum | WKWebView.h | MediaPlaybackState, WebView::request_media_playback_state |
| WKWebView (WKIBActions) | category | WKWebView.h | WebView::{perform_go_back_action, perform_go_forward_action, perform_reload_action, perform_reload_from_origin_action, perform_stop_loading_action} |
| WKWebView (WKNSTextFinderClient) | category | WKWebView.h | TextFinderAction, WebView::{can_perform_text_finder_action, perform_text_finder_action} |
| WKWebViewDataType | enum | WKWebView.h | WebViewDataType, WebView::{fetch_data_of_types, restore_data} |
| WKAudiovisualMediaTypes | enum | WKWebViewConfiguration.h | AudiovisualMediaTypes, WebViewConfiguration::{set_media_types_requiring_user_action_for_playback, media_types_requiring_user_action_for_playback} |
| WKUserInterfaceDirectionPolicy | enum | WKWebViewConfiguration.h | UserInterfaceDirectionPolicy, WebViewConfiguration::{set_user_interface_direction_policy, user_interface_direction_policy} |
| WKWindowFeatures | interface | WKWindowFeatures.h | WindowFeatures, UIDelegateEventDetail::window_features |

## 🔴 GAPS
None.

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| DOMAbstractView | interface | DOMAbstractView.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMAttr | interface | DOMAttr.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMBlob | interface | DOMBlob.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_6, 10_14) |
| DOMCDATASection | interface | DOMCDATASection.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSStyleDeclaration (DOMCSS2Properties) | category | DOMCSS.h | Legacy DOM API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| DOMCSSCharsetRule | interface | DOMCSSCharsetRule.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSFontFaceRule | interface | DOMCSSFontFaceRule.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSImportRule | interface | DOMCSSImportRule.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSMediaRule | interface | DOMCSSMediaRule.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSMediaRule (DOMCSSMediaRuleDeprecated) | category | DOMCSSMediaRule.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSPageRule | interface | DOMCSSPageRule.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSPrimitiveValue | interface | DOMCSSPrimitiveValue.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSPrimitiveValue (DOMCSSPrimitiveValueDeprecated) | category | DOMCSSPrimitiveValue.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSRule | interface | DOMCSSRule.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSRuleList | interface | DOMCSSRuleList.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSStyleDeclaration | interface | DOMCSSStyleDeclaration.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSStyleDeclaration (DOMCSSStyleDeclarationDeprecated) | category | DOMCSSStyleDeclaration.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_5, 10_5) |
| DOMCSSStyleRule | interface | DOMCSSStyleRule.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSStyleSheet | interface | DOMCSSStyleSheet.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSStyleSheet (DOMCSSStyleSheetDeprecated) | category | DOMCSSStyleSheet.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSUnknownRule | interface | DOMCSSUnknownRule.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSValue | interface | DOMCSSValue.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMCSSValueList | interface | DOMCSSValueList.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCharacterData | interface | DOMCharacterData.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCharacterData (DOMCharacterDataDeprecated) | category | DOMCharacterData.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMComment | interface | DOMComment.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMCounter | interface | DOMCounter.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMDocument | interface | DOMDocument.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMDocument (DOMDocumentDeprecated) | category | DOMDocument.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMDocumentFragment | interface | DOMDocumentFragment.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMDocumentType | interface | DOMDocumentType.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMElement | interface | DOMElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMElement (DOMElementDeprecated) | category | DOMElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMEntity | interface | DOMEntity.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMEntityReference | interface | DOMEntityReference.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMEvent | interface | DOMEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMEvent (DOMEventDeprecated) | category | DOMEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMEventException | constant | DOMEventException.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_4, 10_14) |
| DOMEventListener | protocol | DOMEventListener.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMEventTarget | protocol | DOMEventTarget.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMException | constant | DOMException.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_4, 10_14) |
| DOMElement (DOMElementAppKitExtensions) | category | DOMExtensions.h | Legacy DOM API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| DOMHTMLDocument (DOMHTMLDocumentExtensions) | category | DOMExtensions.h | Legacy DOM API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| DOMNode (DOMNodeExtensions) | category | DOMExtensions.h | Legacy DOM API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| DOMFile | interface | DOMFile.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_6, 10_14) |
| DOMFileList | interface | DOMFileList.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_6, 10_14) |
| DOMHTMLAnchorElement | interface | DOMHTMLAnchorElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLAppletElement | interface | DOMHTMLAppletElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLAreaElement | interface | DOMHTMLAreaElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLBRElement | interface | DOMHTMLBRElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLBaseElement | interface | DOMHTMLBaseElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLBaseFontElement | interface | DOMHTMLBaseFontElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLBodyElement | interface | DOMHTMLBodyElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLButtonElement | interface | DOMHTMLButtonElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLCollection | interface | DOMHTMLCollection.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLDListElement | interface | DOMHTMLDListElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLDirectoryElement | interface | DOMHTMLDirectoryElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLDivElement | interface | DOMHTMLDivElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLDocument | interface | DOMHTMLDocument.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLElement | interface | DOMHTMLElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLEmbedElement | interface | DOMHTMLEmbedElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLFieldSetElement | interface | DOMHTMLFieldSetElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLFontElement | interface | DOMHTMLFontElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLFormElement | interface | DOMHTMLFormElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLFrameElement | interface | DOMHTMLFrameElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLFrameSetElement | interface | DOMHTMLFrameSetElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLHRElement | interface | DOMHTMLHRElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLHeadElement | interface | DOMHTMLHeadElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLHeadingElement | interface | DOMHTMLHeadingElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLHtmlElement | interface | DOMHTMLHtmlElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLIFrameElement | interface | DOMHTMLIFrameElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLImageElement | interface | DOMHTMLImageElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLInputElement | interface | DOMHTMLInputElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLLIElement | interface | DOMHTMLLIElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLLabelElement | interface | DOMHTMLLabelElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLLegendElement | interface | DOMHTMLLegendElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLLinkElement | interface | DOMHTMLLinkElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLMapElement | interface | DOMHTMLMapElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLMarqueeElement | interface | DOMHTMLMarqueeElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_5, 10_14) |
| DOMHTMLMenuElement | interface | DOMHTMLMenuElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLMetaElement | interface | DOMHTMLMetaElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLModElement | interface | DOMHTMLModElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLOListElement | interface | DOMHTMLOListElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLObjectElement | interface | DOMHTMLObjectElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLOptGroupElement | interface | DOMHTMLOptGroupElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLOptionElement | interface | DOMHTMLOptionElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLOptionsCollection | interface | DOMHTMLOptionsCollection.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLParagraphElement | interface | DOMHTMLParagraphElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLParamElement | interface | DOMHTMLParamElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLPreElement | interface | DOMHTMLPreElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLQuoteElement | interface | DOMHTMLQuoteElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLScriptElement | interface | DOMHTMLScriptElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLSelectElement | interface | DOMHTMLSelectElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLSelectElement (DOMHTMLSelectElementDeprecated) | category | DOMHTMLSelectElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLStyleElement | interface | DOMHTMLStyleElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLTableCaptionElement | interface | DOMHTMLTableCaptionElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLTableCellElement | interface | DOMHTMLTableCellElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLTableColElement | interface | DOMHTMLTableColElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLTableElement | interface | DOMHTMLTableElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLTableRowElement | interface | DOMHTMLTableRowElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLTableSectionElement | interface | DOMHTMLTableSectionElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLTextAreaElement | interface | DOMHTMLTextAreaElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLTitleElement | interface | DOMHTMLTitleElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMHTMLUListElement | interface | DOMHTMLUListElement.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMImplementation | interface | DOMImplementation.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMImplementation (DOMImplementationDeprecated) | category | DOMImplementation.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMKeyboardEvent | interface | DOMKeyboardEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_5, 10_14) |
| DOMMediaList | interface | DOMMediaList.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMMouseEvent | interface | DOMMouseEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMMouseEvent (DOMMouseEventDeprecated) | category | DOMMouseEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMMutationEvent | interface | DOMMutationEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMMutationEvent (DOMMutationEventDeprecated) | category | DOMMutationEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMNamedNodeMap | interface | DOMNamedNodeMap.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMNamedNodeMap (DOMNamedNodeMapDeprecated) | category | DOMNamedNodeMap.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMNode | interface | DOMNode.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMNode (DOMNodeDeprecated) | category | DOMNode.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMNodeFilter | protocol | DOMNodeFilter.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMNodeIterator | interface | DOMNodeIterator.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMNodeList | interface | DOMNodeList.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMObject | interface | DOMObject.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMObject (DOMLinkStyle) | category | DOMObject.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMObjectInternal | typealias | DOMObject.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMOverflowEvent | interface | DOMOverflowEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_5, 10_14) |
| DOMProcessingInstruction | interface | DOMProcessingInstruction.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMProgressEvent | interface | DOMProgressEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_6, 10_14) |
| DOMRGBColor | interface | DOMRGBColor.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMRange | interface | DOMRange.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMRange (DOMRangeDeprecated) | category | DOMRange.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_4, 10_14) |
| DOMRangeException | constant | DOMRangeException.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_4, 10_14) |
| DOMRect | interface | DOMRect.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMStyleSheet | interface | DOMStyleSheet.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMStyleSheetList | interface | DOMStyleSheetList.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMText | interface | DOMText.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMTreeWalker | interface | DOMTreeWalker.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMUIEvent | interface | DOMUIEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMUIEvent (DOMUIEventDeprecated) | category | DOMUIEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMWheelEvent | interface | DOMWheelEvent.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_5, 10_14) |
| DOMXPathException | constant | DOMXPathException.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_4, 10_14) |
| DOMXPathExpression | interface | DOMXPathExpression.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_5, 10_14) |
| DOMXPathExpression (DOMXPathExpressionDeprecated) | category | DOMXPathExpression.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_5, 10_14) |
| DOMXPathNSResolver | protocol | DOMXPathNSResolver.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_5, 10_14) |
| DOMXPathResult | interface | DOMXPathResult.h | Legacy DOM API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_5, 10_14) |
| WKPreferences (WKDeprecated) | category | WKPreferences.h | Deprecated members live in an explicit WKDeprecated category. | WKDeprecated category |
| WKProcessPool | interface | WKProcessPool.h | Apple deprecated WKProcessPool on macOS 12+. | API_DEPRECATED("Creating and using multiple instances of WKProcessPool no longer has any effect.", macos(10.10, 12.0), ios(8.0, 15.0), visionos(1.0, 1.0)) |
| WKWebView (WKDeprecated) | category | WKWebView.h | Deprecated members live in an explicit WKDeprecated category. | WKDeprecated category |
| WKWebViewConfiguration (WKDeprecated) | category | WKWebViewConfiguration.h | Deprecated members live in an explicit WKDeprecated category. | WKDeprecated category |
| WKWebsiteDataTypeOfflineWebApplicationCache | constant | WKWebsiteDataRecord.h | Apple deprecated offline application cache storage. | API_DEPRECATED("WebApplicationCache is no longer supported", macos(10.11, 26.2), ios(9.0, NA)) |
| WebArchive | interface | WebArchive.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_3, 10_14) |
| WebArchivePboardType | constant | WebArchive.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebBackForwardList | interface | WebBackForwardList.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| WebBackForwardList (WebBackForwardListDeprecated) | category | WebBackForwardList.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| DOMDocument (WebDOMDocumentOperations) | category | WebDOMOperations.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| DOMHTMLFrameElement (WebDOMHTMLFrameElementOperations) | category | WebDOMOperations.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| DOMHTMLIFrameElement (WebDOMHTMLIFrameElementOperations) | category | WebDOMOperations.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| DOMHTMLObjectElement (WebDOMHTMLObjectElementOperations) | category | WebDOMOperations.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| DOMNode (WebDOMNodeOperations) | category | WebDOMOperations.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| DOMRange (WebDOMRangeOperations) | category | WebDOMOperations.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| WebDataSource | interface | WebDataSource.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_3, 10_14) |
| WebDocumentRepresentation | protocol | WebDocument.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebDocumentSearching | protocol | WebDocument.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebDocumentText | protocol | WebDocument.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebDocumentView | protocol | WebDocument.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebDownload | interface | WebDownload.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| WebDownloadDelegate | protocol | WebDownload.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_4, 10_14) |
| WebEditingDelegate | protocol | WebEditingDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_3, 10_14) |
| WebViewInsertAction | enum | WebEditingDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_3, 10_14) |
| WebFrame | interface | WebFrame.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_3, 10_14) |
| WebFrameLoadDelegate | protocol | WebFrameLoadDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebFrameView | interface | WebFrameView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_3, 10_14) |
| WebHistory | interface | WebHistory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_3, 10_14) |
| WebHistoryAllItemsRemovedNotification | constant | WebHistory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebHistoryItemsAddedNotification | constant | WebHistory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebHistoryItemsKey | constant | WebHistory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebHistoryItemsRemovedNotification | constant | WebHistory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebHistoryLoadedNotification | constant | WebHistory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebHistorySavedNotification | constant | WebHistory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebHistoryItem | interface | WebHistoryItem.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_3, 10_14) |
| WebHistoryItemChangedNotification | constant | WebHistoryItem.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebKitErrorDomain | constant | WebKitErrors.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebKitErrorMIMETypeKey | constant | WebKitErrors.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebKitErrorPlugInNameKey | constant | WebKitErrors.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebKitErrorPlugInPageURLStringKey | constant | WebKitErrors.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebPlugInAttributesKey | constant | WebPluginViewFactory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebPlugInBaseURLKey | constant | WebPluginViewFactory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebPlugInContainerKey | constant | WebPluginViewFactory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebPlugInContainingElementKey | constant | WebPluginViewFactory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebPlugInShouldLoadMainResourceKey | constant | WebPluginViewFactory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_6, 10_14) |
| WebPlugInViewFactory | protocol | WebPluginViewFactory.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebActionButtonKey | constant | WebPolicyDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebActionElementKey | constant | WebPolicyDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebActionModifierFlagsKey | constant | WebPolicyDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebActionNavigationTypeKey | constant | WebPolicyDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebActionOriginalURLKey | constant | WebPolicyDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebNavigationType | enum | WebPolicyDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_3, 10_14) |
| WebPolicyDecisionListener | protocol | WebPolicyDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebPolicyDelegate | protocol | WebPolicyDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebCacheModel | enum | WebPreferences.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_5, 10_14) |
| WebPreferences | interface | WebPreferences.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_3, 10_14) |
| WebPreferencesChangedNotification | constant | WebPreferences.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebPreferencesPrivate | typealias | WebPreferences.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_5, 10_14) |
| WebResource | interface | WebResource.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | legacy WebKitLegacy/DOM header |
| WebResourceLoadDelegate | protocol | WebResourceLoadDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebScriptObject | interface | WebScriptObject.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| WebUndefined | interface | WebScriptObject.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_4, 10_14) |
| WebDragDestinationAction | enum | WebUIDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_3, 10_14) |
| WebDragSourceAction | enum | WebUIDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_ENUM_DEPRECATED_MAC(10_3, 10_14) |
| WebOpenPanelResultListener | protocol | WebUIDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebUIDelegate | protocol | WebUIDelegate.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementDOMNodeKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementFrameKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementImageAltStringKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementImageKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementImageRectKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementImageURLKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementIsSelectedKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementLinkLabelKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementLinkTargetFrameKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementLinkTitleKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebElementLinkURLKey | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebView | interface | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_CLASS_DEPRECATED_MAC(10_3, 10_14, "No longer supported, please adopt WKWebView.") |
| WebView (WebIBActions) | category | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebView (WebViewCSS) | category | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebView (WebViewEditing) | category | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebView (WebViewEditingActions) | category | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebView (WebViewUndoableEditing) | category | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebViewDidBeginEditingNotification | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebViewDidChangeNotification | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebViewDidChangeSelectionNotification | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebViewDidChangeTypingStyleNotification | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebViewDidEndEditingNotification | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebViewProgressEstimateChangedNotification | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebViewProgressFinishedNotification | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
| WebViewProgressStartedNotification | constant | WebView.h | Legacy WebKitLegacy API deprecated on macOS and intentionally skipped. | WEBKIT_DEPRECATED_MAC(10_3, 10_14) |
