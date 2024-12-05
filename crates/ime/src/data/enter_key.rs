use ohos_input_method_sys::*;

use enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(InputMethod_EnterKeyType, "InputMethod_EnterKeyType_IME_ENTER_KEY_")]
pub enum EnterKey {
    Unspecified,
    None,
    Go,
    Search,
    Send,
    Next,
    Done,
    Previous,
    Newline,
}
