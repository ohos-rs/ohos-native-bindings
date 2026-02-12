use once_cell::sync::Lazy;

use crate::SysConfig;

pub const FILEURI: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-fileuri-sys",
    headers: vec!["filemanagement/file_uri/oh_file_uri.h"],
    white_list: vec!["OH_.*", "FileManagement.*"],
    block_list: vec![],
    dynamic_library: vec!["ohfileuri"],
    extra: "",
});
