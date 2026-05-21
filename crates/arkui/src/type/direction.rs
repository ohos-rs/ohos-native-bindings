//! Module type::direction wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_Direction, "ArkUI_Direction_ARKUI_DIRECTION_", i32)]
/// Layout direction values used by direction-aware attributes.
pub enum Direction {
    Ltr = 0,
    Rtl,
    Auto = 3,
}
