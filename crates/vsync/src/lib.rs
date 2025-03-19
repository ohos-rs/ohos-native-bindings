use std::{ffi::CString, os::raw::c_void, ptr::NonNull};

use ohos_native_vsync_sys::{
    OH_NativeVSync, OH_NativeVSync_Create, OH_NativeVSync_Destroy, OH_NativeVSync_GetPeriod,
    OH_NativeVSync_RequestFrame, OH_NativeVSync_RequestFrameWithMultiCallback,
};

use std::marker::PhantomData;

pub struct Vsync<'a> {
    raw: NonNull<OH_NativeVSync>,
    _phantom: PhantomData<&'a ()>,
}

unsafe impl<'a> Send for Vsync<'a> {}
unsafe impl<'a> Sync for Vsync<'a> {}

struct VsyncData {
    callback: Box<dyn FnMut(i64)>,
    raw: NonNull<OH_NativeVSync>,
}

impl<'a> Vsync<'a> {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        let name = CString::new(name.as_ref()).expect("CString::new failed");
        let vsync =
            unsafe { OH_NativeVSync_Create(name.as_ptr().cast(), name.to_bytes().len() as u32) };
        Vsync {
            raw: NonNull::new(vsync).expect("OH_NativeVSync_Create failed"),
            _phantom: PhantomData,
        }
    }

    /// This function is used to handle the vsync event.
    ///
    /// The callback function will be called when the vsync event occurs.
    pub fn on_frame_once<F: FnMut(i64) + 'a>(&self, mut callback: F) {
        let callback_with_data = unsafe {
            std::mem::transmute::<Box<dyn FnMut(i64)>, Box<dyn FnMut(i64) + 'static>>(Box::new(
                |time: i64| {
                    callback(time);
                },
            ))
        };

        let data = Box::new(VsyncData {
            callback: Box::new(callback_with_data),
            raw: self.raw,
        });

        unsafe {
            OH_NativeVSync_RequestFrame(
                self.raw.as_ptr(),
                Some(request_frame_callback),
                Box::into_raw(data) as _,
            );
        }
    }

    /// This function is used to handle the vsync event with multiple callbacks.
    pub fn on_frame_once_with_multi_callback<F: FnMut(i64) + 'a>(&self, mut callback: F) {
        let f = unsafe {
            std::mem::transmute::<Box<dyn FnMut(i64)>, Box<dyn FnMut(i64) + 'static>>(Box::new(
                move |t| {
                    callback(t);
                },
            ))
        };
        let data = Box::new(VsyncData {
            callback: Box::new(f),
            raw: self.raw,
        });
        unsafe {
            OH_NativeVSync_RequestFrameWithMultiCallback(
                self.raw.as_ptr(),
                Some(request_frame_callback),
                Box::into_raw(data) as _,
            );
        }
    }

    /// This function is used to handle the vsync event with every frame.
    ///
    /// The callback function will be called every frame, and the callback function can call
    /// `OH_NativeVSync_RequestFrame` to request the next frame.
    pub fn on_frame<F: FnMut(i64) + 'a>(&self, mut callback: F) {
        let callback_with_data = unsafe {
            std::mem::transmute::<Box<dyn FnMut(i64)>, Box<dyn FnMut(i64) + 'static>>(Box::new(
                |time: i64| {
                    callback(time);
                },
            ))
        };

        let data = Box::new(VsyncData {
            callback: Box::new(callback_with_data),
            raw: self.raw,
        });

        unsafe {
            OH_NativeVSync_RequestFrame(
                self.raw.as_ptr(),
                Some(request_frame_callback_with_self),
                Box::into_raw(data) as _,
            );
        }
    }

    pub fn period(&self) -> i64 {
        let mut period = 0;
        unsafe {
            OH_NativeVSync_GetPeriod(self.raw.as_ptr(), &mut period);
        }
        period
    }
}

extern "C" fn request_frame_callback(timestamp: i64, data: *mut c_void) {
    let mut data = unsafe { Box::from_raw(data as *mut VsyncData) };
    let handle = &mut data.callback;
    handle(timestamp);
}

extern "C" fn request_frame_callback_with_self(timestamp: i64, data: *mut c_void) {
    let mut raw_data = unsafe { Box::from_raw(data as *mut VsyncData) };
    // request next frame
    unsafe {
        OH_NativeVSync_RequestFrame(
            raw_data.raw.as_ptr(),
            Some(request_frame_callback_with_self),
            data,
        );
    }
    let handle = &mut raw_data.callback;
    // handle current frame
    handle(timestamp);
    let _ = Box::into_raw(raw_data);
}

impl<'a> Drop for Vsync<'a> {
    fn drop(&mut self) {
        unsafe { OH_NativeVSync_Destroy(self.raw.as_ptr()) }
    }
}
