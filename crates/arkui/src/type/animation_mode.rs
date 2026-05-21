//! Module type::animation_mode wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_AnimationPlayMode,
    "ArkUI_AnimationPlayMode_ARKUI_ANIMATION_PLAY_MODE_"
)]
/// Playback direction pattern for time-based animations.
pub enum AnimationMode {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}
