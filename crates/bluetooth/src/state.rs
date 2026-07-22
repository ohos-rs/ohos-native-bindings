use ohos_bluetooth_sys as sys;
use std::fmt;

use crate::error::{BluetoothError, Result};

/// State of the local bluetooth switch, the Rust form of
/// `Bluetooth_SwitchState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchState {
    /// Bluetooth is off.
    Off,
    /// Bluetooth is turning on.
    TurningOn,
    /// Bluetooth is on and ready for use.
    On,
    /// Bluetooth is turning off.
    TurningOff,
    /// Bluetooth is turning LE-only mode on.
    BleTurningOn,
    /// Bluetooth is in LE-only mode.
    BleOn,
    /// Bluetooth is turning LE-only mode off.
    BleTurningOff,
}

impl SwitchState {
    /// Whether bluetooth is fully on, that is ready for both classic and LE use.
    pub fn is_on(self) -> bool {
        matches!(self, SwitchState::On)
    }

    /// Whether LE operation is available, which covers both the fully-on state
    /// and LE-only mode.
    pub fn is_le_available(self) -> bool {
        matches!(self, SwitchState::On | SwitchState::BleOn)
    }

    /// Whether the switch is in a transient state and is expected to settle into
    /// another one shortly.
    pub fn is_transitioning(self) -> bool {
        matches!(
            self,
            SwitchState::TurningOn
                | SwitchState::TurningOff
                | SwitchState::BleTurningOn
                | SwitchState::BleTurningOff
        )
    }

    /// A short, stable description of the state.
    pub fn describe(self) -> &'static str {
        match self {
            SwitchState::Off => "off",
            SwitchState::TurningOn => "turning on",
            SwitchState::On => "on",
            SwitchState::TurningOff => "turning off",
            SwitchState::BleTurningOn => "turning LE-only mode on",
            SwitchState::BleOn => "LE-only mode",
            SwitchState::BleTurningOff => "turning LE-only mode off",
        }
    }

    /// Map a raw `Bluetooth_SwitchState` onto this enum.
    ///
    /// The constants are matched through qualified `sys::` paths so that an arm
    /// whose constant is absent under the current feature set is a compile error
    /// instead of a catch-all binding pattern. A value the system reports but
    /// this crate does not know becomes [`BluetoothError::UnknownState`].
    pub(crate) fn from_raw(raw: sys::Bluetooth_SwitchState) -> Result<Self> {
        Ok(match raw {
            sys::Bluetooth_SwitchState_BLUETOOTH_STATE_OFF => SwitchState::Off,
            sys::Bluetooth_SwitchState_BLUETOOTH_STATE_TURNING_ON => SwitchState::TurningOn,
            sys::Bluetooth_SwitchState_BLUETOOTH_STATE_ON => SwitchState::On,
            sys::Bluetooth_SwitchState_BLUETOOTH_STATE_TURNING_OFF => SwitchState::TurningOff,
            sys::Bluetooth_SwitchState_BLUETOOTH_STATE_BLE_TURNING_ON => SwitchState::BleTurningOn,
            sys::Bluetooth_SwitchState_BLUETOOTH_STATE_BLE_ON => SwitchState::BleOn,
            sys::Bluetooth_SwitchState_BLUETOOTH_STATE_BLE_TURNING_OFF => {
                SwitchState::BleTurningOff
            }
            other => return Err(BluetoothError::UnknownState(other)),
        })
    }
}

impl fmt::Display for SwitchState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.describe())
    }
}
