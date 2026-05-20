use once_cell::sync::Lazy;

use super::SysConfig;

pub const JSVM: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-jsvm-sys",
    headers: vec!["ark_runtime/jsvm.h", " <ark_runtime/jsvm_types.h"],
    white_list: vec!["OH_JSVM_.*", "JSVM_.*"],
    block_list: vec![],
    dynamic_library: vec!["jsvm"],
    extra: "",
});
