//! Module type::gesture_priority wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_GesturePriority, "ArkUI_GesturePriority_")]
/// Priority mode for concurrent gesture recognizers.
pub enum GesturePriority {
    Normal,
    Priority,
    Parallel,
}
