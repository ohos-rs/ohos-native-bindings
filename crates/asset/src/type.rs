use ohos_asset_sys::{
    Asset_Accessibility_ASSET_ACCESSIBILITY_DEVICE_FIRST_UNLOCKED,
    Asset_Accessibility_ASSET_ACCESSIBILITY_DEVICE_POWERED_ON,
    Asset_Accessibility_ASSET_ACCESSIBILITY_DEVICE_UNLOCKED, Asset_AuthType_ASSET_AUTH_TYPE_ANY,
    Asset_AuthType_ASSET_AUTH_TYPE_NONE, Asset_ConflictResolution_ASSET_CONFLICT_OVERWRITE,
    Asset_ConflictResolution_ASSET_CONFLICT_THROW_ERROR, Asset_ReturnType_ASSET_RETURN_ALL,
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

impl From<ohos_asset_sys::Asset_Tag> for AssetTag {
    fn from(value: ohos_asset_sys::Asset_Tag) -> Self {
        match value {
            ohos_asset_sys::Asset_Tag_ASSET_TAG_SECRET => AssetTag::AssetTagSecret,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_ALIAS => AssetTag::AssetTagAlias,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_ACCESSIBILITY => AssetTag::AssetTagAccessibility,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_REQUIRE_PASSWORD_SET => {
                AssetTag::AssetTagRequirePasswordSet
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_AUTH_TYPE => AssetTag::AssetTagAuthType,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_AUTH_VALIDITY_PERIOD => {
                AssetTag::AssetTagAuthValidityPeriod
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_AUTH_CHALLENGE => AssetTag::AssetTagAuthChallenge,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_AUTH_TOKEN => AssetTag::AssetTagAuthToken,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_SYNC_TYPE => AssetTag::AssetTagSyncType,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_IS_PERSISTENT => AssetTag::AssetTagIsPersistent,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_CRITICAL_1 => {
                AssetTag::AssetTagDataLabelCritical1
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_CRITICAL_2 => {
                AssetTag::AssetTagDataLabelCritical2
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_CRITICAL_3 => {
                AssetTag::AssetTagDataLabelCritical3
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_CRITICAL_4 => {
                AssetTag::AssetTagDataLabelCritical4
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_1 => {
                AssetTag::AssetTagDataLabelNormal1
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_2 => {
                AssetTag::AssetTagDataLabelNormal2
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_3 => {
                AssetTag::AssetTagDataLabelNormal3
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_4 => {
                AssetTag::AssetTagDataLabelNormal4
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_1 => {
                AssetTag::AssetTagDataLabelNormalLocal1
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_2 => {
                AssetTag::AssetTagDataLabelNormalLocal2
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_3 => {
                AssetTag::AssetTagDataLabelNormalLocal3
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_4 => {
                AssetTag::AssetTagDataLabelNormalLocal4
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_RETURN_TYPE => AssetTag::AssetTagReturnType,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_RETURN_LIMIT => AssetTag::AssetTagReturnLimit,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_RETURN_OFFSET => AssetTag::AssetTagReturnOffset,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_RETURN_ORDERED_BY => {
                AssetTag::AssetTagReturnOrderedBy
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_CONFLICT_RESOLUTION => {
                AssetTag::AssetTagConflictResolution
            }
            ohos_asset_sys::Asset_Tag_ASSET_TAG_UPDATE_TIME => AssetTag::AssetTagUpdateTime,
            ohos_asset_sys::Asset_Tag_ASSET_TAG_OPERATION_TYPE => AssetTag::AssetTagOperationType,
            _ => panic!("Unknown ohos_asset_sys::Asset_Tag value: {}", value),
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
