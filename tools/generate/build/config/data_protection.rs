use once_cell::sync::Lazy;

use super::SysConfig;

pub const DLP_PERMISSION: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-dlp-permission-sys",
    headers: vec!["DataProtectionKit/dlp_permission_api.h"],
    white_list: vec!["OH_DLP_.*", "DLP_.*"],
    block_list: vec![],
    dynamic_library: vec!["ohdlp_permission"],
    extra: "",
});
