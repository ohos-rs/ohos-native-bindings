use once_cell::sync::Lazy;

use super::SysConfig;

pub const BUNDLE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-bundle-sys",
    headers: vec!["bundle/native_interface_bundle.h"],
    white_list: vec![],
    block_list: vec![],
    extra: "",
});
