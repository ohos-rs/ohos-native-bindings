//! Module type::animation_finish_type wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_FinishCallbackType,
    "ArkUI_FinishCallbackType_ARKUI_FINISH_CALLBACK_"
)]
/// Why an animation finished.
pub enum AnimationFinishCallbackType {
    Removed,
    Logically,
}
