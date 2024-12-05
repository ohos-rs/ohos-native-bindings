use ohos_input_method_sys::*;

use enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    InputMethod_TextInputType,
    "InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_"
)]
pub enum InputType {
    None,
    Text,
    Multiline,
    Number,
    Phone,
    Datetime,
    EmailAddress,
    Url,
    VisiblePassword,
    NumberPassword,
    ScreenLockPassword,
    UserName,
    NewPassword,
    NumberDecimal,
}
