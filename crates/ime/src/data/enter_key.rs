use ohos_input_method_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(InputMethod_EnterKeyType, "InputMethod_EnterKeyType_IME_ENTER_KEY_")]
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
