use once_cell::sync::Lazy;

use crate::config::SysConfig;

pub const ARKUI: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-arkui-sys",
    headers: vec![
        "arkui/drag_and_drop.h",
        "arkui/drawable_descriptor.h",
        "arkui/native_animate.h",
        "arkui/native_dialog.h",
        "arkui/native_gesture.h",
        "arkui/native_interface.h",
        "arkui/native_node.h",
        "arkui/native_node_napi.h",
        "arkui/native_type.h",
        "arkui/styled_string.h",
    ],
    white_list: vec!["ArkUI_.*", "ARKUI_.*", "OH_.*"],
    block_list: vec!["napi_.*"],
    extra: "\n\nuse napi_sys_ohos::*;\n",
});
