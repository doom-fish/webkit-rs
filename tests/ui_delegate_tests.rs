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

#[test]
fn ui_delegate_detail_deserializes_rich_types() -> Result<(), Box<dyn std::error::Error>> {
    let detail: UIDelegateEventDetail = serde_json::from_str(
        r#"{"kind":"mediaCapturePermission","frameUrl":"https://ui.test/","securityOrigin":{"protocol":"https","host":"ui.test","port":443},"mediaCaptureType":"cameraAndMicrophone","permissionDecision":"deny","windowFeatures":{"menuBarVisibility":true,"statusBarVisibility":false,"toolbarsVisibility":true,"allowsResizing":true,"x":10.0,"y":20.0,"width":800.0,"height":600.0},"openPanelParameters":{"allowsMultipleSelection":true,"allowsDirectories":false}}"#,
    )?;
    let open_panel = UIDelegateEvent {
        kind: "openPanel".to_owned(),
        message: None,
        prompt: None,
        default_text: None,
        frame_url: Some("https://ui.test/".to_owned()),
        response: None,
        allows_multiple_selection: Some(true),
        allows_directories: Some(false),
        host: None,
        r#type: None,
    };

    assert_eq!(
        detail.media_capture_type,
        Some(MediaCaptureType::CameraAndMicrophone)
    );
    assert_eq!(detail.permission_decision, Some(PermissionDecision::Deny));
    assert_eq!(
        detail
            .security_origin
            .as_ref()
            .map(|origin| origin.host.as_str()),
        Some("ui.test")
    );
    assert_eq!(
        detail
            .open_panel_parameters
            .as_ref()
            .map(|params| params.allows_multiple_selection),
        Some(true)
    );
    assert_eq!(
        open_panel.open_panel_parameters(),
        Some(OpenPanelParameters {
            allows_multiple_selection: true,
            allows_directories: false
        })
    );
    Ok(())
}
