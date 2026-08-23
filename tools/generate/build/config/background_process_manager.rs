use once_cell::sync::Lazy;

use crate::SysConfig;

/// Background process priority control (`@library libbackground_process_manager.z.so`,
/// `SystemCapability.Resourceschedule.BackgroundProcessManager`).
///
/// Sets or resets the scheduling priority of a process. No permission is required.
///
/// Every symbol is `@since 17`, so the whole crate is gated behind `feature = "api-17"`.
pub const BACKGROUND_PROCESS_MANAGER: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-background-process-manager-sys",
    headers: vec!["background_process_manager/background_process_manager.h"],
    white_list: vec![
        "OH_BackgroundProcessManager_.*",
        "BackgroundProcessManager_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["background_process_manager.z"],
    extra: "",
});
