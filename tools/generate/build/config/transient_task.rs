use once_cell::sync::Lazy;

use crate::SysConfig;

/// Transient task management (`@library libtransient_task.so`,
/// `SystemCapability.ResourceSchedule.BackgroundTaskManager.TransientTask`).
///
/// Requests a bounded delay before the application is suspended. No permission is required.
///
/// The lowest `@since` in these headers is 13, so the whole crate sits behind
/// `feature = "api-13"`; `OH_BackgroundTaskManager_GetTransientTaskInfo`,
/// `TransientTask_TransientTaskInfo` and `TRANSIENT_TASK_MAX_NUM` are `@since 20`.
pub const TRANSIENT_TASK: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-transient-task-sys",
    headers: vec![
        "transient_task/transient_task_api.h",
        "transient_task/transient_task_type.h",
    ],
    white_list: vec![
        "OH_BackgroundTaskManager_.*",
        "TransientTask_.*",
        "TRANSIENT_TASK_.*",
        "ERR_TRANSIENT_TASK_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["transient_task"],
    extra: "",
});
