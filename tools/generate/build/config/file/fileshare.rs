use once_cell::sync::Lazy;

use crate::SysConfig;

pub const FILESHARE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-fileshare-sys",
    headers: vec!["filemanagement/fileshare/oh_file_share.h"],
    white_list: vec!["OH_.*", "FileManagement.*"],
    block_list: vec![],
    dynamic_library: vec!["ohfileshare"],
    extra: "",
});
