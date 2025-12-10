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
    white_list: vec!["OH_.*"],
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
    dynamic_library: vec!["image_ndk.z", "image_packer_ndk.z", "pixelmap_ndk.z", "image_receiver_ndk.z", "image_source_ndk.z"],
    extra: "\n\nuse napi_sys_ohos::*;\nuse ohos_native_buffer_sys::*;\nuse ohos_resource_manager_sys::*;",
}
});
