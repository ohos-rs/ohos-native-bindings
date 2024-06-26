use core::fmt;
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub enum AssetResultCode {
    Success,
    PermissionDenied,
    InvalidArgument,
    ServiceUnavailable,
    NotFound,
    Duplicated,
    AccessDenied,
    StatusMismatch,
    OutOfMemory,
    DataCorrupted,
    DatabaseError,
    CryptoError,
    IpcError,
    BmsError,
    AccountError,
    AccessTokenError,
    FileOperationError,
    GetSystemTimeError,
    LimitExceeded,
    Unsupported,
    UnknownError(u32),
}

impl From<ohos_asset_sys::Asset_ResultCode> for AssetResultCode {
    fn from(code: ohos_asset_sys::Asset_ResultCode) -> Self {
        match code {
            ohos_asset_sys::Asset_ResultCode_ASSET_SUCCESS => AssetResultCode::Success,
            ohos_asset_sys::Asset_ResultCode_ASSET_PERMISSION_DENIED => {
                AssetResultCode::PermissionDenied
            }
            ohos_asset_sys::Asset_ResultCode_ASSET_INVALID_ARGUMENT => {
                AssetResultCode::InvalidArgument
            }
            ohos_asset_sys::Asset_ResultCode_ASSET_SERVICE_UNAVAILABLE => {
                AssetResultCode::ServiceUnavailable
            }
            ohos_asset_sys::Asset_ResultCode_ASSET_NOT_FOUND => AssetResultCode::NotFound,
            ohos_asset_sys::Asset_ResultCode_ASSET_DUPLICATED => AssetResultCode::Duplicated,
            ohos_asset_sys::Asset_ResultCode_ASSET_ACCESS_DENIED => AssetResultCode::AccessDenied,
            ohos_asset_sys::Asset_ResultCode_ASSET_STATUS_MISMATCH => {
                AssetResultCode::StatusMismatch
            }
            ohos_asset_sys::Asset_ResultCode_ASSET_OUT_OF_MEMORY => AssetResultCode::OutOfMemory,
            ohos_asset_sys::Asset_ResultCode_ASSET_DATA_CORRUPTED => AssetResultCode::DataCorrupted,
            ohos_asset_sys::Asset_ResultCode_ASSET_DATABASE_ERROR => AssetResultCode::DatabaseError,
            ohos_asset_sys::Asset_ResultCode_ASSET_CRYPTO_ERROR => AssetResultCode::CryptoError,
            ohos_asset_sys::Asset_ResultCode_ASSET_IPC_ERROR => AssetResultCode::IpcError,
            ohos_asset_sys::Asset_ResultCode_ASSET_BMS_ERROR => AssetResultCode::BmsError,
            ohos_asset_sys::Asset_ResultCode_ASSET_ACCOUNT_ERROR => AssetResultCode::AccountError,
            ohos_asset_sys::Asset_ResultCode_ASSET_ACCESS_TOKEN_ERROR => {
                AssetResultCode::AccessTokenError
            }
            ohos_asset_sys::Asset_ResultCode_ASSET_FILE_OPERATION_ERROR => {
                AssetResultCode::FileOperationError
            }
            ohos_asset_sys::Asset_ResultCode_ASSET_GET_SYSTEM_TIME_ERROR => {
                AssetResultCode::GetSystemTimeError
            }
            ohos_asset_sys::Asset_ResultCode_ASSET_LIMIT_EXCEEDED => AssetResultCode::LimitExceeded,
            ohos_asset_sys::Asset_ResultCode_ASSET_UNSUPPORTED => AssetResultCode::Unsupported,
            c => AssetResultCode::UnknownError(c),
        }
    }
}

impl AssetResultCode {
    pub fn to_result_code(self) -> ohos_asset_sys::Asset_ResultCode {
        match self {
            AssetResultCode::Success => ohos_asset_sys::Asset_ResultCode_ASSET_SUCCESS,
            AssetResultCode::PermissionDenied => {
                ohos_asset_sys::Asset_ResultCode_ASSET_PERMISSION_DENIED
            }
            AssetResultCode::InvalidArgument => {
                ohos_asset_sys::Asset_ResultCode_ASSET_INVALID_ARGUMENT
            }
            AssetResultCode::ServiceUnavailable => {
                ohos_asset_sys::Asset_ResultCode_ASSET_SERVICE_UNAVAILABLE
            }
            AssetResultCode::NotFound => ohos_asset_sys::Asset_ResultCode_ASSET_NOT_FOUND,
            AssetResultCode::Duplicated => ohos_asset_sys::Asset_ResultCode_ASSET_DUPLICATED,
            AssetResultCode::AccessDenied => ohos_asset_sys::Asset_ResultCode_ASSET_ACCESS_DENIED,
            AssetResultCode::StatusMismatch => {
                ohos_asset_sys::Asset_ResultCode_ASSET_STATUS_MISMATCH
            }
            AssetResultCode::OutOfMemory => ohos_asset_sys::Asset_ResultCode_ASSET_OUT_OF_MEMORY,
            AssetResultCode::DataCorrupted => ohos_asset_sys::Asset_ResultCode_ASSET_DATA_CORRUPTED,
            AssetResultCode::DatabaseError => ohos_asset_sys::Asset_ResultCode_ASSET_DATABASE_ERROR,
            AssetResultCode::CryptoError => ohos_asset_sys::Asset_ResultCode_ASSET_CRYPTO_ERROR,
            AssetResultCode::IpcError => ohos_asset_sys::Asset_ResultCode_ASSET_IPC_ERROR,
            AssetResultCode::BmsError => ohos_asset_sys::Asset_ResultCode_ASSET_BMS_ERROR,
            AssetResultCode::AccountError => ohos_asset_sys::Asset_ResultCode_ASSET_ACCOUNT_ERROR,
            AssetResultCode::AccessTokenError => {
                ohos_asset_sys::Asset_ResultCode_ASSET_ACCESS_TOKEN_ERROR
            }
            AssetResultCode::FileOperationError => {
                ohos_asset_sys::Asset_ResultCode_ASSET_FILE_OPERATION_ERROR
            }
            AssetResultCode::GetSystemTimeError => {
                ohos_asset_sys::Asset_ResultCode_ASSET_GET_SYSTEM_TIME_ERROR
            }
            AssetResultCode::LimitExceeded => ohos_asset_sys::Asset_ResultCode_ASSET_LIMIT_EXCEEDED,
            AssetResultCode::Unsupported => ohos_asset_sys::Asset_ResultCode_ASSET_UNSUPPORTED,
            AssetResultCode::UnknownError(_code) => unimplemented!(),
        }
    }
}

impl Display for AssetResultCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetResultCode::Success => write!(f, "Success"),
            AssetResultCode::PermissionDenied => write!(f, "Permission denied"),
            AssetResultCode::InvalidArgument => write!(f, "Invalid argument"),
            AssetResultCode::ServiceUnavailable => write!(f, "The ASSET service is unavailable."),
            AssetResultCode::NotFound => write!(f, "The asset is not found."),
            AssetResultCode::Duplicated => write!(f, "The asset already exists."),
            AssetResultCode::AccessDenied => write!(f, "Access to the asset is denied."),
            AssetResultCode::StatusMismatch => write!(f, "The screen lock status does not match."),
            AssetResultCode::OutOfMemory => write!(f, "Insufficient memory."),
            AssetResultCode::DataCorrupted => write!(f, "The asset is corrupted."),
            AssetResultCode::DatabaseError => write!(f, "The database operation failed."),
            AssetResultCode::CryptoError => write!(f, "The cryptography operation failed."),
            AssetResultCode::IpcError => write!(f, "IPC failed."),
            AssetResultCode::BmsError => write!(f, "Calling the Bundle Manager service failed."),
            AssetResultCode::AccountError => write!(f, "Calling the OS Account service failed."),
            AssetResultCode::AccessTokenError => {
                write!(f, "Calling the Access Token service failed.")
            }
            AssetResultCode::FileOperationError => write!(f, "The file operation failed."),
            AssetResultCode::GetSystemTimeError => write!(f, "Getting the system time failed."),
            AssetResultCode::LimitExceeded => write!(f, "The cache exceeds the limit."),
            AssetResultCode::Unsupported => write!(f, "The capability is not supported."),
            AssetResultCode::UnknownError(code) => write!(f, "Unknown error code: {}", code),
        }
    }
}
