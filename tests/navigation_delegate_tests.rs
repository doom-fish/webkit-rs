use webkit::prelude::*;

#[test]
fn navigation_event_deserializes_from_json() -> Result<(), Box<dyn std::error::Error>> {
    let event: NavigationEvent = serde_json::from_str(
        r#"{"kind":"didFinish","url":"https://example.test/","error":null,"navigationType":null,"statusCode":200}"#,
    )?;
    assert!(matches!(event.kind, NavigationEventKind::DidFinish));
    assert_eq!(event.status_code, Some(200));
    Ok(())
}

#[test]
fn navigation_event_deserializes_typed_action_details() -> Result<(), Box<dyn std::error::Error>> {
    let event: NavigationEvent = serde_json::from_str(
        r#"{
            "kind":"decidePolicyForAction",
            "url":"https://example.test/",
            "error":null,
            "navigationType":0,
            "statusCode":null,
            "navigationAction":{
                "navigationType":"linkActivated",
                "navigationTypeRawValue":0,
                "requestUrl":"https://example.test/",
                "requestMethod":"GET",
                "requestHeaders":{"Accept":"text/html"},
                "sourceFrame":{
                    "mainFrame":true,
                    "requestUrl":"https://example.test/",
                    "requestMethod":"GET",
                    "securityOriginProtocol":"https",
                    "securityOriginHost":"example.test",
                    "securityOriginPort":443,
                    "webviewUrl":"https://example.test/"
                },
                "targetFrame":null,
                "shouldPerformDownload":false,
                "modifierFlags":0,
                "buttonNumber":0,
                "contentRuleListRedirect":null
            },
            "navigationResponse":null
        }"#,
    )?;

    assert_eq!(
        event.navigation_type_enum(),
        Some(NavigationType::LinkActivated)
    );
    let action = event.navigation_action.expect("expected navigation action");
    assert_eq!(action.navigation_type, NavigationType::LinkActivated);
    assert_eq!(action.request_url, "https://example.test/");
    assert!(action.source_frame.main_frame);
    assert_eq!(
        action.source_frame.security_origin_host.as_deref(),
        Some("example.test")
    );
    assert_eq!(
        action.request_headers.get("Accept").map(String::as_str),
        Some("text/html")
    );
    Ok(())
}

#[test]
fn navigation_event_deserializes_typed_response_details() -> Result<(), Box<dyn std::error::Error>>
{
    let event: NavigationEvent = serde_json::from_str(
        r#"{
            "kind":"decidePolicyForResponse",
            "url":"https://example.test/",
            "error":null,
            "navigationType":null,
            "statusCode":200,
            "navigationAction":null,
            "navigationResponse":{
                "forMainFrame":true,
                "url":"https://example.test/",
                "mimeType":"text/html",
                "expectedContentLength":512,
                "textEncodingName":"utf-8",
                "statusCode":200,
                "headers":{"Content-Type":"text/html"},
                "canShowMimeType":true
            }
        }"#,
    )?;

    let response = event
        .navigation_response
        .expect("expected navigation response");
    assert!(response.for_main_frame);
    assert_eq!(response.mime_type.as_deref(), Some("text/html"));
    assert_eq!(response.status_code, Some(200));
    assert_eq!(
        response.headers.get("Content-Type").map(String::as_str),
        Some("text/html")
    );
    Ok(())
}
