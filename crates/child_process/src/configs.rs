use crate::args::IsolationMode;
use crate::error::{check, ChildProcessError, Result};
use ohos_child_process_sys as sys;
use std::ffi::CString;
use std::ptr::NonNull;

/// An owned child process configs object.
///
/// It collects the settings applied to a child process started with
/// [`start_child_process_with_configs`](crate::start_child_process_with_configs).
/// The native object is created on construction and destroyed on drop.
///
/// ```no_run
/// use ohos_child_process_binding::{ChildProcessConfigs, IsolationMode};
///
/// let configs = ChildProcessConfigs::new()?
///     .with_isolation_mode(IsolationMode::Isolated)?
///     .with_process_name("worker")?;
/// # Ok::<(), ohos_child_process_binding::ChildProcessError>(())
/// ```
pub struct ChildProcessConfigs {
    raw: NonNull<sys::Ability_ChildProcessConfigs>,
}

impl ChildProcessConfigs {
    /// Create a configs object with the platform defaults.
    pub fn new() -> Result<Self> {
        let raw = unsafe { sys::OH_Ability_CreateChildProcessConfigs() };
        NonNull::new(raw)
            .map(|raw| Self { raw })
            .ok_or(ChildProcessError::ConfigsCreationFailed)
    }

    /// Set the isolation mode of the child process.
    pub fn set_isolation_mode(&mut self, isolation_mode: IsolationMode) -> Result<()> {
        check(unsafe {
            sys::OH_Ability_ChildProcessConfigs_SetIsolationMode(
                self.raw.as_ptr(),
                isolation_mode.to_raw(),
            )
        })
    }

    /// Builder form of [`set_isolation_mode`](Self::set_isolation_mode).
    pub fn with_isolation_mode(mut self, isolation_mode: IsolationMode) -> Result<Self> {
        self.set_isolation_mode(isolation_mode)?;
        Ok(self)
    }

    /// Choose whether the child process runs under an independent UID
    /// (`true`) or under the UID of the parent process (`false`).
    ///
    /// Only takes effect together with [`IsolationMode::Isolated`].
    #[cfg(feature = "api-21")]
    pub fn set_isolation_uid(&mut self, isolation_uid: bool) -> Result<()> {
        check(unsafe {
            sys::OH_Ability_ChildProcessConfigs_SetIsolationUid(self.raw.as_ptr(), isolation_uid)
        })
    }

    /// Builder form of [`set_isolation_uid`](Self::set_isolation_uid).
    #[cfg(feature = "api-21")]
    pub fn with_isolation_uid(mut self, isolation_uid: bool) -> Result<Self> {
        self.set_isolation_uid(isolation_uid)?;
        Ok(self)
    }

    /// Set the process name.
    ///
    /// The name must be non-empty, at most 64 characters long and made of
    /// letters, digits or underscores; the runtime rejects anything else with
    /// an invalid-parameter error. The process is finally named
    /// `{bundleName}:{name}`. Returns [`ChildProcessError::InteriorNul`] if
    /// `name` contains a NUL byte.
    pub fn set_process_name(&mut self, name: &str) -> Result<()> {
        let name = CString::new(name).map_err(|_| ChildProcessError::InteriorNul)?;
        check(unsafe {
            sys::OH_Ability_ChildProcessConfigs_SetProcessName(self.raw.as_ptr(), name.as_ptr())
        })
    }

    /// Builder form of [`set_process_name`](Self::set_process_name).
    pub fn with_process_name(mut self, name: &str) -> Result<Self> {
        self.set_process_name(name)?;
        Ok(self)
    }

    pub(crate) fn as_raw(&self) -> *mut sys::Ability_ChildProcessConfigs {
        self.raw.as_ptr()
    }
}

impl Drop for ChildProcessConfigs {
    fn drop(&mut self) {
        unsafe { sys::OH_Ability_DestroyChildProcessConfigs(self.raw.as_ptr()) };
    }
}
