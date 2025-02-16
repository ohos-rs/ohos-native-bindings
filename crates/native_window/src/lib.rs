use ohos_native_window_sys::{
    NativeWindow as NativeWindowRaw, OH_NativeWindow_NativeWindowHandleOpt,
};
use std::{os::raw::c_void, ptr::NonNull};

mod operation;

pub use operation::*;

pub struct NativeWindow {
    window: NonNull<NativeWindowRaw>,
}

impl NativeWindow {
    /// Create a new `NativeWindow` from a raw pointer.
    pub fn from_raw(window: *mut c_void) -> Self {
        #[cfg(debug_assertions)]
        assert!(!window.is_null(), "The window pointer must not be null.");
        Self {
            window: unsafe { NonNull::new_unchecked(window.cast()) },
        }
    }

    pub fn set_buffer_geometry(&self, width: i32, height: i32) {
        unsafe {
            OH_NativeWindow_NativeWindowHandleOpt(
                self.window.as_ptr(),
                Operation::SetBufferGeometry.into_i32(),
                width,
                height,
            );
        }
    }

    pub fn set_format(&self) {}
}
