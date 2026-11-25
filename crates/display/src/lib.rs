pub use cut_info::{DisplayCutInfo, Rect, WaterfallDisplayAreaRect};
#[cfg(feature = "api-14")]
pub use display_info::{all_displays, display_by_id, primary_display, DisplayInfo, DisplayState};
#[cfg(feature = "api-20")]
pub use display_info::{display_available_area, display_position, display_source_mode, SourceMode};
use error::check_status;
pub use error::{DisplayError, Result};
#[cfg(feature = "api-20")]
pub use event::{on_available_area_change, on_display_add, on_display_remove};
pub use event::{on_display_change, on_fold_display_mode_change, DisplayListener};
pub use fold_mode::FoldMode;
use ohos_display_sys::{
    NativeDisplayManager_CutoutInfo, NativeDisplayManager_ErrorCode,
    OH_NativeDisplayManager_CreateDefaultDisplayCutoutInfo,
    OH_NativeDisplayManager_DestroyDefaultDisplayCutoutInfo,
    OH_NativeDisplayManager_GetDefaultDisplayDensityDpi,
    OH_NativeDisplayManager_GetDefaultDisplayDensityPixels,
    OH_NativeDisplayManager_GetDefaultDisplayDensityXdpi,
    OH_NativeDisplayManager_GetDefaultDisplayDensityYdpi,
    OH_NativeDisplayManager_GetDefaultDisplayHeight, OH_NativeDisplayManager_GetDefaultDisplayId,
    OH_NativeDisplayManager_GetDefaultDisplayOrientation,
    OH_NativeDisplayManager_GetDefaultDisplayRefreshRate,
    OH_NativeDisplayManager_GetDefaultDisplayRotation,
    OH_NativeDisplayManager_GetDefaultDisplayScaledDensity,
    OH_NativeDisplayManager_GetDefaultDisplayVirtualPixelRatio,
    OH_NativeDisplayManager_GetDefaultDisplayWidth, OH_NativeDisplayManager_GetFoldDisplayMode,
    OH_NativeDisplayManager_IsFoldable,
};
pub use orientation::Orientation;
pub use rotation::Rotation;

mod cut_info;
#[cfg(feature = "api-14")]
mod display_info;
mod error;
mod event;
mod fold_mode;
mod orientation;
mod rotation;

struct CutoutGuard<
    D: FnOnce(*mut NativeDisplayManager_CutoutInfo) -> NativeDisplayManager_ErrorCode,
> {
    ptr: *mut NativeDisplayManager_CutoutInfo,
    destroy: Option<D>,
}

impl<D: FnOnce(*mut NativeDisplayManager_CutoutInfo) -> NativeDisplayManager_ErrorCode>
    CutoutGuard<D>
{
    fn new(ptr: *mut NativeDisplayManager_CutoutInfo, destroy: D) -> Self {
        Self {
            ptr,
            destroy: Some(destroy),
        }
    }

    fn finish(mut self) -> Result<()> {
        if let (false, Some(destroy)) = (self.ptr.is_null(), self.destroy.take()) {
            return check_status(destroy(self.ptr));
        }
        Ok(())
    }
}

impl<D: FnOnce(*mut NativeDisplayManager_CutoutInfo) -> NativeDisplayManager_ErrorCode> Drop
    for CutoutGuard<D>
{
    fn drop(&mut self) {
        if let (false, Some(destroy)) = (self.ptr.is_null(), self.destroy.take()) {
            let _ = destroy(self.ptr);
        }
    }
}

fn read_cutout_with<C, D>(create: C, destroy: D) -> Result<DisplayCutInfo>
where
    C: FnOnce(&mut *mut NativeDisplayManager_CutoutInfo) -> NativeDisplayManager_ErrorCode,
    D: FnOnce(*mut NativeDisplayManager_CutoutInfo) -> NativeDisplayManager_ErrorCode,
{
    let mut info = std::ptr::null_mut();
    check_status(create(&mut info))?;
    if info.is_null() {
        return Err(DisplayError::UnexpectedNull);
    }
    let guard = CutoutGuard::new(info, destroy);
    // SAFETY: `info` is non-null and initialized by `create` on success.
    let cut_info = DisplayCutInfo::from(unsafe { *guard.ptr });
    guard.finish()?;
    Ok(cut_info)
}

fn query_default<T: Default>(
    f: impl FnOnce(&mut T) -> NativeDisplayManager_ErrorCode,
) -> Result<T> {
    let mut value = T::default();
    check_status(f(&mut value))?;
    Ok(value)
}

pub fn default_display_cutout_info() -> Result<DisplayCutInfo> {
    read_cutout_with(
        |info| unsafe { OH_NativeDisplayManager_CreateDefaultDisplayCutoutInfo(info) },
        |info| unsafe { OH_NativeDisplayManager_DestroyDefaultDisplayCutoutInfo(info) },
    )
}

pub fn default_display_density_dpi() -> Result<f32> {
    query_default(|dpi| unsafe { OH_NativeDisplayManager_GetDefaultDisplayDensityDpi(dpi) })
        .map(|dpi| dpi as f32)
}

pub fn default_display_density_pixel() -> Result<f32> {
    query_default(|pixel| unsafe { OH_NativeDisplayManager_GetDefaultDisplayDensityPixels(pixel) })
}

pub fn default_display_density_x_dpi() -> Result<f32> {
    query_default(|x_dpi| unsafe { OH_NativeDisplayManager_GetDefaultDisplayDensityXdpi(x_dpi) })
}

pub fn default_display_density_y_dpi() -> Result<f32> {
    query_default(|y_dpi| unsafe { OH_NativeDisplayManager_GetDefaultDisplayDensityYdpi(y_dpi) })
}

pub fn default_display_height() -> Result<i32> {
    query_default(|height| unsafe { OH_NativeDisplayManager_GetDefaultDisplayHeight(height) })
}

pub fn default_display_width() -> Result<i32> {
    query_default(|width| unsafe { OH_NativeDisplayManager_GetDefaultDisplayWidth(width) })
}

pub fn default_display_id() -> Result<u64> {
    query_default(|id| unsafe { OH_NativeDisplayManager_GetDefaultDisplayId(id) })
}

pub fn default_display_orientation() -> Result<Orientation> {
    query_default(|orientation| unsafe {
        OH_NativeDisplayManager_GetDefaultDisplayOrientation(orientation)
    })
    .map(Orientation::from)
}

pub fn default_display_refresh_rate() -> Result<u32> {
    query_default(|rate| unsafe { OH_NativeDisplayManager_GetDefaultDisplayRefreshRate(rate) })
}

pub fn default_display_rotation() -> Result<Rotation> {
    query_default(|rotation| unsafe { OH_NativeDisplayManager_GetDefaultDisplayRotation(rotation) })
        .map(Rotation::from)
}

pub fn default_display_scaled_density() -> Result<f32> {
    query_default(|density| unsafe {
        OH_NativeDisplayManager_GetDefaultDisplayScaledDensity(density)
    })
}

pub fn default_display_virtual_pixel_ratio() -> Result<f32> {
    query_default(|ratio| unsafe {
        OH_NativeDisplayManager_GetDefaultDisplayVirtualPixelRatio(ratio)
    })
}

pub fn fold_display_mode() -> Result<FoldMode> {
    query_default(|mode| unsafe { OH_NativeDisplayManager_GetFoldDisplayMode(mode) })
        .map(FoldMode::from)
}

pub fn is_foldable() -> bool {
    unsafe { OH_NativeDisplayManager_IsFoldable() }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use ohos_display_sys::{
        NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK, NativeDisplayManager_Rect,
        NativeDisplayManager_WaterfallDisplayAreaRects,
    };

    use super::*;

    fn empty_raw_rect() -> NativeDisplayManager_Rect {
        NativeDisplayManager_Rect {
            left: 0,
            top: 0,
            width: 0,
            height: 0,
        }
    }

    fn empty_cutout_info() -> NativeDisplayManager_CutoutInfo {
        NativeDisplayManager_CutoutInfo {
            boundingRectsLength: 0,
            boundingRects: ptr::null_mut(),
            waterfallDisplayAreaRects: NativeDisplayManager_WaterfallDisplayAreaRects {
                left: empty_raw_rect(),
                top: empty_raw_rect(),
                right: empty_raw_rect(),
                bottom: empty_raw_rect(),
            },
        }
    }

    #[test]
    fn check_status_maps_ok_and_error() {
        assert_eq!(
            check_status(NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK),
            Ok(())
        );
        assert_eq!(check_status(801), Err(DisplayError::InternalError(801)));
    }

    #[test]
    fn query_default_propagates_ok_and_error() {
        assert_eq!(
            query_default(|out: &mut u32| {
                *out = 120;
                NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK
            }),
            Ok(120)
        );
        assert_eq!(
            query_default(|_out: &mut u64| 1400001),
            Err(DisplayError::InternalError(1400001))
        );
    }

    #[test]
    fn read_cutout_returns_create_error_without_destroying() {
        let mut destroyed = false;
        let result = read_cutout_with(
            |_| 1400002,
            |_| {
                destroyed = true;
                NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK
            },
        );
        assert_eq!(result, Err(DisplayError::InternalError(1400002)));
        assert!(!destroyed);
    }

    #[test]
    fn read_cutout_returns_unexpected_null_without_destroying() {
        let mut destroyed = false;
        let result = read_cutout_with(
            |slot| {
                *slot = ptr::null_mut();
                NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK
            },
            |_| {
                destroyed = true;
                NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK
            },
        );
        assert_eq!(result, Err(DisplayError::UnexpectedNull));
        assert!(!destroyed);
    }

    #[test]
    fn read_cutout_copies_and_destroys_valid_info() {
        let mut raw = empty_cutout_info();
        let mut destroyed_ptr = ptr::null_mut();
        let raw_ptr = &raw mut raw;
        let result = read_cutout_with(
            |slot| {
                *slot = raw_ptr;
                NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK
            },
            |ptr| {
                destroyed_ptr = ptr;
                NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK
            },
        );
        assert!(result.is_ok());
        assert_eq!(destroyed_ptr, raw_ptr);
    }

    #[test]
    fn read_cutout_propagates_destroy_error() {
        let mut raw = empty_cutout_info();
        let raw_ptr = &raw mut raw;
        let result = read_cutout_with(
            |slot| {
                *slot = raw_ptr;
                NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK
            },
            |_| 1400003,
        );
        assert_eq!(result, Err(DisplayError::InternalError(1400003)));
    }
}
