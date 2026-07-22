//! Safe Rust bindings for the OpenHarmony **time service** time zone API.
//!
//! The native `time_service.h` C API exposes a single entry point,
//! `OH_TimeService_GetTimeZone`, which writes the current system time zone ID
//! into a caller-provided character array. This crate wraps it so that callers
//! never deal with the output buffer, its size, the terminating character or
//! the numeric error codes.
//!
//! The whole API is available since API 12, so none of it sits behind an
//! `api-*` feature. The raw bindings are re-exported as [`sys`] for anything
//! not covered here.
//!
//! # Example
//!
//! ```no_run
//! use ohos_time_service_binding as time_service;
//!
//! let zone = time_service::get_time_zone()?;
//! println!("current time zone: {zone}");
//! # Ok::<(), time_service::TimeServiceError>(())
//! ```

pub use ohos_time_service_sys as sys;

mod error;

use error::check;
pub use error::{describe, Result, TimeServiceError};

use std::os::raw::c_char;

/// Buffer size of the first [`get_time_zone`] attempt, in bytes.
///
/// The native documentation recommends at least 31 bytes and gives no upper
/// bound, so the first attempt is comfortably above the recommendation and
/// covers every IANA time zone ID in practice.
const INITIAL_CAPACITY: usize = 64;

/// Largest buffer [`get_time_zone`] grows to, in bytes.
const MAX_CAPACITY: usize = 4096;

/// Obtain the current system time zone ID, for example `Asia/Shanghai`.
///
/// The output buffer is managed internally: it starts at 64 bytes and is
/// doubled up to 4096 bytes while the service reports that the ID does not fit.
/// Use [`get_time_zone_with_capacity`] to pick the buffer size explicitly.
pub fn get_time_zone() -> Result<String> {
    let mut capacity = INITIAL_CAPACITY;
    loop {
        match get_time_zone_with_capacity(capacity) {
            Err(TimeServiceError::BufferTooSmall(_)) if capacity < MAX_CAPACITY => {
                capacity = (capacity * 2).min(MAX_CAPACITY);
            }
            result => return result,
        }
    }
}

/// Obtain the current system time zone ID using a buffer of `capacity` bytes.
///
/// `capacity` covers the terminating character as well, so it must exceed the
/// length of the time zone ID; the native documentation recommends at least 31
/// bytes. A capacity that is too small yields
/// [`TimeServiceError::BufferTooSmall`], a capacity of zero yields it without
/// calling into the service.
pub fn get_time_zone_with_capacity(capacity: usize) -> Result<String> {
    let len = u32::try_from(capacity).unwrap_or(u32::MAX);
    if len == 0 {
        return Err(TimeServiceError::BufferTooSmall(capacity));
    }

    let mut buffer = vec![0u8; len as usize];
    let code =
        unsafe { sys::OH_TimeService_GetTimeZone(buffer.as_mut_ptr().cast::<c_char>(), len) };
    if code == sys::TimeService_ErrCode_TIMESERVICE_ERR_INVALID_PARAMETER {
        // The pointer is never null and the length is never zero here, so the
        // only remaining cause is a time zone ID that does not fit.
        return Err(TimeServiceError::BufferTooSmall(buffer.len()));
    }
    check(code)?;

    let end = buffer
        .iter()
        .position(|byte| *byte == 0)
        .ok_or(TimeServiceError::BufferTooSmall(buffer.len()))?;
    std::str::from_utf8(&buffer[..end])
        .map(str::to_owned)
        .map_err(|_| TimeServiceError::InvalidUtf8)
}
