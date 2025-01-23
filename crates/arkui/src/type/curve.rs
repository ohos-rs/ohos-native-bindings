use ohos_arkui_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_AnimationCurve, "ArkUI_AnimationCurve_ARKUI_CURVE_")]
pub enum Curve {
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    FastOutSlowIn,
    LinearOutSlowIn,
    FastOutLinearIn,
    ExtremeDeceleration,
    Sharp,
    Rhythm,
    Smooth,
    Friction,
}
