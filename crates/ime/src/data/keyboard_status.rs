use ohos_input_method_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    InputMethod_KeyboardStatus,
    "InputMethod_KeyboardStatus_IME_KEYBOARD_STATUS_"
)]
pub enum KeyboardStatus {
    None,
    Hide,
    Show,
}
