use once_cell::sync::Lazy;

use crate::SysConfig;

pub const NATIVE_BUFFER: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-native-buffer-sys",
    headers: vec![
        "native_buffer/buffer_common.h",
        "native_buffer/graphic_error_code.h",
        "native_buffer/native_buffer.h",
    ],
    white_list: vec!["OH_.*"],
    block_list: vec!["NativeWindow.*"],
    extra: "\n\nuse ohos_native_window_sys::*;",
});
