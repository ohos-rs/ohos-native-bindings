use once_cell::sync::Lazy;

use crate::SysConfig;

pub const IMAGE_NATIVE: Lazy<SysConfig> = Lazy::new(|| {
    SysConfig {
    name: "ohos-image-native-sys",
    headers: vec![
        "multimedia/image_framework/image/image_common.h",
        "multimedia/image_framework/image/image_native.h",
        "multimedia/image_framework/image/image_packer_native.h",
        "multimedia/image_framework/image/image_receiver_native.h",
        "multimedia/image_framework/image/image_source_native.h",
        "multimedia/image_framework/image/picture_native.h",
        "multimedia/image_framework/image/pixelmap_native.h",
    ],
    // `PIXELMAP_ALPHA_TYPE` is used through `int32_t` in the native API, so
    // bindgen cannot discover it recursively from the allowlisted functions.
    white_list: vec!["OH_.*", "PIXELMAP_ALPHA_TYPE"],
    block_list: vec![
        "napi_.*",
        "OH_NativeBuffer.*",
        "OH_NativeWindow.*",
        "BufferHandle",
        "OH_ResourceManager.*",
        "RawFile.*",
        "OHNativeWindow.*",
        "NativeWindow.*",
        "Region",
    ],
    // Link both old and new library names: the OH_ImageReceiverNative_* /
    // OH_PixelMapNative_* families live in the old-named libs on device
    // (libimage_receiver.so / libpixelmap.so); the *_ndk.z.so libs only
    // carry the renamed OH_Image_Receiver_* / OH_PixelMap_* APIs.
    dynamic_library: vec![
        "image_ndk.z",
        "image_packer_ndk.z",
        "pixelmap_ndk.z",
        "image_receiver_ndk.z",
        "image_source_ndk.z",
        "image_source",
        "image_receiver",
        "pixelmap",
        "ohimage",
    ],
    extra: "\n\nuse napi_sys_ohos::*;\nuse ohos_native_buffer_sys::*;\nuse ohos_resource_manager_sys::*;",
}
});
