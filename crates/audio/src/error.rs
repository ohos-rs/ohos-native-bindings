use std::fmt;

use ohos_audio_sys as sys;

/// Errors returned by OpenHarmony OHAudio stream and session APIs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AudioError {
    /// Native OHAudio API returned a non-zero error code.
    InternalError(u32),
    /// Common error (e.g. invalid argument, null handle, or closed renderer).
    CommonError(String),
}

impl AudioError {
    /// Returns the raw native error code when this error originated from OHAudio.
    pub const fn code(&self) -> Option<u32> {
        match self {
            Self::InternalError(code) => Some(*code),
            Self::CommonError(_) => None,
        }
    }
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InternalError(code) => write!(f, "OHAudio API failed with error code {code}"),
            Self::CommonError(msg) => write!(f, "Common error: {msg}"),
        }
    }
}

impl std::error::Error for AudioError {}

pub type Result<T> = std::result::Result<T, AudioError>;

#[allow(non_upper_case_globals)]
pub(crate) fn check_stream(code: sys::OH_AudioStream_Result) -> Result<()> {
    if code == sys::OH_AudioStream_Result_AUDIOSTREAM_SUCCESS {
        Ok(())
    } else {
        Err(AudioError::InternalError(code))
    }
}

#[allow(non_upper_case_globals)]
pub(crate) fn check_common(code: sys::OH_AudioCommon_Result) -> Result<()> {
    if code == sys::OH_AudioCommon_Result_AUDIOCOMMON_RESULT_SUCCESS {
        Ok(())
    } else {
        Err(AudioError::InternalError(code))
    }
}
