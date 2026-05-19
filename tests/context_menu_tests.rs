use webkit::prelude::*;

#[test]
fn context_menu_preview_metadata_surfaces_are_available() {
    assert!(unsafe { ContextMenuElementInfo::from_raw(std::ptr::null_mut()) }.is_none());
    assert!(unsafe { PreviewElementInfo::from_raw(std::ptr::null_mut()) }.is_none());
    assert!(unsafe { PreviewActionItem::from_raw(std::ptr::null_mut()) }.is_none());

    let _: fn(&ContextMenuElementInfo) -> *mut std::ffi::c_void = ContextMenuElementInfo::as_raw;
    let _: fn(&ContextMenuElementInfo) -> Option<String> = ContextMenuElementInfo::link_url;
    let _: fn(&PreviewElementInfo) -> *mut std::ffi::c_void = PreviewElementInfo::as_raw;
    let _: fn(&PreviewElementInfo) -> Option<String> = PreviewElementInfo::link_url;
    let _: fn(&PreviewActionItem) -> *mut std::ffi::c_void = PreviewActionItem::as_raw;
    let _: fn(&PreviewActionItem) -> String = PreviewActionItem::identifier;
    let _: fn(&PreviewActionItem) -> String = PreviewActionItem::title;
}
