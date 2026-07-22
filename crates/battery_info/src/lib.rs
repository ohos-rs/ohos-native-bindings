//! Safe Rust bindings for OpenHarmony **battery info**.
//!
//! The native `ohbattery_info.h` module exposes the current battery level and
//! the power source the device is plugged into, plus the keys carried by the
//! battery-related common events. This crate wraps that C API with a safe
//! layer: no `unsafe` for callers, values validated against the contract the
//! header documents, and the plugged-type enum modelled as a Rust enum.
//!
//! The whole native module was introduced in API 13, so every function here
//! sits behind the `api-13` feature. The common-event keys are plain string
//! constants and are always available.
//!
//! The raw bindings are re-exported as [`sys`] for anything not covered here.
//!
//! # Example
//!
//! ```no_run
//! use ohos_battery_info_binding as battery;
//!
//! let level = battery::capacity()?;
//! let source = battery::plugged_type()?;
//! println!("battery at {level}%, {source}, charging: {}", source.is_plugged());
//! # Ok::<(), battery::BatteryInfoError>(())
//! ```

pub use ohos_battery_info_sys as sys;

mod error;

pub use error::{BatteryInfoError, Result};

#[cfg(feature = "api-13")]
mod plugged_type;

#[cfg(feature = "api-13")]
pub use plugged_type::PluggedType;

use std::ffi::CStr;

/// Key of the battery-level field carried by the battery common event.
///
/// Kept NUL-terminated so it can be handed to the common-event APIs directly;
/// use [`CStr::to_str`] for the plain `&str` form.
pub const COMMON_EVENT_KEY_CAPACITY: &CStr = c"soc";

/// Key of the charge-state field carried by the battery common event.
pub const COMMON_EVENT_KEY_CHARGE_STATE: &CStr = c"chargeState";

/// Key of the plugged-type field carried by the battery common event.
pub const COMMON_EVENT_KEY_PLUGGED_TYPE: &CStr = c"pluggedType";

/// The current battery level, as a percentage in the range 0..=100.
///
/// The native call always returns a value; a value outside the documented
/// range is reported as [`BatteryInfoError::CapacityOutOfRange`] rather than
/// passed on, since it means the battery service did not answer.
#[cfg(feature = "api-13")]
pub fn capacity() -> Result<u8> {
    let raw = unsafe { sys::OH_BatteryInfo_GetCapacity() };
    match u8::try_from(raw) {
        Ok(percent) if percent <= 100 => Ok(percent),
        _ => Err(BatteryInfoError::CapacityOutOfRange(raw)),
    }
}

/// The power source the device is currently plugged into.
///
/// The native module reports an explicitly unknown source as
/// [`PluggedType::Unknown`]; only a value outside the enum is an error.
#[cfg(feature = "api-13")]
pub fn plugged_type() -> Result<PluggedType> {
    PluggedType::from_raw(unsafe { sys::OH_BatteryInfo_GetPluggedType() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_event_keys_match_the_native_constants() {
        assert_eq!(
            COMMON_EVENT_KEY_CAPACITY.to_bytes_with_nul(),
            sys::COMMON_EVENT_KEY_CAPACITY
        );
        assert_eq!(
            COMMON_EVENT_KEY_CHARGE_STATE.to_bytes_with_nul(),
            sys::COMMON_EVENT_KEY_CHARGE_STATE
        );
        assert_eq!(
            COMMON_EVENT_KEY_PLUGGED_TYPE.to_bytes_with_nul(),
            sys::COMMON_EVENT_KEY_PLUGGED_TYPE
        );
    }
}
