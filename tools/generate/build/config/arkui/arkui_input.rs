use once_cell::sync::Lazy;

use crate::config::SysConfig;

pub const ARKUI_INPUT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-arkui-input-sys",
    headers: vec!["arkui/ui_input_event.h"],
    white_list: vec![
        ".*UIInputEvent.*",
        ".*UI_TOUCH_EVENT.*",
        ".*UI_INPUT_EVENT.*",
        ".*UI_AXIS_EVENT.*",
        ".*UI_MOUSE_EVENT.*",
        ".*UI_FOCUS_AXIS.*",
        ".*HitTestMode.*",
        ".*ArkUI_ModifierKeyName.*",
        ".*ArkUI_InteractionHand.*",
        "OH_ArkUI_UIInputEvent.*",
        "OH_ArkUI_PointerEvent.*",
        "OH_ArkUI_TouchEvent.*",
        "OH_ArkUI_AxisEvent.*",
        "OH_ArkUI_MouseEvent.*",
        "OH_ArkUI_FocusAxisEvent.*",
        "OH_ArkUI_HoverEvent.*",
        "OH_ArkUI_TouchTestInfo.*",
        "OH_ArkUI_CoastingAxisEvent.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["ace_ndk.z"],
    extra: "",
});
