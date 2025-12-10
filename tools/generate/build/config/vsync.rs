use once_cell::sync::Lazy;

use super::SysConfig;

pub const VSYNC: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-native-vsync-sys",
    headers: vec!["native_vsync/native_vsync.h"],
    white_list: vec![],
    block_list: vec![],
    dynamic_library: vec!["native_vsync"],
    extra: "",
});
