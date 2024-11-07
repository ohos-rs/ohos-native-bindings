use ohos_arkui_sys::*;

use enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    ArkUI_ScrollBarDisplayMode,
    "ArkUI_ScrollBarDisplayMode_ARKUI_SCROLL_BAR_DISPLAY_MODE_"
)]
pub enum ScrollBarDisplayMode {
    Off = 0,
    Auto,
    On,
}

impl From<ScrollBarDisplayMode> for i32 {
    fn from(value: ScrollBarDisplayMode) -> Self {
        value as i32
    }
}
