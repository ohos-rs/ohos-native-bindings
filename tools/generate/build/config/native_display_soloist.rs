use once_cell::sync::Lazy;

use super::SysConfig;

pub const NATIVE_DISPLAY_SOLOIST: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-display-soloist-sys",
    headers: vec!["native_display_soloist/native_display_soloist.h"],
    white_list: vec!["OH_.*", "DisplaySoloist.*"],
    block_list: vec![],
    extra: "",
});
