use once_cell::sync::Lazy;

use crate::SysConfig;

pub const EGL: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-egl-sys",
    headers: vec![
        "EGL/egl.h",
        "EGL/eglext.h",
        "EGL/eglplatform.h",
        "KHR/khrplatform.h",
    ],
    white_list: vec!["EGL_.*", "egl[A-Z].*", "EGL[A-Z].*", "khronos_.*"],
    block_list: vec![],
    dynamic_library: vec!["EGL"],
    extra: "",
});
