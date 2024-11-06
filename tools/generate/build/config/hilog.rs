use once_cell::sync::Lazy;

use super::SysConfig;

// ohos-hilog-sys already exists
pub const HILOG: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-hilogs-sys",
    headers: vec!["hilog/log.h"],
    white_list: vec![],
    block_list: vec![],
    extra: "",
});
