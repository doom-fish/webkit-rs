use webkit::prelude::*;

#[test]
fn pdf_configuration_builder_sets_fields() {
    let configuration = PDFConfiguration::new()
        .with_rect(Rect::new(10.0, 20.0, 30.0, 40.0))
        .with_allow_transparent_background(true);
    assert_eq!(configuration.rect, Some(Rect::new(10.0, 20.0, 30.0, 40.0)));
    assert!(configuration.allow_transparent_background);
}
