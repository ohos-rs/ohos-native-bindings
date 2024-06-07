use ohos_asset_sys::{
    Asset_Accessibility_ASSET_ACCESSIBILITY_DEVICE_FIRST_UNLOCKED,
    Asset_Accessibility_ASSET_ACCESSIBILITY_DEVICE_POWERED_ON,
    Asset_Accessibility_ASSET_ACCESSIBILITY_DEVICE_UNLOCKED, Asset_AuthType_ASSET_AUTH_TYPE_ANY,
    Asset_AuthType_ASSET_AUTH_TYPE_NONE, Asset_ConflictResolution_ASSET_CONFLICT_OVERWRITE,
    Asset_ConflictResolution_ASSET_CONFLICT_THROW_ERROR, Asset_ResultCode_ASSET_ACCESS_DENIED,
    Asset_ResultCode_ASSET_ACCESS_TOKEN_ERROR, Asset_ResultCode_ASSET_ACCOUNT_ERROR,
    Asset_ResultCode_ASSET_BMS_ERROR, Asset_ResultCode_ASSET_CRYPTO_ERROR,
    Asset_ResultCode_ASSET_DATABASE_ERROR, Asset_ResultCode_ASSET_DATA_CORRUPTED,
    Asset_ResultCode_ASSET_DUPLICATED, Asset_ResultCode_ASSET_FILE_OPERATION_ERROR,
    Asset_ResultCode_ASSET_GET_SYSTEM_TIME_ERROR, Asset_ResultCode_ASSET_INVALID_ARGUMENT,
    Asset_ResultCode_ASSET_IPC_ERROR, Asset_ResultCode_ASSET_LIMIT_EXCEEDED,
    Asset_ResultCode_ASSET_NOT_FOUND, Asset_ResultCode_ASSET_OUT_OF_MEMORY,
    Asset_ResultCode_ASSET_PERMISSION_DENIED, Asset_ResultCode_ASSET_SERVICE_UNAVAILABLE,
    Asset_ResultCode_ASSET_STATUS_MISMATCH, Asset_ResultCode_ASSET_SUCCESS,
    Asset_ResultCode_ASSET_UNSUPPORTED, Asset_ReturnType_ASSET_RETURN_ALL,
    Asset_ReturnType_ASSET_RETURN_ATTRIBUTES, Asset_SyncType_ASSET_SYNC_TYPE_NEVER,
    Asset_SyncType_ASSET_SYNC_TYPE_THIS_DEVICE, Asset_SyncType_ASSET_SYNC_TYPE_TRUSTED_DEVICE,
};

#[derive(Debug)]
pub enum AssetAccessibility {
    AssetAccessibilityDevicePoweredOn,
    AssetAccessibilityDeviceFirstUnlocked,
    AssetAccessibilityDeviceUnlocked,
}

impl From<AssetAccessibility> for ohos_asset_sys::Asset_Accessibility {
    fn from(value: AssetAccessibility) -> Self {
        match value {
            AssetAccessibility::AssetAccessibilityDeviceFirstUnlocked => {
                Asset_Accessibility_ASSET_ACCESSIBILITY_DEVICE_FIRST_UNLOCKED
            }
            AssetAccessibility::AssetAccessibilityDevicePoweredOn => {
                Asset_Accessibility_ASSET_ACCESSIBILITY_DEVICE_POWERED_ON
            }
            AssetAccessibility::AssetAccessibilityDeviceUnlocked => {
                Asset_Accessibility_ASSET_ACCESSIBILITY_DEVICE_UNLOCKED
            }
        }
    }
}

#[derive(Debug)]
pub enum AssetAuthType {
    AssetAuthTypeNone,
    AssetAuthTypeAny,
}

impl From<AssetAuthType> for ohos_asset_sys::Asset_AuthType {
    fn from(value: AssetAuthType) -> Self {
        match value {
            AssetAuthType::AssetAuthTypeAny => Asset_AuthType_ASSET_AUTH_TYPE_ANY,
            AssetAuthType::AssetAuthTypeNone => Asset_AuthType_ASSET_AUTH_TYPE_NONE,
        }
    }
}

#[derive(Debug)]
pub enum AssetConflictResolution {
    AssetConflictOverwrite,
    AssetConflictThrowError,
}

impl From<AssetConflictResolution> for ohos_asset_sys::Asset_ConflictResolution {
    fn from(value: AssetConflictResolution) -> Self {
        match value {
            AssetConflictResolution::AssetConflictOverwrite => {
                Asset_ConflictResolution_ASSET_CONFLICT_OVERWRITE
            }
            AssetConflictResolution::AssetConflictThrowError => {
                Asset_ConflictResolution_ASSET_CONFLICT_THROW_ERROR
            }
        }
    }
}

#[derive(Debug)]
pub enum AssetResultCode {
    AssetSuccess,
    AssetPermissionDenied,
    AssetInvalidArgument,
    AssetServiceUnavailable,
    AssetNotFound,
    AssetDuplicated,
    AssetAccessDenied,
    AssetStatusMismatch,
    AssetOutOfMemory,
    AssetDataCorrupted,
    AssetDatabaseError,
    AssetCryptoError,
    AssetIpcError,
    AssetBmsError,
    AssetAccountError,
    AssetAccessTokenError,
    AssetFileOperationError,
    AssetGetSystemTimeError,
    AssetLimitExceeded,
    AssetUnsupported,
}

impl From<AssetResultCode> for ohos_asset_sys::Asset_ResultCode {
    fn from(value: AssetResultCode) -> Self {
        match value {
            AssetResultCode::AssetSuccess => Asset_ResultCode_ASSET_SUCCESS,
            AssetResultCode::AssetPermissionDenied => Asset_ResultCode_ASSET_PERMISSION_DENIED,
            AssetResultCode::AssetInvalidArgument => Asset_ResultCode_ASSET_INVALID_ARGUMENT,
            AssetResultCode::AssetServiceUnavailable => Asset_ResultCode_ASSET_SERVICE_UNAVAILABLE,
            AssetResultCode::AssetNotFound => Asset_ResultCode_ASSET_NOT_FOUND,
            AssetResultCode::AssetDuplicated => Asset_ResultCode_ASSET_DUPLICATED,
            AssetResultCode::AssetAccessDenied => Asset_ResultCode_ASSET_ACCESS_DENIED,
            AssetResultCode::AssetStatusMismatch => Asset_ResultCode_ASSET_STATUS_MISMATCH,
            AssetResultCode::AssetOutOfMemory => Asset_ResultCode_ASSET_OUT_OF_MEMORY,
            AssetResultCode::AssetDataCorrupted => Asset_ResultCode_ASSET_DATA_CORRUPTED,
            AssetResultCode::AssetDatabaseError => Asset_ResultCode_ASSET_DATABASE_ERROR,
            AssetResultCode::AssetCryptoError => Asset_ResultCode_ASSET_CRYPTO_ERROR,
            AssetResultCode::AssetIpcError => Asset_ResultCode_ASSET_IPC_ERROR,
            AssetResultCode::AssetBmsError => Asset_ResultCode_ASSET_BMS_ERROR,
            AssetResultCode::AssetAccountError => Asset_ResultCode_ASSET_ACCOUNT_ERROR,
            AssetResultCode::AssetAccessTokenError => Asset_ResultCode_ASSET_ACCESS_TOKEN_ERROR,
            AssetResultCode::AssetFileOperationError => Asset_ResultCode_ASSET_FILE_OPERATION_ERROR,
            AssetResultCode::AssetGetSystemTimeError => {
                Asset_ResultCode_ASSET_GET_SYSTEM_TIME_ERROR
            }
            AssetResultCode::AssetLimitExceeded => Asset_ResultCode_ASSET_LIMIT_EXCEEDED,
            AssetResultCode::AssetUnsupported => Asset_ResultCode_ASSET_UNSUPPORTED,
        }
    }
}

#[derive(Debug)]
pub enum AssetReturnType {
    AssetReturnAll,
    AssetReturnAttributes,
}

impl From<AssetReturnType> for ohos_asset_sys::Asset_ReturnType {
    fn from(value: AssetReturnType) -> Self {
        match value {
            AssetReturnType::AssetReturnAll => Asset_ReturnType_ASSET_RETURN_ALL,
            AssetReturnType::AssetReturnAttributes => Asset_ReturnType_ASSET_RETURN_ATTRIBUTES,
        }
    }
}

#[derive(Debug)]
pub enum AssetSyncType {
    AssetSyncTypeNever,
    AssetSyncTypeThisDevice,
    AssetSyncTypeTrustedDevice,
    AssetSyncTypeTrustedAccount,
}

impl From<AssetSyncType> for ohos_asset_sys::Asset_SyncType {
    fn from(value: AssetSyncType) -> Self {
        match value {
            AssetSyncType::AssetSyncTypeNever => Asset_SyncType_ASSET_SYNC_TYPE_NEVER,
            AssetSyncType::AssetSyncTypeThisDevice => Asset_SyncType_ASSET_SYNC_TYPE_THIS_DEVICE,
            AssetSyncType::AssetSyncTypeTrustedAccount => {
                Asset_SyncType_ASSET_SYNC_TYPE_TRUSTED_DEVICE
            }
            AssetSyncType::AssetSyncTypeTrustedDevice => {
                Asset_SyncType_ASSET_SYNC_TYPE_TRUSTED_DEVICE
            }
        }
    }
}
