use webkit::prelude::*;

#[test]
fn script_message_deserializes_from_json() -> Result<(), Box<dyn std::error::Error>> {
    let message: ScriptMessage = serde_json::from_str(
        r#"{"name":"bridge","body":"hello","frameUrl":"https://example.test/","isMainFrame":true,"world":"page"}"#,
    )?;
    assert_eq!(message.name, "bridge");
    assert!(message.is_main_frame);
    Ok(())
}
