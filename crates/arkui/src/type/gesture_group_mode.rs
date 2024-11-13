use ohos_arkui_sys::*;

use enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_GroupGestureMode, "ArkUI_GroupGestureMode_")]
pub enum GestureGroupMode {
    SequentialGroup,
    ParallelGroup,
    ExclusiveGroup,
}
