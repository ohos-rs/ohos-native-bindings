use once_cell::sync::Lazy;

use crate::SysConfig;

pub const HUKS: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-huks-sys",
    headers: vec!["huks/native_huks_api.h", "huks/native_huks_param.h"],
    white_list: vec!["OH_Huks.*", "OH_HUKS.*"],
    block_list: vec![],
    dynamic_library: vec!["huks_ndk.z"],
    extra: "",
});
