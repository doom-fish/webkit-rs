use core::ffi::c_void;
use core::hash::{Hash, Hasher};
use core::ptr;

use crate::ffi;

/// Wraps `WKNavigation`.
pub struct Navigation {
    ptr: *mut c_void,
}

unsafe impl Send for Navigation {}

impl Navigation {
    #[must_use]
    pub(crate) fn from_ptr(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Returns the corresponding value from `WKNavigation`.
    #[must_use]
    pub fn id(&self) -> usize {
        self.ptr as usize
    }
}

impl core::fmt::Debug for Navigation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Navigation")
            .field("id", &self.id())
            .finish()
    }
}

impl PartialEq for Navigation {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl Eq for Navigation {}

impl Hash for Navigation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

impl Drop for Navigation {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wk_navigation_release(self.ptr) }
            self.ptr = ptr::null_mut();
        }
    }
}
