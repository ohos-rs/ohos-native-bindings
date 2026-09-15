use std::{ffi::CString, ptr::NonNull};

use crate::{sys, IsolationMode, NativeChildProcessError as Error, Result};

/// Owns the official configs object and destroys it on drop.
#[derive(Debug)]
pub struct ChildProcessConfigs {
    raw: NonNull<sys::Ability_ChildProcessConfigs>,
    // Keep setter input alive even if a runtime retains its string pointer.
    process_name: Option<CString>,
}

impl ChildProcessConfigs {
    pub fn new() -> Result<Self> {
        // SAFETY: This no-argument native constructor returns a caller-owned object.
        let raw = NonNull::new(unsafe { sys::OH_Ability_CreateChildProcessConfigs() })
            .ok_or(Error::NullPointer)?;
        Ok(Self {
            raw,
            process_name: None,
        })
    }

    pub fn set_isolation_mode(&mut self, mode: IsolationMode) -> Result<()> {
        // SAFETY: The uniquely borrowed configs object is live.
        Error::check(unsafe {
            sys::OH_Ability_ChildProcessConfigs_SetIsolationMode(self.raw.as_ptr(), mode.raw())
        })
    }

    /// Native validates the name: 1–64 letters, digits or underscores. The final
    /// process name is `{bundleName}:{processName}`.
    pub fn set_process_name(&mut self, name: &str) -> Result<()> {
        let name = CString::new(name)?;
        // SAFETY: Configs and the terminated input string remain live.
        Error::check(unsafe {
            sys::OH_Ability_ChildProcessConfigs_SetProcessName(self.raw.as_ptr(), name.as_ptr())
        })?;
        self.process_name = Some(name);
        Ok(())
    }

    /// Effective only in isolated mode: true uses an independent UID.
    #[cfg(feature = "api-21")]
    pub fn set_isolation_uid(&mut self, isolated: bool) -> Result<()> {
        // SAFETY: The uniquely borrowed configs object is live.
        Error::check(unsafe {
            sys::OH_Ability_ChildProcessConfigs_SetIsolationUid(self.raw.as_ptr(), isolated)
        })
    }

    pub(crate) fn as_ptr(&mut self) -> *mut sys::Ability_ChildProcessConfigs {
        self.raw.as_ptr()
    }
}

impl Drop for ChildProcessConfigs {
    fn drop(&mut self) {
        // SAFETY: This owner destroys its live object exactly once; setter inputs
        // remain alive until destruction completes.
        let _ = unsafe { sys::OH_Ability_DestroyChildProcessConfigs(self.raw.as_ptr()) };
    }
}
