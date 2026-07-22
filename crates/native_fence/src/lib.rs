//! Safe Rust bindings for OpenHarmony **native fence**.
//!
//! A fence is a synchronization primitive handed around as a file descriptor:
//! the graphics stack returns one alongside a buffer, and the descriptor
//! becomes readable once the producer is done with that buffer. This crate
//! wraps the native `native_fence.h` C API with a safe layer built around file
//! descriptor ownership.
//!
//! The whole kit was introduced in API 20, so every item below sits behind the
//! `api-20` feature.
//!
//! # Descriptor ownership
//!
//! The native API splits into two groups, and the header is explicit about it:
//!
//! * `OH_NativeFence_Wait` and `OH_NativeFence_WaitForever` only borrow — "the
//!   incoming fenceFd needs to be closed by the user themselves". They
//!   duplicate the descriptor internally for the poll and leave the original
//!   untouched, so a fence may be waited on repeatedly.
//! * `OH_NativeFence_Close` takes ownership and closes the descriptor, which
//!   must be "a non negative integer".
//! * `OH_NativeFence_IsValid` is a pure check that only rejects negative
//!   descriptors.
//!
//! [`Fence`] therefore owns a non-negative descriptor and closes it on drop,
//! while the waiting operations take `&self`; [`FenceRef`] is the same set of
//! read-only operations on a descriptor owned elsewhere and closes nothing.
//! A `-1` sentinel — the graphics stack's way of saying "no fence, the work is
//! already done" — is rejected by [`Fence::from_raw_fd`] rather than stored.
//!
//! The raw bindings are re-exported as [`sys`].
//!
//! # Example
//!
//! ```no_run
//! use std::os::fd::OwnedFd;
//! use std::time::Duration;
//!
//! use ohos_native_fence_binding::{Fence, FenceError};
//!
//! # fn acquire_fence() -> OwnedFd { unimplemented!() }
//! // Take over a fence produced by the graphics stack.
//! let fence = Fence::from(acquire_fence());
//!
//! match fence.wait(Duration::from_millis(100)) {
//!     Ok(()) => println!("buffer is ready"),
//!     Err(FenceError::NotSignaled) => println!("still busy, keep the buffer"),
//!     Err(error) => return Err(error),
//! }
//!
//! // Dropping the fence closes the descriptor; `close` does it early.
//! fence.close();
//! # Ok::<(), FenceError>(())
//! ```

pub use ohos_native_fence_sys as sys;

mod error;

pub use error::{FenceError, Result};

#[cfg(feature = "api-20")]
mod fence;

#[cfg(feature = "api-20")]
pub use fence::{Fence, FenceRef};
