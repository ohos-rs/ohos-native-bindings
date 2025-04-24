use cut_info::DisplayCutInfo;
use fold_mode::FoldMode;
use ohos_display_sys::{
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
use orientation::Orientation;
use rotation::Rotation;

mod cut_info;
mod event;
mod fold_mode;
mod orientation;
mod rotation;

pub fn default_display_cutout_info() -> DisplayCutInfo {
    let mut info = std::ptr::null_mut();
    let ret = unsafe { OH_NativeDisplayManager_CreateDefaultDisplayCutoutInfo(&mut info) };
    assert_eq!(ret, 0);
    let cut_info = DisplayCutInfo::from(unsafe { *info });
    let ret = unsafe { OH_NativeDisplayManager_DestroyDefaultDisplayCutoutInfo(info) };
    assert_eq!(ret, 0);
    cut_info
}

pub fn default_display_density_dpi() -> f32 {
    let mut dpi = 0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayDensityDpi(&mut dpi) };
    assert_eq!(ret, 0);
    dpi as f32
}

pub fn default_display_density_pixel() -> f32 {
    let mut pixel = 0.0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayDensityPixels(&mut pixel) };
    assert_eq!(ret, 0);
    pixel
}

pub fn default_display_density_x_dpi() -> f32 {
    let mut x_dpi = 0.0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayDensityXdpi(&mut x_dpi) };
    assert_eq!(ret, 0);
    x_dpi
}

pub fn default_display_density_y_dpi() -> f32 {
    let mut y_dpi = 0.0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayDensityYdpi(&mut y_dpi) };
    assert_eq!(ret, 0);
    y_dpi
}

pub fn default_display_height() -> i32 {
    let mut height = 0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayHeight(&mut height) };
    assert_eq!(ret, 0);
    height
}

pub fn default_display_width() -> i32 {
    let mut width = 0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayWidth(&mut width) };
    assert_eq!(ret, 0);
    width
}

pub fn default_display_id() -> u64 {
    let mut id = 0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayId(&mut id) };
    assert_eq!(ret, 0);
    id
}

pub fn default_display_orientation() -> Orientation {
    let mut orientation = 0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayOrientation(&mut orientation) };
    assert_eq!(ret, 0);
    Orientation::from(orientation)
}

pub fn default_display_refresh_rate() -> u32 {
    let mut rate = 0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayRefreshRate(&mut rate) };
    assert_eq!(ret, 0);
    rate
}

pub fn default_display_rotation() -> Rotation {
    let mut rotation = 0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayRotation(&mut rotation) };
    assert_eq!(ret, 0);
    Rotation::from(rotation)
}

pub fn default_display_scaled_density() -> f32 {
    let mut density = 0.0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayScaledDensity(&mut density) };
    assert_eq!(ret, 0);
    density
}

pub fn default_display_virtual_pixel_ratio() -> f32 {
    let mut ratio = 0.0;
    let ret = unsafe { OH_NativeDisplayManager_GetDefaultDisplayVirtualPixelRatio(&mut ratio) };
    assert_eq!(ret, 0);
    ratio
}

pub fn fold_display_mode() -> FoldMode {
    let mut mode = 0;
    let ret = unsafe { OH_NativeDisplayManager_GetFoldDisplayMode(&mut mode) };
    assert_eq!(ret, 0);
    FoldMode::from(mode)
}

pub fn is_foldable() -> bool {
    unsafe { OH_NativeDisplayManager_IsFoldable() }
}
