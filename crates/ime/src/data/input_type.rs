use ohos_input_method_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
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
