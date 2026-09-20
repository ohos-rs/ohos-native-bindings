//! Rust bindings for AbilityKit extended native child processes.
//!
//! APIs follow `AbilityKit/native_child_process.h`. Enable the API feature that
//! matches the application's minimum supported runtime version.
//! UIAbility and HAP independent-process placement belongs to the component APIs.

pub use ohos_native_child_process_sys as sys;

#[cfg(feature = "api-13")]
mod args;
#[cfg(feature = "api-20")]
mod configs;
mod error;
#[cfg(feature = "api-13")]
mod options;
mod process;

#[cfg(feature = "api-13")]
pub use args::{ChildProcessArgs, ChildProcessArgsRef, ChildProcessFd};
#[cfg(feature = "api-20")]
pub use configs::ChildProcessConfigs;
pub use error::{NativeChildProcessError, Result};
#[cfg(feature = "api-13")]
pub use options::{ChildProcessOptions, IsolationMode};
#[cfg(feature = "api-20")]
pub use process::ChildProcessExitCallback;
pub use process::{ChildProcessStartedCallback, NativeChildProcess};

/// Exports an AbilityKit entry with the official `NativeChildProcess_Args` ABI.
/// The handler receives borrowed arguments. Returning ends the child process.
/// Handler failures and panics are contained before returning to native code.
#[cfg(feature = "api-13")]
#[macro_export]
macro_rules! native_child_entry {
    ($symbol:ident, $handler:path) => {
        /// AbilityKit native child-process entry.
        ///
        /// # Safety
        /// AbilityKit must supply valid argument strings, a finite FD list,
        /// and live descriptors for the duration of this invocation.
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $symbol(args: $crate::sys::NativeChildProcess_Args) {
            let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                // SAFETY: AbilityKit keeps the supplied arguments valid until
                // this entry returns. The handler cannot retain this borrow.
                let args = unsafe { $crate::ChildProcessArgsRef::from_raw(&args) };
                let _ = $handler(args);
            }));
            if let Err(payload) = result {
                // A custom payload destructor must not unwind across the ABI.
                ::std::mem::forget(payload);
            }
        }
    };
}
