use ohos_arkui_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_TextAlignment, "ArkUI_TextAlignment_ARKUI_TEXT_ALIGNMENT_")]
pub enum TextAlignment {
    Start = 0,
    Center,
    End,
    Justify,
}

impl From<TextAlignment> for i32 {
    fn from(value: TextAlignment) -> Self {
        value as i32
    }
}
