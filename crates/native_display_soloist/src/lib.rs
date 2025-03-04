use std::marker::PhantomData;
use std::sync::atomic::AtomicBool;
use std::{os::raw::c_void, ptr::NonNull, sync::RwLock};

use ohos_display_soloist_sys::{
    OH_DisplaySoloist, OH_DisplaySoloist_Create, OH_DisplaySoloist_Destroy,
    OH_DisplaySoloist_SetExpectedFrameRateRange, OH_DisplaySoloist_Start, OH_DisplaySoloist_Stop,
};

pub use ohos_display_soloist_sys::DisplaySoloist_ExpectedRateRange;

pub struct DisplaySoloist<'a> {
    raw: NonNull<OH_DisplaySoloist>,
    phantom: PhantomData<&'a ()>,
    inner: RwLock<Option<*mut DisplaySoloistInner>>,
    is_running: AtomicBool,
}

struct DisplaySoloistInner {
    callback: Box<dyn FnMut(i64, i64)>,
}

impl<'a> DisplaySoloist<'a> {
    pub fn new(use_exclusive_thread: bool) -> Self {
        let raw = unsafe { OH_DisplaySoloist_Create(use_exclusive_thread) };
        Self {
            raw: NonNull::new(raw).expect("Failed to create DisplaySoloist"),
            phantom: PhantomData,
            inner: RwLock::new(None),
            is_running: AtomicBool::new(false),
        }
    }

    pub fn set_frame_rate(&self, frame_rate: DisplaySoloist_ExpectedRateRange) {
        unsafe {
            OH_DisplaySoloist_SetExpectedFrameRateRange(
                self.raw.as_ptr(),
                &frame_rate as *const _ as *mut _,
            )
        };
    }

    /// callback will execute in the sub-thread as the caller
    /// we need to ensure the callback is thread-safe
    pub fn on_frame<F>(&self, mut callback: F)
    where
        F: FnMut(i64, i64) + 'a,
    {
        let f = unsafe {
            std::mem::transmute::<Box<dyn FnMut(i64, i64)>, Box<dyn FnMut(i64, i64) + 'static>>(
                Box::new(move |ts, tts| {
                    callback(ts, tts);
                }),
            )
        };
        let data = Box::into_raw(Box::new(DisplaySoloistInner { callback: f }));
        // save the data to the inner and drop it with the DisplaySoloist drop
        let mut guard = self.inner.write().expect("Failed to write inner");
        *guard = Some(data);

        unsafe { OH_DisplaySoloist_Start(self.raw.as_ptr(), Some(frame_callback), data as _) };
    }

    pub fn stop(&self) {
        if self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            unsafe {
                OH_DisplaySoloist_Stop(self.raw.as_ptr());
            }
            self.is_running
                .store(false, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

extern "C" fn frame_callback(timestamp: i64, target_timestamp: i64, data: *mut c_void) {
    let raw_data = unsafe { &mut *(data as *mut DisplaySoloistInner) };
    let handle = &mut raw_data.callback;
    handle(timestamp, target_timestamp);
}

unsafe impl<'a> Send for DisplaySoloist<'a> {}
unsafe impl<'a> Sync for DisplaySoloist<'a> {}

impl<'a> Drop for DisplaySoloist<'a> {
    fn drop(&mut self) {
        if self.is_running.load(std::sync::atomic::Ordering::Relaxed) {
            self.stop();
        }
        let mut guard = self.inner.write().expect("Failed to write inner");
        // Drop it before destroying the DisplaySoloist
        if let Some(data) = guard.take() {
            let _ = unsafe { Box::from_raw(data) };
        }
        unsafe {
            OH_DisplaySoloist_Destroy(self.raw.as_ptr());
        }
    }
}
