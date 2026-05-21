//! Module type::scroll wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_ScrollBarDisplayMode,
    "ArkUI_ScrollBarDisplayMode_ARKUI_SCROLL_BAR_DISPLAY_MODE_",
    i32
)]
/// Scrollbar visibility policy for scrollable components.
pub enum ScrollBarDisplayMode {
    Off = 0,
    Auto,
    On,
}
