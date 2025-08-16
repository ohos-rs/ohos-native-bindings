use once_cell::sync::Lazy;

use crate::SysConfig;

// source code with default value which is not supported by clang
// you should change it.
pub const RESOURCE_MANAGER: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-resource-manager-sys",
    headers: vec![
        "resourcemanager/ohresmgr.h",
        "resourcemanager/resmgr_common.h",
        "rawfile/raw_dir.h",
        "rawfile/raw_file.h",
        "rawfile/raw_file_manager.h",
    ],
    white_list: vec![
        "OH_ResourceManager_.*",
        "NativeResourceManager",
        "ScreenDensity",
    ],
    block_list: vec!["napi_.*"],
    extra: "\n\nuse napi_sys_ohos::*;",
});
