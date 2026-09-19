use once_cell::sync::Lazy;

use crate::SysConfig;

pub const WINDOW_MANAGER: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-window-manager-sys",
    headers: vec![
        "window_manager/oh_window_comm.h",
        "window_manager/oh_window.h",
        "window_manager/oh_window_event_filter.h",
    ],
    white_list: vec![
        "WindowManager_.*",
        "OH_WindowManager_.*",
        "OH_NativeWindowManager_.*",
    ],
    block_list: vec![],
    // Host-side unit tests exercise the safe conversions without linking an
    // OpenHarmony system image. The library is required only for OHOS targets.
    dynamic_library: vec![],
    extra: r#"#[cfg_attr(target_env = "ohos", link(name = "native_window_manager"))]
unsafe extern "C" {}"#,
});
