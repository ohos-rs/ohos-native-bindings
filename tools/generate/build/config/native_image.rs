use once_cell::sync::Lazy;

use crate::SysConfig;

pub const NATIVE_IMAGE: Lazy<SysConfig> = Lazy::new(|| {
    SysConfig {
    name: "ohos-native-image-sys",
    headers: vec![
        "native_image/native_image.h",
        "native_image/graphic_error_code.h",
    ],
    white_list: vec![
        "OH_NativeImage.*",
        "OH_ConsumerSurface.*",
        "OH_OnFrameAvailable.*",
        "OHNativeErrorCode",
        "NATIVE_ERROR_.*",
    ],
    // Pulled in transitively by the allowlisted signatures; owned by the window/buffer crates.
    block_list: vec![
        "OHNativeWindow",
        "NativeWindow",
        "OHNativeWindowBuffer",
        "NativeWindowBuffer",
        "OH_NativeBuffer.*",
    ],
    dynamic_library: vec!["native_image"],
    extra: "\n\n#[allow(unused_imports)]\nuse ohos_native_buffer_sys::*;\n#[allow(unused_imports)]\nuse ohos_native_window_sys::OHNativeWindow;",
}
});
