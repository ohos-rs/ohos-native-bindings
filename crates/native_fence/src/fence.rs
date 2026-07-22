use std::fmt;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use std::time::Duration;

use crate::error::{FenceError, Result};
use crate::sys;

/// Translate a [`Duration`] into the millisecond timeout the native API takes.
fn timeout_millis(timeout: Duration) -> Result<u32> {
    let millis = timeout.as_millis();
    if millis == 0 {
        return Err(FenceError::ZeroTimeout);
    }
    u32::try_from(millis).map_err(|_| FenceError::TimeoutTooLong)
}

fn is_valid_raw(fd: RawFd) -> bool {
    unsafe { sys::OH_NativeFence_IsValid(fd) }
}

fn wait_raw(fd: RawFd, timeout: Duration) -> Result<()> {
    let millis = timeout_millis(timeout)?;
    if unsafe { sys::OH_NativeFence_Wait(fd, millis) } {
        Ok(())
    } else {
        Err(FenceError::NotSignaled)
    }
}

fn wait_forever_raw(fd: RawFd) -> Result<()> {
    if unsafe { sys::OH_NativeFence_WaitForever(fd) } {
        Ok(())
    } else {
        Err(FenceError::NotSignaled)
    }
}

/// A borrowed view of a fence file descriptor.
///
/// Waiting never consumes the descriptor — the native header states that "the
/// incoming fenceFd needs to be closed by the user themselves" — so every
/// read-only operation is available on a borrow. The descriptor stays owned by
/// whoever created it, and this view closes nothing when it goes out of scope.
#[derive(Debug, Clone, Copy)]
pub struct FenceRef<'fence> {
    fd: BorrowedFd<'fence>,
}

impl<'fence> FenceRef<'fence> {
    /// Borrow an existing fence file descriptor.
    pub fn new(fd: BorrowedFd<'fence>) -> Self {
        FenceRef { fd }
    }

    /// The borrowed file descriptor.
    pub fn as_fd(&self) -> BorrowedFd<'fence> {
        self.fd
    }

    /// Ask the native layer whether the descriptor denotes a fence.
    ///
    /// The check only rejects negative descriptors.
    pub fn is_valid(&self) -> bool {
        is_valid_raw(self.fd.as_raw_fd())
    }

    /// Wait for the fence to signal, giving up after `timeout`.
    ///
    /// The timeout is rounded down to whole milliseconds and must be at least
    /// one millisecond; see [`FenceError::ZeroTimeout`]. The descriptor is only
    /// borrowed for the duration of the call.
    pub fn wait(&self, timeout: Duration) -> Result<()> {
        wait_raw(self.fd.as_raw_fd(), timeout)
    }

    /// Wait for the fence to signal without a deadline.
    pub fn wait_forever(&self) -> Result<()> {
        wait_forever_raw(self.fd.as_raw_fd())
    }
}

impl AsFd for FenceRef<'_> {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd
    }
}

impl AsRawFd for FenceRef<'_> {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

impl<'fence> From<&'fence Fence> for FenceRef<'fence> {
    fn from(fence: &'fence Fence) -> Self {
        fence.as_fence_ref()
    }
}

/// An owned fence file descriptor, closed on drop.
///
/// The native header documents `OH_NativeFence_Close` as taking a non-negative
/// descriptor and closing it, while the waiting functions leave closing to the
/// caller. This type encodes that split: it owns the descriptor and closes it
/// exactly once, and the waiting functions take `&self`.
///
/// The wrapped descriptor is always non-negative, so the `-1` sentinel the
/// graphics stack uses for "already signalled, no fence" can never be held by a
/// `Fence`; it is rejected at construction time.
pub struct Fence {
    fd: RawFd,
}

impl Fence {
    /// Take ownership of a raw fence file descriptor.
    ///
    /// Returns [`FenceError::InvalidFd`] for a negative descriptor, such as the
    /// `-1` sentinel returned in place of a fence that has already signalled.
    ///
    /// # Safety
    ///
    /// The caller transfers ownership of `fd`: it must be an open file
    /// descriptor that no other object closes, and it must not be used through
    /// any other handle for the lifetime of the returned `Fence`.
    pub unsafe fn from_raw_fd(fd: RawFd) -> Result<Self> {
        if fd < 0 {
            return Err(FenceError::InvalidFd);
        }
        Ok(Fence { fd })
    }

    /// The owned file descriptor, without transferring ownership.
    pub fn as_raw_fd(&self) -> RawFd {
        self.fd
    }

    /// Give up ownership and return the raw file descriptor.
    ///
    /// The descriptor is no longer closed by this crate; closing it becomes the
    /// caller's responsibility.
    pub fn into_raw_fd(self) -> RawFd {
        let fence = std::mem::ManuallyDrop::new(self);
        fence.fd
    }

    /// Borrow the fence for a read-only operation.
    pub fn as_fence_ref(&self) -> FenceRef<'_> {
        FenceRef::new(unsafe { BorrowedFd::borrow_raw(self.fd) })
    }

    /// Ask the native layer whether the descriptor denotes a fence.
    ///
    /// Always `true` for a `Fence`, which never holds a negative descriptor;
    /// the call is exposed for parity with the native API.
    pub fn is_valid(&self) -> bool {
        is_valid_raw(self.fd)
    }

    /// Wait for the fence to signal, giving up after `timeout`.
    ///
    /// The timeout is rounded down to whole milliseconds and must be at least
    /// one millisecond; see [`FenceError::ZeroTimeout`]. Waiting does not
    /// consume the fence and may be repeated.
    pub fn wait(&self, timeout: Duration) -> Result<()> {
        wait_raw(self.fd, timeout)
    }

    /// Wait for the fence to signal without a deadline.
    pub fn wait_forever(&self) -> Result<()> {
        wait_forever_raw(self.fd)
    }

    /// Close the fence now instead of at the end of the scope.
    pub fn close(self) {
        drop(self);
    }
}

impl fmt::Debug for Fence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fence").field("fd", &self.fd).finish()
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe { sys::OH_NativeFence_Close(self.fd) };
    }
}

impl AsFd for Fence {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.fd) }
    }
}

impl AsRawFd for Fence {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl IntoRawFd for Fence {
    fn into_raw_fd(self) -> RawFd {
        Fence::into_raw_fd(self)
    }
}

impl From<OwnedFd> for Fence {
    fn from(fd: OwnedFd) -> Self {
        Fence {
            fd: fd.into_raw_fd(),
        }
    }
}

impl From<Fence> for OwnedFd {
    fn from(fence: Fence) -> Self {
        // The descriptor is non-negative by construction and ownership moves
        // out of `fence` without closing it.
        unsafe { OwnedFd::from_raw_fd(Fence::into_raw_fd(fence)) }
    }
}
