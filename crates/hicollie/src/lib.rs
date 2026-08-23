//! Safe Rust bindings for OpenHarmony **hicollie** (software watchdog).
//!
//! hicollie helps an application detect that its own threads are stuck or
//! janky. A business thread registers a periodic liveness task (stuck
//! detection) or a pair of begin/end stubs bracketing event processing (jank
//! detection); the monitor thread can then report a freeze event. This crate
//! wraps the native `hicollie.h` C API with a safe layer.
//!
//! Everything introduced after API 12 sits behind the matching `api-*` feature.
//!
//! The raw bindings are re-exported as [`sys`] for anything not covered here,
//! including the API 18 timer (`OH_HiCollie_SetTimer` / `OH_HiCollie_CancelTimer`)
//! and the API 24 freeze callback and associate-process report.
//!
//! # Example
//!
//! ```no_run
//! use ohos_hicollie_binding as hicollie;
//!
//! // Called every 3 seconds on a HiCollie-owned thread to probe liveness.
//! unsafe extern "C" fn detect() {
//!     // signal the business thread and check whether it answered in time
//! }
//!
//! // Must run off the main thread.
//! hicollie::init_stuck_detection(detect)?;
//!
//! // From the HiCollie monitor thread, report a stuck event; the flag tells
//! // whether the thread has been stuck for six seconds (`true`) or three.
//! let six_second = hicollie::report()?;
//! println!("stuck for six seconds: {six_second}");
//! # Ok::<(), hicollie::HiCollieError>(())
//! ```

pub use ohos_hicollie_sys as sys;

mod error;

use error::check;
pub use error::{describe, HiCollieError, Result};

use std::os::raw::c_char;

/// Periodic liveness probe for stuck detection, invoked by HiCollie on its own
/// thread. It carries no user-data pointer, so it is an `extern "C" fn` rather
/// than a closure.
pub type StuckTask = unsafe extern "C" fn();

/// A stub function returned by [`init_jank_detection`], to be called before and
/// after each event the business thread processes.
pub type JankStubFn = unsafe extern "C" fn(event_name: *const c_char);

/// The begin/end stub pair returned by [`init_jank_detection`].
pub struct JankStubs {
    /// Call this before processing each event.
    pub begin: JankStubFn,
    /// Call this after processing each event.
    pub end: JankStubFn,
}

/// Register a periodic task that HiCollie runs every 3 seconds to detect a
/// stuck business thread.
///
/// Must not be called from the main thread.
pub fn init_stuck_detection(task: StuckTask) -> Result<()> {
    check(unsafe { sys::OH_HiCollie_Init_StuckDetection(Some(task)) })
}

/// Register a stuck-detection task with a custom interval.
///
/// `stuck_timeout` is the detection interval in seconds and must be in the
/// range 3..=15. Must not be called from the main thread.
#[cfg(feature = "api-18")]
pub fn init_stuck_detection_with_timeout(task: StuckTask, stuck_timeout: u32) -> Result<()> {
    check(unsafe { sys::OH_HiCollie_Init_StuckDetectionWithTimeout(Some(task), stuck_timeout) })
}

/// Set up jank detection and obtain the begin/end stubs to bracket each event.
///
/// `sample_stack_trigger_time` is the threshold (in milliseconds) above which a
/// stack sample is collected; it is ignored on API 12 (reserved for future
/// versions) and may be left at zero. Must not be called from the main thread.
///
/// Jank detection is set up at most once per thread: on a repeated call the
/// runtime reports success without handing out the stubs again, which this
/// function surfaces as [`HiCollieError::JankAlreadyInitialized`]. Call
/// [`deinit_jank_detection`] first to set it up anew.
pub fn init_jank_detection(sample_stack_trigger_time: i32) -> Result<JankStubs> {
    let mut begin: sys::OH_HiCollie_BeginFunc = None;
    let mut end: sys::OH_HiCollie_EndFunc = None;
    let param = sys::HiCollie_DetectionParam {
        sampleStackTriggerTime: sample_stack_trigger_time,
        reserved: 0,
    };
    let code = unsafe { sys::OH_HiCollie_Init_JankDetection(&mut begin, &mut end, param) };
    check(code)?;
    match (begin, end) {
        (Some(begin), Some(end)) => Ok(JankStubs { begin, end }),
        _ => Err(HiCollieError::JankAlreadyInitialized),
    }
}

/// Tear down jank detection for the calling thread.
///
/// Passing no stubs is the native deregistration form: the thread is
/// unregistered and the default watcher restored, after which
/// [`init_jank_detection`] may be called again.
pub fn deinit_jank_detection() -> Result<()> {
    let param = sys::HiCollie_DetectionParam {
        sampleStackTriggerTime: 0,
        reserved: 0,
    };
    check(unsafe {
        sys::OH_HiCollie_Init_JankDetection(std::ptr::null_mut(), std::ptr::null_mut(), param)
    })
}

/// Report a stuck event from the HiCollie monitor thread.
///
/// Returns whether the thread has been stuck for six seconds (`true`) or three
/// (`false`). May only be called from the HiCollie internal monitor thread on
/// which a [`StuckTask`] runs.
pub fn report() -> Result<bool> {
    let mut is_six_second = false;
    let code = unsafe { sys::OH_HiCollie_Report(&mut is_six_second) };
    check(code)?;
    Ok(is_six_second)
}

/// Report a stuck event caused by an unresponsive user input.
#[cfg(feature = "api-24")]
pub fn report_input_block() -> Result<()> {
    check(unsafe { sys::OH_HiCollie_ReportInputBlock() })
}
