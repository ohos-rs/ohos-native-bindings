//! Module type::alignment wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_Alignment, "ArkUI_Alignment_ARKUI_ALIGNMENT_", i32)]
/// Logical alignment values used by layout and text-related attributes.
pub enum Alignment {
    TopStart = 0,
    Top,
    TopEnd,
    Start,
    Center,
    End,
    BottomStart,
    Bottom,
    BottomEnd,
}
