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

#[test]
fn script_message_reply_surface_is_available() {
    fn install_handler(view: &mut WebView) {
        view.set_message_handler_with_reply(|name, body| {
            Ok(Some(serde_json::json!({ "name": name, "body": body })))
        });
    }

    let _: fn(&WebViewConfiguration, &str) = WebViewConfiguration::add_message_handler_with_reply;
    let _ = install_handler as fn(&mut WebView);
}
