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
    extra: "\n\nuse napi_sys_ohos::*;\nuse ohos_resource_manager_sys::*;",
});
