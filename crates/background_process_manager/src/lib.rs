//! Safe Rust bindings for OpenHarmony **background process manager**.
//!
//! The background process manager lets an application hint the resource
//! schedule service about how much work one of its processes still has to do
//! once that process is no longer in the foreground. Lowering the priority of
//! an idle process frees resources for the rest of the system; the hint is
//! dropped again by resetting the priority. This crate wraps the native
//! `background_process_manager.h` C API with a safe layer.
//!
//! The whole API was introduced in API 17, so every item below sits behind the
//! `api-17` feature. The raw bindings are re-exported as [`sys`].

pub use ohos_background_process_manager_sys as sys;

#[cfg(feature = "api-17")]
mod error;
#[cfg(feature = "api-17")]
mod priority;

#[cfg(feature = "api-17")]
pub use error::{describe, BackgroundProcessManagerError, Result};
#[cfg(feature = "api-17")]
pub use priority::ProcessPriority;

/// Set the background priority of a process.
///
/// `pid` is the process to act on: the calling process itself or one of the
/// child processes it owns. The hint takes effect while the process runs in the
/// background and stays in place until [`reset_process_priority`] is called.
///
/// # Example
///
/// ```no_run
/// use ohos_background_process_manager_binding as bpm;
/// use ohos_background_process_manager_binding::ProcessPriority;
///
/// let worker_pid: i32 = 1234;
///
/// // A worker process that has finished its job may be deprioritized.
/// bpm::set_process_priority(worker_pid, ProcessPriority::Background)?;
///
/// // Restore the default scheduling once it has work again.
/// bpm::reset_process_priority(worker_pid)?;
/// # Ok::<(), bpm::BackgroundProcessManagerError>(())
/// ```
#[cfg(feature = "api-17")]
pub fn set_process_priority(pid: i32, priority: ProcessPriority) -> Result<()> {
    error::check(unsafe {
        sys::OH_BackgroundProcessManager_SetProcessPriority(pid, priority.raw())
    })
}

/// Reset a process to its default priority, dropping any hint previously set
/// through [`set_process_priority`].
#[cfg(feature = "api-17")]
pub fn reset_process_priority(pid: i32) -> Result<()> {
    error::check(unsafe { sys::OH_BackgroundProcessManager_ResetProcessPriority(pid) })
}

/// Set the background priority of the calling process.
///
/// Convenience wrapper around [`set_process_priority`] for the current pid.
#[cfg(feature = "api-17")]
pub fn set_current_process_priority(priority: ProcessPriority) -> Result<()> {
    set_process_priority(current_pid(), priority)
}

/// Reset the calling process to its default priority.
///
/// Convenience wrapper around [`reset_process_priority`] for the current pid.
#[cfg(feature = "api-17")]
pub fn reset_current_process_priority() -> Result<()> {
    reset_process_priority(current_pid())
}

/// The pid of the calling process, as the `int` the native API expects. Process
/// ids on OpenHarmony are well below `i32::MAX`, so the conversion is exact.
#[cfg(feature = "api-17")]
fn current_pid() -> i32 {
    std::process::id() as i32
}
