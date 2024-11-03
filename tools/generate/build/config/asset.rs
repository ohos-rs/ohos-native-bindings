use once_cell::sync::Lazy;

use super::SysConfig;

pub const ASSET: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-asset-sys",
    headers: vec!["asset/asset_api.h", "asset/asset_type.h"],
    white_list: vec![],
    block_list: vec![],
    extra: "",
});
