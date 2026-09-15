use std::{
    ffi::{c_void, CString},
    ptr::NonNull,
};

use crate::{IsolationMode, NativeChildProcessError as Error};

/// SDK process suffix: 1..64 ASCII letters, digits or underscores.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildProcessName(CString);

impl ChildProcessName {
    pub fn new(name: &str) -> Result<Self, Error> {
        if name.as_bytes().contains(&0) {
            return Err(Error::InteriorNul {
                field: "process name",
            });
        }
        if name.len() > 64 {
            return Err(Error::LimitExceeded {
                field: "process name",
                limit: 64,
            });
        }
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(Error::InvalidProcessName);
        }
        Ok(Self(
            CString::new(name).map_err(|_| Error::InvalidProcessName)?,
        ))
    }
    pub fn as_c_str(&self) -> &std::ffi::CStr {
        &self.0
    }
}

struct ConfigOwner {
    pointer: NonNull<c_void>,
    destroy: unsafe fn(NonNull<c_void>),
}

impl Drop for ConfigOwner {
    fn drop(&mut self) {
        // SAFETY: The constructor installs the destructor for this sole,
        // non-null owner. Drop runs once and no pointer is exposed publicly.
        unsafe { (self.destroy)(self.pointer) };
    }
}

/// Non-null RAII owner; destruction is attempted exactly once. No Clone and no
/// Send/Sync promise is made for the SDK's opaque mutable object. Destruction
/// errors cannot be returned by Drop and must be investigated on the device.
pub struct ChildProcessConfigs {
    // Host methods return HostUnsupported without creating a platform owner.
    #[cfg_attr(not(target_env = "ohos"), allow(dead_code))]
    owner: ConfigOwner,
    #[cfg_attr(not(target_env = "ohos"), allow(dead_code))]
    process_name: Option<ChildProcessName>,
}

impl ChildProcessConfigs {
    pub fn new() -> Result<Self, Error> {
        #[cfg(target_env = "ohos")]
        {
            // SAFETY: SDK creates an opaque object with caller-owned lifetime.
            let pointer =
                unsafe { ohos_native_child_process_sys::OH_Ability_CreateChildProcessConfigs() };
            Ok(Self {
                owner: ConfigOwner {
                    pointer: NonNull::new(pointer.cast()).ok_or(Error::NullConfigs)?,
                    destroy: destroy_configs,
                },
                process_name: None,
            })
        }
        #[cfg(not(target_env = "ohos"))]
        {
            Err(Error::HostUnsupported)
        }
    }
    pub fn set_isolation_mode(&mut self, isolation: IsolationMode) -> Result<(), Error> {
        #[cfg(target_env = "ohos")]
        {
            // SAFETY: The configs owner remains live and uniquely borrowed.
            Error::check(unsafe {
                ohos_native_child_process_sys::OH_Ability_ChildProcessConfigs_SetIsolationMode(
                    self.raw(),
                    isolation.raw(),
                )
            })
        }
        #[cfg(not(target_env = "ohos"))]
        {
            let _ = isolation;
            Err(Error::HostUnsupported)
        }
    }
    pub fn set_process_name(&mut self, name: ChildProcessName) -> Result<(), Error> {
        #[cfg(target_env = "ohos")]
        {
            // SAFETY: Both opaque owner and validated string are live throughout
            // the call. Retain the string in case the platform borrows it.
            Error::check(unsafe {
                ohos_native_child_process_sys::OH_Ability_ChildProcessConfigs_SetProcessName(
                    self.raw(),
                    name.as_c_str().as_ptr(),
                )
            })?;
            self.process_name = Some(name);
            Ok(())
        }
        #[cfg(not(target_env = "ohos"))]
        {
            let _ = name;
            Err(Error::HostUnsupported)
        }
    }
    #[cfg(feature = "api-21")]
    pub fn set_isolation_uid(&mut self, independent_uid: bool) -> Result<(), Error> {
        #[cfg(target_env = "ohos")]
        {
            // SAFETY: The mutable configs owner is live. This faithfully exposes
            // the SDK flag, not any independent-process manifest capability.
            Error::check(unsafe {
                ohos_native_child_process_sys::OH_Ability_ChildProcessConfigs_SetIsolationUid(
                    self.raw(),
                    independent_uid,
                )
            })
        }
        #[cfg(not(target_env = "ohos"))]
        {
            let _ = independent_uid;
            Err(Error::HostUnsupported)
        }
    }
    #[cfg(target_env = "ohos")]
    pub(crate) fn raw(&self) -> *mut ohos_native_child_process_sys::Ability_ChildProcessConfigs {
        self.owner.pointer.as_ptr().cast()
    }
}

#[cfg(target_env = "ohos")]
unsafe fn destroy_configs(pointer: NonNull<c_void>) {
    // SAFETY: ConfigOwner supplies the unique live SDK-created object once.
    let _ = unsafe {
        ohos_native_child_process_sys::OH_Ability_DestroyChildProcessConfigs(
            pointer.as_ptr().cast(),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    unsafe fn fake_destroy(_: NonNull<c_void>) {
        DROPS.fetch_add(1, Ordering::Relaxed);
    }
    #[test]
    fn config_resource_is_destroyed_once() {
        let before = DROPS.load(Ordering::Relaxed);
        let owner = ConfigOwner {
            pointer: NonNull::dangling(),
            destroy: fake_destroy,
        };
        drop(owner);
        assert_eq!(DROPS.load(Ordering::Relaxed), before + 1);
    }
    #[test]
    fn process_names_match_header_contract() {
        assert!(ChildProcessName::new("probe_1").is_ok());
        assert!(ChildProcessName::new("123").is_ok());
        for name in ["", "a-b", "a.b", "a/b", "x\0y"] {
            assert!(ChildProcessName::new(name).is_err());
        }
        assert!(ChildProcessName::new(&"a".repeat(65)).is_err());
    }
}
