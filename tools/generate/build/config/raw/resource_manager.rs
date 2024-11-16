use once_cell::sync::Lazy;

use crate::SysConfig;

// source code with default value which is not supported by clang
// you should change it.
pub const RESOURCE_MANAGER: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-resource-manager-sys",
    headers: vec![
        "resourcemanager/ohresmgr.h",
        "resourcemanager/resmgr_common.h",
    ],
    white_list: vec![
        "OH_ResourceManager_GetMediaBase64",
        "OH_ResourceManager_GetMediaBase64ByName",
        "OH_ResourceManager_GetMedia",
        "OH_ResourceManager_GetMediaByName",
        "OH_ResourceManager_GetDrawableDescriptor",
        "OH_ResourceManager_GetDrawableDescriptorByName",
        "ScreenDensity",
    ],
    block_list: vec!["napi_.*", "NativeResourceManager"],
    extra: "\n\nuse ohos_raw_sys::*;",
});
