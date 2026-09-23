use core::ffi::c_void;
use core::fmt;

use crate::ffi;

#[allow(missing_docs)]
pub struct FrameHandle {
    ptr: *mut c_void,
}

unsafe impl Send for FrameHandle {}
unsafe impl Sync for FrameHandle {}

impl FrameHandle {
    pub(crate) fn from_retained(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl fmt::Debug for FrameHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FrameHandle").finish_non_exhaustive()
    }
}

impl PartialEq for FrameHandle {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl Eq for FrameHandle {}

crate::utils::retained::wk_retained!(
    FrameHandle,
    field = ptr,
    retain = ffi::wk_frame_info_retain,
    release = ffi::wk_frame_info_release,
);
