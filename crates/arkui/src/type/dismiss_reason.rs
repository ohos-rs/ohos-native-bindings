use ohos_arkui_sys::*;

use enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_DismissReason, "ArkUI_DismissReason_DIALOG_DISMISS_")]
pub enum DismissReason {
    BackPress = 0,
    TouchOutside,
    CloseButton,
    SlideDown,
}

impl From<DismissReason> for i32 {
    fn from(value: DismissReason) -> Self {
        value as i32
    }
}
