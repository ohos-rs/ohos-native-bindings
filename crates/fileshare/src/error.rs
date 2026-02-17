use ohos_fileshare_sys::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileShareError {
    PermissionError,
    InvalidParameter,
    DeviceNotSupported,
    OperationNotPermitted,
    NoSuchFileOrDirectory,
    OutOfMemory,
    Unknown,
    NullByteError,
    ConversionError,
}

impl std::fmt::Display for FileShareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileShareError::PermissionError => write!(f, "Permission verification failed"),
            FileShareError::InvalidParameter => write!(f, "Invalid input parameter"),
            FileShareError::DeviceNotSupported => write!(f, "Device not supported"),
            FileShareError::OperationNotPermitted => write!(f, "Operation not permitted"),
            FileShareError::NoSuchFileOrDirectory => write!(f, "No such file or directory"),
            FileShareError::OutOfMemory => write!(f, "Out of memory"),
            FileShareError::Unknown => write!(f, "Unknown error"),
            FileShareError::NullByteError => write!(f, "String contains null byte"),
            FileShareError::ConversionError => write!(f, "Failed to convert result from C"),
        }
    }
}

impl std::error::Error for FileShareError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyErrorCode {
    PersistenceForbidden,
    InvalidMode,
    InvalidPath,
    PermissionNotPersisted,
    Unknown(u32),
}

impl From<u32> for PolicyErrorCode {
    fn from(value: u32) -> Self {
        if value == FileShare_PolicyErrorCode_PERSISTENCE_FORBIDDEN {
            Self::PersistenceForbidden
        } else if value == FileShare_PolicyErrorCode_INVALID_MODE {
            Self::InvalidMode
        } else if value == FileShare_PolicyErrorCode_INVALID_PATH {
            Self::InvalidPath
        } else if value == FileShare_PolicyErrorCode_PERMISSION_NOT_PERSISTED {
            Self::PermissionNotPersisted
        } else {
            Self::Unknown(value)
        }
    }
}

#[allow(non_upper_case_globals)]
pub(crate) fn error_from_code(code: FileManagement_ErrCode) -> FileShareError {
    match code {
        FileManagement_ErrCode_ERR_PERMISSION_ERROR => FileShareError::PermissionError,
        FileManagement_ErrCode_ERR_INVALID_PARAMETER => FileShareError::InvalidParameter,
        FileManagement_ErrCode_ERR_DEVICE_NOT_SUPPORTED => FileShareError::DeviceNotSupported,
        FileManagement_ErrCode_ERR_EPERM => FileShareError::OperationNotPermitted,
        FileManagement_ErrCode_ERR_ENOENT => FileShareError::NoSuchFileOrDirectory,
        FileManagement_ErrCode_ERR_ENOMEM => FileShareError::OutOfMemory,
        _ => FileShareError::Unknown,
    }
}
