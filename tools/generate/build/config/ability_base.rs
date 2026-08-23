use once_cell::sync::Lazy;

use crate::SysConfig;

pub const ABILITY_BASE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-ability-base-sys",
    headers: vec![
        "AbilityKit/ability_base/ability_base_common.h",
        "AbilityKit/ability_base/want.h",
    ],
    white_list: vec!["OH_AbilityBase_.*", "AbilityBase_.*", "ABILITY_BASE_.*"],
    block_list: vec![],
    dynamic_library: vec!["ability_base_want"],
    extra: "",
});

pub const CHILD_PROCESS: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-child-process-sys",
    headers: vec!["AbilityKit/native_child_process.h"],
    white_list: vec![
        "OH_Ability_.*",
        "Ability_.*",
        "NativeChildProcess_.*",
        "NCP_.*",
    ],
    // The start callback hands back an `OHIPCRemoteProxy`. Reuse IPCKit's definition instead
    // of generating a second one, which would be a distinct and incompatible Rust type.
    block_list: vec!["OHIPCRemoteProxy"],
    dynamic_library: vec!["child_process"],
    extra: "\n\n#[allow(unused_imports)]\nuse ohos_ipc_sys::*;",
});
