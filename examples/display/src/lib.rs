use std::fmt::Display;
use std::panic::{catch_unwind, AssertUnwindSafe};

use napi_derive_ohos::napi;
use ohos_display_binding::{
    default_display_cutout_info, default_display_density_dpi, default_display_density_pixel,
    default_display_density_x_dpi, default_display_density_y_dpi, default_display_height,
    default_display_id, default_display_orientation, default_display_refresh_rate,
    default_display_rotation, default_display_scaled_density, default_display_virtual_pixel_ratio,
    default_display_width, fold_display_mode, is_foldable,
};

fn call<T: Display>(label: &str, f: impl FnOnce() -> T) -> String {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(value) => format!("{label}={value}"),
        Err(_) => format!("{label}=panic"),
    }
}

#[napi]
pub fn smoke() -> String {
    [
        call("id", default_display_id),
        call("width", default_display_width),
        call("height", default_display_height),
        call("dpi", default_display_density_dpi),
        call("pixel", default_display_density_pixel),
        call("xdpi", default_display_density_x_dpi),
        call("ydpi", default_display_density_y_dpi),
        call("orientation", || {
            format!("{:?}", default_display_orientation())
        }),
        call("rotation", || format!("{:?}", default_display_rotation())),
        call("refresh", default_display_refresh_rate),
        call("scaled_density", default_display_scaled_density),
        call("vpr", default_display_virtual_pixel_ratio),
        call("foldable", is_foldable),
        call("fold_mode", || format!("{:?}", fold_display_mode())),
        call("cutout", || format!("{:?}", default_display_cutout_info())),
    ]
    .join("\n")
}
