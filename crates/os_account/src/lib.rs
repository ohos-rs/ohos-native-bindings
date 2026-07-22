//! Safe Rust bindings for OpenHarmony **os account**.
//!
//! The native `os_account.h` C API exposes the OS account the calling process
//! belongs to. As of API 24 that is a single call, `OH_OsAccount_GetName`,
//! which writes the account name into a caller-owned character array; the
//! platform allocates nothing, so there is no release call to pair with it.
//!
//! This crate wraps that call so a caller never handles a raw buffer: the name
//! comes back as a `String` (or as a `&str` borrowed from a caller-provided
//! byte slice), NUL-terminated and UTF-8 validated.
//!
//! The raw bindings are re-exported as [`sys`] for anything not covered here.
//!
//! # Example
//!
//! ```no_run
//! use ohos_os_account_binding as os_account;
//!
//! // Grows the buffer as needed until the name fits.
//! let name = os_account::name()?;
//! println!("account name: {name}");
//!
//! // Or read into a buffer owned by the caller, without allocating.
//! let mut buffer = [0u8; 128];
//! let name = os_account::read_name_into(&mut buffer)?;
//! println!("account name: {name}");
//! # Ok::<(), os_account::OsAccountError>(())
//! ```

pub use ohos_os_account_sys as sys;

mod error;

use error::{check, is_buffer_too_small};
pub use error::{describe, OsAccountError, Result};

use std::ffi::CStr;
use std::os::raw::c_char;

/// Buffer size, in bytes, that [`name`] starts with.
pub const INITIAL_NAME_BUFFER_SIZE: usize = 64;

/// Largest buffer, in bytes, that [`name`] allocates before giving up with
/// [`OsAccountError::NameTooLong`].
pub const MAX_NAME_BUFFER_SIZE: usize = 4096;

/// Get the name of the OS account the calling process belongs to.
///
/// The buffer starts at [`INITIAL_NAME_BUFFER_SIZE`] bytes and doubles while
/// the platform reports that the name does not fit, up to
/// [`MAX_NAME_BUFFER_SIZE`]. Use [`name_with_capacity`] to make exactly one
/// native call with a buffer size of your own choosing.
pub fn name() -> Result<String> {
    let mut capacity = INITIAL_NAME_BUFFER_SIZE;
    loop {
        match name_with_capacity(capacity) {
            Err(OsAccountError::Native(code)) if is_buffer_too_small(code) => {
                if capacity >= MAX_NAME_BUFFER_SIZE {
                    return Err(OsAccountError::NameTooLong { limit: capacity });
                }
                capacity = (capacity * 2).min(MAX_NAME_BUFFER_SIZE);
            }
            other => return other,
        }
    }
}

/// Get the name of the OS account the calling process belongs to, using a
/// buffer of `capacity` bytes.
///
/// The capacity must leave room for the terminating NUL byte; a name that does
/// not fit is reported by the platform as an invalid-parameter error. A
/// capacity of zero is raised to one, the smallest buffer the native call
/// accepts.
pub fn name_with_capacity(capacity: usize) -> Result<String> {
    let mut buffer = vec![0u8; capacity.max(1)];
    let name = read_name_into(&mut buffer)?;
    Ok(name.to_owned())
}

/// Read the name of the OS account the calling process belongs to into
/// `buffer`, and borrow it back as a string slice.
///
/// This performs no allocation. The buffer must leave room for the terminating
/// NUL byte; a name that does not fit is reported by the platform as an
/// invalid-parameter error. An empty buffer is rejected the same way.
pub fn read_name_into(buffer: &mut [u8]) -> Result<&str> {
    let code =
        unsafe { sys::OH_OsAccount_GetName(buffer.as_mut_ptr().cast::<c_char>(), buffer.len()) };
    check(code)?;
    let name = CStr::from_bytes_until_nul(buffer).map_err(|_| OsAccountError::NameNotTerminated)?;
    name.to_str().map_err(|_| OsAccountError::NameNotUtf8)
}
