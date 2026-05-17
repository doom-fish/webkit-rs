#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

pub mod attributed_string;
pub mod content_rule_list_store;
pub mod http_cookie_store;
pub mod url_scheme;
pub mod web_extension;
pub mod website_data_store;

pub use attributed_string::*;
pub use content_rule_list_store::*;
pub use http_cookie_store::*;
pub use url_scheme::*;
pub use web_extension::*;
pub use website_data_store::*;

pub type WKNavCallback = unsafe extern "C" fn(user_info: *mut c_void, event_json: *const c_char);

pub type WKMsgCallback =
    unsafe extern "C" fn(user_info: *mut c_void, handler_name: *const c_char, body: *const c_char);

pub type WKMsgReplyCallback = unsafe extern "C" fn(
    user_info: *mut c_void,
    handler_name: *const c_char,
    body: *const c_char,
    out_reply: *mut *mut c_char,
    out_err: *mut *mut c_char,
) -> i32;

unsafe extern "C" {
    // Lifecycle
    pub fn wk_string_free(s: *mut c_char);
    pub fn wk_bytes_free(ptr: *mut u8, len: usize);
    pub fn wk_run_loop_pump(seconds: f64);
    pub fn wk_init_app();

    // Navigation / download lifecycle
    pub fn wk_navigation_release(ptr: *mut c_void);
    pub fn wk_download_release(ptr: *mut c_void);
    pub fn wk_download_copy_original_request_url(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_download_is_user_initiated(ptr: *mut c_void) -> bool;
    pub fn wk_download_copy_events_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_download_set_redirect_policy(ptr: *mut c_void, raw_value: i32);
    pub fn wk_download_get_redirect_policy(ptr: *mut c_void) -> i32;
    pub fn wk_download_cancel(
        ptr: *mut c_void,
        out_resume_data: *mut *mut u8,
        out_resume_data_len: *mut usize,
        out_err: *mut *mut c_char,
    ) -> i32;

    // Configuration
    pub fn wk_config_new() -> *mut c_void;
    pub fn wk_config_release(ptr: *mut c_void);
    pub fn wk_config_set_application_name(ptr: *mut c_void, name: *const c_char);
    pub fn wk_config_copy_application_name(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_config_set_allows_airplay(ptr: *mut c_void, v: bool);
    pub fn wk_config_get_allows_airplay(ptr: *mut c_void) -> bool;
    pub fn wk_config_set_media_types_requiring_user_action_for_playback(
        ptr: *mut c_void,
        raw_value: u64,
    );
    pub fn wk_config_get_media_types_requiring_user_action_for_playback(ptr: *mut c_void) -> u64;
    pub fn wk_config_set_user_interface_direction_policy(ptr: *mut c_void, raw_value: i32);
    pub fn wk_config_get_user_interface_direction_policy(ptr: *mut c_void) -> i32;
    pub fn wk_config_set_allows_content_javascript(ptr: *mut c_void, v: bool);
    pub fn wk_config_get_allows_content_javascript(ptr: *mut c_void) -> bool;
    pub fn wk_config_set_preferences_json(ptr: *mut c_void, json: *const c_char);
    pub fn wk_config_copy_preferences_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_config_set_website_data_store(ptr: *mut c_void, store: *mut c_void);
    pub fn wk_config_copy_website_data_store(ptr: *mut c_void) -> *mut c_void;
    pub fn wk_config_use_nonpersistent_data_store(ptr: *mut c_void);
    pub fn wk_config_add_content_rule_list(ptr: *mut c_void, rule_list: *mut c_void);
    pub fn wk_config_remove_content_rule_list(ptr: *mut c_void, rule_list: *mut c_void);
    pub fn wk_config_remove_all_content_rule_lists(ptr: *mut c_void);
    pub fn wk_config_add_user_script(
        ptr: *mut c_void,
        source: *const c_char,
        injection_time: i32,
        main_frame_only: bool,
        content_world_name: *const c_char,
    );
    pub fn wk_config_remove_all_user_scripts(ptr: *mut c_void);
    pub fn wk_config_add_message_handler_name(ptr: *mut c_void, name: *const c_char);
    pub fn wk_config_add_message_handler_with_reply_name(ptr: *mut c_void, name: *const c_char);

    // WebView
    pub fn wk_webview_new(cfg: *mut c_void) -> *mut c_void;
    pub fn wk_webview_release(ptr: *mut c_void);
    pub fn wk_webview_set_nav_callback(
        ptr: *mut c_void,
        callback: Option<WKNavCallback>,
        user_info: *mut c_void,
    );
    pub fn wk_webview_set_msg_callback(
        ptr: *mut c_void,
        callback: Option<WKMsgCallback>,
        user_info: *mut c_void,
    );
    pub fn wk_webview_set_msg_reply_callback(
        ptr: *mut c_void,
        callback: Option<WKMsgReplyCallback>,
        user_info: *mut c_void,
    );
    pub fn wk_webview_set_navigation_delegate_config(
        ptr: *mut c_void,
        action_policy: i32,
        response_policy: i32,
    );
    pub fn wk_webview_drain_navigation_events_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_webview_set_ui_delegate_config(
        ptr: *mut c_void,
        confirm_response: bool,
        prompt_response: *const c_char,
    );
    pub fn wk_webview_drain_ui_events_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_webview_drain_script_messages_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_webview_load_url(
        ptr: *mut c_void,
        url: *const c_char,
        out_navigation: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_load_html(
        ptr: *mut c_void,
        html: *const c_char,
        base_url: *const c_char,
        out_navigation: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_load_file_url(
        ptr: *mut c_void,
        file_url: *const c_char,
        read_access_url: *const c_char,
        out_navigation: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_load_data(
        ptr: *mut c_void,
        bytes: *const u8,
        len: usize,
        mime_type: *const c_char,
        encoding_name: *const c_char,
        base_url: *const c_char,
        out_navigation: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_go_back(ptr: *mut c_void, out_navigation: *mut *mut c_void) -> bool;
    pub fn wk_webview_go_forward(ptr: *mut c_void, out_navigation: *mut *mut c_void) -> bool;
    pub fn wk_webview_reload(ptr: *mut c_void, out_navigation: *mut *mut c_void) -> bool;
    pub fn wk_webview_reload_from_origin(
        ptr: *mut c_void,
        out_navigation: *mut *mut c_void,
    ) -> bool;
    pub fn wk_webview_stop_loading(ptr: *mut c_void);
    pub fn wk_webview_perform_go_back_action(ptr: *mut c_void);
    pub fn wk_webview_perform_go_forward_action(ptr: *mut c_void);
    pub fn wk_webview_perform_reload_action(ptr: *mut c_void);
    pub fn wk_webview_perform_reload_from_origin_action(ptr: *mut c_void);
    pub fn wk_webview_perform_stop_loading_action(ptr: *mut c_void);
    pub fn wk_webview_validate_text_finder_action(ptr: *mut c_void, action: i32) -> bool;
    pub fn wk_webview_perform_text_finder_action(ptr: *mut c_void, action: i32);
    pub fn wk_webview_go_to_back_forward_index(
        ptr: *mut c_void,
        index: isize,
        out_navigation: *mut *mut c_void,
    ) -> bool;
    pub fn wk_webview_copy_title(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_webview_copy_url(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_webview_is_loading(ptr: *mut c_void) -> bool;
    pub fn wk_webview_get_estimated_progress(ptr: *mut c_void) -> f64;
    pub fn wk_webview_get_has_only_secure_content(ptr: *mut c_void) -> bool;
    pub fn wk_webview_get_can_go_back(ptr: *mut c_void) -> bool;
    pub fn wk_webview_get_can_go_forward(ptr: *mut c_void) -> bool;
    pub fn wk_webview_copy_back_forward_list_json(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_webview_set_custom_user_agent(ptr: *mut c_void, value: *const c_char);
    pub fn wk_webview_copy_custom_user_agent(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_webview_set_allows_link_preview(ptr: *mut c_void, value: bool);
    pub fn wk_webview_get_allows_link_preview(ptr: *mut c_void) -> bool;
    pub fn wk_webview_set_page_zoom(ptr: *mut c_void, value: f64);
    pub fn wk_webview_get_page_zoom(ptr: *mut c_void) -> f64;
    pub fn wk_webview_set_media_type(ptr: *mut c_void, value: *const c_char);
    pub fn wk_webview_copy_media_type(ptr: *mut c_void) -> *mut c_char;
    pub fn wk_webview_set_inspectable(ptr: *mut c_void, value: bool);
    pub fn wk_webview_get_inspectable(ptr: *mut c_void) -> bool;
    pub fn wk_webview_request_media_playback_state(
        ptr: *mut c_void,
        out_state: *mut i32,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_get_camera_capture_state(ptr: *mut c_void) -> i32;
    pub fn wk_webview_get_microphone_capture_state(ptr: *mut c_void) -> i32;
    pub fn wk_webview_set_camera_capture_state(
        ptr: *mut c_void,
        raw_value: i32,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_set_microphone_capture_state(
        ptr: *mut c_void,
        raw_value: i32,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_get_fullscreen_state(ptr: *mut c_void) -> i32;
    pub fn wk_webview_fetch_data_of_types(
        ptr: *mut c_void,
        data_types: u64,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_restore_data(
        ptr: *mut c_void,
        bytes: *const u8,
        len: usize,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_evaluate_js(
        ptr: *mut c_void,
        js: *const c_char,
        out_result: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_call_async_js(
        ptr: *mut c_void,
        js: *const c_char,
        out_result: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_find_string(
        ptr: *mut c_void,
        query: *const c_char,
        configuration_json: *const c_char,
        out_result: *mut *mut c_char,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_take_snapshot_png(
        ptr: *mut c_void,
        has_rect: bool,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        has_snapshot_width: bool,
        snapshot_width: f64,
        after_screen_updates: bool,
        out_png: *mut *mut u8,
        out_len: *mut usize,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_create_pdf(
        ptr: *mut c_void,
        has_rect: bool,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        allow_transparent_background: bool,
        out_bytes: *mut *mut u8,
        out_len: *mut usize,
        out_err: *mut *mut c_char,
    ) -> i32;
    pub fn wk_webview_start_download(
        ptr: *mut c_void,
        url: *const c_char,
        destination_directory: *const c_char,
        out_download: *mut *mut c_void,
        out_err: *mut *mut c_char,
    ) -> i32;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const UNSUPPORTED: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
    pub const FRAMEWORK_ERROR: i32 = -5;
    pub const UNKNOWN: i32 = -99;
}
