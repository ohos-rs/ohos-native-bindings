use once_cell::sync::Lazy;

use crate::SysConfig;

pub const XCOMPONENT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-xcomponent-sys",
    headers: vec![
        "ace/xcomponent/native_interface_xcomponent.h",
        "ace/xcomponent/native_xcomponent_key_event.h",
    ],
    white_list: vec!["OH_.*"],
    block_list: vec!["ArkUI_[A-WY-Z].*"],
    dynamic_library: vec!["ace_ndk.z"],
    extra: "\n\nuse ohos_arkui_sys::*;\nuse ohos_accessibility_sys::*;",
});
