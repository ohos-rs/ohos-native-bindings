use std::fmt;

/// Platform errors retain exact numeric codes; local validation failures are
/// separate variants and do not masquerade as a successful platform call.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NativeChildProcessError {
    InvalidParameter,
    NotSupported,
    Internal,
    Busy,
    Timeout,
    Service,
    MultiProcessDisabled,
    AlreadyInChild,
    MaxChildProcessesReached,
    LibraryLoadingFailed,
    ConnectionFailed,
    CallbackNotExist,
    InvalidPid,
    UnknownPlatformCode(u32),
    InvalidEntry,
    InteriorNul { field: &'static str },
    LimitExceeded { field: &'static str, limit: usize },
    InvalidFdName,
    DuplicateFdName,
    DuplicateFd,
    MissingFd,
    FdAlreadyTaken,
    InvalidProcessName,
    NullConfigs,
    InvalidChildArguments,
    ArgumentsAlreadyDecoded,
    ObservationAmbiguous,
    StaleHandle,
    CapacityExceeded,
    CallbackPanicked,
    HostUnsupported,
    Io { operation: &'static str, code: i32 },
}

impl NativeChildProcessError {
    /// Converts SDK results without depending on a particular enabled API.
    pub fn from_platform_code(code: u32) -> Option<Self> {
        Some(match code {
            0 => return None,
            401 => Self::InvalidParameter,
            801 => Self::NotSupported,
            16000050 => Self::Internal,
            16010001 => Self::Busy,
            16010002 => Self::Timeout,
            16010003 => Self::Service,
            16010004 => Self::MultiProcessDisabled,
            16010005 => Self::AlreadyInChild,
            16010006 => Self::MaxChildProcessesReached,
            16010007 => Self::LibraryLoadingFailed,
            16010008 => Self::ConnectionFailed,
            16010009 => Self::CallbackNotExist,
            16010010 => Self::InvalidPid,
            unknown => Self::UnknownPlatformCode(unknown),
        })
    }

    /// Returns the original numeric platform error, if this is a platform error.
    pub fn platform_code(&self) -> Option<u32> {
        Some(match self {
            Self::InvalidParameter => 401,
            Self::NotSupported => 801,
            Self::Internal => 16000050,
            Self::Busy => 16010001,
            Self::Timeout => 16010002,
            Self::Service => 16010003,
            Self::MultiProcessDisabled => 16010004,
            Self::AlreadyInChild => 16010005,
            Self::MaxChildProcessesReached => 16010006,
            Self::LibraryLoadingFailed => 16010007,
            Self::ConnectionFailed => 16010008,
            Self::CallbackNotExist => 16010009,
            Self::InvalidPid => 16010010,
            Self::UnknownPlatformCode(code) => *code,
            _ => return None,
        })
    }

    #[cfg(any(all(target_env = "ohos", feature = "api-13"), test))]
    pub(crate) fn check(code: u32) -> Result<(), Self> {
        Self::from_platform_code(code).map_or(Ok(()), Err)
    }

    #[cfg(feature = "api-13")]
    pub(crate) fn last_io(operation: &'static str) -> Self {
        Self::Io {
            operation,
            code: std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        }
    }
}

impl fmt::Display for NativeChildProcessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for NativeChildProcessError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_all_platform_codes_and_unknowns() {
        for code in [
            401,
            801,
            16000050,
            16010001,
            16010002,
            16010003,
            16010004,
            16010005,
            16010006,
            16010007,
            16010008,
            16010009,
            16010010,
            u32::MAX,
        ] {
            assert_eq!(
                NativeChildProcessError::from_platform_code(code)
                    .unwrap()
                    .platform_code(),
                Some(code)
            );
        }
        assert!(NativeChildProcessError::check(0).is_ok());
        assert_eq!(NativeChildProcessError::InvalidEntry.platform_code(), None);
    }
}
