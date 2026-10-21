//! API 12 hit-testing and accessibility policies.

use ohos_arkui_sys::*;
use ohos_enum_derive::EnumFrom;

/// Native hit-test behavior of a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_HitTestMode, "ArkUI_HitTestMode_ARKUI_HIT_TEST_MODE_", i32)]
pub enum HitTestMode {
    /// Test this component and its children, blocking siblings behind it.
    Default,
    /// Test this component and block child/sibling hit tests.
    Block,
    /// Test this component and its children without blocking siblings.
    Transparent,
    /// Skip this component while allowing its children and ancestors.
    None,
}

/// How a component participates in accessibility traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_AccessibilityMode,
    "ArkUI_AccessibilityMode_ARKUI_ACCESSIBILITY_MODE_",
    i32
)]
pub enum AccessibilityMode {
    /// Let the component choose its normal policy.
    Auto,
    /// Expose this component.
    Enabled,
    /// Skip this component.
    Disabled,
    /// Skip this component and all descendants.
    DisabledForDescendants,
}

/// API 12 component blend modes (distinct from the drawing-canvas enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_BlendMode, "ArkUI_BlendMode_ARKUI_BLEND_MODE_", i32)]
#[allow(missing_docs)]
pub enum BlendMode {
    None,
    Clear,
    Src,
    Dst,
    SrcOver,
    DstOver,
    SrcIn,
    DstIn,
    SrcOut,
    DstOut,
    SrcAtop,
    DstAtop,
    Xor,
    Plus,
    Modulate,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Multiply,
    Hue,
    Saturation,
    Color,
    Luminosity,
}
