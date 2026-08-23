use crate::error::{check, Result, TransientTaskError};
use ohos_transient_task_sys as sys;
use std::ffi::CString;
use std::time::Duration;

/// Callback invoked by the system when the granted delay is about to expire.
///
/// The native type is `void (*)(void)`: it carries no user-data pointer, so a
/// Rust closure cannot be handed to it safely. Pass an `extern "C" fn` and reach
/// any state it needs through a `static`.
///
/// It runs on a system-owned thread, so it must not unwind: a panic crossing the
/// C frame is undefined behaviour. Keep the body short — the process is about to
/// be suspended.
pub type ExpiredCallback = unsafe extern "C" fn();

/// What the system granted for one delay request, mirroring
/// `TransientTask_DelaySuspendInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DelaySuspendInfo {
    request_id: i32,
    actual_delay_time_ms: i32,
}

impl DelaySuspendInfo {
    pub(crate) fn from_raw(raw: sys::TransientTask_DelaySuspendInfo) -> Self {
        DelaySuspendInfo {
            request_id: raw.requestId,
            actual_delay_time_ms: raw.actualDelayTime,
        }
    }

    /// The unique identifier of the delay request.
    pub fn request_id(&self) -> i32 {
        self.request_id
    }

    /// The granted delay duration in milliseconds, exactly as reported.
    ///
    /// The system decides this value; it is not the amount asked for, because
    /// the C API has no way to ask for one.
    pub fn actual_delay_time_ms(&self) -> i32 {
        self.actual_delay_time_ms
    }

    /// [`DelaySuspendInfo::actual_delay_time_ms`] as a [`Duration`], clamping a
    /// negative report to zero.
    pub fn actual_delay(&self) -> Duration {
        millis_to_duration(self.actual_delay_time_ms)
    }
}

/// A granted transient task: the application is kept out of the suspended state
/// until the delay expires or the request is cancelled.
///
/// The request is cancelled on [`Drop`], so the task lives exactly as long as
/// the guard. Use [`SuspendDelay::cancel`] to cancel it early and see whether
/// the cancellation succeeded.
///
/// An application may hold only a small number of concurrent requests, so keep
/// the guard no longer than the work it covers.
///
/// ```no_run
/// use ohos_transient_task_binding::SuspendDelay;
///
/// // Runs on a system thread shortly before the delay expires; must not unwind.
/// unsafe extern "C" fn on_expired() {
///     // signal the worker to wind down
/// }
///
/// let task = SuspendDelay::request("uploading pending records", on_expired)?;
/// println!("granted {:?} for request {}", task.actual_delay(), task.request_id());
///
/// // ... do the short piece of background work ...
/// let left = task.remaining_delay()?;
///
/// // Cancel explicitly to observe failures; dropping cancels silently.
/// task.cancel()?;
/// # Ok::<(), ohos_transient_task_binding::TransientTaskError>(())
/// ```
#[derive(Debug)]
pub struct SuspendDelay {
    info: DelaySuspendInfo,
    cancelled: bool,
}

impl SuspendDelay {
    /// Request a delayed transition to the suspended state.
    ///
    /// `reason` is recorded by the system and must not contain an interior NUL
    /// byte. `on_expired` is invoked when the granted delay is about to run out.
    pub fn request(reason: &str, on_expired: ExpiredCallback) -> Result<Self> {
        let reason = CString::new(reason).map_err(|_| TransientTaskError::InvalidReason)?;
        let mut raw = sys::TransientTask_DelaySuspendInfo {
            requestId: 0,
            actualDelayTime: 0,
        };
        // SAFETY: `reason` is a NUL-terminated string that outlives the call and
        // `raw` is a local the service writes the granted request into.
        check(unsafe {
            sys::OH_BackgroundTaskManager_RequestSuspendDelay(
                reason.as_ptr(),
                Some(on_expired),
                &mut raw,
            )
        })?;
        Ok(SuspendDelay {
            info: DelaySuspendInfo::from_raw(raw),
            cancelled: false,
        })
    }

    /// What the system granted for this request.
    pub fn info(&self) -> DelaySuspendInfo {
        self.info
    }

    /// The unique identifier of this delay request.
    pub fn request_id(&self) -> i32 {
        self.info.request_id()
    }

    /// The granted delay duration in milliseconds, exactly as reported.
    pub fn actual_delay_time_ms(&self) -> i32 {
        self.info.actual_delay_time_ms()
    }

    /// The granted delay as a [`Duration`].
    pub fn actual_delay(&self) -> Duration {
        self.info.actual_delay()
    }

    /// The time left before the application enters the suspended state, in
    /// milliseconds, exactly as reported.
    pub fn remaining_delay_time_ms(&self) -> Result<i32> {
        let mut remaining = 0i32;
        // SAFETY: `remaining` is a local slot the service writes the value into.
        check(unsafe {
            sys::OH_BackgroundTaskManager_GetRemainingDelayTime(self.request_id(), &mut remaining)
        })?;
        Ok(remaining)
    }

    /// [`SuspendDelay::remaining_delay_time_ms`] as a [`Duration`], clamping a
    /// negative report to zero.
    pub fn remaining_delay(&self) -> Result<Duration> {
        Ok(millis_to_duration(self.remaining_delay_time_ms()?))
    }

    /// Cancel the request, reporting failures instead of swallowing them as
    /// [`Drop`] does. Consumes the guard either way.
    pub fn cancel(mut self) -> Result<()> {
        // Set first: the request must not be cancelled a second time by `Drop`,
        // whatever the call returns.
        self.cancelled = true;
        // SAFETY: `request_id` was handed out by a successful request and has
        // not been cancelled yet.
        check(unsafe { sys::OH_BackgroundTaskManager_CancelSuspendDelay(self.request_id()) })
    }
}

impl Drop for SuspendDelay {
    fn drop(&mut self) {
        if self.cancelled {
            return;
        }
        // SAFETY: `request_id` was handed out by a successful request and has
        // not been cancelled yet.
        unsafe { sys::OH_BackgroundTaskManager_CancelSuspendDelay(self.info.request_id) };
    }
}

fn millis_to_duration(millis: i32) -> Duration {
    Duration::from_millis(millis.max(0) as u64)
}
