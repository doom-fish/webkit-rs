#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn wk_web_extension_copy_constants_json() -> *mut c_char;

    pub fn wk_web_extension_create_with_resource_base_url(
        path: *const c_char,
        out_extension: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_create_with_app_extension_bundle(
        path: *const c_char,
        out_extension: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_release(ptr: *mut c_void);
    pub fn wk_web_extension_copy_summary_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_web_extension_supports_manifest_version(
        ptr: *mut c_void,
        manifest_version: f64,
    ) -> bool;

    pub fn wk_web_extension_match_pattern_register_custom_url_scheme(
        scheme: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_match_pattern_all_urls() -> *mut c_void;
    pub fn wk_web_extension_match_pattern_all_hosts_and_schemes() -> *mut c_void;
    pub fn wk_web_extension_match_pattern_with_string(
        pattern: *const c_char,
        out_pattern: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_match_pattern_with_components(
        scheme: *const c_char,
        host: *const c_char,
        path: *const c_char,
        out_pattern: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_match_pattern_release(ptr: *mut c_void);
    pub fn wk_web_extension_match_pattern_copy_summary_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_web_extension_match_pattern_matches_url(
        ptr: *mut c_void,
        url: *const c_char,
        options: u64,
    ) -> bool;
    pub fn wk_web_extension_match_pattern_matches_pattern(
        ptr: *mut c_void,
        other_ptr: *mut c_void,
        options: u64,
    ) -> bool;

    pub fn wk_web_extension_controller_configuration_default() -> *mut c_void;
    pub fn wk_web_extension_controller_configuration_nonpersistent() -> *mut c_void;
    pub fn wk_web_extension_controller_configuration_with_identifier(
        identifier: *const c_char,
        out_configuration: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_controller_configuration_release(ptr: *mut c_void);
    pub fn wk_web_extension_controller_configuration_copy_summary_json(
        ptr: *mut c_void,
    ) -> *mut c_char;
    pub fn wk_web_extension_controller_configuration_set_webview_configuration(
        ptr: *mut c_void,
        config_ptr: *mut c_void,
    );
    pub fn wk_web_extension_controller_configuration_copy_webview_configuration(
        ptr: *mut c_void,
    ) -> *mut c_void;
    pub fn wk_web_extension_controller_configuration_set_default_website_data_store(
        ptr: *mut c_void,
        store_ptr: *mut c_void,
    );
    pub fn wk_web_extension_controller_configuration_copy_default_website_data_store(
        ptr: *mut c_void,
    ) -> *mut c_void;

    pub fn wk_web_extension_controller_new() -> *mut c_void;
    pub fn wk_web_extension_controller_with_configuration(
        configuration_ptr: *mut c_void,
    ) -> *mut c_void;
    pub fn wk_web_extension_controller_release(ptr: *mut c_void);
    pub fn wk_web_extension_controller_copy_configuration(ptr: *mut c_void) -> *mut c_void;
    pub fn wk_web_extension_controller_load_context(
        ptr: *mut c_void,
        context_ptr: *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_controller_unload_context(
        ptr: *mut c_void,
        context_ptr: *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_controller_copy_context_for_extension(
        ptr: *mut c_void,
        extension_ptr: *mut c_void,
    ) -> *mut c_void;
    pub fn wk_web_extension_controller_copy_context_for_url(
        ptr: *mut c_void,
        url: *const c_char,
    ) -> *mut c_void;
    pub fn wk_web_extension_controller_copy_all_data_types_json() -> *mut c_char;
    pub fn wk_web_extension_controller_copy_data_records_json(
        ptr: *mut c_void,
        data_types_json: *const c_char,
        out_json: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_controller_copy_data_record_json_for_context(
        ptr: *mut c_void,
        data_types_json: *const c_char,
        context_ptr: *mut c_void,
        out_json: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_controller_remove_data_for_identifiers(
        ptr: *mut c_void,
        data_types_json: *const c_char,
        identifiers_json: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;

    pub fn wk_web_extension_context_new_for_extension(extension_ptr: *mut c_void) -> *mut c_void;
    pub fn wk_web_extension_context_release(ptr: *mut c_void);
    pub fn wk_web_extension_context_copy_summary_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_web_extension_context_set_base_url(
        ptr: *mut c_void,
        url: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_context_set_unique_identifier(
        ptr: *mut c_void,
        identifier: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_context_set_inspectable(ptr: *mut c_void, value: bool);
    pub fn wk_web_extension_context_set_inspection_name(ptr: *mut c_void, name: *const c_char);
    pub fn wk_web_extension_context_set_unsupported_apis_json(
        ptr: *mut c_void,
        apis_json: *const c_char,
    );
    pub fn wk_web_extension_context_set_requested_optional_access_to_all_hosts(
        ptr: *mut c_void,
        value: bool,
    );
    pub fn wk_web_extension_context_set_access_to_private_data(ptr: *mut c_void, value: bool);
    pub fn wk_web_extension_context_copy_webview_configuration(ptr: *mut c_void) -> *mut c_void;
    pub fn wk_web_extension_context_has_permission(
        ptr: *mut c_void,
        permission: *const c_char,
    ) -> bool;
    pub fn wk_web_extension_context_has_access_to_url(ptr: *mut c_void, url: *const c_char)
        -> bool;
    pub fn wk_web_extension_context_permission_status_for_permission(
        ptr: *mut c_void,
        permission: *const c_char,
    ) -> i64;
    pub fn wk_web_extension_context_set_permission_status_for_permission(
        ptr: *mut c_void,
        status: i64,
        permission: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_context_permission_status_for_url(
        ptr: *mut c_void,
        url: *const c_char,
    ) -> i64;
    pub fn wk_web_extension_context_set_permission_status_for_url(
        ptr: *mut c_void,
        status: i64,
        url: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_context_permission_status_for_match_pattern(
        ptr: *mut c_void,
        pattern_ptr: *mut c_void,
    ) -> i64;
    pub fn wk_web_extension_context_set_permission_status_for_match_pattern(
        ptr: *mut c_void,
        status: i64,
        pattern_ptr: *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_context_load_background_content(
        ptr: *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_context_copy_default_action_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_web_extension_context_perform_default_action(ptr: *mut c_void);
    pub fn wk_web_extension_context_copy_commands_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_web_extension_context_perform_command_for_identifier(
        ptr: *mut c_void,
        identifier: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;

    pub fn wk_web_extension_message_port_release(ptr: *mut c_void);
    pub fn wk_web_extension_message_port_copy_application_identifier(
        ptr: *mut c_void,
    ) -> *mut c_char;
    pub fn wk_web_extension_message_port_is_disconnected(ptr: *mut c_void) -> bool;
    pub fn wk_web_extension_message_port_send_message_json(
        ptr: *mut c_void,
        message_json: *const c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_web_extension_message_port_disconnect(ptr: *mut c_void);
    pub fn wk_web_extension_message_port_disconnect_with_error(
        ptr: *mut c_void,
        message: *const c_char,
    );
}
