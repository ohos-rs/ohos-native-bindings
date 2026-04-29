use once_cell::sync::Lazy;

use crate::SysConfig;

pub const UDMF: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-udmf-sys",
    headers: vec![
        "database/udmf/udmf.h",
        "database/udmf/udmf_err_code.h",
        "database/udmf/udmf_meta.h",
        "database/udmf/uds.h",
        "database/udmf/utd.h",
    ],
    white_list: vec!["OH_Udmf.*", "OH_Uds.*", "OH_Utd.*", "Udmf_.*", "UDMF_.*"],
    block_list: vec![
        "OH_PixelmapNative$",
        "OH_PixelmapNative_.*",
        "OH_PixelmapNative_AntiAliasingLevel.*",
        "OH_Pixelmap_.*",
        "OH_NativeColorSpaceManager$",
        "OH_PictureMetadata.*",
        "Image_.*",
    ],
    dynamic_library: vec!["udmf"],
    extra: "\n\n#[allow(unused_imports)]\nuse ohos_image_native_sys::*;",
});
