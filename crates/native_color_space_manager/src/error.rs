use std::fmt;

/// Result alias for color space manager operations.
pub type Result<T> = std::result::Result<T, ColorSpaceError>;

/// An error returned by a color space manager operation.
///
/// The native color space manager API carries no error codes: creation returns
/// a null pointer and every getter has a documented failure sentinel. Those
/// sentinels are turned into the variants below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpaceError {
    /// The native layer returned no instance; the documented cause is memory
    /// shortage.
    CreationFailed,
    /// The instance could not report its color space name (the native call
    /// returned the failure sentinel `0`).
    NameUnavailable,
    /// The instance reported a color space value that is outside the range this
    /// binding knows.
    UnknownName(u32),
    /// The instance could not report its white point (the native call returned
    /// the failure sentinel `(0.0, 0.0)`).
    WhitePointUnavailable,
    /// The instance could not report its gamma (the native call returned the
    /// failure sentinel `0.0`).
    GammaUnavailable,
}

impl fmt::Display for ColorSpaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorSpaceError::CreationFailed => {
                write!(f, "failed to create a color space manager instance")
            }
            ColorSpaceError::NameUnavailable => write!(f, "color space name is unavailable"),
            ColorSpaceError::UnknownName(value) => {
                write!(f, "unknown color space name value {value}")
            }
            ColorSpaceError::WhitePointUnavailable => write!(f, "white point is unavailable"),
            ColorSpaceError::GammaUnavailable => write!(f, "gamma is unavailable"),
        }
    }
}

impl std::error::Error for ColorSpaceError {}
