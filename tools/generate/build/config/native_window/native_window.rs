use once_cell::sync::Lazy;

use crate::SysConfig;

pub const NATIVE_WINDOW: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-native-window-sys",
    headers: vec![
        "native_window/buffer_handle.h",
        "native_window/external_window.h",
        "native_window/graphic_error_code.h",
    ],
    white_list: vec!["OH_.*", "NativeWindowOperation", "OH_NativeBuffer.*"],
    block_list: vec!["OH_NativeBuffer.*", "BufferHandle", "NativeWindowBuffer"],
    extra: "\n\nuse ohos_native_buffer_sys::*;",
});
