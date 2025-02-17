use ohos_native_window_sys::{
    NativeWindow as NativeWindowRaw, OH_NativeWindow_NativeObjectReference,
    OH_NativeWindow_NativeObjectUnreference, OH_NativeWindow_NativeWindowHandleOpt,
};
use std::{os::raw::c_void, ptr::NonNull};

mod operation;

pub use operation::*;

pub struct NativeWindow {
    window: NonNull<NativeWindowRaw>,
}

impl NativeWindow {
    /// Create a new `NativeWindow` from a raw pointer and add a reference to it.
    pub fn clone_from_ptr(window: *mut c_void) -> Self {
        #[cfg(debug_assertions)]
        assert!(!window.is_null(), "The window pointer must not be null.");

        let ret = unsafe { OH_NativeWindow_NativeObjectReference(window) };
        #[cfg(debug_assertions)]
        assert!(ret == 1, "OH_NativeWindow_NativeObjectReference failed");

        unsafe {
            NativeWindow {
                window: NonNull::new_unchecked(window as *mut NativeWindowRaw),
            }
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

impl Drop for NativeWindow {
    fn drop(&mut self) {
        let ret = unsafe { OH_NativeWindow_NativeObjectUnreference(self.window.as_ptr().cast()) };
        #[cfg(debug_assertions)]
        assert!(ret == 1, "OH_NativeWindow_NativeObjectUnreference failed");
    }
}
