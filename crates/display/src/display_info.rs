#![allow(non_upper_case_globals)]

use std::ptr;

use ohos_display_sys::{
    NativeDisplayManager_DisplayColorSpace, NativeDisplayManager_DisplayHdrFormat,
    NativeDisplayManager_DisplayInfo, NativeDisplayManager_DisplayState,
    NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_DOZE,
    NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_DOZE_SUSPEND,
    NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_OFF,
    NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_ON,
    NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_ON_SUSPEND,
    NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_UNKNOWN,
    NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_VR,
    NativeDisplayManager_DisplaysInfo, OH_NativeDisplayManager_CreateAllDisplays,
    OH_NativeDisplayManager_CreateDisplayById, OH_NativeDisplayManager_CreatePrimaryDisplay,
    OH_NativeDisplayManager_DestroyAllDisplays, OH_NativeDisplayManager_DestroyDisplay,
};

#[cfg(feature = "api-20")]
use ohos_display_sys::{
    NativeDisplayManager_Rect, NativeDisplayManager_SourceMode,
    NativeDisplayManager_SourceMode_DISPLAY_SOURCE_MODE_ALONE,
    NativeDisplayManager_SourceMode_DISPLAY_SOURCE_MODE_EXTEND,
    NativeDisplayManager_SourceMode_DISPLAY_SOURCE_MODE_MAIN,
    NativeDisplayManager_SourceMode_DISPLAY_SOURCE_MODE_MIRROR,
    NativeDisplayManager_SourceMode_DISPLAY_SOURCE_MODE_NONE,
    OH_NativeDisplayManager_CreateAvailableArea, OH_NativeDisplayManager_DestroyAvailableArea,
    OH_NativeDisplayManager_GetDisplayPosition, OH_NativeDisplayManager_GetDisplaySourceMode,
};

#[cfg(feature = "api-20")]
use crate::cut_info::Rect;
use crate::{
    error::{check_status, DisplayError, Result},
    orientation::Orientation,
    rotation::Rotation,
};

/// Power/activity state of a display (`@since 14`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayState {
    Unknown,
    Off,
    On,
    Doze,
    DozeSuspend,
    Vr,
    OnSuspend,
    Other(u32),
}

impl From<NativeDisplayManager_DisplayState> for DisplayState {
    fn from(value: NativeDisplayManager_DisplayState) -> Self {
        match value {
            NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_UNKNOWN => {
                Self::Unknown
            }
            NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_OFF => Self::Off,
            NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_ON => Self::On,
            NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_DOZE => Self::Doze,
            NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_DOZE_SUSPEND => {
                Self::DozeSuspend
            }
            NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_VR => Self::Vr,
            NativeDisplayManager_DisplayState_DISPLAY_MANAGER_DISPLAY_STATE_ON_SUSPEND => {
                Self::OnSuspend
            }
            other => Self::Other(other),
        }
    }
}

/// Owned snapshot of display metadata, HDR formats, and color spaces (`@since 14`).
#[derive(Clone, Debug, PartialEq)]
pub struct DisplayInfo {
    pub id: u32,
    pub name: String,
    pub is_alive: bool,
    pub width: i32,
    pub height: i32,
    pub physical_width: i32,
    pub physical_height: i32,
    pub refresh_rate: u32,
    pub available_width: u32,
    pub available_height: u32,
    pub density_dpi: f32,
    pub density_pixels: f32,
    pub scaled_density: f32,
    pub x_dpi: f32,
    pub y_dpi: f32,
    pub rotation: Rotation,
    pub state: DisplayState,
    pub orientation: Orientation,
    pub hdr_formats: Vec<u32>,
    pub color_spaces: Vec<u32>,
}

impl DisplayInfo {
    pub(crate) unsafe fn from_raw(raw: &NativeDisplayManager_DisplayInfo) -> Self {
        let name_bytes: &[u8] =
            unsafe { std::slice::from_raw_parts(raw.name.as_ptr().cast::<u8>(), raw.name.len()) };
        let name_end = name_bytes
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(name_bytes.len());
        let name = String::from_utf8_lossy(&name_bytes[..name_end]).into_owned();

        let hdr_formats = unsafe { copy_hdr_formats(raw.hdrFormat) };
        let color_spaces = unsafe { copy_color_spaces(raw.colorSpace) };

        Self {
            id: raw.id,
            name,
            is_alive: raw.isAlive,
            width: raw.width,
            height: raw.height,
            physical_width: raw.physicalWidth,
            physical_height: raw.physicalHeight,
            refresh_rate: raw.refreshRate,
            available_width: raw.availableWidth,
            available_height: raw.availableHeight,
            density_dpi: raw.densityDPI,
            density_pixels: raw.densityPixels,
            scaled_density: raw.scaledDensity,
            x_dpi: raw.xDPI,
            y_dpi: raw.yDPI,
            rotation: Rotation::from(raw.rotation),
            state: DisplayState::from(raw.state),
            orientation: Orientation::from(raw.orientation),
            hdr_formats,
            color_spaces,
        }
    }
}

unsafe fn copy_hdr_formats(ptr: *const NativeDisplayManager_DisplayHdrFormat) -> Vec<u32> {
    if ptr.is_null() {
        return Vec::new();
    }
    let raw = unsafe { &*ptr };
    if raw.hdrFormatLength == 0 || raw.hdrFormats.is_null() {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(raw.hdrFormats, raw.hdrFormatLength as usize) }.to_vec()
}

unsafe fn copy_color_spaces(ptr: *const NativeDisplayManager_DisplayColorSpace) -> Vec<u32> {
    if ptr.is_null() {
        return Vec::new();
    }
    let raw = unsafe { &*ptr };
    if raw.colorSpaceLength == 0 || raw.colorSpaces.is_null() {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(raw.colorSpaces, raw.colorSpaceLength as usize) }.to_vec()
}

struct OwnedDisplayPtr(*mut NativeDisplayManager_DisplayInfo);

impl Drop for OwnedDisplayPtr {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { OH_NativeDisplayManager_DestroyDisplay(self.0) };
        }
    }
}

struct OwnedDisplaysPtr(*mut NativeDisplayManager_DisplaysInfo);

impl Drop for OwnedDisplaysPtr {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { OH_NativeDisplayManager_DestroyAllDisplays(self.0) };
        }
    }
}

/// Queries the primary display (`@since 14`).
pub fn primary_display() -> Result<DisplayInfo> {
    let mut raw = ptr::null_mut();
    check_status(unsafe { OH_NativeDisplayManager_CreatePrimaryDisplay(&mut raw) })?;
    let guard = OwnedDisplayPtr(raw);
    if guard.0.is_null() {
        return Err(DisplayError::UnexpectedNull);
    }
    Ok(unsafe { DisplayInfo::from_raw(&*guard.0) })
}

/// Queries a display by its numeric ID (`@since 14`).
pub fn display_by_id(display_id: u32) -> Result<DisplayInfo> {
    let mut raw = ptr::null_mut();
    check_status(unsafe { OH_NativeDisplayManager_CreateDisplayById(display_id, &mut raw) })?;
    let guard = OwnedDisplayPtr(raw);
    if guard.0.is_null() {
        return Err(DisplayError::UnexpectedNull);
    }
    Ok(unsafe { DisplayInfo::from_raw(&*guard.0) })
}

/// Queries all connected displays (`@since 14`).
pub fn all_displays() -> Result<Vec<DisplayInfo>> {
    let mut raw = ptr::null_mut();
    check_status(unsafe { OH_NativeDisplayManager_CreateAllDisplays(&mut raw) })?;
    let guard = OwnedDisplaysPtr(raw);
    if guard.0.is_null() {
        return Err(DisplayError::UnexpectedNull);
    }
    let list = unsafe { &*guard.0 };
    if list.displaysLength == 0 {
        return Ok(Vec::new());
    }
    if list.displaysInfo.is_null() {
        return Err(DisplayError::UnexpectedNull);
    }
    let slice =
        unsafe { std::slice::from_raw_parts(list.displaysInfo, list.displaysLength as usize) };
    Ok(slice
        .iter()
        .map(|item| unsafe { DisplayInfo::from_raw(item) })
        .collect())
}

/// Display source mode (`@since 20`).
#[cfg(feature = "api-20")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceMode {
    None,
    Main,
    Mirror,
    Extend,
    Alone,
    Other(u32),
}

#[cfg(feature = "api-20")]
impl From<NativeDisplayManager_SourceMode> for SourceMode {
    fn from(value: NativeDisplayManager_SourceMode) -> Self {
        match value {
            NativeDisplayManager_SourceMode_DISPLAY_SOURCE_MODE_NONE => Self::None,
            NativeDisplayManager_SourceMode_DISPLAY_SOURCE_MODE_MAIN => Self::Main,
            NativeDisplayManager_SourceMode_DISPLAY_SOURCE_MODE_MIRROR => Self::Mirror,
            NativeDisplayManager_SourceMode_DISPLAY_SOURCE_MODE_EXTEND => Self::Extend,
            NativeDisplayManager_SourceMode_DISPLAY_SOURCE_MODE_ALONE => Self::Alone,
            other => Self::Other(other),
        }
    }
}

/// Queries the available area of a display (`@since 20`).
#[cfg(feature = "api-20")]
pub fn display_available_area(display_id: u64) -> Result<Rect> {
    let mut raw: *mut NativeDisplayManager_Rect = ptr::null_mut();
    check_status(unsafe { OH_NativeDisplayManager_CreateAvailableArea(display_id, &mut raw) })?;
    if raw.is_null() {
        return Err(DisplayError::UnexpectedNull);
    }
    let rect = unsafe { *raw };
    let destroy = unsafe { OH_NativeDisplayManager_DestroyAvailableArea(raw) };
    check_status(destroy)?;
    Ok(Rect {
        left: rect.left,
        top: rect.top,
        width: rect.width,
        height: rect.height,
    })
}

/// Queries the origin `(x, y)` of a display relative to the main screen (`@since 20`).
#[cfg(feature = "api-20")]
pub fn display_position(display_id: u64) -> Result<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    check_status(unsafe {
        OH_NativeDisplayManager_GetDisplayPosition(display_id, &mut x, &mut y)
    })?;
    Ok((x, y))
}

/// Queries the source mode of a display (`@since 20`).
#[cfg(feature = "api-20")]
pub fn display_source_mode(display_id: u64) -> Result<SourceMode> {
    let mut mode = 0;
    check_status(unsafe { OH_NativeDisplayManager_GetDisplaySourceMode(display_id, &mut mode) })?;
    Ok(SourceMode::from(mode))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_display_info_with_color_spaces_and_hdr_formats() {
        let mut hdr = [1u32, 2u32];
        let mut hdr_struct = NativeDisplayManager_DisplayHdrFormat {
            hdrFormatLength: hdr.len() as u32,
            hdrFormats: hdr.as_mut_ptr(),
        };
        let mut cs = [3u32, 4u32];
        let mut cs_struct = NativeDisplayManager_DisplayColorSpace {
            colorSpaceLength: cs.len() as u32,
            colorSpaces: cs.as_mut_ptr(),
        };
        let mut name = [0 as std::os::raw::c_char; 33];
        for (dst, &src) in name.iter_mut().zip(b"Built-in Screen".iter()) {
            *dst = src as std::os::raw::c_char;
        }
        let raw = NativeDisplayManager_DisplayInfo {
            id: 0,
            name,
            isAlive: true,
            width: 2880,
            height: 1920,
            physicalWidth: 2880,
            physicalHeight: 1920,
            refreshRate: 120,
            availableWidth: 2880,
            availableHeight: 1840,
            densityDPI: 320.0,
            densityPixels: 2.0,
            scaledDensity: 2.0,
            xDPI: 264.0,
            yDPI: 264.0,
            rotation: 0,
            state: 2,
            orientation: 1,
            hdrFormat: &mut hdr_struct,
            colorSpace: &mut cs_struct,
        };

        let info = unsafe { DisplayInfo::from_raw(&raw) };
        assert_eq!(info.id, 0);
        assert_eq!(info.name, "Built-in Screen");
        assert!(info.is_alive);
        assert_eq!((info.width, info.height), (2880, 1920));
        assert_eq!(info.state, DisplayState::On);
        assert_eq!(info.orientation, Orientation::Landscape);
        assert_eq!(info.rotation, Rotation::Rotation0);
        assert_eq!(info.hdr_formats, vec![1, 2]);
        assert_eq!(info.color_spaces, vec![3, 4]);
    }

    #[test]
    fn handles_null_optional_arrays_and_unterminated_name() {
        let name = [b'X' as std::os::raw::c_char; 33];
        let raw = NativeDisplayManager_DisplayInfo {
            id: 7,
            name,
            isAlive: false,
            width: 1920,
            height: 1080,
            physicalWidth: 1920,
            physicalHeight: 1080,
            refreshRate: 60,
            availableWidth: 1920,
            availableHeight: 1080,
            densityDPI: 160.0,
            densityPixels: 1.0,
            scaledDensity: 1.0,
            xDPI: 160.0,
            yDPI: 160.0,
            rotation: 1,
            state: 99,
            orientation: 0,
            hdrFormat: ptr::null_mut(),
            colorSpace: ptr::null_mut(),
        };

        let info = unsafe { DisplayInfo::from_raw(&raw) };
        assert_eq!(info.id, 7);
        assert_eq!(info.name.len(), 33);
        assert_eq!(info.state, DisplayState::Other(99));
        assert!(info.hdr_formats.is_empty());
        assert!(info.color_spaces.is_empty());
    }
}
