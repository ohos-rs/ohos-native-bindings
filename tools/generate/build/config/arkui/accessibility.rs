use once_cell::sync::Lazy;

use crate::SysConfig;

pub const ACCESSIBILITY: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-accessibility-sys",
    headers: vec!["arkui/native_interface_accessibility.h"],
    white_list: vec!["ArkUI_.*", "ARKUI_.*", "OH_.*", "UI_.*", "Hit.*"],
    block_list: vec![],
    dynamic_library: vec!["ace_ndk.z"],
    extra: "",
});
