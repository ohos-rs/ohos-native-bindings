use ohos_input_method_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    InputMethod_KeyboardStatus,
    "InputMethod_KeyboardStatus_IME_KEYBOARD_STATUS_"
)]
pub enum KeyboardStatus {
    None,
    Hide,
    Show,
}
