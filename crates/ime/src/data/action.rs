use ohos_input_method_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    InputMethod_ExtendAction,
    "InputMethod_ExtendAction_IME_EXTEND_ACTION_"
)]
pub enum Action {
    SelectAll,
    Cut,
    Copy,
    Paste,
}
