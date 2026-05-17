use webkit::prelude::*;

#[test]
fn user_script_builder_preserves_configuration() {
    let script = UserScript::new("window.test = 1;")
        .with_injection_time(InjectionTime::AtDocumentStart)
        .with_main_frame_only(false)
        .with_content_world("rust");

    assert!(matches!(
        script.injection_time,
        InjectionTime::AtDocumentStart
    ));
    assert!(!script.main_frame_only);
    assert_eq!(script.content_world.as_deref(), Some("rust"));
}
