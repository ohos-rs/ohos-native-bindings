use once_cell::sync::Lazy;

use crate::SysConfig;

// raw and resource file manager
// raw_file deps on string, so we changed the raw_file content with c library
// #include <stdint.h>
// #include <stddef.h>
// #include <stdbool.h>
pub const RAW: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-raw-sys",
    headers: vec![
        "rawfile/raw_dir.h",
        "rawfile/raw_file.h",
        "rawfile/raw_file_manager.h",
    ],
    white_list: vec!["OH_ResourceManager_.*"],
    block_list: vec!["napi_.*"],
    extra: "\n\nuse napi_sys_ohos::*;\n",
});
