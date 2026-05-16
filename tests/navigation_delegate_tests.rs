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
