use ohos_arkui_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_Alignment, "ArkUI_Alignment_ARKUI_ALIGNMENT_")]
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

impl From<Alignment> for i32 {
    fn from(value: Alignment) -> Self {
        value as i32
    }
}
