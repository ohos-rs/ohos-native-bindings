#[cfg(feature = "api-22")]
use std::ptr::{self, NonNull};

#[cfg(feature = "api-22")]
use ohos_enum_derive::EnumFrom;
use ohos_multi_modal_input_sys as sys;
#[cfg(feature = "api-22")]
use ohos_multi_modal_input_sys::*;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(Input_PointerStyle, "Input_PointerStyle_")]
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
    #[suffix("SCREENRECORDER_CURSOR")]
    ScreenRecorderCursor,
    LaserCursor,
    LaserCursorDot,
    LaserCursorDotRed,
    DeveloperDefinedIcon,
}

/// Sets whether the mouse pointer is visible in the current window (`@since 22`).
#[cfg(feature = "api-22")]
pub fn set_pointer_visible(visible: bool) -> Result<()> {
    check_status(unsafe { sys::OH_Input_SetPointerVisible(visible) })
}

/// Sets the mouse pointer style for an application window (`@since 22`).
#[cfg(feature = "api-22")]
pub fn set_pointer_style(window_id: i32, style: PointerStyle) -> Result<()> {
    check_status(unsafe { sys::OH_Input_SetPointerStyle(window_id, style.into()) })
}

/// Queries the mouse pointer style for a window (`window_id = -1` for global) (`@since 22`).
#[cfg(feature = "api-22")]
pub fn pointer_style(window_id: i32) -> Result<PointerStyle> {
    let mut style = 0;
    check_status(unsafe { sys::OH_Input_GetPointerStyle(window_id, &mut style) })?;
    PointerStyle::try_from_raw(style)
        .ok_or_else(|| InputError::CommonError(format!("unknown pointer style {style}")))
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
    let raw = NonNull::new(unsafe { sys::OH_Input_CursorInfo_Create() }).ok_or_else(|| {
        InputError::CommonError("OH_Input_CursorInfo_Create returned null".into())
    })?;
    let info = OwnedCursorInfo(raw);
    check_status(unsafe { sys::OH_Input_GetCursorInfo(info.0.as_ptr(), ptr::null_mut()) })?;
    let mut visible = false;
    check_status(unsafe { sys::OH_Input_CursorInfo_IsVisible(info.0.as_ptr(), &mut visible) })?;
    let mut raw_style = 0;
    let style = (unsafe { sys::OH_Input_CursorInfo_GetStyle(info.0.as_ptr(), &mut raw_style) }
        == sys::Input_Result_INPUT_SUCCESS)
        .then_some(raw_style)
        .and_then(PointerStyle::try_from_raw);
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
    .ok_or_else(|| InputError::CommonError("OH_Input_CustomCursor_Create returned null".into()))?;
    let cursor = OwnedCustomCursor(cursor);
    let config = NonNull::new(unsafe { sys::OH_Input_CursorConfig_Create(follow_system) })
        .ok_or_else(|| {
            InputError::CommonError("OH_Input_CursorConfig_Create returned null".into())
        })?;
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
            PointerStyle::ScreenRecorderCursor,
            PointerStyle::DeveloperDefinedIcon,
        ] {
            let raw: sys::Input_PointerStyle = style.into();
            assert_eq!(PointerStyle::try_from_raw(raw), Some(style));
        }
        assert_eq!(PointerStyle::try_from_raw(999), None);
    }
}
