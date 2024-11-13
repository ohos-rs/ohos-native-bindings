use ohos_arkui_sys::*;

use enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_GestureMask, "ArkUI_GestureMask_")]
pub enum GestureMask {
    NormalGestureMask,
    IgnoreInternalGestureMask,
}
