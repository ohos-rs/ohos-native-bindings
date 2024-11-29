use once_cell::sync::Lazy;

use super::SysConfig;

pub const VSYNC: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-vsync-sys",
    headers: vec!["native_vsync/native_vsync.h"],
    white_list: vec![],
    block_list: vec![],
    extra: "",
});
