use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;
use std::{os::raw::c_void, ptr::NonNull};

use ohos_display_soloist_sys::{
    OH_DisplaySoloist, OH_DisplaySoloist_Create, OH_DisplaySoloist_Destroy,
    OH_DisplaySoloist_SetExpectedFrameRateRange, OH_DisplaySoloist_Start, OH_DisplaySoloist_Stop,
};

pub use ohos_display_soloist_sys::DisplaySoloist_ExpectedRateRange;

pub struct DisplaySoloist {
    raw: NonNull<OH_DisplaySoloist>,
    inner: RwLock<Option<*mut DisplaySoloistInner>>,
    is_running: AtomicBool,
}

struct DisplaySoloistInner {
    callback: Box<dyn FnMut(i64, i64) + Send>,
}

impl DisplaySoloist {
    pub fn new(use_exclusive_thread: bool) -> Self {
        let raw = unsafe { OH_DisplaySoloist_Create(use_exclusive_thread) };
        Self {
            raw: NonNull::new(raw).expect("Failed to create DisplaySoloist"),
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

    /// Start frame callbacks.
    ///
    /// The callback executes on the soloist's own thread, which is why it
    /// must be `Send + 'static` — the previous signature accepted short-lived,
    /// thread-bound closures and laundered them with a lifetime transmute.
    /// Calling `on_frame` again replaces the previous callback: the soloist
    /// is stopped first so the old closure cannot run concurrently with its
    /// release.
    pub fn on_frame<F>(&self, callback: F)
    where
        F: FnMut(i64, i64) + Send + 'static,
    {
        self.stop_and_release_callback();

        let data = Box::into_raw(Box::new(DisplaySoloistInner {
            callback: Box::new(callback),
        }));
        {
            let mut guard = match self.inner.write() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            *guard = Some(data);
        }
        unsafe { OH_DisplaySoloist_Start(self.raw.as_ptr(), Some(frame_callback), data as _) };
        self.is_running.store(true, Ordering::Release);
    }

    pub fn stop(&self) {
        if self.is_running.swap(false, Ordering::AcqRel) {
            unsafe {
                OH_DisplaySoloist_Stop(self.raw.as_ptr());
            }
        }
    }

    /// Stop callbacks and free the stored closure.
    ///
    /// `OH_DisplaySoloist_Stop` synchronizes with the soloist thread, so the
    /// closure cannot be mid-invocation once it returns.
    fn stop_and_release_callback(&self) {
        self.stop();
        let taken = {
            let mut guard = match self.inner.write() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            guard.take()
        };
        if let Some(data) = taken {
            // SAFETY: produced by `Box::into_raw` in `on_frame`; the soloist
            // is stopped, so the frame callback no longer dereferences it.
            let _ = unsafe { Box::from_raw(data) };
        }
    }
}

extern "C" fn frame_callback(timestamp: i64, target_timestamp: i64, data: *mut c_void) {
    // SAFETY: `data` outlives every callback invocation: it is freed only in
    // `stop_and_release_callback`, which stops the soloist (synchronizing
    // with this thread) before reclaiming the box.
    let raw_data = unsafe { &mut *(data as *mut DisplaySoloistInner) };
    (raw_data.callback)(timestamp, target_timestamp);
}

// SAFETY: the stored callback is `Send`, the callback pointer cell is behind
// an `RwLock`, and the run flag is atomic; transferring the owner between
// threads moves no thread-bound state.
unsafe impl Send for DisplaySoloist {}
// SAFETY: `&self` methods mutate shared state only through the `RwLock` and
// the atomic flag; native start/stop/set-rate calls are serialized against
// callback teardown by `OH_DisplaySoloist_Stop`'s synchronization.
unsafe impl Sync for DisplaySoloist {}

impl Drop for DisplaySoloist {
    fn drop(&mut self) {
        // Stop (synchronizes with the soloist thread), destroy the native
        // object, then free the closure — never free while callbacks can run.
        self.stop_and_release_callback();
        unsafe {
            OH_DisplaySoloist_Destroy(self.raw.as_ptr());
        }
    }
}
