//! Module type::direction wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_Direction, "ArkUI_Direction_ARKUI_DIRECTION_")]
/// Layout direction values used by direction-aware attributes.
pub enum Direction {
    Ltr = 0,
    Rtl,
    Auto = 3,
}

impl From<Direction> for i32 {
    fn from(value: Direction) -> Self {
        value as i32
    }
}
