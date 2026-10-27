use once_cell::sync::Lazy;

use super::SysConfig;

pub const DEVICE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-device-sys",
    headers: vec!["deviceinfo.h"],
    white_list: vec!["OH_Get.*"],
    block_list: vec![],
    dynamic_library: vec!["deviceinfo_ndk.z"],
    extra: "",
});
