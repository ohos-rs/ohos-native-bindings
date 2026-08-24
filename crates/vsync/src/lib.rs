use std::sync::{Arc, Mutex};
use std::{ffi::CString, os::raw::c_void, ptr::NonNull};

use ohos_native_vsync_sys::{
    OH_NativeVSync, OH_NativeVSync_Create, OH_NativeVSync_Destroy, OH_NativeVSync_GetPeriod,
    OH_NativeVSync_RequestFrameWithMultiCallback,
};

#[cfg(feature = "api-14")]
use ohos_native_vsync_sys::OH_NativeVSync_Create_ForAssociatedWindow;

#[cfg(feature = "api-20")]
pub use ohos_native_vsync_sys::OH_NativeVSync_ExpectedRateRange;

#[cfg(feature = "api-20")]
use ohos_native_vsync_sys::OH_NativeVSync_SetExpectedFrameRateRange;

/// Shared liveness gate between a [`Vsync`] owner and its in-flight frame
/// callbacks.
///
/// The NDK has no API to cancel an already requested frame, so `Drop` cannot
/// guarantee that no callback fires afterwards. Instead the handle lives
/// behind this mutex: `Drop` takes it out and destroys it while holding the
/// lock, and callbacks re-check it under the same lock before touching the
/// native handle. A late callback finds `None`, drops its state and returns —
/// a bounded no-op instead of a use-after-free.
struct VsyncCore {
    raw: Mutex<Option<RawVsync>>,
}

/// Address of the native vsync object.
#[derive(Clone, Copy)]
struct RawVsync(NonNull<OH_NativeVSync>);

// SAFETY: `RawVsync` is a plain address; every dereference happens through
// NDK calls that are themselves guarded by the `VsyncCore` mutex.
unsafe impl Send for RawVsync {}

pub struct Vsync {
    raw: NonNull<OH_NativeVSync>,
    core: Arc<VsyncCore>,
}

// SAFETY: all native access from this handle goes through `core`'s mutex (or
// through `raw` for calls made while `self` is alive, which `Drop`'s `&mut`
// exclusivity orders correctly for a single owner). Frame callbacks run on
// the vsync thread and synchronize through the same mutex.
unsafe impl Send for Vsync {}
// SAFETY: `&self` methods only issue NDK calls documented as callable from
// any thread while the object is alive; mutation of shared Rust state is
// mutex-guarded in `core`.
unsafe impl Sync for Vsync {}

struct VsyncData {
    callback: Box<dyn FnMut(i64) + Send + 'static>,
    core: Arc<VsyncCore>,
}

impl Vsync {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self::try_new(name).expect("OH_NativeVSync_Create failed")
    }

    pub fn try_new<T: AsRef<str>>(name: T) -> Option<Self> {
        let name = CString::new(name.as_ref()).ok()?;
        let vsync =
            unsafe { OH_NativeVSync_Create(name.as_ptr().cast(), name.to_bytes().len() as u32) };
        Self::from_raw(vsync)
    }

    #[cfg(feature = "api-14")]
    pub fn try_new_for_associated_window<T: AsRef<str>>(window_id: u64, name: T) -> Option<Self> {
        let name = CString::new(name.as_ref()).ok()?;
        let vsync = unsafe {
            OH_NativeVSync_Create_ForAssociatedWindow(
                window_id,
                name.as_ptr().cast(),
                name.to_bytes().len() as u32,
            )
        };
        Self::from_raw(vsync)
    }

    fn from_raw(vsync: *mut OH_NativeVSync) -> Option<Self> {
        let raw = NonNull::new(vsync)?;
        Some(Vsync {
            raw,
            core: Arc::new(VsyncCore {
                raw: Mutex::new(Some(RawVsync(raw))),
            }),
        })
    }

    /// This function is used to handle the vsync event.
    ///
    /// The callback function will be called when the vsync event occurs.
    pub fn on_frame_once<F: FnMut(i64) + Send + 'static>(&self, callback: F) {
        let _ = self.request_frame_once(callback);
    }

    pub fn request_frame_once<F: FnMut(i64) + Send + 'static>(&self, callback: F) -> i32 {
        self.request_frame_raw(callback, request_frame_callback)
    }

    /// This function is used to handle the vsync event with multiple callbacks.
    pub fn on_frame_once_with_multi_callback<F: FnMut(i64) + Send + 'static>(
        &self,
        callback: F,
    ) -> i32 {
        self.request_frame_once(callback)
    }

    /// This function is used to handle the vsync event with every frame.
    ///
    /// The callback function will be called every frame by repeatedly requesting the next frame
    /// with `OH_NativeVSync_RequestFrameWithMultiCallback`. The chain stops on
    /// its own once this `Vsync` is dropped.
    pub fn on_frame<F: FnMut(i64) + Send + 'static>(&self, callback: F) -> i32 {
        self.request_frame_raw(callback, request_frame_callback_with_self)
    }

    fn request_frame_raw<F: FnMut(i64) + Send + 'static>(
        &self,
        callback: F,
        native_callback: extern "C" fn(i64, *mut c_void),
    ) -> i32 {
        let data = Box::into_raw(Box::new(VsyncData {
            callback: Box::new(callback),
            core: self.core.clone(),
        }));
        let ret = unsafe {
            OH_NativeVSync_RequestFrameWithMultiCallback(
                self.raw.as_ptr(),
                Some(native_callback),
                data as _,
            )
        };
        if ret != 0 {
            // SAFETY: on failure the NDK retains no reference to `data`, so
            // this is the only ownership path.
            unsafe {
                drop(Box::from_raw(data));
            }
        }
        ret
    }

    pub fn period(&self) -> i64 {
        let mut period = 0;
        unsafe {
            OH_NativeVSync_GetPeriod(self.raw.as_ptr(), &mut period);
        }
        period
    }

    #[cfg(feature = "api-20")]
    pub fn set_expected_frame_rate_range(&self, range: OH_NativeVSync_ExpectedRateRange) -> i32 {
        let mut range = range;
        unsafe { OH_NativeVSync_SetExpectedFrameRateRange(self.raw.as_ptr(), &mut range) }
    }
}

fn is_alive(core: &VsyncCore) -> bool {
    core.raw.lock().map(|raw| raw.is_some()).unwrap_or(false)
}

extern "C" fn request_frame_callback(timestamp: i64, data: *mut c_void) {
    // SAFETY: `data` was produced by `Box::into_raw` in `request_frame_raw`
    // and the NDK delivers each requested frame exactly once.
    let mut data = unsafe { Box::from_raw(data as *mut VsyncData) };
    if !is_alive(&data.core) {
        return;
    }
    (data.callback)(timestamp);
}

extern "C" fn request_frame_callback_with_self(timestamp: i64, data: *mut c_void) {
    // SAFETY: same ownership contract as `request_frame_callback`.
    let mut raw_data = unsafe { Box::from_raw(data as *mut VsyncData) };
    if !is_alive(&raw_data.core) {
        return;
    }
    (raw_data.callback)(timestamp);

    // Re-request while holding the liveness lock: `Drop` destroys the native
    // handle under the same lock, so the handle cannot die between the check
    // and the call.
    let core = raw_data.core.clone();
    let Ok(guard) = core.raw.lock() else {
        return;
    };
    let Some(raw) = *guard else {
        return;
    };
    let data = Box::into_raw(raw_data);
    let ret = unsafe {
        OH_NativeVSync_RequestFrameWithMultiCallback(
            raw.0.as_ptr(),
            Some(request_frame_callback_with_self),
            data as _,
        )
    };
    if ret != 0 {
        // SAFETY: the NDK rejected the request and retains no reference.
        unsafe {
            drop(Box::from_raw(data));
        }
    }
}

impl Drop for Vsync {
    fn drop(&mut self) {
        // Take the handle out under the lock so no frame callback can race
        // the destruction; late callbacks observe `None` and back off.
        let taken = match self.core.raw.lock() {
            Ok(mut raw) => raw.take(),
            Err(poisoned) => poisoned.into_inner().take(),
        };
        if let Some(raw) = taken {
            unsafe { OH_NativeVSync_Destroy(raw.0.as_ptr()) }
        }
    }
}
