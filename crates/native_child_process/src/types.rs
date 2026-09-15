use std::{
    ffi::{CStr, CString},
    num::NonZeroI32,
    path::Path,
};

use crate::NativeChildProcessError as Error;

pub(crate) const MAX_FDS: usize = 16;
pub(crate) const MAX_NAME: usize = 64;
pub(crate) const MAX_PARAMS: usize = 64 * 1024;

/// Owned, validated application-packaged library basename and C export symbol.
/// Paths, traversal, arbitrary executable names and interior NULs are rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildProcessEntry(CString);

impl ChildProcessEntry {
    pub fn new(library: &Path, symbol: &str) -> Result<Self, Error> {
        let library = library.to_str().ok_or(Error::InvalidEntry)?;
        if library.as_bytes().contains(&0) || symbol.as_bytes().contains(&0) {
            return Err(Error::InteriorNul { field: "entry" });
        }
        if library.len() > 255 || symbol.len() > 255 {
            return Err(Error::LimitExceeded {
                field: "entry component",
                limit: 255,
            });
        }
        if !library.starts_with("lib")
            || !library.ends_with(".so")
            || library.len() <= 6
            || !library
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
            || library.contains("..")
            || !valid_identifier(symbol)
        {
            return Err(Error::InvalidEntry);
        }
        Ok(Self(
            CString::new(format!("{library}:{symbol}")).map_err(|_| Error::InvalidEntry)?,
        ))
    }

    pub fn as_c_str(&self) -> &CStr {
        &self.0
    }
}

impl TryFrom<&str> for ChildProcessEntry {
    type Error = Error;
    fn try_from(entry: &str) -> Result<Self, Self::Error> {
        let (library, symbol) = entry.split_once(':').ok_or(Error::InvalidEntry)?;
        Self::new(Path::new(library), symbol)
    }
}

/// Validated, unique FD key. Binding limit: 64 ASCII bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildFdName(CString);

impl ChildFdName {
    pub fn new(name: &str) -> Result<Self, Error> {
        if name.as_bytes().contains(&0) {
            return Err(Error::InteriorNul { field: "FD name" });
        }
        if name.len() > MAX_NAME {
            return Err(Error::LimitExceeded {
                field: "FD name",
                limit: MAX_NAME,
            });
        }
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
        {
            return Err(Error::InvalidFdName);
        }
        Ok(Self(CString::new(name).map_err(|_| Error::InvalidFdName)?))
    }
    pub fn as_c_str(&self) -> &CStr {
        &self.0
    }
}

/// Positive platform process identity; generation stays attached to its handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChildProcessId(NonZeroI32);

impl ChildProcessId {
    #[cfg(any(target_env = "ohos", test))]
    pub(crate) fn new(pid: i32) -> Result<Self, Error> {
        if pid <= 0 {
            return Err(Error::InvalidPid);
        }
        Ok(Self(NonZeroI32::new(pid).ok_or(Error::InvalidPid)?))
    }
    pub const fn get(self) -> i32 {
        self.0.get()
    }
}

/// Faithful generic SDK enum. This is not independent-process configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IsolationMode {
    #[default]
    Normal,
    Isolated,
}

impl IsolationMode {
    #[cfg(target_env = "ohos")]
    pub(crate) fn raw(self) -> ohos_native_child_process_sys::NativeChildProcess_IsolationMode {
        match self {
            Self::Normal => ohos_native_child_process_sys::NativeChildProcess_IsolationMode_NCP_ISOLATION_MODE_NORMAL,
            Self::Isolated => ohos_native_child_process_sys::NativeChildProcess_IsolationMode_NCP_ISOLATION_MODE_ISOLATED,
        }
    }
}

/// Typed options; the reserved SDK field is always zero.
#[derive(Debug, Clone, Copy, Default)]
pub struct ChildProcessOptions {
    isolation: IsolationMode,
}

impl ChildProcessOptions {
    pub const fn new(isolation: IsolationMode) -> Self {
        Self { isolation }
    }
    pub const fn isolation(self) -> IsolationMode {
        self.isolation
    }
    #[cfg(target_env = "ohos")]
    pub(crate) fn raw(self) -> ohos_native_child_process_sys::NativeChildProcess_Options {
        ohos_native_child_process_sys::NativeChildProcess_Options {
            isolationMode: self.isolation.raw(),
            reserved: 0,
        }
    }
}

pub(crate) fn valid_identifier(value: &str) -> bool {
    let mut bytes = value.bytes();
    bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_entries_without_lossy_path_identity() {
        assert!(ChildProcessEntry::try_from("libchild.so:ChildMain").is_ok());
        for entry in [
            "aria2c:main",
            "../libchild.so:Main",
            "/libchild.so:Main",
            "libchild.so:1main",
            "libchild.so:",
            "libchild.so:Main:Other",
            "libchild.so:bad\0",
            "lib..so:Main",
        ] {
            assert!(ChildProcessEntry::try_from(entry).is_err(), "{entry:?}");
        }
        use std::os::unix::ffi::OsStrExt;
        assert!(ChildProcessEntry::new(
            Path::new(std::ffi::OsStr::from_bytes(b"lib\xff.so")),
            "Main"
        )
        .is_err());
    }
    #[test]
    fn validates_names_and_pid() {
        assert!(ChildFdName::new("probe.control").is_ok());
        for name in ["", "a/b", "a\0b", "你好"] {
            assert!(ChildFdName::new(name).is_err());
        }
        assert!(ChildFdName::new(&"a".repeat(65)).is_err());
        assert!(ChildProcessId::new(0).is_err());
        assert!(ChildProcessId::new(-1).is_err());
        assert_eq!(ChildProcessId::new(7).unwrap().get(), 7);
        assert_eq!(
            ChildProcessOptions::default().isolation(),
            IsolationMode::Normal
        );
    }
}
