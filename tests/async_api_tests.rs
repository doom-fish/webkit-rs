#[cfg(feature = "async")]
mod async_tests {
    use webkit::async_api::{
        AsyncContentRuleListStore, AsyncHttpCookieStore, AsyncWebView, AsyncWebsiteDataStore,
    };
    use webkit::{
        ContentRuleListStore, WebView, WebViewConfiguration, WebsiteDataStore, WebsiteDataType,
    };

    fn make_view() -> WebView {
        webkit::init_app();
        let cfg = WebViewConfiguration::new();
        cfg.use_nonpersistent_data_store();
        let view = WebView::with_config(&cfg).expect("WebView::with_config failed");
        webkit::pump_run_loop(0.1);
        view
    }

    #[test]
    #[ignore = "Async WKWebView smoke tests must run on the process main thread; examples cover live validation"]
    fn test_evaluate_javascript_happy_path() {
        let view = make_view();
        let result = pollster::block_on(AsyncWebView::evaluate_javascript(&view, "2 + 2"))
            .expect("evaluate_javascript failed");
        assert_eq!(result.trim(), "4");
    }

    #[test]
    #[ignore = "Async WKWebView smoke tests must run on the process main thread; examples cover live validation"]
    fn test_evaluate_javascript_error_path() {
        let view = make_view();
        let result = pollster::block_on(AsyncWebView::evaluate_javascript(
            &view,
            "throw new Error('boom')",
        ));
        assert!(result.is_err(), "expected error from thrown JS");
    }

    #[test]
    #[ignore = "Async WKWebView smoke tests must run on the process main thread; examples cover live validation"]
    fn test_call_async_javascript() {
        let view = make_view();
        let result = pollster::block_on(AsyncWebView::call_async_javascript(&view, "return 1 + 2"))
            .expect("call_async_javascript failed");
        assert_eq!(result.trim(), "3");
    }

    #[test]
    #[ignore = "Async WKWebView smoke tests must run on the process main thread; examples cover live validation"]
    fn test_create_web_archive() {
        let view = make_view();
        view.load_html("<html><body>test</body></html>", None)
            .expect("load_html failed");
        webkit::pump_run_loop(0.5);
        let bytes = pollster::block_on(AsyncWebView::create_web_archive(&view))
            .expect("create_web_archive failed");
        assert!(!bytes.is_empty(), "web archive should not be empty");
    }

    #[test]
    #[ignore = "Async WKWebView smoke tests must run on the process main thread; examples cover live validation"]
    fn test_take_snapshot() {
        let view = make_view();
        view.load_html("<html><body>snapshot test</body></html>", None)
            .expect("load_html failed");
        webkit::pump_run_loop(0.5);
        let bytes = pollster::block_on(AsyncWebView::take_snapshot(&view, None))
            .expect("take_snapshot failed");
        assert!(!bytes.is_empty(), "snapshot should return PNG bytes");
        assert_eq!(&bytes[..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    #[test]
    #[ignore = "Async WKWebView smoke tests must run on the process main thread; examples cover live validation"]
    fn test_create_pdf() {
        let view = make_view();
        view.load_html("<html><body>pdf test</body></html>", None)
            .expect("load_html failed");
        webkit::pump_run_loop(0.5);
        let bytes =
            pollster::block_on(AsyncWebView::create_pdf(&view, None)).expect("create_pdf failed");
        assert!(!bytes.is_empty(), "PDF should not be empty");
        assert_eq!(&bytes[..4], b"%PDF");
    }

    #[test]
    #[ignore = "Async WKWebsiteDataStore smoke tests must run on the process main thread; examples cover live validation"]
    fn test_fetch_data_records() {
        webkit::init_app();
        let store = WebsiteDataStore::non_persistent();
        let json = pollster::block_on(AsyncWebsiteDataStore::fetch_data_records(
            &store,
            &[WebsiteDataType::cookies()],
        ))
        .expect("fetch_data_records failed");
        assert!(json.starts_with('['), "expected JSON array, got: {json}");
    }

    #[test]
    #[ignore = "Async WKHTTPCookieStore smoke tests must run on the process main thread; examples cover live validation"]
    fn test_get_all_cookies() {
        webkit::init_app();
        let store = WebsiteDataStore::non_persistent();
        let cookie_store = store.http_cookie_store().expect("http_cookie_store failed");
        let json = pollster::block_on(AsyncHttpCookieStore::get_all_cookies(&cookie_store))
            .expect("get_all_cookies failed");
        assert!(json.starts_with('['), "expected JSON array, got: {json}");
    }

    #[test]
    #[ignore = "Async WKContentRuleListStore smoke tests must run on the process main thread; examples cover live validation"]
    fn test_compile_content_rule_list() {
        webkit::init_app();
        let rule_list_store = ContentRuleListStore::default_store();
        let rules =
            r#"[{"trigger":{"url-filter":".*"},"action":{"type":"ignore-previous-rules"}}]"#;
        let _result = pollster::block_on(AsyncContentRuleListStore::compile(
            &rule_list_store,
            "test-async-rule",
            rules,
        ));
    }
}
