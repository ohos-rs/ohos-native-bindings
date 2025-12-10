use once_cell::sync::Lazy;

use super::SysConfig;

pub const ASSET: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-asset-sys",
    headers: vec!["asset/asset_api.h", "asset/asset_type.h"],
    white_list: vec!["Asset.*", "OH_Asset.*", "ASSET_TAG_TYPE_MASK"],
    block_list: vec![],
    dynamic_library: vec!["asset_ndk.z"],
    extra: "",
});
