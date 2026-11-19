use once_cell::sync::Lazy;

use super::SysConfig;

pub const HITRACE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-hitrace-sys",
    headers: vec!["hitrace/trace.h"],
    white_list: vec!["OH_HiTrace_(StartTrace|FinishTrace|CountTrace)"],
    block_list: vec![],
    dynamic_library: vec!["hitrace_ndk.z"],
    extra: "",
});
