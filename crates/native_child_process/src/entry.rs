use std::{
    ffi::CStr,
    os::fd::{AsFd, BorrowedFd, OwnedFd},
};

use crate::{ChildFdName, NativeChildProcessError as Error};

struct ChildFd<'entry> {
    name: &'entry CStr,
    fd: Option<OwnedFd>,
}

/// Entry-scoped borrowed strings and owned delivered FDs. Each FD can be taken
/// once. Untaken FDs close on return, including handler error/panic. Taking an
/// `OwnedFd` may extend FD lifetime, but entry strings cannot escape the adapter.
pub struct ChildLaunchArgs<'entry> {
    params: &'entry CStr,
    fds: Vec<ChildFd<'entry>>,
}

impl<'entry> ChildLaunchArgs<'entry> {
    #[cfg(any(target_env = "ohos", test))]
    fn from_delivered(
        params: &'entry CStr,
        delivered: Vec<(&'entry CStr, OwnedFd)>,
    ) -> Result<Self, Error> {
        use crate::types::{MAX_FDS, MAX_PARAMS};
        if params.to_bytes().len() > MAX_PARAMS || delivered.len() > MAX_FDS {
            return Err(Error::InvalidChildArguments);
        }
        let mut fds: Vec<ChildFd<'entry>> = Vec::new();
        for (name, fd) in delivered {
            ChildFdName::new(name.to_str().map_err(|_| Error::InvalidFdName)?)?;
            if fds.iter().any(|item| item.name == name) {
                return Err(Error::DuplicateFdName);
            }
            fds.push(ChildFd { name, fd: Some(fd) });
        }
        Ok(Self { params, fds })
    }
    pub fn entry_params(&self) -> &CStr {
        self.params
    }
    pub fn fd_count(&self) -> usize {
        self.fds.len()
    }
    pub fn named_fd(&self, name: &ChildFdName) -> Result<BorrowedFd<'_>, Error> {
        self.fds
            .iter()
            .find(|item| item.name == name.as_c_str())
            .ok_or(Error::MissingFd)?
            .fd
            .as_ref()
            .map(AsFd::as_fd)
            .ok_or(Error::FdAlreadyTaken)
    }
    pub fn take_fd(&mut self, name: &ChildFdName) -> Result<OwnedFd, Error> {
        self.fds
            .iter_mut()
            .find(|item| item.name == name.as_c_str())
            .ok_or(Error::MissingFd)?
            .fd
            .take()
            .ok_or(Error::FdAlreadyTaken)
    }

    /// API17 alternative entry bootstrap. Executes an entry-scoped closure over
    /// the platform's current args, at most once per child process. Calling from
    /// an adapter handler returns ArgumentsAlreadyDecoded, not a second FD owner.
    /// No borrowed parameter/FD value can be returned by the higher-ranked closure.
    #[cfg(feature = "api-17")]
    pub fn with_current<R>(
        handler: impl for<'current> FnOnce(ChildLaunchArgs<'current>) -> R,
    ) -> Result<R, Error> {
        #[cfg(target_env = "ohos")]
        {
            // SAFETY: The SDK returns entry args for the current child or null;
            // they remain available during this closure. The claim prevents
            // duplicate Rust ownership within this binding.
            let raw =
                unsafe { ohos_native_child_process_sys::OH_Ability_GetCurrentChildProcessArgs() };
            if raw.is_null() {
                return Err(Error::InvalidChildArguments);
            }
            claim_args()?;
            crate::contain_panic(|| {
                // SAFETY: A non-null SDK result and the single child-side claim
                // permit exactly one borrowed decoding and FD adoption.
                let args = unsafe { ChildLaunchArgs::decode(*raw) }?;
                Ok(handler(args))
            })?
        }
        #[cfg(not(target_env = "ohos"))]
        {
            let _ = handler;
            Err(Error::HostUnsupported)
        }
    }
}

#[cfg(target_env = "ohos")]
static ARGS_CLAIMED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(target_env = "ohos")]
fn claim_args() -> Result<(), Error> {
    ARGS_CLAIMED
        .compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
        )
        .map(|_| ())
        .map_err(|_| Error::ArgumentsAlreadyDecoded)
}

/// Exact entry adapter used by `native_child_entry!`.
///
/// # Safety
/// The value must originate from AbilityKit in the current child invocation.
/// Pointers must reference readable NUL-terminated strings/list nodes for the
/// entire call. Delivered descriptors must not have other Rust owners. No raw
/// caller may separately consume current args or the delivered descriptors.
#[doc(hidden)]
#[cfg(target_env = "ohos")]
pub unsafe fn run_child_entry(
    raw: ohos_native_child_process_sys::NativeChildProcess_Args,
    handler: impl for<'entry> FnOnce(ChildLaunchArgs<'entry>) -> Result<(), Error>,
) -> Result<(), Error> {
    crate::contain_panic(|| {
        claim_args()?;
        // SAFETY: The exact caller contract establishes the readable SDK data
        // and unique FD ownership; the higher-ranked handler bounds borrows.
        handler(unsafe { ChildLaunchArgs::decode(raw) }?)
    })?
}

#[cfg(target_env = "ohos")]
impl<'entry> ChildLaunchArgs<'entry> {
    unsafe fn decode(
        raw: ohos_native_child_process_sys::NativeChildProcess_Args,
    ) -> Result<ChildLaunchArgs<'entry>, Error> {
        use crate::types::{MAX_FDS, MAX_NAME, MAX_PARAMS};
        use std::os::fd::FromRawFd;

        let mut seen_nodes = Vec::new();
        let mut seen_fds = Vec::new();
        let mut delivered = Vec::new();
        let mut cursor = raw.fdList.head;
        while !cursor.is_null() {
            if seen_nodes.len() == MAX_FDS || seen_nodes.contains(&cursor) {
                return Err(Error::InvalidChildArguments);
            }
            seen_nodes.push(cursor);
            // SAFETY: Entry contract guarantees the current SDK list node is readable.
            let node = unsafe { &*cursor };
            if node.fd < 0 || seen_fds.contains(&node.fd) {
                return Err(Error::DuplicateFd);
            }
            // SAFETY: F_GETFD queries a delivered FD without taking ownership.
            if unsafe { libc::fcntl(node.fd, libc::F_GETFD) } < 0 {
                return Err(Error::last_io("validate child FD"));
            }
            // Adopt all bounded list descriptors before string validation. Invalid
            // params/key metadata cannot strand a later delivered descriptor.
            seen_fds.push(node.fd);
            // SAFETY: AbilityKit delivered this live descriptor; the claim and seen
            // descriptor check ensure it has exactly one Rust owner.
            let owned = unsafe { OwnedFd::from_raw_fd(node.fd) };
            delivered.push((node.fdName, owned));
            cursor = node.next;
        }
        // SAFETY: Forwarded entry contract guarantees readable SDK strings. Bounds
        // constrain work; they cannot validate an arbitrary forged address.
        let params = unsafe { bounded_c_str(raw.entryParams, MAX_PARAMS) }?;
        let delivered = delivered
            .into_iter()
            .map(|(name, owned)| {
                // SAFETY: Each key points into invocation-scoped readable SDK storage.
                Ok((unsafe { bounded_c_str(name, MAX_NAME) }?, owned))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        ChildLaunchArgs::from_delivered(params, delivered)
    }
}

#[cfg(target_env = "ohos")]
unsafe fn bounded_c_str<'entry>(
    pointer: *const std::ffi::c_char,
    limit: usize,
) -> Result<&'entry CStr, Error> {
    if pointer.is_null() {
        return Err(Error::InvalidChildArguments);
    }
    // SAFETY: Only called for readable NUL-terminated SDK entry strings. strnlen
    // stops at NUL and checks the binding's maximum permitted payload length.
    let length = unsafe { libc::strnlen(pointer, limit + 1) };
    if length > limit {
        return Err(Error::InvalidChildArguments);
    }
    // SAFETY: The discovered NUL terminator and entry-scoped platform storage
    // establish a valid CStr that the adapter never lets escape.
    Ok(unsafe { CStr::from_ptr(pointer) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{ffi::CString, os::fd::AsRawFd, os::unix::net::UnixStream};

    #[test]
    fn child_ownership_is_taken_once_and_other_fds_close() {
        let _serial = crate::FD_TEST_LOCK.lock().unwrap();
        let name = CString::new("control").unwrap();
        let spare_name = CString::new("spare").unwrap();
        let params = CString::new("probe-v1").unwrap();
        let (fd, _) = UnixStream::pair().unwrap();
        let owned: OwnedFd = fd.into();
        let raw = owned.as_raw_fd();
        let (spare, _spare_peer) = UnixStream::pair().unwrap();
        let spare_raw = spare.as_raw_fd();
        let mut args = ChildLaunchArgs {
            params: &params,
            fds: vec![
                ChildFd {
                    name: &name,
                    fd: Some(owned),
                },
                ChildFd {
                    name: &spare_name,
                    fd: Some(spare.into()),
                },
            ],
        };
        let key = ChildFdName::new("control").unwrap();
        assert_eq!(args.named_fd(&key).unwrap().as_raw_fd(), raw);
        let taken = args.take_fd(&key).unwrap();
        assert_eq!(args.take_fd(&key).unwrap_err(), Error::FdAlreadyTaken);
        drop(args);
        // SAFETY: The untaken entry-owned descriptor must have closed on Drop.
        assert_eq!(unsafe { libc::fcntl(spare_raw, libc::F_GETFD) }, -1);
        // SAFETY: F_GETFD only queries the taken, still-owned descriptor.
        assert!(unsafe { libc::fcntl(raw, libc::F_GETFD) } >= 0);
        drop(taken);
        // SAFETY: Querying a closed descriptor is permitted and yields EBADF.
        assert_eq!(unsafe { libc::fcntl(raw, libc::F_GETFD) }, -1);
    }
    #[test]
    fn invalid_metadata_closes_every_adopted_descriptor() {
        let _serial = crate::FD_TEST_LOCK.lock().unwrap();
        let name = CString::new("control").unwrap();
        let params = CString::new("v1").unwrap();
        let (first, second) = UnixStream::pair().unwrap();
        let first_raw = first.as_raw_fd();
        let second_raw = second.as_raw_fd();
        assert!(matches!(
            ChildLaunchArgs::from_delivered(
                &params,
                vec![(&name, first.into()), (&name, second.into())]
            ),
            Err(Error::DuplicateFdName)
        ));
        // SAFETY: F_GETFD only queries descriptor state, including closed FDs.
        assert_eq!(unsafe { libc::fcntl(first_raw, libc::F_GETFD) }, -1);
        // SAFETY: Same non-mutating query for the other adopted descriptor.
        assert_eq!(unsafe { libc::fcntl(second_raw, libc::F_GETFD) }, -1);
    }
    #[test]
    fn panics_and_panicking_payload_drops_are_contained() {
        struct Payload;
        impl Drop for Payload {
            fn drop(&mut self) {
                panic!("payload drop must not run");
            }
        }
        assert_eq!(
            crate::contain_panic(|| std::panic::panic_any(Payload)),
            Err(Error::CallbackPanicked)
        );
    }
}
