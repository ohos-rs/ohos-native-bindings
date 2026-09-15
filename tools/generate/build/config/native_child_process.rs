use once_cell::sync::Lazy;

use crate::SysConfig;

pub const NATIVE_CHILD_PROCESS: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-native-child-process-sys",
    headers: vec!["AbilityKit/native_child_process.h"],
    white_list: vec![
        "Ability_NativeChildProcess_.*",
        "Ability_ChildProcessConfigs",
        "NativeChildProcess_.*",
        "OH_Ability_.*ChildProcess.*",
        "NCP_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["child_process"],
    extra: "",
});
