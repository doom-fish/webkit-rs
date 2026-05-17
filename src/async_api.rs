//! Async API for `webkit-rs`
//!
//! This module provides `Future`-based wrappers for WebKit completion-handler
//! APIs. Enable the `async` feature to use it.
//!
//! ## Wrapped APIs
//!
//! | Type | Description |
//! |------|-------------|
//! | [`AsyncWebView`] | Async `WKWebView` operations |
//! | [`AsyncWebsiteDataStore`] | Async `WKWebsiteDataStore` operations |
//! | [`AsyncHttpCookieStore`] | Async `WKHTTPCookieStore` operations |
//! | [`AsyncContentRuleListStore`] | Async `WKContentRuleListStore` compilation |
//! | [`AsyncDownload`] | Async `WKDownload` cancellation |
//!
//! These futures pump the main run loop while waiting for WebKit callbacks so
//! they work in headless CLI examples with `pollster::block_on`.
//!
//! ## WKWebExtensionContext
//!
//! `WKWebExtensionContext` async methods are deferred to Tier 2 / deferred work:
//! they involve multi-step delegate interactions rather than a simple single-shot
//! completion callback.
//!
//! ## Example
//!
//! ```rust,no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # #[cfg(feature = "async")]
//! # pollster::block_on(async {
//! use webkit::async_api::AsyncWebView;
//! use webkit::{WebView, WebViewConfiguration};
//!
//! webkit::init_app();
//! let cfg = WebViewConfiguration::new();
//! let view = WebView::with_config(&cfg)?;
//! view.load_html("<html><body>hello</body></html>", None)?;
//! webkit::pump_run_loop(0.3);
//!
//! let result = AsyncWebView::evaluate_javascript(&view, "1 + 1").await?;
//! println!("result: {result}");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # })?;
//! # Ok(())
//! # }
//! ```

use core::ffi::{c_char, c_void, CStr};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};

use crate::content_rule_list_store::{ContentRuleList, ContentRuleListStore};
use crate::download::Download;
use crate::error::WebKitError;
use crate::find::FindConfiguration;
use crate::http_cookie_store::HttpCookieStore;
use crate::pdf_configuration::PDFConfiguration;
use crate::private::{to_cstring, to_json_cstring};
use crate::snapshot_configuration::SnapshotConfiguration;
use crate::website_data_store::{WebsiteDataRecord, WebsiteDataStore, WebsiteDataType};
use crate::webview::WebView;

fn poll_completion<T>(
    inner: &mut AsyncCompletionFuture<T>,
    cx: &mut Context<'_>,
) -> Poll<Result<T, String>> {
    loop {
        match Pin::new(&mut *inner).poll(cx) {
            Poll::Ready(result) => return Poll::Ready(result),
            Poll::Pending => crate::pump_run_loop(0.01),
        }
    }
}

macro_rules! string_future {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[must_use = "futures do nothing unless polled"]
        pub struct $name {
            inner: AsyncCompletionFuture<String>,
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }

        impl Future for $name {
            type Output = Result<String, WebKitError>;

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                poll_completion(&mut self.inner, cx)
                    .map(|result| result.map_err(WebKitError::FrameworkError))
            }
        }
    };
}

macro_rules! bytes_future {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[must_use = "futures do nothing unless polled"]
        pub struct $name {
            inner: AsyncCompletionFuture<Vec<u8>>,
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }

        impl Future for $name {
            type Output = Result<Vec<u8>, WebKitError>;

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                poll_completion(&mut self.inner, cx)
                    .map(|result| result.map_err(WebKitError::FrameworkError))
            }
        }
    };
}

macro_rules! unit_future {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[must_use = "futures do nothing unless polled"]
        pub struct $name {
            inner: AsyncCompletionFuture<()>,
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }

        impl Future for $name {
            type Output = Result<(), WebKitError>;

            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                poll_completion(&mut self.inner, cx)
                    .map(|result| result.map_err(WebKitError::FrameworkError))
            }
        }
    };
}

unsafe extern "C" fn string_cb(result: *const c_char, error: *const c_char, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<String>::complete_err(ctx, msg) };
        return;
    }

    let value = if result.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(result).to_string_lossy().into_owned() }
    };
    unsafe { AsyncCompletion::complete_ok(ctx, value) };
}

unsafe extern "C" fn bytes_cb(
    bytes: *const u8,
    len: usize,
    error: *const c_char,
    ctx: *mut c_void,
) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Vec<u8>>::complete_err(ctx, msg) };
        return;
    }

    let value = if bytes.is_null() || len == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(bytes, len).to_vec() }
    };
    unsafe { AsyncCompletion::complete_ok(ctx, value) };
}

unsafe extern "C" fn unit_cb(_result: *const c_char, error: *const c_char, ctx: *mut c_void) {
    if error.is_null() {
        unsafe { AsyncCompletion::complete_ok(ctx, ()) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
    }
}

unsafe extern "C" fn rule_list_cb(result: *mut c_void, error: *const c_char, ctx: *mut c_void) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<*mut c_void>::complete_err(ctx, msg) };
    } else if result.is_null() {
        unsafe {
            AsyncCompletion::<*mut c_void>::complete_err(
                ctx,
                "compile returned no rule list".to_owned(),
            );
        }
    } else {
        unsafe { AsyncCompletion::complete_ok(ctx, result) };
    }
}

unsafe extern "C" fn bytes_cb_discard(
    _bytes: *const u8,
    _len: usize,
    error: *const c_char,
    ctx: *mut c_void,
) {
    if error.is_null() {
        unsafe { AsyncCompletion::complete_ok(ctx, ()) };
    } else {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<()>::complete_err(ctx, msg) };
    }
}

string_future!(
    EvaluateJavaScriptFuture,
    "Future returned by [`AsyncWebView::evaluate_javascript`]."
);
string_future!(
    CallAsyncJavaScriptFuture,
    "Future returned by [`AsyncWebView::call_async_javascript`]."
);
bytes_future!(
    TakeSnapshotFuture,
    "Future returned by [`AsyncWebView::take_snapshot`]."
);
bytes_future!(
    CreatePdfFuture,
    "Future returned by [`AsyncWebView::create_pdf`]."
);
bytes_future!(
    CreateWebArchiveFuture,
    "Future returned by [`AsyncWebView::create_web_archive`]."
);
string_future!(
    FindStringFuture,
    "Future returned by [`AsyncWebView::find_string`]."
);
string_future!(
    FetchDataRecordsFuture,
    "Future returned by [`AsyncWebsiteDataStore::fetch_data_records`]."
);
unit_future!(
    RemoveDataFuture,
    "Future returned by [`AsyncWebsiteDataStore::remove_data`]."
);
string_future!(
    GetAllCookiesFuture,
    "Future returned by [`AsyncHttpCookieStore::get_all_cookies`]."
);
unit_future!(
    DownloadCancelFuture,
    "Future returned by [`AsyncDownload::cancel`]."
);
bytes_future!(
    CancelWithResumeDataFuture,
    "Future returned by [`AsyncDownload::cancel_with_resume_data`]."
);

/// Future returned by [`AsyncContentRuleListStore::compile`].
#[must_use = "futures do nothing unless polled"]
pub struct CompileRuleListFuture {
    inner: AsyncCompletionFuture<*mut c_void>,
}

// SAFETY: The raw pointer is a retained Objective-C object pointer managed by the bridge.
unsafe impl Send for CompileRuleListFuture {}

impl std::fmt::Debug for CompileRuleListFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompileRuleListFuture")
            .finish_non_exhaustive()
    }
}

impl Future for CompileRuleListFuture {
    type Output = Result<ContentRuleList, WebKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        poll_completion(&mut self.inner, cx).map(|result| {
            result.map_err(WebKitError::FrameworkError).and_then(|ptr| {
                ContentRuleList::from_ptr(ptr).ok_or_else(|| {
                    WebKitError::FrameworkError("compile returned null rule list".to_owned())
                })
            })
        })
    }
}

/// Async wrappers for `WKWebView` completion-handler APIs.
pub struct AsyncWebView;

impl AsyncWebView {
    /// Evaluate JavaScript and return the result as a string.
    pub fn evaluate_javascript(view: &WebView, js: &str) -> EvaluateJavaScriptFuture {
        let c_js = to_cstring(js);
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_webview_evaluate_js_async(view.as_ptr(), c_js.as_ptr(), string_cb, ctx);
        }
        EvaluateJavaScriptFuture { inner: future }
    }

    /// Call async JavaScript and return the result as a string.
    pub fn call_async_javascript(view: &WebView, js: &str) -> CallAsyncJavaScriptFuture {
        let c_js = to_cstring(js);
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_webview_call_async_js_async(
                view.as_ptr(),
                c_js.as_ptr(),
                string_cb,
                ctx,
            );
        }
        CallAsyncJavaScriptFuture { inner: future }
    }

    /// Take a PNG snapshot of the web view.
    pub fn take_snapshot(
        view: &WebView,
        configuration: Option<&SnapshotConfiguration>,
    ) -> TakeSnapshotFuture {
        let (
            has_rect,
            x,
            y,
            width,
            height,
            has_snapshot_width,
            snapshot_width,
            after_screen_updates,
        ) = configuration.map_or(
            (false, 0.0, 0.0, 0.0, 0.0, false, 0.0, false),
            |configuration| {
                let rect = configuration.rect();
                let snapshot_width = configuration.snapshot_width();
                (
                    rect.is_some(),
                    rect.map_or(0.0, |rect| rect.x),
                    rect.map_or(0.0, |rect| rect.y),
                    rect.map_or(0.0, |rect| rect.width),
                    rect.map_or(0.0, |rect| rect.height),
                    snapshot_width.is_some(),
                    snapshot_width.unwrap_or_default(),
                    configuration.after_screen_updates(),
                )
            },
        );

        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_webview_take_snapshot_async(
                view.as_ptr(),
                has_rect,
                x,
                y,
                width,
                height,
                has_snapshot_width,
                snapshot_width,
                after_screen_updates,
                bytes_cb,
                ctx,
            );
        }
        TakeSnapshotFuture { inner: future }
    }

    /// Create a PDF representation of the current page.
    pub fn create_pdf(view: &WebView, configuration: Option<&PDFConfiguration>) -> CreatePdfFuture {
        let (has_rect, x, y, width, height, allow_transparent_background) =
            configuration.map_or((false, 0.0, 0.0, 0.0, 0.0, false), |configuration| {
                let rect = configuration.rect();
                (
                    rect.is_some(),
                    rect.map_or(0.0, |rect| rect.x),
                    rect.map_or(0.0, |rect| rect.y),
                    rect.map_or(0.0, |rect| rect.width),
                    rect.map_or(0.0, |rect| rect.height),
                    configuration.allows_transparent_background(),
                )
            });

        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_webview_create_pdf_async(
                view.as_ptr(),
                has_rect,
                x,
                y,
                width,
                height,
                allow_transparent_background,
                bytes_cb,
                ctx,
            );
        }
        CreatePdfFuture { inner: future }
    }

    /// Create a web archive of the current page.
    pub fn create_web_archive(view: &WebView) -> CreateWebArchiveFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_webview_create_web_archive_async(view.as_ptr(), bytes_cb, ctx);
        }
        CreateWebArchiveFuture { inner: future }
    }

    /// Find a string and return the JSON-encoded `FindResult` payload.
    pub fn find_string(
        view: &WebView,
        query: &str,
        configuration: Option<&FindConfiguration>,
    ) -> FindStringFuture {
        let c_query = to_cstring(query);
        let configuration_json = configuration.map(to_json_cstring);
        let configuration_ptr = configuration_json
            .as_ref()
            .map_or(core::ptr::null(), |value| value.as_ptr());
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_webview_find_string_async(
                view.as_ptr(),
                c_query.as_ptr(),
                configuration_ptr,
                string_cb,
                ctx,
            );
        }
        FindStringFuture { inner: future }
    }
}

/// Async wrappers for `WKWebsiteDataStore` completion-handler APIs.
pub struct AsyncWebsiteDataStore;

impl AsyncWebsiteDataStore {
    /// Fetch data records and return the JSON array payload.
    pub fn fetch_data_records(
        store: &WebsiteDataStore,
        data_types: &[WebsiteDataType],
    ) -> FetchDataRecordsFuture {
        let data_types_json = to_json_cstring(data_types);
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_website_data_store_fetch_data_records_async(
                store.as_ptr(),
                data_types_json.as_ptr(),
                string_cb,
                ctx,
            );
        }
        FetchDataRecordsFuture { inner: future }
    }

    /// Remove matching data records.
    pub fn remove_data(
        store: &WebsiteDataStore,
        data_types: &[WebsiteDataType],
        records: &[WebsiteDataRecord],
    ) -> RemoveDataFuture {
        let data_types_json = to_json_cstring(data_types);
        let display_names: Vec<&str> = records
            .iter()
            .map(|record| record.display_name.as_str())
            .collect();
        let display_names_json = to_json_cstring(&display_names);
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_website_data_store_remove_data_async(
                store.as_ptr(),
                data_types_json.as_ptr(),
                display_names_json.as_ptr(),
                unit_cb,
                ctx,
            );
        }
        RemoveDataFuture { inner: future }
    }
}

/// Async wrappers for `WKHTTPCookieStore` completion-handler APIs.
pub struct AsyncHttpCookieStore;

impl AsyncHttpCookieStore {
    /// Fetch all cookies and return the JSON array payload.
    pub fn get_all_cookies(store: &HttpCookieStore) -> GetAllCookiesFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_http_cookie_store_get_all_cookies_async(store.as_ptr(), string_cb, ctx);
        }
        GetAllCookiesFuture { inner: future }
    }
}

/// Async wrappers for `WKContentRuleListStore` completion-handler APIs.
pub struct AsyncContentRuleListStore;

impl AsyncContentRuleListStore {
    /// Compile a content rule list.
    pub fn compile(
        store: &ContentRuleListStore,
        identifier: &str,
        encoded_rule_list: &str,
    ) -> CompileRuleListFuture {
        let c_identifier = to_cstring(identifier);
        let c_rule_list = to_cstring(encoded_rule_list);
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_content_rule_list_store_compile_async(
                store.as_ptr(),
                c_identifier.as_ptr(),
                c_rule_list.as_ptr(),
                rule_list_cb,
                ctx,
            );
        }
        CompileRuleListFuture { inner: future }
    }
}

/// Async wrappers for `WKDownload` completion-handler APIs.
pub struct AsyncDownload;

impl AsyncDownload {
    /// Cancel the download and wait for completion, discarding any resume data.
    pub fn cancel(download: &Download) -> DownloadCancelFuture {
        let (future, ctx) = AsyncCompletion::<()>::create();
        unsafe {
            crate::ffi::wk_download_cancel_async(download.as_ptr(), bytes_cb_discard, ctx);
        }
        DownloadCancelFuture { inner: future }
    }

    /// Cancel the download and return the raw resume-data bytes, if any.
    pub fn cancel_with_resume_data(download: &Download) -> CancelWithResumeDataFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            crate::ffi::wk_download_cancel_async(download.as_ptr(), bytes_cb, ctx);
        }
        CancelWithResumeDataFuture { inner: future }
    }
}
