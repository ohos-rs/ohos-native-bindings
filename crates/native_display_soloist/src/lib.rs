use std::{
    cell::RefCell,
    os::raw::c_void,
    sync::{LazyLock, RwLock},
};

use ohos_display_soloist_sys::{
    DisplaySoloist_ExpectedRateRange, OH_DisplaySoloist, OH_DisplaySoloist_Create,
    OH_DisplaySoloist_Destroy, OH_DisplaySoloist_SetExpectedFrameRateRange,
    OH_DisplaySoloist_Start, OH_DisplaySoloist_Stop,
};

static DISPLAY_SOLOIST: LazyLock<
    RwLock<Option<Option<Box<dyn Fn(i64, i64, *mut c_void) + Send + Sync>>>>,
> = LazyLock::new(|| RwLock::new(None));

pub use ohos_display_soloist_sys::DisplaySoloist_ExpectedRateRange;

pub struct DisplaySoloist {
    raw: *mut OH_DisplaySoloist,
    running: RefCell<bool>,
}

impl DisplaySoloist {
    pub fn new(use_exclusive_thread: bool) -> Self {
        Self {
            raw: unsafe { OH_DisplaySoloist_Create(use_exclusive_thread) },
            running: RefCell::new(false),
        }
    }

    pub fn set_frame_rate(&self, frame_rate: DisplaySoloist_ExpectedRateRange) {
        unsafe {
            OH_DisplaySoloist_SetExpectedFrameRateRange(self.raw, &frame_rate as *const _ as *mut _)
        };
    }

    /// callback will execute in the sub-thread as the caller
    /// we need to ensure the callback is thread-safe
    pub fn on_frame<F>(&self, data: *mut c_void, callback: F)
    where
        F: Fn(i64, i64, *mut c_void) + Send + Sync + 'static,
    {
        let mut display_soloist = DISPLAY_SOLOIST.write().unwrap();
        *display_soloist = Some(Some(Box::new(callback)));

        unsafe { OH_DisplaySoloist_Start(self.raw, Some(frame_callback), data) };
        self.running.replace(true);
    }

    pub fn stop(&self) {
        unsafe {
            OH_DisplaySoloist_Stop(self.raw);
        }
        self.running.replace(false);
    }
}

extern "C" fn frame_callback(timestamp: i64, target_timestamp: i64, data: *mut c_void) {
    let display_soloist = DISPLAY_SOLOIST.read().unwrap();
    if let Some(Some(callback)) = &*display_soloist {
        callback(timestamp, target_timestamp, data);
    }
}

impl Drop for DisplaySoloist {
    fn drop(&mut self) {
        if self.running.borrow().clone() {
            self.stop();
        }
        unsafe {
            OH_DisplaySoloist_Destroy(self.raw);
        }
    }
}
