use std::fmt;

/// Result alias for battery info operations.
pub type Result<T> = std::result::Result<T, BatteryInfoError>;

/// An error returned by a battery info operation.
///
/// The native module has no error-code family: every call returns a plain
/// value. The variants below therefore describe the ways a returned value can
/// fall outside the contract documented by `ohbattery_info.h`, which is the
/// only failure the safe layer can observe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryInfoError {
    /// The native call returned a capacity outside the documented 0..=100
    /// range, which usually means the battery service is unavailable.
    CapacityOutOfRange(i32),
    /// The native call returned a plugged type that is not part of
    /// `BatteryInfo_BatteryPluggedType`.
    UnknownPluggedType(u32),
}

impl BatteryInfoError {
    /// A short, stable description of this error.
    pub fn describe(&self) -> &'static str {
        match self {
            BatteryInfoError::CapacityOutOfRange(_) => "battery capacity out of the 0..=100 range",
            BatteryInfoError::UnknownPluggedType(_) => "unrecognized plugged type",
        }
    }
}

impl fmt::Display for BatteryInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatteryInfoError::CapacityOutOfRange(value) => {
                write!(f, "{} (got {value})", self.describe())
            }
            BatteryInfoError::UnknownPluggedType(value) => {
                write!(f, "{} (got {value})", self.describe())
            }
        }
    }
}

impl std::error::Error for BatteryInfoError {}
