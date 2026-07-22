//! Process-wide IPC facilities: the worker thread pool and the identity of the
//! caller currently being served.

use crate::alloc::with_allocator;
use crate::error::{check, IpcError, Result};
use ohos_ipc_sys as sys;
use std::ffi::CString;
use std::marker::PhantomData;
use std::os::raw::c_int;

/// Join the calling thread to the IPC worker thread pool.
///
/// The call blocks until [`stop_work_thread`] is invoked, so a service
/// typically calls it from a dedicated thread after publishing its stub.
pub fn join_work_thread() {
    // SAFETY: the call takes no arguments and blocks the calling thread.
    unsafe { sys::OH_IPCSkeleton_JoinWorkThread() };
}

/// Stop the calling thread's participation in the IPC worker thread pool.
pub fn stop_work_thread() {
    // SAFETY: the call takes no arguments.
    unsafe { sys::OH_IPCSkeleton_StopWorkThread() };
}

/// Token ID of the caller being served.
///
/// Outside an IPC context this is the local token ID.
pub fn calling_token_id() -> u64 {
    // SAFETY: the call takes no arguments and returns a plain value.
    unsafe { sys::OH_IPCSkeleton_GetCallingTokenId() }
}

/// Token ID of the first caller in the call chain.
pub fn first_token_id() -> u64 {
    // SAFETY: the call takes no arguments and returns a plain value.
    unsafe { sys::OH_IPCSkeleton_GetFirstTokenId() }
}

/// Token ID of this process.
pub fn self_token_id() -> u64 {
    // SAFETY: the call takes no arguments and returns a plain value.
    unsafe { sys::OH_IPCSkeleton_GetSelfTokenId() }
}

/// Process ID of the caller being served.
///
/// Outside an IPC context this is the current process ID.
pub fn calling_pid() -> u64 {
    // SAFETY: the call takes no arguments and returns a plain value.
    unsafe { sys::OH_IPCSkeleton_GetCallingPid() }
}

/// UID of the caller being served.
///
/// Outside an IPC context this is the current UID.
pub fn calling_uid() -> u64 {
    // SAFETY: the call takes no arguments and returns a plain value.
    unsafe { sys::OH_IPCSkeleton_GetCallingUid() }
}

/// Whether the call being served comes from the local device.
pub fn is_local_calling() -> bool {
    // SAFETY: the call takes no arguments and returns a plain value.
    unsafe { sys::OH_IPCSkeleton_IsLocalCalling() != 0 }
}

/// Whether an IPC request is currently being handled on this thread.
pub fn is_handling_transaction() -> bool {
    // SAFETY: the call takes no arguments and returns a plain value.
    unsafe { sys::OH_IPCSkeleton_IsHandlingTransaction() != 0 }
}

/// Set the maximum number of IPC worker threads.
///
/// The value must be in the range 1..=32; the default is 16.
pub fn set_max_work_thread_num(max: u32) -> Result<()> {
    let max = c_int::try_from(max)
        .map_err(|_| IpcError::InvalidArgument("the thread count is larger than i32::MAX"))?;
    // SAFETY: the argument is passed by value and validated by the native side.
    check(unsafe { sys::OH_IPCSkeleton_SetMaxWorkThreadNum(max) })
}

/// The caller credentials saved by [`reset_calling_identity`].
///
/// The contents are opaque and only meant to be handed back to
/// [`set_calling_identity`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallingIdentity(CString);

impl CallingIdentity {
    /// The credentials as a string, with invalid UTF-8 replaced.
    pub fn to_string_lossy(&self) -> std::borrow::Cow<'_, str> {
        self.0.to_string_lossy()
    }
}

/// Reset the caller credentials (token ID, UID and PID) of the current IPC
/// context to those of this process, returning the previous ones.
///
/// Pass the returned value to [`set_calling_identity`] to restore the context.
pub fn reset_calling_identity() -> Result<CallingIdentity> {
    let identity = with_allocator(
        "failed to reset the calling identity",
        |out, len, allocator| {
            // SAFETY: `out` and `len` are live locals and the allocator matches
            // the deallocator used on the returned buffer.
            unsafe { sys::OH_IPCSkeleton_ResetCallingIdentity(out, len, allocator) }
        },
    )?;
    Ok(CallingIdentity(identity))
}

/// Restore caller credentials previously obtained from
/// [`reset_calling_identity`].
pub fn set_calling_identity(identity: &CallingIdentity) -> Result<()> {
    // SAFETY: the string is NUL-terminated and only read during the call.
    check(unsafe { sys::OH_IPCSkeleton_SetCallingIdentity(identity.0.as_ptr()) })
}

/// The caller credentials of the IPC context, restored when the guard is
/// dropped.
///
/// Resetting the identity has to be undone before the stub returns, otherwise
/// the rest of the transaction — and every transaction served afterwards on the
/// same thread — runs with this process as the apparent caller. The guard is
/// what pairs the reset with the restore across early returns and unwinds.
///
/// The identity lives in the thread's IPC context, so the guard is `!Send`:
/// restoring it from another thread would overwrite that thread's context.
#[must_use = "the caller identity is restored as soon as the guard is dropped"]
pub struct CallingIdentityGuard {
    identity: CallingIdentity,
    restored: bool,
    _not_send: PhantomData<*const ()>,
}

impl CallingIdentityGuard {
    /// The credentials that will be restored.
    pub fn identity(&self) -> &CallingIdentity {
        &self.identity
    }

    /// Restore the credentials, reporting the failure instead of swallowing it
    /// as [`Drop`] does.
    pub fn restore(mut self) -> Result<()> {
        self.restore_inner()
    }

    fn restore_inner(&mut self) -> Result<()> {
        if self.restored {
            return Ok(());
        }
        self.restored = true;
        set_calling_identity(&self.identity)
    }
}

impl Drop for CallingIdentityGuard {
    fn drop(&mut self) {
        let _ = self.restore_inner();
    }
}

/// Reset the caller credentials for as long as the returned guard lives.
///
/// The scoped form of [`reset_calling_identity`] / [`set_calling_identity`]:
/// the credentials are restored on drop, so an early return from the stub
/// cannot leave the IPC context pointing at this process.
///
/// ```no_run
/// use ohos_ipc_binding as ipc;
///
/// let _identity = ipc::reset_calling_identity_scoped()?;
/// // Calls made here appear to come from this process; the caller
/// // credentials are restored when the guard goes out of scope.
/// # Ok::<(), ipc::IpcError>(())
/// ```
pub fn reset_calling_identity_scoped() -> Result<CallingIdentityGuard> {
    Ok(CallingIdentityGuard {
        identity: reset_calling_identity()?,
        restored: false,
        _not_send: PhantomData,
    })
}
