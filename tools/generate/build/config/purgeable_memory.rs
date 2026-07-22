use once_cell::sync::Lazy;

use super::SysConfig;

pub const PURGEABLE_MEMORY: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-purgeable-memory-sys",
    headers: vec!["purgeable_memory/purgeable_memory.h"],
    // `PurgMem` is the opaque struct tag behind the `OH_PurgeableMemory` typedef;
    // it does not share the public prefix.
    white_list: vec!["OH_PurgeableMemory.*", "PurgMem"],
    block_list: vec![],
    dynamic_library: vec!["purgeable_memory_ndk.z"],
    extra: "",
});
