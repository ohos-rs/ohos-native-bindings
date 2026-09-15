use std::ffi::CString;
#[cfg(feature = "api-17")]
use std::ptr::NonNull;

#[cfg(feature = "api-20")]
use crate::ChildProcessConfigs;
#[cfg(feature = "api-13")]
use crate::{args::PreparedArgs, ChildProcessArgs, ChildProcessOptions};
use crate::{sys, NativeChildProcessError as Error, Result};

/// Official IPC startup callback, invoked on an independent native thread.
/// The receiver releases a successful proxy through IPCKit's
/// `OH_IPCRemoteProxy_Destroy`.
pub type ChildProcessStartedCallback = unsafe extern "C" fn(i32, *mut sys::OHIPCRemoteProxy);

/// Official process-wide exit callback. Keep its work short and do not unwind.
#[cfg(feature = "api-20")]
pub type ChildProcessExitCallback = extern "C" fn(i32, i32);

/// AbilityKit extended native child-process operations. PID and callback lifecycles
/// follow the native API; no launch implicitly registers an exit callback.
#[derive(Debug)]
pub struct NativeChildProcess;

impl NativeChildProcess {
    /// Requests an IPC child. Success acknowledges the request; the callback
    /// receives the actual startup result. The library must export
    /// `NativeChildProcess_OnConnect` and `NativeChildProcess_MainProc`.
    ///
    /// # Safety
    /// The callback must satisfy the official native callback contract, contain
    /// panics, synchronize shared state, and release any received IPC proxy.
    pub unsafe fn create(library: &str, callback: ChildProcessStartedCallback) -> Result<()> {
        let library = CString::new(library)?;
        // SAFETY: Input is terminated and callback obligations belong to the caller.
        Error::check(unsafe {
            sys::OH_Ability_CreateNativeChildProcess(library.as_ptr(), Some(callback)) as u32
        })
    }

    /// Starts `library:exportedFunction`, for example `libchild.so:Main`.
    /// Returns the official PID. Native validates and loads the entry. Keep this
    /// synchronous operation off the application's UI thread.
    #[cfg(feature = "api-13")]
    pub fn start(
        entry: &str,
        args: &ChildProcessArgs<'_>,
        options: ChildProcessOptions,
    ) -> Result<i32> {
        let entry = CString::new(entry)?;
        let mut args = PreparedArgs::new(args);
        let mut pid = 0;
        // SAFETY: Entry, FD borrows, strings and linked nodes remain live through
        // native's synchronous argument transfer; reserved options are zero.
        Error::check(unsafe {
            sys::OH_Ability_StartNativeChildProcess(
                entry.as_ptr(),
                args.raw(),
                options.raw(),
                &mut pid,
            )
        })?;
        Ok(pid)
    }

    /// Starts an entry using the official native configs object.
    #[cfg(feature = "api-20")]
    pub fn start_with_configs(
        entry: &str,
        args: &ChildProcessArgs<'_>,
        configs: &mut ChildProcessConfigs,
    ) -> Result<i32> {
        let entry = CString::new(entry)?;
        let mut args = PreparedArgs::new(args);
        let mut pid = 0;
        // SAFETY: The input resources and uniquely borrowed configs stay live
        // through native's synchronous argument transfer.
        Error::check(unsafe {
            sys::OH_Ability_StartNativeChildProcessWithConfigs(
                entry.as_ptr(),
                args.raw(),
                configs.as_ptr(),
                &mut pid,
            )
        })?;
        Ok(pid)
    }

    /// Requests an IPC child using native configs. See [`Self::create`].
    ///
    /// # Safety
    /// The callback must uphold the same contract as [`Self::create`].
    #[cfg(feature = "api-20")]
    pub unsafe fn create_with_configs(
        library: &str,
        configs: &mut ChildProcessConfigs,
        callback: ChildProcessStartedCallback,
    ) -> Result<()> {
        let library = CString::new(library)?;
        // SAFETY: Inputs stay live and callback obligations belong to the caller.
        Error::check(unsafe {
            sys::OH_Ability_CreateNativeChildProcessWithConfigs(
                library.as_ptr(),
                configs.as_ptr(),
                Some(callback),
            )
        })
    }

    /// Returns the system-owned args pointer for the current native child, or
    /// None. Dereferencing it requires the native lifetime/FD contract; it does
    /// not grant ownership of the struct, strings, list nodes or descriptors.
    #[cfg(feature = "api-17")]
    pub fn current_args() -> Option<NonNull<sys::NativeChildProcess_Args>> {
        // SAFETY: This no-argument query is valid in the current process.
        NonNull::new(unsafe { sys::OH_Ability_GetCurrentChildProcessArgs() })
    }

    /// Registers a process-wide exit callback. Native deduplicates repeated
    /// registration of the same function; callbacks run on the native thread.
    #[cfg(feature = "api-20")]
    pub fn register_exit_callback(callback: ChildProcessExitCallback) -> Result<()> {
        // SAFETY: A non-null, safe C function accepts the official PID/signal ABI.
        Error::check(unsafe {
            sys::OH_Ability_RegisterNativeChildProcessExitCallback(Some(callback))
        })
    }

    /// Unregisters exactly the function passed to registration.
    #[cfg(feature = "api-20")]
    pub fn unregister_exit_callback(callback: ChildProcessExitCallback) -> Result<()> {
        // SAFETY: The function pointer uses the official callback ABI.
        Error::check(unsafe {
            sys::OH_Ability_UnregisterNativeChildProcessExitCallback(Some(callback))
        })
    }

    /// Requests termination of the given native child PID. Native checks process
    /// identity and rejects SELF_FORK children. Success is not an exit observation.
    #[cfg(feature = "api-22")]
    pub fn kill(pid: i32) -> Result<()> {
        // SAFETY: The platform validates this numeric PID.
        Error::check(unsafe { sys::OH_Ability_KillChildProcess(pid) })
    }

    #[cfg(feature = "api-26")]
    pub fn is_supported() -> bool {
        // SAFETY: This no-argument query has no additional preconditions.
        unsafe { sys::OH_Ability_IsNativeChildProcessSupported() }
    }
}
