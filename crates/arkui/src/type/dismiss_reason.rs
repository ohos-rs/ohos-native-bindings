//! Module type::dismiss_reason wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_DismissReason, "ArkUI_DismissReason_DIALOG_DISMISS_", i32)]
/// Reason reported when a dialog is dismissed.
pub enum DismissReason {
    BackPress = 0,
    TouchOutside,
    CloseButton,
    SlideDown,
}
