use once_cell::sync::Lazy;

use super::SysConfig;

// HiviewDFX observability family. `hilog` already has its own config (see `hilog.rs`).
// Each entry maps 1:1 to one NDK `@library`, matching the repo convention
// (one sys crate per shared object, cf. net_connection / net_stack).

/// hiappevent: application event logging (`@library libhiappevent_ndk.z.so`).
pub const HIAPPEVENT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-hiappevent-sys",
    headers: vec![
        "hiappevent/hiappevent_cfg.h",
        "hiappevent/hiappevent_event.h",
        "hiappevent/hiappevent_param.h",
        "hiappevent/hiappevent.h",
    ],
    white_list: vec![
        // functions, opaque handles, callback typedefs, error codes
        "OH_HiAppEvent_.*",
        "HiAppEvent_.*",
        "HIAPPEVENT_.*",
        "ParamList.*",
        "EventType",
        // string macros from hiappevent_event.h / _param.h / _cfg.h
        "EVENT_.*",
        "PARAM_.*",
        "DOMAIN_.*",
        "MAIN_THREAD_JANK_PARAM_.*",
        "OH_APP_CRASH_PARAM_.*",
        "DISABLE",
        "MAX_STORAGE",
    ],
    block_list: vec![],
    dynamic_library: vec!["hiappevent_ndk.z"],
    extra: "",
});

/// hitrace: distributed trace chain + trace slices (`@library libhitrace_ndk.z.so`).
pub const HITRACE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-hitrace-sys",
    headers: vec!["hitrace/trace.h"],
    white_list: vec!["OH_HiTrace.*", "HiTrace.*", "HITRACE_.*"],
    block_list: vec![],
    dynamic_library: vec!["hitrace_ndk.z"],
    extra: "",
});

/// hidebug: cpu / memory / trace-capture / backtrace debugging (`@library libohhidebug.so`).
pub const HIDEBUG: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-hidebug-sys",
    headers: vec!["hidebug/hidebug_type.h", "hidebug/hidebug.h"],
    white_list: vec!["OH_HiDebug.*", "HiDebug_.*", "HIDEBUG_.*"],
    block_list: vec![],
    dynamic_library: vec!["ohhidebug"],
    extra: "",
});
