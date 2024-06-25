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
    Asset_SyncType_ASSET_SYNC_TYPE_THIS_DEVICE, Asset_SyncType_ASSET_SYNC_TYPE_TRUSTED_ACCOUNT,
    Asset_SyncType_ASSET_SYNC_TYPE_TRUSTED_DEVICE,
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
                Asset_SyncType_ASSET_SYNC_TYPE_TRUSTED_ACCOUNT
            }
            AssetSyncType::AssetSyncTypeTrustedDevice => {
                Asset_SyncType_ASSET_SYNC_TYPE_TRUSTED_DEVICE
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AssetTag {
    AssetTagSecret,
    AssetTagAlias,
    AssetTagAccessibility,
    AssetTagRequirePasswordSet,
    AssetTagAuthType,
    AssetTagAuthValidityPeriod,
    AssetTagAuthChallenge,
    AssetTagAuthToken,
    AssetTagSyncType,
    AssetTagIsPersistent,
    AssetTagDataLabelCritical1,
    AssetTagDataLabelCritical2,
    AssetTagDataLabelCritical3,
    AssetTagDataLabelCritical4,
    AssetTagDataLabelNormal1,
    AssetTagDataLabelNormal2,
    AssetTagDataLabelNormal3,
    AssetTagDataLabelNormal4,
    AssetTagDataLabelNormalLocal1,
    AssetTagDataLabelNormalLocal2,
    AssetTagDataLabelNormalLocal3,
    AssetTagDataLabelNormalLocal4,
    AssetTagReturnType,
    AssetTagReturnLimit,
    AssetTagReturnOffset,
    AssetTagReturnOrderedBy,
    AssetTagConflictResolution,
    AssetTagUpdateTime,
    AssetTagOperationType,
}

impl From<AssetTag> for ohos_asset_sys::Asset_Tag {
    fn from(value: AssetTag) -> Self {
        match value {
            AssetTag::AssetTagSecret => ohos_asset_sys::Asset_Tag_ASSET_TAG_SECRET,
            AssetTag::AssetTagAlias => ohos_asset_sys::Asset_Tag_ASSET_TAG_ALIAS,
            AssetTag::AssetTagAccessibility => ohos_asset_sys::Asset_Tag_ASSET_TAG_ACCESSIBILITY,
            AssetTag::AssetTagRequirePasswordSet => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_REQUIRE_PASSWORD_SET
            }
            AssetTag::AssetTagAuthType => ohos_asset_sys::Asset_Tag_ASSET_TAG_AUTH_TYPE,
            AssetTag::AssetTagAuthValidityPeriod => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_AUTH_VALIDITY_PERIOD
            }
            AssetTag::AssetTagAuthChallenge => ohos_asset_sys::Asset_Tag_ASSET_TAG_AUTH_CHALLENGE,
            AssetTag::AssetTagAuthToken => ohos_asset_sys::Asset_Tag_ASSET_TAG_AUTH_TOKEN,
            AssetTag::AssetTagSyncType => ohos_asset_sys::Asset_Tag_ASSET_TAG_AUTH_TYPE,
            AssetTag::AssetTagIsPersistent => ohos_asset_sys::Asset_Tag_ASSET_TAG_IS_PERSISTENT,
            AssetTag::AssetTagDataLabelCritical1 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_CRITICAL_1
            }
            AssetTag::AssetTagDataLabelCritical2 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_CRITICAL_2
            }
            AssetTag::AssetTagDataLabelCritical3 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_CRITICAL_3
            }
            AssetTag::AssetTagDataLabelCritical4 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_CRITICAL_4
            }
            AssetTag::AssetTagDataLabelNormal1 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_1
            }
            AssetTag::AssetTagDataLabelNormal2 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_2
            }
            AssetTag::AssetTagDataLabelNormal3 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_3
            }
            AssetTag::AssetTagDataLabelNormal4 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_4
            }
            AssetTag::AssetTagDataLabelNormalLocal1 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_1
            }
            AssetTag::AssetTagDataLabelNormalLocal2 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_2
            }
            AssetTag::AssetTagDataLabelNormalLocal3 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_3
            }
            AssetTag::AssetTagDataLabelNormalLocal4 => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_4
            }
            AssetTag::AssetTagReturnType => ohos_asset_sys::Asset_Tag_ASSET_TAG_RETURN_TYPE,
            AssetTag::AssetTagReturnLimit => ohos_asset_sys::Asset_Tag_ASSET_TAG_RETURN_LIMIT,
            AssetTag::AssetTagReturnOffset => ohos_asset_sys::Asset_Tag_ASSET_TAG_RETURN_OFFSET,
            AssetTag::AssetTagReturnOrderedBy => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_RETURN_ORDERED_BY
            }
            AssetTag::AssetTagConflictResolution => {
                ohos_asset_sys::Asset_Tag_ASSET_TAG_CONFLICT_RESOLUTION
            }
            AssetTag::AssetTagUpdateTime => ohos_asset_sys::Asset_Tag_ASSET_TAG_UPDATE_TIME,
            AssetTag::AssetTagOperationType => ohos_asset_sys::Asset_Tag_ASSET_TAG_OPERATION_TYPE,
        }
    }
}

#[derive(Debug)]
pub enum AssetTagType {
    AssetTypeBool,
    AssetTypeNumber,
    AssetTypeBytes,
}

impl From<AssetTagType> for ohos_asset_sys::Asset_TagType {
    fn from(value: AssetTagType) -> Self {
        match value {
            AssetTagType::AssetTypeBool => ohos_asset_sys::Asset_TagType_ASSET_TYPE_BOOL,
            AssetTagType::AssetTypeNumber => ohos_asset_sys::Asset_TagType_ASSET_TYPE_NUMBER,
            AssetTagType::AssetTypeBytes => ohos_asset_sys::Asset_TagType_ASSET_TYPE_BYTES,
        }
    }
}

#[derive(Debug)]
pub enum AssetOperationType {
    AssetNeedSync,
    AssetNeedLogout,
}

impl From<AssetOperationType> for ohos_asset_sys::Asset_OperationType {
    fn from(value: AssetOperationType) -> Self {
        match value {
            AssetOperationType::AssetNeedLogout => {
                ohos_asset_sys::Asset_OperationType_ASSET_NEED_LOGOUT
            }
            AssetOperationType::AssetNeedSync => {
                ohos_asset_sys::Asset_OperationType_ASSET_NEED_SYNC
            }
        }
    }
}
