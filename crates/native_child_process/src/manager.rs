#![cfg_attr(not(target_env = "ohos"), allow(dead_code))]

use crate::{
    ChildProcessArgsBuilder, ChildProcessEntry, ChildProcessId, ChildProcessOptions,
    NativeChildProcessError as Error,
};

/// Starts packaged shared-library entries, not executables. API20 installs its
/// global exit callback before launch. Do not mix these handles with raw starts
/// or separately register/unregister the binding's callback. Start is synchronous
/// and must be kept off the application's UI thread.
#[derive(Debug, Default, Clone, Copy)]
pub struct NativeChildProcessManager;

impl NativeChildProcessManager {
    pub const fn new() -> Self {
        Self
    }

    /// Takes the builder by value. All parent-side launch duplicates close after
    /// this synchronous call (success or failure). The system's FD duplication
    /// timing still requires device proof; this is not a host-verified promise.
    pub fn start(
        &self,
        entry: &ChildProcessEntry,
        args: ChildProcessArgsBuilder<'_>,
        options: ChildProcessOptions,
    ) -> Result<ChildProcessHandle, Error> {
        #[cfg(target_env = "ohos")]
        {
            let launch = || {
                let mut args = crate::args::PreparedArgs::new(args);
                let mut pid = 0;
                // SAFETY: Typed entry, zero-reserved options, stable nodes and
                // owned strings/FDs remain live through the synchronous FFI.
                Error::check(unsafe {
                    ohos_native_child_process_sys::OH_Ability_StartNativeChildProcess(
                        entry.as_c_str().as_ptr(),
                        args.raw(),
                        options.raw(),
                        &mut pid,
                    )
                })?;
                ChildProcessId::new(pid)
            };
            #[cfg(feature = "api-20")]
            {
                let runtime = crate::registry::Runtime::global()?;
                Ok(ChildProcessHandle {
                    record: runtime.launch(launch)?,
                    #[cfg(feature = "api-22")]
                    runtime,
                })
            }
            #[cfg(not(feature = "api-20"))]
            {
                Ok(ChildProcessHandle { pid: launch()? })
            }
        }
        #[cfg(not(target_env = "ohos"))]
        {
            let _ = (entry, args, options);
            Err(Error::HostUnsupported)
        }
    }

    /// Generic SDK API20 config start. Product launch policy may forbid this
    /// method even though the reusable binding faithfully exposes it.
    #[cfg(feature = "api-20")]
    pub fn start_with_configs(
        &self,
        entry: &ChildProcessEntry,
        args: ChildProcessArgsBuilder<'_>,
        configs: &mut crate::ChildProcessConfigs,
    ) -> Result<ChildProcessHandle, Error> {
        #[cfg(target_env = "ohos")]
        {
            let runtime = crate::registry::Runtime::global()?;
            let record = runtime.launch(|| {
                let mut args = crate::args::PreparedArgs::new(args);
                let mut pid = 0;
                // SAFETY: All prepared args and the uniquely borrowed configs
                // owner remain live; entry is a validated packaged artifact.
                Error::check(unsafe {
                    ohos_native_child_process_sys::OH_Ability_StartNativeChildProcessWithConfigs(
                        entry.as_c_str().as_ptr(),
                        args.raw(),
                        configs.raw(),
                        &mut pid,
                    )
                })?;
                ChildProcessId::new(pid)
            })?;
            Ok(ChildProcessHandle {
                record,
                #[cfg(feature = "api-22")]
                runtime,
            })
        }
        #[cfg(not(target_env = "ohos"))]
        {
            let _ = (entry, args, configs);
            Err(Error::HostUnsupported)
        }
    }

    #[cfg(feature = "api-26")]
    pub fn is_supported(&self) -> Result<bool, Error> {
        #[cfg(target_env = "ohos")]
        {
            // SAFETY: This no-argument query is imported only under API26.
            Ok(
                unsafe {
                    ohos_native_child_process_sys::OH_Ability_IsNativeChildProcessSupported()
                },
            )
        }
        #[cfg(not(target_env = "ohos"))]
        {
            Err(Error::HostUnsupported)
        }
    }
}

/// Owns identity, never process lifetime. Drop only releases Rust observation
/// state; it never sends a signal or implicitly invokes KillChildProcess.
pub struct ChildProcessHandle {
    #[cfg(not(feature = "api-20"))]
    pid: ChildProcessId,
    #[cfg(feature = "api-20")]
    record: std::sync::Arc<crate::registry::Record>,
    #[cfg(all(feature = "api-22", target_env = "ohos"))]
    runtime: std::sync::Arc<crate::registry::Runtime>,
}

impl ChildProcessHandle {
    pub fn pid(&self) -> ChildProcessId {
        #[cfg(feature = "api-20")]
        {
            self.record.pid
        }
        #[cfg(not(feature = "api-20"))]
        {
            self.pid
        }
    }
    #[cfg(feature = "api-20")]
    pub fn generation(&self) -> u64 {
        self.record.generation
    }
    #[cfg(feature = "api-20")]
    pub fn subscribe_exit(
        &self,
        callback: impl Fn(crate::ChildProcessExitEvent) + Send + Sync + 'static,
    ) -> Result<crate::ChildProcessExitSubscription, Error> {
        self.record.subscribe(callback)
    }

    /// Explicit API22 forced termination, not a retry loop. Success acknowledges
    /// the platform request, not synchronous exit. Repeated calls may return
    /// InvalidPid after observed exit; stale/reused/ambiguous identities never
    /// authorize a new request. Graceful control shutdown should be attempted first.
    #[cfg(feature = "api-22")]
    pub fn kill(&self) -> Result<(), Error> {
        #[cfg(target_env = "ohos")]
        {
            let _launch = self
                .runtime
                .launch_lock
                .try_lock()
                .map_err(|_| Error::Busy)?;
            self.runtime
                .registry
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .validate_handle(&self.record)?;
            // SAFETY: Generation validation plus launch serialization protect
            // binding-managed PID identity; the platform checks child ownership.
            Error::check(unsafe {
                ohos_native_child_process_sys::OH_Ability_KillChildProcess(self.pid().get())
            })
        }
        #[cfg(not(target_env = "ohos"))]
        {
            Err(Error::HostUnsupported)
        }
    }
}
