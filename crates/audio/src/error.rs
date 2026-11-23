use std::fmt;

use ohos_audio_sys as sys;

/// Errors returned by OpenHarmony OHAudio stream and session APIs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioError {
    InvalidParam,
    IllegalState,
    System,
    NoMemory,
    Unsupported,
    Timeout,
    StreamLimit,
    PermissionDenied,
    UnsupportedFormat,
    ServiceDied,
    UnexpectedNull,
    Other(u32),
}

impl AudioError {
    #[allow(non_upper_case_globals)]
    pub const fn from_stream_result(code: sys::OH_AudioStream_Result) -> Self {
        match code {
            sys::OH_AudioStream_Result_AUDIOSTREAM_ERROR_INVALID_PARAM => Self::InvalidParam,
            sys::OH_AudioStream_Result_AUDIOSTREAM_ERROR_ILLEGAL_STATE => Self::IllegalState,
            sys::OH_AudioStream_Result_AUDIOSTREAM_ERROR_SYSTEM => Self::System,
            #[cfg(feature = "api-19")]
            sys::OH_AudioStream_Result_AUDIOSTREAM_ERROR_UNSUPPORTED_FORMAT => {
                Self::UnsupportedFormat
            }
            #[cfg(feature = "api-26")]
            sys::OH_AudioStream_Result_AUDIOSTREAM_ERROR_UNSUPPORTED_ABILITY => Self::Unsupported,
            #[cfg(feature = "api-26")]
            sys::OH_AudioStream_Result_AUDIOSTREAM_ERROR_SERVICE_DIED => Self::ServiceDied,
            other => Self::Other(other),
        }
    }

    #[allow(non_upper_case_globals)]
    pub const fn from_common_result(code: sys::OH_AudioCommon_Result) -> Self {
        match code {
            sys::OH_AudioCommon_Result_AUDIOCOMMON_RESULT_ERROR_INVALID_PARAM => Self::InvalidParam,
            sys::OH_AudioCommon_Result_AUDIOCOMMON_RESULT_ERROR_NO_MEMORY => Self::NoMemory,
            sys::OH_AudioCommon_Result_AUDIOCOMMON_RESULT_ERROR_ILLEGAL_STATE => Self::IllegalState,
            sys::OH_AudioCommon_Result_AUDIOCOMMON_RESULT_ERROR_UNSUPPORTED => Self::Unsupported,
            sys::OH_AudioCommon_Result_AUDIOCOMMON_RESULT_ERROR_TIMEOUT => Self::Timeout,
            sys::OH_AudioCommon_Result_AUDIOCOMMON_RESULT_ERROR_STREAM_LIMIT => Self::StreamLimit,
            sys::OH_AudioCommon_Result_AUDIOCOMMON_RESULT_ERROR_SYSTEM => Self::System,
            #[cfg(feature = "api-26")]
            sys::OH_AudioCommon_Result_AUDIOCOMMON_RESULT_ERROR_PERMISSION_DENIED => {
                Self::PermissionDenied
            }
            other => Self::Other(other),
        }
    }
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParam => f.write_str("invalid audio parameter"),
            Self::IllegalState => f.write_str("illegal audio stream or session state"),
            Self::System => f.write_str("audio system error"),
            Self::NoMemory => f.write_str("out of memory in audio service"),
            Self::Unsupported => f.write_str("unsupported audio operation"),
            Self::Timeout => f.write_str("audio operation timed out"),
            Self::StreamLimit => f.write_str("audio stream limit reached"),
            Self::PermissionDenied => f.write_str("audio permission denied"),
            Self::UnsupportedFormat => f.write_str("unsupported audio format"),
            Self::ServiceDied => f.write_str("audio server process died"),
            Self::UnexpectedNull => f.write_str("native audio API returned null"),
            Self::Other(code) => write!(f, "audio error code {code}"),
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
        Err(AudioError::from_stream_result(code))
    }
}

#[allow(non_upper_case_globals)]
pub(crate) fn check_common(code: sys::OH_AudioCommon_Result) -> Result<()> {
    if code == sys::OH_AudioCommon_Result_AUDIOCOMMON_RESULT_SUCCESS {
        Ok(())
    } else {
        Err(AudioError::from_common_result(code))
    }
}
