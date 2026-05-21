//! Module type::gesture_group_mode wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_GroupGestureMode, "ArkUI_GroupGestureMode_")]
/// Combination rule applied inside a gesture group.
pub enum GestureGroupMode {
    SequentialGroup,
    ParallelGroup,
    ExclusiveGroup,
}
