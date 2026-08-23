use ohos_bluetooth_sys as sys;
use std::fmt;

/// Result alias for bluetooth operations.
pub type Result<T> = std::result::Result<T, BluetoothError>;

/// An error returned by a bluetooth operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BluetoothError {
    /// A native call reported this `Bluetooth_ResultCode`.
    Native(u32),
    /// The call succeeded but reported a switch state this crate does not know
    /// about, most likely one added by a newer system version.
    UnknownState(u32),
}

impl BluetoothError {
    /// The raw bluetooth result code (`Bluetooth_ResultCode`), for errors that
    /// carry one.
    pub fn code(&self) -> Option<u32> {
        match self {
            BluetoothError::Native(code) => Some(*code),
            BluetoothError::UnknownState(_) => None,
        }
    }
}

impl fmt::Display for BluetoothError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BluetoothError::Native(code) => {
                write!(f, "bluetooth error {code} ({})", describe(*code))
            }
            BluetoothError::UnknownState(raw) => {
                write!(f, "unknown bluetooth switch state {raw}")
            }
        }
    }
}

impl std::error::Error for BluetoothError {}

/// Map a raw `Bluetooth_ResultCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::Bluetooth_ResultCode_BLUETOOTH_SUCCESS => "success",
        sys::Bluetooth_ResultCode_BLUETOOTH_INVALID_PARAM => "invalid parameter",
        _ => "unknown error",
    }
}

/// Turn a raw `Bluetooth_ResultCode` into `Result<()>`.
pub(crate) fn check(code: sys::Bluetooth_ResultCode) -> Result<()> {
    if code == sys::Bluetooth_ResultCode_BLUETOOTH_SUCCESS {
        Ok(())
    } else {
        Err(BluetoothError::Native(code))
    }
}
