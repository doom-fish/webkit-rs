use webkit::prelude::*;

#[test]
fn script_message_deserializes_from_json() -> Result<(), Box<dyn std::error::Error>> {
    let message: ScriptMessage = serde_json::from_str(
        r#"{"name":"bridge","body":"hello","frame":{"mainFrame":false,"requestUrl":"https://ads.example/frame","requestMethod":"GET","securityOrigin":{"protocol":"https","host":"ads.example","port":0},"webviewUrl":"https://app.example/"},"world":{"kind":"named","name":"host"}}"#,
    )?;
    assert_eq!(message.name, "bridge");
    assert_eq!(message.body, "hello");
    assert!(!message.frame.main_frame);
    assert_eq!(message.frame.request_url, "https://ads.example/frame");
    assert_eq!(message.frame.security_origin.host, "ads.example");
    assert_eq!(message.frame.security_origin.protocol, "https");
    assert_eq!(message.world, ContentWorld::Named("host".to_owned()));
    assert!(message.frame_handle.is_none());
    Ok(())
}

#[test]
fn script_message_reply_surface_is_available() {
    fn install_handler(view: &mut WebView) {
        view.set_message_handler_with_reply(|message| {
            if !message.frame.main_frame {
                return Err(WebKitError::InvalidArgument(
                    "cross-origin frame".to_owned(),
                ));
            }
            Ok(Some(
                serde_json::json!({ "name": message.name, "body": message.body }),
            ))
        });
    }

    let _: fn(&WebViewConfiguration, &str, &ContentWorld) -> Result<(), WebKitError> =
        WebViewConfiguration::add_message_handler_with_reply;
    let _: fn(&WebViewConfiguration, &str, &ContentWorld) -> Result<(), WebKitError> =
        WebViewConfiguration::remove_message_handler;
    let _ = install_handler as fn(&mut WebView);
}
