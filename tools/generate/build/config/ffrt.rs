use once_cell::sync::Lazy;

use super::SysConfig;

// All ffrt headers share the types declared in type_def.h (ffrt_task_attr_t, ffrt_qos_t,
// ffrt_function_header_t, ffrt_queue_t, ffrt_timer_t), so they must live in one crate.
pub const FFRT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-ffrt-sys",
    headers: vec![
        "ffrt/type_def.h",
        "ffrt/task.h",
        "ffrt/queue.h",
        "ffrt/loop.h",
        "ffrt/mutex.h",
        "ffrt/shared_mutex.h",
        "ffrt/condition_variable.h",
        "ffrt/sleep.h",
        "ffrt/timer.h",
        "ffrt/fiber.h",
    ],
    white_list: vec!["ffrt_.*"],
    block_list: vec![],
    dynamic_library: vec!["ffrt.z"],
    extra: "",
});
