//! Safe Rust bindings for OpenHarmony **bluetooth**.
//!
//! The native `oh_bluetooth.h` header exposes a single capability: querying the
//! state of the local bluetooth switch. Everything else in the platform's
//! bluetooth stack (scanning, pairing, GATT, profiles) is reachable only from
//! the ArkTS side, so this crate stays deliberately small.
//!
//! The whole API arrived in API 13 and therefore sits behind the `api-13`
//! feature; without it the crate exposes nothing but [`sys`].
//!
//! The raw bindings are re-exported as [`sys`] for anything not covered here.

pub use ohos_bluetooth_sys as sys;

#[cfg(feature = "api-13")]
mod error;
#[cfg(feature = "api-13")]
mod state;

#[cfg(feature = "api-13")]
use error::check;
#[cfg(feature = "api-13")]
pub use error::{describe, BluetoothError, Result};
#[cfg(feature = "api-13")]
pub use state::SwitchState;

/// Read the state of the local bluetooth switch.
///
/// This is a plain query: it neither turns bluetooth on nor requires the user
/// to grant a permission.
///
/// # Example
///
/// ```no_run
/// use ohos_bluetooth_binding as bluetooth;
///
/// let state = bluetooth::switch_state()?;
/// println!("bluetooth is {state}");
/// if state.is_le_available() {
///     // start LE work
/// }
/// # Ok::<(), bluetooth::BluetoothError>(())
/// ```
#[cfg(feature = "api-13")]
pub fn switch_state() -> Result<SwitchState> {
    let mut raw: sys::Bluetooth_SwitchState = sys::Bluetooth_SwitchState_BLUETOOTH_STATE_OFF;
    check(unsafe { sys::OH_Bluetooth_GetBluetoothSwitchState(&mut raw) })?;
    SwitchState::from_raw(raw)
}

/// Whether bluetooth is fully on, that is ready for both classic and LE use.
///
/// A convenience wrapper over [`switch_state`]; transient and LE-only states
/// all read as `false`.
#[cfg(feature = "api-13")]
pub fn is_enabled() -> Result<bool> {
    Ok(switch_state()?.is_on())
}
