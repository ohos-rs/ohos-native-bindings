//! Safe Rust bindings for the OpenHarmony **native child process** API.
//!
//! An application can start a child process that loads one of its own dynamic
//! libraries and runs an exported entry function there. The parent hands over a
//! parameter string and a set of named file descriptors, chooses how far the
//! child is isolated from it, and is notified when the child exits. This crate
//! wraps the native `native_child_process.h` C API with a safe layer:
//! `Result`-based error handling, owned argument and configuration types, and
//! an RAII [`ChildProcessConfigs`] object.
//!
//! Every item sits behind the `api-*` feature matching the version that
//! introduced it; without any feature the crate exposes only the raw bindings,
//! re-exported as [`sys`].
//!
//! # Coverage
//!
//! The entry-function flow (`OH_Ability_StartNativeChildProcess`, API 13, and
//! its API 20 configs variant) is wrapped here. The older callback flow
//! (`OH_Ability_CreateNativeChildProcess`, API 12) hands an `OHIPCRemoteProxy`
//! to a C callback; that object belongs to the IPC module and cannot be owned
//! safely from this crate, so the callback flow stays available through [`sys`]
//! only.
//!
//! # Example
//!
//! ```no_run
//! use ohos_child_process_binding as child_process;
//! use child_process::{ChildProcessArgs, ChildProcessConfigs, IsolationMode};
//!
//! // Report every child process that exits. The registration lasts as long as
//! // the guard is held.
//! unsafe extern "C" fn on_exit(pid: i32, signal: i32) {
//!     println!("child {pid} exited with signal {signal}");
//! }
//! let _exit_callback = child_process::register_exit_callback(on_exit)?;
//!
//! let args = ChildProcessArgs::new()
//!     .with_entry_params("--mode=worker")?
//!     .with_fd("log", 3)?;
//!
//! let configs = ChildProcessConfigs::new()?
//!     .with_isolation_mode(IsolationMode::Isolated)?
//!     .with_process_name("worker")?;
//!
//! // "libEntry.so:Main" is the library and the entry function to run.
//! let pid = child_process::start_child_process_with_configs(
//!     "libEntry.so:Main",
//!     &args,
//!     &configs,
//! )?;
//! println!("started child process {pid}");
//! # Ok::<(), child_process::ChildProcessError>(())
//! ```

pub use ohos_child_process_sys as sys;

mod error;
pub use error::{describe, ChildProcessError, Result};

#[cfg(feature = "api-13")]
mod args;
#[cfg(feature = "api-13")]
pub use args::{ChildProcessArgs, ChildProcessOptions, IsolationMode};

#[cfg(feature = "api-20")]
mod configs;
#[cfg(feature = "api-20")]
pub use configs::ChildProcessConfigs;

#[cfg(feature = "api-13")]
use error::check;
#[cfg(feature = "api-13")]
use std::ffi::CString;

/// Callback invoked when a native child process of this process exits.
///
/// It runs on a runtime-owned thread and carries no user data pointer, so it is
/// an `extern "C" fn` rather than a closure.
#[cfg(feature = "api-20")]
pub type ExitCallback = unsafe extern "C" fn(pid: i32, signal: i32);

/// Start a child process and return its pid.
///
/// `entry` names the dynamic library and the entry function to run in the child
/// process, such as `"libEntry.so:Main"`. The entry function takes the C form
/// of `args` as its only parameter; the child process exits once it returns.
///
/// # Example
///
/// ```no_run
/// use ohos_child_process_binding as child_process;
/// use child_process::{ChildProcessArgs, ChildProcessOptions, IsolationMode};
///
/// let pid = child_process::start_child_process(
///     "libEntry.so:Main",
///     &ChildProcessArgs::new().with_entry_params("--mode=worker")?,
///     ChildProcessOptions::new(IsolationMode::Normal),
/// )?;
/// println!("started child process {pid}");
/// # Ok::<(), child_process::ChildProcessError>(())
/// ```
#[cfg(feature = "api-13")]
pub fn start_child_process(
    entry: &str,
    args: &ChildProcessArgs,
    options: ChildProcessOptions,
) -> Result<i32> {
    let entry = CString::new(entry).map_err(|_| ChildProcessError::InteriorNul)?;
    let raw_args = args.to_raw();
    let mut pid = 0;
    check(unsafe {
        sys::OH_Ability_StartNativeChildProcess(
            entry.as_ptr(),
            raw_args.args(),
            options.to_raw(),
            &mut pid,
        )
    })?;
    Ok(pid)
}

/// Start a child process configured by `configs` and return its pid.
///
/// Same as [`start_child_process`], with the isolation mode, the UID isolation
/// flag and the process name taken from an owned [`ChildProcessConfigs`].
#[cfg(feature = "api-20")]
pub fn start_child_process_with_configs(
    entry: &str,
    args: &ChildProcessArgs,
    configs: &ChildProcessConfigs,
) -> Result<i32> {
    let entry = CString::new(entry).map_err(|_| ChildProcessError::InteriorNul)?;
    let raw_args = args.to_raw();
    let mut pid = 0;
    check(unsafe {
        sys::OH_Ability_StartNativeChildProcessWithConfigs(
            entry.as_ptr(),
            raw_args.args(),
            configs.as_raw(),
            &mut pid,
        )
    })?;
    Ok(pid)
}

/// Read back the arguments the current child process was started with.
///
/// Returns `None` outside of a native child process, or when the runtime holds
/// no arguments. The values are copied out, so the result is independent of the
/// runtime-owned buffers.
#[cfg(feature = "api-17")]
pub fn current_child_process_args() -> Option<ChildProcessArgs> {
    let raw = unsafe { sys::OH_Ability_GetCurrentChildProcessArgs() };
    if raw.is_null() {
        return None;
    }
    // SAFETY: the pointer is non-null and owned by the runtime, which keeps the
    // arguments alive for the lifetime of the child process.
    Some(unsafe { ChildProcessArgs::from_raw(&*raw) })
}

/// A registered [`ExitCallback`], unregistered when dropped.
///
/// The runtime keeps the callback in a process-wide list and dispatches into it
/// from a thread of its own, so a registration that outlives the code it
/// belongs to keeps a stale function pointer live. The guard is what pairs
/// [`register_exit_callback`] with
/// [`OH_Ability_UnregisterNativeChildProcessExitCallback`](sys::OH_Ability_UnregisterNativeChildProcessExitCallback).
///
/// A callback meant to stay for the lifetime of the process is kept by
/// [`std::mem::forget`]ing the guard, or by holding it in a `static`.
#[cfg(feature = "api-20")]
#[derive(Debug)]
#[must_use = "the callback is unregistered as soon as the guard is dropped"]
pub struct ExitCallbackRegistration {
    callback: ExitCallback,
    unregistered: bool,
}

#[cfg(feature = "api-20")]
impl ExitCallbackRegistration {
    /// Unregister the callback, reporting the failure instead of swallowing it
    /// as [`Drop`] does.
    pub fn unregister(mut self) -> Result<()> {
        self.unregister_inner()
    }

    fn unregister_inner(&mut self) -> Result<()> {
        if self.unregistered {
            return Ok(());
        }
        self.unregistered = true;
        unregister_exit_callback(self.callback)
    }
}

#[cfg(feature = "api-20")]
impl Drop for ExitCallbackRegistration {
    fn drop(&mut self) {
        let _ = self.unregister_inner();
    }
}

/// Register a callback notified when a native child process exits.
///
/// Registering the same callback more than once keeps a single registration.
#[cfg(feature = "api-20")]
pub fn register_exit_callback(callback: ExitCallback) -> Result<ExitCallbackRegistration> {
    check(unsafe { sys::OH_Ability_RegisterNativeChildProcessExitCallback(Some(callback)) })?;
    Ok(ExitCallbackRegistration {
        callback,
        unregistered: false,
    })
}

/// Unregister a callback previously passed to [`register_exit_callback`].
///
/// Returns a native error carrying `NCP_ERR_CALLBACK_NOT_EXIST` if the callback
/// is not registered.
#[cfg(feature = "api-20")]
pub fn unregister_exit_callback(callback: ExitCallback) -> Result<()> {
    check(unsafe { sys::OH_Ability_UnregisterNativeChildProcessExitCallback(Some(callback)) })
}

/// Terminate a child process started by this process.
///
/// Child processes created in SELF_FORK mode cannot be terminated this way.
#[cfg(feature = "api-22")]
pub fn kill_child_process(pid: i32) -> Result<()> {
    check(unsafe { sys::OH_Ability_KillChildProcess(pid) })
}
