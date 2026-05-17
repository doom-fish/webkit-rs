use webkit::prelude::*;

#[test]
fn webkit_error_code_roundtrips() {
    assert_eq!(WEBKIT_ERROR_DOMAIN, WebKitErrorCode::domain());
    assert_eq!(
        WebKitErrorCode::from_raw(4),
        Some(WebKitErrorCode::JavaScriptExceptionOccurred)
    );
    assert_eq!(
        WebKitErrorCode::AttributedStringContentFailedToLoad.as_raw(),
        10
    );
    assert_eq!(WebKitErrorCode::from_raw(999), None);
}
