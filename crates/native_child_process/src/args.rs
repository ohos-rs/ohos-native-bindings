use std::{
    ffi::CString,
    marker::PhantomData,
    os::fd::{AsRawFd, BorrowedFd, FromRawFd, OwnedFd},
};

use crate::{
    types::{MAX_FDS, MAX_PARAMS},
    ChildFdName, NativeChildProcessError as Error,
};

pub(crate) struct LaunchFd {
    pub(crate) name: ChildFdName,
    // A host facade never launches, but must still retain/close its duplicates.
    #[cfg_attr(not(target_env = "ohos"), allow(dead_code))]
    pub(crate) fd: OwnedFd,
}

/// Retains C strings, launch duplicates and borrow provenance through the
/// synchronous start. Every duplicate has CLOEXEC; originals stay parent-owned.
/// Binding limits (not SDK guarantees): 16 FDs and 64 KiB parameter bytes.
pub struct ChildProcessArgsBuilder<'fd> {
    pub(crate) params: CString,
    pub(crate) fds: Vec<LaunchFd>,
    borrow: PhantomData<BorrowedFd<'fd>>,
}

impl<'fd> Default for ChildProcessArgsBuilder<'fd> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'fd> ChildProcessArgsBuilder<'fd> {
    pub fn new() -> Self {
        Self {
            params: CString::default(),
            fds: Vec::new(),
            borrow: PhantomData,
        }
    }
    pub fn entry_params(mut self, params: &str) -> Result<Self, Error> {
        if params.len() > MAX_PARAMS {
            return Err(Error::LimitExceeded {
                field: "entry params",
                limit: MAX_PARAMS,
            });
        }
        self.params = CString::new(params).map_err(|_| Error::InteriorNul {
            field: "entry params",
        })?;
        Ok(self)
    }
    pub fn named_fd(mut self, name: ChildFdName, fd: BorrowedFd<'fd>) -> Result<Self, Error> {
        if self.fds.len() == MAX_FDS {
            return Err(Error::LimitExceeded {
                field: "FD count",
                limit: MAX_FDS,
            });
        }
        if self.fds.iter().any(|item| item.name == name) {
            return Err(Error::DuplicateFdName);
        }
        self.fds.push(LaunchFd::new(name, fd)?);
        Ok(self)
    }
}

impl LaunchFd {
    fn new(name: ChildFdName, fd: BorrowedFd<'_>) -> Result<Self, Error> {
        loop {
            // SAFETY: BorrowedFd guarantees a live descriptor; fcntl duplicates it
            // without consuming the original. A successful result is a new owner.
            let duplicate = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 0) };
            if duplicate >= 0 {
                // SAFETY: This fresh fcntl descriptor has no other Rust owner.
                return Ok(Self {
                    name,
                    fd: unsafe { OwnedFd::from_raw_fd(duplicate) },
                });
            }
            let error = Error::last_io("duplicate launch FD");
            if matches!(
                error,
                Error::Io {
                    code: libc::EINTR,
                    ..
                }
            ) {
                continue;
            }
            return Err(error);
        }
    }
}

#[cfg(target_env = "ohos")]
pub(crate) struct PreparedArgs<'fd> {
    builder: ChildProcessArgsBuilder<'fd>,
    nodes: crate::list::LinkedNodes<ohos_native_child_process_sys::NativeChildProcess_Fd>,
}

#[cfg(target_env = "ohos")]
impl<'fd> PreparedArgs<'fd> {
    pub(crate) fn new(builder: ChildProcessArgsBuilder<'fd>) -> Self {
        use ohos_native_child_process_sys::NativeChildProcess_Fd;
        let nodes = builder
            .fds
            .iter()
            .map(|item| NativeChildProcess_Fd {
                fdName: item.name.as_c_str().as_ptr().cast_mut(),
                fd: item.fd.as_raw_fd(),
                next: std::ptr::null_mut(),
            })
            .collect();
        let nodes = crate::list::LinkedNodes::new(nodes, |node, next| node.next = next);
        Self { builder, nodes }
    }
    pub(crate) fn raw(&mut self) -> ohos_native_child_process_sys::NativeChildProcess_Args {
        ohos_native_child_process_sys::NativeChildProcess_Args {
            entryParams: self.builder.params.as_ptr().cast_mut(),
            fdList: ohos_native_child_process_sys::NativeChildProcess_FdList {
                head: self.nodes.head(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{os::fd::AsFd, os::unix::net::UnixStream};

    #[test]
    fn launch_duplicates_close_but_originals_survive() {
        let _serial = crate::FD_TEST_LOCK.lock().unwrap();
        let (original, _peer) = UnixStream::pair().unwrap();
        let args = ChildProcessArgsBuilder::new()
            .entry_params("v1")
            .unwrap()
            .named_fd(ChildFdName::new("one").unwrap(), original.as_fd())
            .unwrap()
            .named_fd(ChildFdName::new("two").unwrap(), original.as_fd())
            .unwrap();
        let duplicate = args.fds[0].fd.as_raw_fd();
        assert_ne!(duplicate, original.as_raw_fd());
        // SAFETY: fcntl F_GETFD only queries a descriptor.
        assert_ne!(
            unsafe { libc::fcntl(duplicate, libc::F_GETFD) } & libc::FD_CLOEXEC,
            0
        );
        drop(args);
        // SAFETY: These fcntl operations only query descriptor liveness.
        assert_eq!(unsafe { libc::fcntl(duplicate, libc::F_GETFD) }, -1);
        // SAFETY: The original socket is still owned and live.
        assert!(unsafe { libc::fcntl(original.as_raw_fd(), libc::F_GETFD) } >= 0);
    }
    #[test]
    fn rejects_duplicate_names_nuls_and_limits() {
        let _serial = crate::FD_TEST_LOCK.lock().unwrap();
        let (fd, _) = UnixStream::pair().unwrap();
        let args = ChildProcessArgsBuilder::new()
            .named_fd(ChildFdName::new("one").unwrap(), fd.as_fd())
            .unwrap();
        assert!(matches!(
            args.named_fd(ChildFdName::new("one").unwrap(), fd.as_fd()),
            Err(Error::DuplicateFdName)
        ));
        assert!(ChildProcessArgsBuilder::new().entry_params("a\0b").is_err());
        assert!(ChildProcessArgsBuilder::new()
            .entry_params(&"a".repeat(MAX_PARAMS + 1))
            .is_err());
        let mut args = ChildProcessArgsBuilder::new();
        for index in 0..MAX_FDS {
            args = args
                .named_fd(ChildFdName::new(&format!("fd{index}")).unwrap(), fd.as_fd())
                .unwrap();
        }
        assert!(args
            .named_fd(ChildFdName::new("extra").unwrap(), fd.as_fd())
            .is_err());
    }
    #[test]
    #[cfg(not(target_env = "ohos"))]
    fn host_platform_failure_still_closes_launch_duplicates() {
        let _serial = crate::FD_TEST_LOCK.lock().unwrap();
        let (original, _peer) = UnixStream::pair().unwrap();
        let args = ChildProcessArgsBuilder::new()
            .named_fd(ChildFdName::new("control").unwrap(), original.as_fd())
            .unwrap();
        let raw = args.fds[0].fd.as_raw_fd();
        let result = crate::NativeChildProcessManager::new().start(
            &crate::ChildProcessEntry::try_from("libchild.so:Main").unwrap(),
            args,
            crate::ChildProcessOptions::default(),
        );
        assert!(matches!(result, Err(Error::HostUnsupported)));
        // SAFETY: F_GETFD is a non-mutating liveness query.
        assert_eq!(unsafe { libc::fcntl(raw, libc::F_GETFD) }, -1);
        // SAFETY: The parent-owned socket remains live after failed launch.
        assert!(unsafe { libc::fcntl(original.as_raw_fd(), libc::F_GETFD) } >= 0);
    }
}
