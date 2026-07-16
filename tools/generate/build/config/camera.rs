use once_cell::sync::Lazy;

use crate::SysConfig;

pub const CAMERA: Lazy<SysConfig> = Lazy::new(|| {
    SysConfig {
    name: "ohos-camera-sys",
    headers: vec![
        "ohcamera/camera.h",
        "ohcamera/camera_manager.h",
        "ohcamera/camera_input.h",
        "ohcamera/capture_session.h",
        "ohcamera/preview_output.h",
        "ohcamera/photo_output.h",
        "ohcamera/photo_native.h",
    ],
    white_list: vec![
        "Camera_.*",
        "OH_Camera.*",
        "OH_CaptureSession.*",
        "OH_PreviewOutput.*",
        "OH_PhotoOutput.*",
        "OH_PhotoNative.*",
    ],
    block_list: vec!["OH_ImageNative", "OH_PictureNative"],
    dynamic_library: vec!["ohcamera"],
    extra: "\n\npub use ohos_image_native_sys::OH_ImageNative;\n#[cfg(feature = \"api-23\")]\npub use ohos_image_native_sys::OH_PictureNative;",
}
});
