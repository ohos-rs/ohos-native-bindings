use once_cell::sync::Lazy;

use crate::SysConfig;

pub const EVENT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-event-sys",
    headers: vec!["arkui/ui_input_event.h"],
    white_list: vec!["ArkUI_.*", "ARKUI_.*", "OH_.*", "UI_.*", "Hit.*"],
    block_list: vec![],
    dynamic_library: vec!["ace_ndk.z"],
    extra: "",
});
