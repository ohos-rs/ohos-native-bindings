use ohos_fileuri_sys::*;

/// Error type for file URI operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileUriError {
    /// Permission verification failed
    PermissionError,
    /// Invalid input parameter
    InvalidParameter,
    /// Device not supported
    DeviceNotSupported,
    /// Operation not permitted
    OperationNotPermitted,
    /// No such file or directory
    NoSuchFileOrDirectory,
    /// Out of memory
    OutOfMemory,
    /// Unknown error
    Unknown,
    /// Failed to convert string to CString (contains null bytes)
    NullByteError,
    /// Failed to convert result from C
    ConversionError,
}

impl std::fmt::Display for FileUriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileUriError::PermissionError => write!(f, "Permission verification failed"),
            FileUriError::InvalidParameter => write!(f, "Invalid input parameter"),
            FileUriError::DeviceNotSupported => write!(f, "Device not supported"),
            FileUriError::OperationNotPermitted => write!(f, "Operation not permitted"),
            FileUriError::NoSuchFileOrDirectory => write!(f, "No such file or directory"),
            FileUriError::OutOfMemory => write!(f, "Out of memory"),
            FileUriError::Unknown => write!(f, "Unknown error"),
            FileUriError::NullByteError => write!(f, "String contains null byte"),
            FileUriError::ConversionError => write!(f, "Failed to convert result from C"),
        }
    }
}

impl std::error::Error for FileUriError {}

/// Convert error code to FileUriError
#[allow(non_upper_case_globals)]
pub(crate) fn error_from_code(code: FileManagement_ErrCode) -> FileUriError {
    match code {
        FileManagement_ErrCode_ERR_PERMISSION_ERROR => FileUriError::PermissionError,
        FileManagement_ErrCode_ERR_INVALID_PARAMETER => FileUriError::InvalidParameter,
        FileManagement_ErrCode_ERR_DEVICE_NOT_SUPPORTED => FileUriError::DeviceNotSupported,
        FileManagement_ErrCode_ERR_EPERM => FileUriError::OperationNotPermitted,
        FileManagement_ErrCode_ERR_ENOENT => FileUriError::NoSuchFileOrDirectory,
        FileManagement_ErrCode_ERR_ENOMEM => FileUriError::OutOfMemory,
        _ => FileUriError::Unknown,
    }
}
