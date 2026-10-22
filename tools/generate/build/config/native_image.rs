use once_cell::sync::Lazy;

use crate::SysConfig;

pub const NATIVE_IMAGE: Lazy<SysConfig> = Lazy::new(|| {
    SysConfig {
    name: "ohos-native-image-sys",
    headers: vec!["native_image/native_image.h"],
    white_list: vec![
        "OH_NativeImage.*",
        "OH_ConsumerSurface.*",
        "OH_OnFrameAvailable.*",
    ],
    block_list: vec![
        "OHNativeWindow.*",
        "NativeWindow.*",
        "OH_NativeBuffer.*",
    ],
    dynamic_library: vec!["native_image"],
    extra: "\nuse ohos_native_window_sys::{OHNativeWindow, OHNativeWindowBuffer};\n#[cfg(feature = \"api-22\")]\nuse ohos_native_buffer_sys::OH_NativeBuffer_ColorSpace;",
}
});
