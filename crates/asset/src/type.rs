use ohos_asset_sys::*;
use ohos_enum_derive::EnumFrom;

#[derive(Debug, EnumFrom)]
#[config(Asset_Accessibility, "Asset_Accessibility_")]
pub enum AssetAccessibility {
    AssetAccessibilityDevicePoweredOn,
    AssetAccessibilityDeviceFirstUnlocked,
    AssetAccessibilityDeviceUnlocked,
}

#[derive(Debug, EnumFrom)]
#[config(Asset_AuthType, "Asset_AuthType_")]
pub enum AssetAuthType {
    AssetAuthTypeNone,
    AssetAuthTypeAny,
}

#[derive(Debug, EnumFrom)]
#[config(Asset_ConflictResolution, "Asset_ConflictResolution_")]
pub enum AssetConflictResolution {
    AssetConflictOverwrite,
    AssetConflictThrowError,
}

#[derive(Debug, EnumFrom)]
#[config(Asset_ReturnType, "Asset_ReturnType_")]
pub enum AssetReturnType {
    AssetReturnAll,
    AssetReturnAttributes,
}

#[derive(Debug, EnumFrom)]
#[config(Asset_SyncType, "Asset_SyncType_")]
pub enum AssetSyncType {
    AssetSyncTypeNever,
    AssetSyncTypeThisDevice,
    AssetSyncTypeTrustedDevice,
    AssetSyncTypeTrustedAccount,
}

#[derive(Debug, Clone, Copy, EnumFrom)]
#[config(Asset_Tag, "Asset_Tag_")]
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
    #[suffix("ASSET_TAG_DATA_LABEL_CRITICAL_1")]
    AssetTagDataLabelCritical1,
    #[suffix("ASSET_TAG_DATA_LABEL_CRITICAL_2")]
    AssetTagDataLabelCritical2,
    #[suffix("ASSET_TAG_DATA_LABEL_CRITICAL_3")]
    AssetTagDataLabelCritical3,
    #[suffix("ASSET_TAG_DATA_LABEL_CRITICAL_4")]
    AssetTagDataLabelCritical4,
    #[suffix("ASSET_TAG_DATA_LABEL_NORMAL_1")]
    AssetTagDataLabelNormal1,
    #[suffix("ASSET_TAG_DATA_LABEL_NORMAL_2")]
    AssetTagDataLabelNormal2,
    #[suffix("ASSET_TAG_DATA_LABEL_NORMAL_3")]
    AssetTagDataLabelNormal3,
    #[suffix("ASSET_TAG_DATA_LABEL_NORMAL_4")]
    AssetTagDataLabelNormal4,
    #[suffix("ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_1")]
    AssetTagDataLabelNormalLocal1,
    #[suffix("ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_2")]
    AssetTagDataLabelNormalLocal2,
    #[suffix("ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_3")]
    AssetTagDataLabelNormalLocal3,
    #[suffix("ASSET_TAG_DATA_LABEL_NORMAL_LOCAL_4")]
    AssetTagDataLabelNormalLocal4,
    AssetTagReturnType,
    AssetTagReturnLimit,
    AssetTagReturnOffset,
    AssetTagReturnOrderedBy,
    AssetTagConflictResolution,
    AssetTagUpdateTime,
    AssetTagOperationType,
}

#[derive(Debug, EnumFrom)]
#[config(Asset_TagType, "Asset_TagType_")]
pub enum AssetTagType {
    AssetTypeBool,
    AssetTypeNumber,
    AssetTypeBytes,
}

#[derive(Debug, EnumFrom)]
#[config(Asset_OperationType, "Asset_OperationType_")]
pub enum AssetOperationType {
    AssetNeedSync,
    AssetNeedLogout,
}
