use ohos_arkui_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_GesturePriority, "ArkUI_GesturePriority_")]
pub enum GesturePriority {
    Normal,
    Priority,
    Parallel,
}
