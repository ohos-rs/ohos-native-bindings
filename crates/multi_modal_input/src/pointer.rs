#[cfg(feature = "api-22")]
use std::ptr::{self, NonNull};

use ohos_multi_modal_input_sys as sys;

#[cfg(feature = "api-22")]
use crate::error::InputError;
use crate::error::{check_status, Result};

/// Current mouse pointer location on a screen (`@since 20`).
#[cfg(feature = "api-20")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointerLocation {
    pub display_id: i32,
    pub x: f64,
    pub y: f64,
}

/// Queries the current mouse pointer location on screen (`@since 20`).
#[cfg(feature = "api-20")]
pub fn pointer_location() -> Result<PointerLocation> {
    let mut display_id = 0;
    let mut x = 0.0;
    let mut y = 0.0;
    check_status(unsafe { sys::OH_Input_GetPointerLocation(&mut display_id, &mut x, &mut y) })?;
    Ok(PointerLocation { display_id, x, y })
}

/// System mouse pointer cursor styles (`@since 22`).
#[cfg(feature = "api-22")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerStyle {
    Default,
    East,
    West,
    South,
    North,
    WestEast,
    NorthSouth,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    NorthEastSouthWest,
    NorthWestSouthEast,
    Cross,
    CursorCopy,
    CursorForbid,
    ColorSucker,
    HandGrabbing,
    HandOpen,
    HandPointing,
    Help,
    Move,
    ResizeLeftRight,
    ResizeUpDown,
    ScreenshotChoose,
    ScreenshotCursor,
    TextCursor,
    ZoomIn,
    ZoomOut,
    MiddleBtnEast,
    MiddleBtnWest,
    MiddleBtnSouth,
    MiddleBtnNorth,
    MiddleBtnNorthSouth,
    MiddleBtnNorthEast,
    MiddleBtnNorthWest,
    MiddleBtnSouthEast,
    MiddleBtnSouthWest,
    MiddleBtnNorthSouthWestEast,
    HorizontalTextCursor,
    CursorCross,
    CursorCircle,
    Loading,
    Running,
    MiddleBtnEastWest,
    RunningLeft,
    RunningRight,
    AechDeveloperDefinedIcon,
    ScreenRecorderCursor,
    LaserCursor,
    LaserCursorDot,
    LaserCursorDotRed,
    DeveloperDefinedIcon,
    Other(i32),
}

#[cfg(feature = "api-22")]
impl PointerStyle {
    pub const fn to_raw(self) -> sys::Input_PointerStyle {
        match self {
            Self::Default => sys::Input_PointerStyle_DEFAULT,
            Self::East => sys::Input_PointerStyle_EAST,
            Self::West => sys::Input_PointerStyle_WEST,
            Self::South => sys::Input_PointerStyle_SOUTH,
            Self::North => sys::Input_PointerStyle_NORTH,
            Self::WestEast => sys::Input_PointerStyle_WEST_EAST,
            Self::NorthSouth => sys::Input_PointerStyle_NORTH_SOUTH,
            Self::NorthEast => sys::Input_PointerStyle_NORTH_EAST,
            Self::NorthWest => sys::Input_PointerStyle_NORTH_WEST,
            Self::SouthEast => sys::Input_PointerStyle_SOUTH_EAST,
            Self::SouthWest => sys::Input_PointerStyle_SOUTH_WEST,
            Self::NorthEastSouthWest => sys::Input_PointerStyle_NORTH_EAST_SOUTH_WEST,
            Self::NorthWestSouthEast => sys::Input_PointerStyle_NORTH_WEST_SOUTH_EAST,
            Self::Cross => sys::Input_PointerStyle_CROSS,
            Self::CursorCopy => sys::Input_PointerStyle_CURSOR_COPY,
            Self::CursorForbid => sys::Input_PointerStyle_CURSOR_FORBID,
            Self::ColorSucker => sys::Input_PointerStyle_COLOR_SUCKER,
            Self::HandGrabbing => sys::Input_PointerStyle_HAND_GRABBING,
            Self::HandOpen => sys::Input_PointerStyle_HAND_OPEN,
            Self::HandPointing => sys::Input_PointerStyle_HAND_POINTING,
            Self::Help => sys::Input_PointerStyle_HELP,
            Self::Move => sys::Input_PointerStyle_MOVE,
            Self::ResizeLeftRight => sys::Input_PointerStyle_RESIZE_LEFT_RIGHT,
            Self::ResizeUpDown => sys::Input_PointerStyle_RESIZE_UP_DOWN,
            Self::ScreenshotChoose => sys::Input_PointerStyle_SCREENSHOT_CHOOSE,
            Self::ScreenshotCursor => sys::Input_PointerStyle_SCREENSHOT_CURSOR,
            Self::TextCursor => sys::Input_PointerStyle_TEXT_CURSOR,
            Self::ZoomIn => sys::Input_PointerStyle_ZOOM_IN,
            Self::ZoomOut => sys::Input_PointerStyle_ZOOM_OUT,
            Self::MiddleBtnEast => sys::Input_PointerStyle_MIDDLE_BTN_EAST,
            Self::MiddleBtnWest => sys::Input_PointerStyle_MIDDLE_BTN_WEST,
            Self::MiddleBtnSouth => sys::Input_PointerStyle_MIDDLE_BTN_SOUTH,
            Self::MiddleBtnNorth => sys::Input_PointerStyle_MIDDLE_BTN_NORTH,
            Self::MiddleBtnNorthSouth => sys::Input_PointerStyle_MIDDLE_BTN_NORTH_SOUTH,
            Self::MiddleBtnNorthEast => sys::Input_PointerStyle_MIDDLE_BTN_NORTH_EAST,
            Self::MiddleBtnNorthWest => sys::Input_PointerStyle_MIDDLE_BTN_NORTH_WEST,
            Self::MiddleBtnSouthEast => sys::Input_PointerStyle_MIDDLE_BTN_SOUTH_EAST,
            Self::MiddleBtnSouthWest => sys::Input_PointerStyle_MIDDLE_BTN_SOUTH_WEST,
            Self::MiddleBtnNorthSouthWestEast => {
                sys::Input_PointerStyle_MIDDLE_BTN_NORTH_SOUTH_WEST_EAST
            }
            Self::HorizontalTextCursor => sys::Input_PointerStyle_HORIZONTAL_TEXT_CURSOR,
            Self::CursorCross => sys::Input_PointerStyle_CURSOR_CROSS,
            Self::CursorCircle => sys::Input_PointerStyle_CURSOR_CIRCLE,
            Self::Loading => sys::Input_PointerStyle_LOADING,
            Self::Running => sys::Input_PointerStyle_RUNNING,
            Self::MiddleBtnEastWest => sys::Input_PointerStyle_MIDDLE_BTN_EAST_WEST,
            Self::RunningLeft => sys::Input_PointerStyle_RUNNING_LEFT,
            Self::RunningRight => sys::Input_PointerStyle_RUNNING_RIGHT,
            Self::AechDeveloperDefinedIcon => sys::Input_PointerStyle_AECH_DEVELOPER_DEFINED_ICON,
            Self::ScreenRecorderCursor => sys::Input_PointerStyle_SCREENRECORDER_CURSOR,
            Self::LaserCursor => sys::Input_PointerStyle_LASER_CURSOR,
            Self::LaserCursorDot => sys::Input_PointerStyle_LASER_CURSOR_DOT,
            Self::LaserCursorDotRed => sys::Input_PointerStyle_LASER_CURSOR_DOT_RED,
            Self::DeveloperDefinedIcon => sys::Input_PointerStyle_DEVELOPER_DEFINED_ICON,
            Self::Other(raw) => raw,
        }
    }

    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::Input_PointerStyle) -> Self {
        match raw {
            sys::Input_PointerStyle_DEFAULT => Self::Default,
            sys::Input_PointerStyle_EAST => Self::East,
            sys::Input_PointerStyle_WEST => Self::West,
            sys::Input_PointerStyle_SOUTH => Self::South,
            sys::Input_PointerStyle_NORTH => Self::North,
            sys::Input_PointerStyle_WEST_EAST => Self::WestEast,
            sys::Input_PointerStyle_NORTH_SOUTH => Self::NorthSouth,
            sys::Input_PointerStyle_NORTH_EAST => Self::NorthEast,
            sys::Input_PointerStyle_NORTH_WEST => Self::NorthWest,
            sys::Input_PointerStyle_SOUTH_EAST => Self::SouthEast,
            sys::Input_PointerStyle_SOUTH_WEST => Self::SouthWest,
            sys::Input_PointerStyle_NORTH_EAST_SOUTH_WEST => Self::NorthEastSouthWest,
            sys::Input_PointerStyle_NORTH_WEST_SOUTH_EAST => Self::NorthWestSouthEast,
            sys::Input_PointerStyle_CROSS => Self::Cross,
            sys::Input_PointerStyle_CURSOR_COPY => Self::CursorCopy,
            sys::Input_PointerStyle_CURSOR_FORBID => Self::CursorForbid,
            sys::Input_PointerStyle_COLOR_SUCKER => Self::ColorSucker,
            sys::Input_PointerStyle_HAND_GRABBING => Self::HandGrabbing,
            sys::Input_PointerStyle_HAND_OPEN => Self::HandOpen,
            sys::Input_PointerStyle_HAND_POINTING => Self::HandPointing,
            sys::Input_PointerStyle_HELP => Self::Help,
            sys::Input_PointerStyle_MOVE => Self::Move,
            sys::Input_PointerStyle_RESIZE_LEFT_RIGHT => Self::ResizeLeftRight,
            sys::Input_PointerStyle_RESIZE_UP_DOWN => Self::ResizeUpDown,
            sys::Input_PointerStyle_SCREENSHOT_CHOOSE => Self::ScreenshotChoose,
            sys::Input_PointerStyle_SCREENSHOT_CURSOR => Self::ScreenshotCursor,
            sys::Input_PointerStyle_TEXT_CURSOR => Self::TextCursor,
            sys::Input_PointerStyle_ZOOM_IN => Self::ZoomIn,
            sys::Input_PointerStyle_ZOOM_OUT => Self::ZoomOut,
            sys::Input_PointerStyle_MIDDLE_BTN_EAST => Self::MiddleBtnEast,
            sys::Input_PointerStyle_MIDDLE_BTN_WEST => Self::MiddleBtnWest,
            sys::Input_PointerStyle_MIDDLE_BTN_SOUTH => Self::MiddleBtnSouth,
            sys::Input_PointerStyle_MIDDLE_BTN_NORTH => Self::MiddleBtnNorth,
            sys::Input_PointerStyle_MIDDLE_BTN_NORTH_SOUTH => Self::MiddleBtnNorthSouth,
            sys::Input_PointerStyle_MIDDLE_BTN_NORTH_EAST => Self::MiddleBtnNorthEast,
            sys::Input_PointerStyle_MIDDLE_BTN_NORTH_WEST => Self::MiddleBtnNorthWest,
            sys::Input_PointerStyle_MIDDLE_BTN_SOUTH_EAST => Self::MiddleBtnSouthEast,
            sys::Input_PointerStyle_MIDDLE_BTN_SOUTH_WEST => Self::MiddleBtnSouthWest,
            sys::Input_PointerStyle_MIDDLE_BTN_NORTH_SOUTH_WEST_EAST => {
                Self::MiddleBtnNorthSouthWestEast
            }
            sys::Input_PointerStyle_HORIZONTAL_TEXT_CURSOR => Self::HorizontalTextCursor,
            sys::Input_PointerStyle_CURSOR_CROSS => Self::CursorCross,
            sys::Input_PointerStyle_CURSOR_CIRCLE => Self::CursorCircle,
            sys::Input_PointerStyle_LOADING => Self::Loading,
            sys::Input_PointerStyle_RUNNING => Self::Running,
            sys::Input_PointerStyle_MIDDLE_BTN_EAST_WEST => Self::MiddleBtnEastWest,
            sys::Input_PointerStyle_RUNNING_LEFT => Self::RunningLeft,
            sys::Input_PointerStyle_RUNNING_RIGHT => Self::RunningRight,
            sys::Input_PointerStyle_AECH_DEVELOPER_DEFINED_ICON => Self::AechDeveloperDefinedIcon,
            sys::Input_PointerStyle_SCREENRECORDER_CURSOR => Self::ScreenRecorderCursor,
            sys::Input_PointerStyle_LASER_CURSOR => Self::LaserCursor,
            sys::Input_PointerStyle_LASER_CURSOR_DOT => Self::LaserCursorDot,
            sys::Input_PointerStyle_LASER_CURSOR_DOT_RED => Self::LaserCursorDotRed,
            sys::Input_PointerStyle_DEVELOPER_DEFINED_ICON => Self::DeveloperDefinedIcon,
            other => Self::Other(other),
        }
    }
}

/// Sets whether the mouse pointer is visible in the current window (`@since 22`).
#[cfg(feature = "api-22")]
pub fn set_pointer_visible(visible: bool) -> Result<()> {
    check_status(unsafe { sys::OH_Input_SetPointerVisible(visible) })
}

/// Sets the mouse pointer style for an application window (`@since 22`).
#[cfg(feature = "api-22")]
pub fn set_pointer_style(window_id: i32, style: PointerStyle) -> Result<()> {
    check_status(unsafe { sys::OH_Input_SetPointerStyle(window_id, style.to_raw()) })
}

/// Queries the mouse pointer style for a window (`window_id = -1` for global) (`@since 22`).
#[cfg(feature = "api-22")]
pub fn pointer_style(window_id: i32) -> Result<PointerStyle> {
    let mut style = 0;
    check_status(unsafe { sys::OH_Input_GetPointerStyle(window_id, &mut style) })?;
    Ok(PointerStyle::from_raw(style))
}

/// Snapshot of the current mouse cursor state (`@since 22`).
#[cfg(feature = "api-22")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorInfo {
    pub visible: bool,
    pub style: Option<PointerStyle>,
    pub size_level: Option<i32>,
    pub color: Option<u32>,
}

#[cfg(feature = "api-22")]
struct OwnedCursorInfo(NonNull<sys::Input_CursorInfo>);

#[cfg(feature = "api-22")]
impl Drop for OwnedCursorInfo {
    fn drop(&mut self) {
        let mut raw = self.0.as_ptr();
        unsafe { sys::OH_Input_CursorInfo_Destroy(&mut raw) };
    }
}

/// Queries the current cursor visibility, style, size level, and ARGB color (`@since 22`).
#[cfg(feature = "api-22")]
pub fn cursor_info() -> Result<CursorInfo> {
    let raw = NonNull::new(unsafe { sys::OH_Input_CursorInfo_Create() })
        .ok_or(InputError::AllocationFailed)?;
    let info = OwnedCursorInfo(raw);
    check_status(unsafe { sys::OH_Input_GetCursorInfo(info.0.as_ptr(), ptr::null_mut()) })?;
    let mut visible = false;
    check_status(unsafe { sys::OH_Input_CursorInfo_IsVisible(info.0.as_ptr(), &mut visible) })?;
    let mut raw_style = 0;
    let style = (unsafe { sys::OH_Input_CursorInfo_GetStyle(info.0.as_ptr(), &mut raw_style) }
        == sys::Input_Result_INPUT_SUCCESS)
        .then_some(PointerStyle::from_raw(raw_style));
    let mut raw_size = 0;
    let size_level =
        (unsafe { sys::OH_Input_CursorInfo_GetSizeLevel(info.0.as_ptr(), &mut raw_size) }
            == sys::Input_Result_INPUT_SUCCESS)
            .then_some(raw_size);
    let mut raw_color = 0;
    let color = (unsafe { sys::OH_Input_CursorInfo_GetColor(info.0.as_ptr(), &mut raw_color) }
        == sys::Input_Result_INPUT_SUCCESS)
        .then_some(raw_color);
    Ok(CursorInfo {
        visible,
        style,
        size_level,
        color,
    })
}

#[cfg(all(feature = "api-22", feature = "image"))]
struct OwnedCustomCursor(NonNull<sys::Input_CustomCursor>);

#[cfg(all(feature = "api-22", feature = "image"))]
impl Drop for OwnedCustomCursor {
    fn drop(&mut self) {
        let mut raw = self.0.as_ptr();
        unsafe { sys::OH_Input_CustomCursor_Destroy(&mut raw) };
    }
}

#[cfg(all(feature = "api-22", feature = "image"))]
struct OwnedCursorConfig(NonNull<sys::Input_CursorConfig>);

#[cfg(all(feature = "api-22", feature = "image"))]
impl Drop for OwnedCursorConfig {
    fn drop(&mut self) {
        let mut raw = self.0.as_ptr();
        unsafe { sys::OH_Input_CursorConfig_Destroy(&mut raw) };
    }
}

/// Sets a custom bitmap cursor for an application window (`@since 22`, requires `image` feature).
#[cfg(all(feature = "api-22", feature = "image"))]
pub fn set_custom_cursor(
    window_id: i32,
    pixel_map: &ohos_image_native_binding::PixelMap,
    anchor_x: i32,
    anchor_y: i32,
    follow_system: bool,
) -> Result<()> {
    let cursor = NonNull::new(unsafe {
        sys::OH_Input_CustomCursor_Create(pixel_map.as_raw().cast(), anchor_x, anchor_y)
    })
    .ok_or(InputError::AllocationFailed)?;
    let cursor = OwnedCustomCursor(cursor);
    let config = NonNull::new(unsafe { sys::OH_Input_CursorConfig_Create(follow_system) })
        .ok_or(InputError::AllocationFailed)?;
    let config = OwnedCursorConfig(config);
    check_status(unsafe {
        sys::OH_Input_SetCustomCursor(window_id, cursor.0.as_ptr(), config.0.as_ptr())
    })
}

#[cfg(all(test, feature = "api-22"))]
mod tests {
    use super::*;

    #[test]
    fn pointer_style_round_trips() {
        for style in [
            PointerStyle::Default,
            PointerStyle::HandPointing,
            PointerStyle::TextCursor,
            PointerStyle::WestEast,
            PointerStyle::NorthSouth,
            PointerStyle::Move,
            PointerStyle::CursorForbid,
            PointerStyle::DeveloperDefinedIcon,
            PointerStyle::Other(999),
        ] {
            assert_eq!(PointerStyle::from_raw(style.to_raw()), style);
        }
    }
}
