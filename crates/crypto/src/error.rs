use ohos_crypto_sys::*;
use std::fmt;

/// Result alias for crypto operations.
pub type Result<T> = std::result::Result<T, CryptoError>;

/// An error returned by a crypto operation.
///
/// The C API has no error message out-parameter, so descriptions come from
/// [`describe`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CryptoError {
    code: i32,
}

impl CryptoError {
    pub(crate) fn new(code: OH_Crypto_ErrCode) -> Self {
        CryptoError { code: code as i32 }
    }

    /// The raw error code (`OH_Crypto_ErrCode`).
    pub fn code(&self) -> i32 {
        self.code
    }
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "crypto error {} ({})", self.code, describe(self.code))
    }
}

impl std::error::Error for CryptoError {}

/// Map a raw crypto error code to a short, stable description.
#[allow(non_upper_case_globals)] // matching bindgen's mixed-case `OH_Crypto_ErrCode_*` consts
pub fn describe(code: i32) -> &'static str {
    match code as u32 {
        OH_Crypto_ErrCode_CRYPTO_SUCCESS => "success",
        OH_Crypto_ErrCode_CRYPTO_INVALID_PARAMS => "invalid parameters",
        OH_Crypto_ErrCode_CRYPTO_NOT_SUPPORTED => "not supported",
        OH_Crypto_ErrCode_CRYPTO_MEMORY_ERROR => "memory error",
        // CRYPTO_PARAMETER_CHECK_FAILED (@since 20), matched by value so a
        // default-features build still describes it when the OS returns it.
        17620003 => "parameter check failed",
        OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR => "crypto operation failed",
        _ => "unknown error",
    }
}

/// Turn a raw `OH_Crypto_ErrCode` into `Result<()>`.
pub(crate) fn check(code: OH_Crypto_ErrCode) -> Result<()> {
    if code == OH_Crypto_ErrCode_CRYPTO_SUCCESS {
        Ok(())
    } else {
        Err(CryptoError::new(code))
    }
}

/// Reject a length that does not fit the `int` the C API takes.
#[cfg(feature = "api-20")]
pub(crate) fn checked_len(len: usize) -> Result<std::os::raw::c_int> {
    std::os::raw::c_int::try_from(len)
        .map_err(|_| CryptoError::new(OH_Crypto_ErrCode_CRYPTO_INVALID_PARAMS))
}

/// Reject a length that does not fit the `uint32_t` the C API takes.
#[cfg(feature = "api-20")]
pub(crate) fn checked_u32(len: usize) -> Result<u32> {
    u32::try_from(len).map_err(|_| CryptoError::new(OH_Crypto_ErrCode_CRYPTO_INVALID_PARAMS))
}
