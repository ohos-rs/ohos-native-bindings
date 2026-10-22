//! Module type::scroll wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

/// Source of an API-12 will-scroll callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_ScrollSource, "ArkUI_ScrollSource_ARKUI_SCROLL_SOURCE_", i32)]
pub enum ScrollSource {
    /// Finger dragging.
    Drag,
    /// Inertia after finger dragging.
    Fling,
    /// Overscroll spring animation.
    EdgeEffect,
    /// Mouse wheel or keyboard input.
    OtherUserInput,
    /// Scrollbar dragging.
    ScrollBar,
    /// Inertia after scrollbar dragging.
    ScrollBarFling,
    /// Immediate controller command.
    Scroller,
    /// Animated controller command.
    Animation,
}

/// Axis accepted by a native scroll container (API 12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_ScrollDirection,
    "ArkUI_ScrollDirection_ARKUI_SCROLL_DIRECTION_",
    i32
)]
pub enum ScrollDirection {
    /// Vertical scrolling.
    Vertical,
    /// Horizontal scrolling.
    Horizontal,
    /// Scrolling disabled.
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_ScrollBarDisplayMode,
    "ArkUI_ScrollBarDisplayMode_ARKUI_SCROLL_BAR_DISPLAY_MODE_",
    i32
)]
/// Scrollbar visibility policy for scrollable components.
pub enum ScrollBarDisplayMode {
    Off = 0,
    Auto,
    On,
}
