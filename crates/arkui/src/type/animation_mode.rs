use ohos_arkui_sys::*;

use enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    ArkUI_AnimationPlayMode,
    "ArkUI_AnimationPlayMode_ARKUI_ANIMATION_PLAY_MODE_"
)]
pub enum AnimationMode {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}
