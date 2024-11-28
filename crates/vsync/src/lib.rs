use std::{cell::RefCell, ffi::CString, os::raw::c_void};

use ohos_vsync_sys::{
    OH_NativeVSync, OH_NativeVSync_Create, OH_NativeVSync_Destroy, OH_NativeVSync_GetPeriod,
    OH_NativeVSync_RequestFrame, OH_NativeVSync_RequestFrameWithMultiCallback,
};

pub struct Vsync(*const OH_NativeVSync);

thread_local! {
    static VSYNC_HANDLE: RefCell<Option<Box<dyn Fn(u32, *mut c_void) + 'static>>> = RefCell::new(None);
}

impl Vsync {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        let name = CString::new(name.as_ref()).expect("CString::new failed");
        let vsync =
            unsafe { OH_NativeVSync_Create(name.as_ptr().cast(), name.to_bytes().len() as u32) };
        Vsync(vsync)
    }

    pub fn on_frame<F: Fn(u32, *mut c_void) + 'static>(&self, data: *mut c_void, callback: F) {
        VSYNC_HANDLE.with_borrow_mut(|f| {
            *f = Some(Box::new(callback));
        });
        unsafe {
            OH_NativeVSync_RequestFrame(
                self.0 as *mut OH_NativeVSync,
                Some(request_frame_callback),
                data,
            );
        }
    }

    pub fn on_frame_with_multi_callback<F: Fn(u32, *mut c_void) + 'static>(
        &self,
        data: *mut c_void,
        callback: F,
    ) {
        VSYNC_HANDLE.with_borrow_mut(|f| {
            *f = Some(Box::new(callback));
        });
        unsafe {
            OH_NativeVSync_RequestFrameWithMultiCallback(
                self.0 as *mut OH_NativeVSync,
                Some(request_frame_callback),
                data,
            );
        }
    }

    pub fn period(&self) -> i64 {
        let mut period = 0;
        unsafe {
            OH_NativeVSync_GetPeriod(self.0 as *mut OH_NativeVSync, &mut period);
        }
        period
    }
}

extern "C" fn request_frame_callback(timestamp: i64, data: *mut c_void) {
    VSYNC_HANDLE.with_borrow(|f| {
        if let Some(f) = f.as_ref() {
            f(timestamp as u32, data);
        }
    })
}

impl Drop for Vsync {
    fn drop(&mut self) {
        unsafe { OH_NativeVSync_Destroy(self.0 as *mut OH_NativeVSync) }
    }
}
