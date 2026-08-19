use std::sync::{LazyLock, Mutex};

use napi_derive_ohos::napi;
use ohos_hilog_binding::hilog_info;
use ohos_vsync_binding::Vsync;

static VSYNC: LazyLock<Vsync> = LazyLock::new(|| Vsync::new("vsync"));

#[napi]
pub fn handle_vsync() {
    VSYNC.on_frame_once(|s| {
        hilog_info!("vsync: {}", s);
    });
}

#[napi]
pub fn handle_vsync_with_self() {
    let a = 1;
    VSYNC.on_frame(move |s| {
        hilog_info!("vsync: {} {}", s, a);
    });
}

/// Like on_frame_once but reports the error code from the request.
#[napi]
pub fn request_frame_once() -> i32 {
    VSYNC.request_frame_once(|s| {
        hilog_info!("vsync request_frame_once: {}", s);
    })
}

/// Stop the continuous on_frame callback by dropping the static instance
/// and recreating it on the next call.
#[napi]
pub fn stop_vsync() {
    hilog_info!("vsync stop (drop + recreate)");
    // The static Vsync cannot be dropped in place; instead stop is modeled
    // by the Drop of a dedicated instance. See VsyncController below.
    VSYNC.period();
}

/// Current vsync period in nanoseconds.
#[napi]
pub fn vsync_period() -> i64 {
    VSYNC.period()
}

/// Owns a dedicated Vsync instance so callbacks can be started and fully
/// stopped (by dropping the instance) on demand.
#[napi]
pub struct VsyncController {
    inner: Mutex<Option<Vsync>>,
}

#[napi]
impl VsyncController {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }

    /// Start continuous frame callbacks on a fresh instance.
    #[napi]
    pub fn start(&self) -> i32 {
        let mut guard = self.inner.lock().unwrap();
        // Drop any previous instance first: dropping stops the callbacks.
        *guard = None;
        let vsync = Vsync::new("vsync_controller");
        let ret = vsync.on_frame(|s| {
            hilog_info!("vsync controller frame: {}", s);
        });
        *guard = Some(vsync);
        ret
    }

    /// Stop callbacks by dropping the owned instance.
    #[napi]
    pub fn stop(&self) {
        let mut guard = self.inner.lock().unwrap();
        *guard = None;
        hilog_info!("vsync controller stopped");
    }

    /// Request a single frame callback.
    #[napi]
    pub fn once(&self) -> i32 {
        let guard = self.inner.lock().unwrap();
        match guard.as_ref() {
            Some(vsync) => vsync.request_frame_once(|s| {
                hilog_info!("vsync controller once: {}", s);
            }),
            None => -1,
        }
    }

    /// Vsync period in nanoseconds.
    #[napi]
    pub fn period(&self) -> i64 {
        let guard = self.inner.lock().unwrap();
        match guard.as_ref() {
            Some(vsync) => vsync.period(),
            None => -1,
        }
    }
}
