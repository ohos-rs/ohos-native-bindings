use once_cell::sync::Lazy;

use crate::SysConfig;

pub const GLES: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-gles-sys",
    headers: vec![
        "GLES3/gl3.h",
        "GLES3/gl31.h",
        "GLES3/gl32.h",
        "GLES2/gl2.h",
        "GLES2/gl2ext.h",
        "GLES2/gl2platform.h",
        "GLES3/gl3platform.h",
        "KHR/khrplatform.h",
    ],
    white_list: vec!["GL_.*", "gl[A-Z].*", "GL[A-Z].*", "khronos_.*"],
    block_list: vec![],
    dynamic_library: vec!["GLESv3"],
    extra: "",
});
