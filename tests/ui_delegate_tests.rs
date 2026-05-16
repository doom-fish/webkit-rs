use webkit::prelude::*;

#[test]
fn ui_delegate_event_deserializes_from_json() -> Result<(), Box<dyn std::error::Error>> {
    let event: UIDelegateEvent = serde_json::from_str(
        r#"{"kind":"prompt","message":"hello","prompt":"enter name","defaultText":"guest","frameUrl":"https://ui.test/","response":"Rust","allowsMultipleSelection":false,"allowsDirectories":false,"host":"ui.test","type":null}"#,
    )?;
    let config = UIDelegateConfig {
        confirm_response: true,
        prompt_response: Some("Rust".to_owned()),
    };
    assert_eq!(event.kind, "prompt");
    assert!(config.confirm_response);
    Ok(())
}
