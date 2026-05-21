//! Module type::text_alignment wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_TextAlignment, "ArkUI_TextAlignment_ARKUI_TEXT_ALIGNMENT_", i32)]
/// Horizontal text alignment options.
pub enum TextAlignment {
    Start = 0,
    Center,
    End,
    Justify,
}
