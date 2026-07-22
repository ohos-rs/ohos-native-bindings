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

pub const ABILITY_RUNTIME: Lazy<SysConfig> = Lazy::new(|| {
    SysConfig {
    name: "ohos-ability-runtime-sys",
    headers: vec![
        "AbilityKit/ability_runtime/ability_runtime_common.h",
        "AbilityKit/ability_runtime/context_constant.h",
        "AbilityKit/ability_runtime/context.h",
        "AbilityKit/ability_runtime/extension_ability.h",
        "AbilityKit/ability_runtime/start_options.h",
        "AbilityKit/ability_runtime/application_context.h",
    ],
    white_list: vec![
        "OH_AbilityRuntime_.*",
        "AbilityRuntime_.*",
        "ABILITY_RUNTIME_.*",
    ],
    block_list: vec![
        // Want types are owned by ohos-ability-base-sys; regenerating them here would
        // produce a second, incompatible `AbilityBase_Want` that no caller could bridge.
        "AbilityBase_.*",
        "OH_AbilityBase_.*",
        "ABILITY_BASE_.*",
        // Pulled in transitively by start_options.h; owned by ohos-image-native-sys.
        "OH_Pixelmap.*",
        "Image_.*",
        // Not a library export: the application must define this symbol for the runtime
        // to find. Binding it would hand out a call that never links.
        "OH_AbilityRuntime_OnNativeExtensionCreate",
        "AbilityRuntime_Extension_CreateFunc",
    ],
    dynamic_library: vec!["ability_runtime"],
    extra: "\n\n#[allow(unused_imports)]\nuse ohos_ability_base_sys::*;\n#[allow(unused_imports)]\nuse ohos_image_native_sys::*;",
}
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
