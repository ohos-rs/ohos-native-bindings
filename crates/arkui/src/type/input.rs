//! API 12 text editor configuration enums.

use ohos_arkui_sys::*;
use ohos_enum_derive::EnumFrom;

/// API-12 semantic autofill hints, shared by TextInput and TextArea.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_TextInputContentType,
    "ArkUI_TextInputContentType_ARKUI_TEXTINPUT_CONTENT_TYPE_",
    i32
)]
pub enum TextContentType {
    UserName,
    Password,
    NewPassword,
    FullStreetAddress,
    HouseNumber,
    DistrictAddress,
    CityAddress,
    ProvinceAddress,
    CountryAddress,
    PersonFullName,
    PersonLastName,
    PersonFirstName,
    PhoneNumber,
    PhoneCountryCode,
    FullPhoneNumber,
    #[prefix("ArkUI_TextInputContentType_ARKUI_TEXTINPUT_CONTENT_")]
    EmailAddress,
    BankCardNumber,
    IdCardNumber,
    Nickname,
    DetailInfoWithoutStreet,
    FormatAddress,
}

/// Keyboard and secure-entry mode of a single-line editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_TextInputType, "ArkUI_TextInputType_ARKUI_TEXTINPUT_TYPE_", i32)]
pub enum TextInputType {
    /// General text.
    Normal,
    /// Digits.
    Number,
    /// Telephone number.
    PhoneNumber,
    /// E-mail address.
    Email,
    /// Secure text.
    Password,
    /// Secure digits.
    NumberPassword,
    /// Lock-screen password.
    ScreenLockPassword,
    /// Account name.
    UserName,
    /// New password.
    NewPassword,
    /// Decimal number.
    NumberDecimal,
}

/// Keyboard mode of a multiline editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_TextAreaType, "ArkUI_TextAreaType_ARKUI_TEXTAREA_TYPE_", i32)]
pub enum TextAreaType {
    /// General text.
    Normal,
    /// Digits.
    Number,
    /// Telephone number.
    PhoneNumber,
    /// E-mail address.
    Email,
}

/// Action shown on the software keyboard's return key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_EnterKeyType, "ArkUI_EnterKeyType_ARKUI_ENTER_KEY_TYPE_", i32)]
pub enum EnterKeyType {
    /// Open or navigate.
    Go,
    /// Search.
    Search,
    /// Send.
    Send,
    /// Advance focus.
    Next,
    /// Finish editing.
    Done,
    /// Return to the previous field.
    Previous,
    /// Insert a newline.
    NewLine,
}
