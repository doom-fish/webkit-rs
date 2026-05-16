use webkit::prelude::*;

#[test]
fn snapshot_configuration_builder_sets_fields() {
    let configuration = SnapshotConfiguration::new()
        .with_rect(Rect::new(1.0, 2.0, 3.0, 4.0))
        .with_snapshot_width(250.0)
        .with_after_screen_updates(false);
    assert_eq!(configuration.rect, Some(Rect::new(1.0, 2.0, 3.0, 4.0)));
    assert_eq!(configuration.snapshot_width, Some(250.0));
    assert!(!configuration.after_screen_updates);
}
