use once_cell::sync::Lazy;

use crate::SysConfig;

pub const WINDOW_MANAGER: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-native-window-manager-sys",
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
    // These opaque event types are owned by InputKit. Reuse the canonical
    // definitions instead of generating window-manager-local Rust types.
    block_list: vec!["Input_.*"],
    dynamic_library: vec!["native_window_manager"],
    extra:
        "pub use ohos_multi_modal_input_sys::{Input_KeyEvent, Input_MouseEvent, Input_TouchEvent};",
});
