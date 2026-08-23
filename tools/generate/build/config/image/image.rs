use once_cell::sync::Lazy;

use crate::SysConfig;

pub const IMAGE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-image-sys",
    headers: vec![
        "multimedia/image_framework/image_mdk.h",
        "multimedia/image_framework/image_mdk_common.h",
        "multimedia/image_framework/image_packer_mdk.h",
        "multimedia/image_framework/image_pixel_map_mdk.h",
        "multimedia/image_framework/image_pixel_map_napi.h",
        "multimedia/image_framework/image_receiver_mdk.h",
        "multimedia/image_framework/image_source_mdk.h",
    ],
    white_list: vec!["OH_.*"],
    block_list: vec!["OH_ResourceManager.*", "RawFile.*", "napi_.*"],
    // Link both old and new library names: on device the symbols live in the
    // *_ndk.z.so SDK libraries, while the old names only resolve against NDK
    // stubs (see also the image_native config).
    dynamic_library: vec![
        "ohimage",
        "image_packer",
        "picture",
        "image_receiver",
        "image_source",
        "pixelmap",
        "image_ndk.z",
        "image_packer_ndk.z",
        "image_receiver_ndk.z",
        "image_source_ndk.z",
        "pixelmap_ndk.z",
    ],
    extra: "\n\nuse napi_sys_ohos::*;\nuse ohos_resource_manager_sys::*;",
});
