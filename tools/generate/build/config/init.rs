use once_cell::sync::Lazy;

use super::SysConfig;

pub const INIT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-init-sys",
    headers: vec!["syscap_ndk.h"],
    white_list: vec!["canIUse"],
    block_list: vec![],
    dynamic_library: vec!["deviceinfo_ndk.z"],
    extra: "",
});
