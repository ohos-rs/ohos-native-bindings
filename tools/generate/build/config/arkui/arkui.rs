use once_cell::sync::Lazy;

use crate::config::SysConfig;

pub const ARKUI: Lazy<SysConfig> = Lazy::new(|| {
    SysConfig {
        name: "ohos-arkui-sys",
        headers: vec![
            "arkui/drag_and_drop.h",
            "arkui/drawable_descriptor.h",
            "arkui/native_animate.h",
            "arkui/native_dialog.h",
            "arkui/native_gesture.h",
            "arkui/native_interface.h",
            "arkui/native_key_event.h",
            "arkui/native_node.h",
            "arkui/native_node_napi.h",
            "arkui/native_type.h",
            "arkui/styled_string.h",
        ],
        white_list: vec!["ArkUI_.*", "ARKUI_.*", "OH_ArkUI_.*"],
        block_list: vec![
            "napi_.*",
            "ArkUI_ErrorCode",
            ".*UIInputEvent.*",
            "ArkUI_NodeHandle",
            "ArkUI_Node",
            "OH_NativeBuffer.*",
            "OH_PixelmapNative$",
            "OH_PixelmapNative_.*",
            "OH_PixelmapNative_AntiAliasingLevel.*",
            "OH_Pixelmap_.*",
            "OH_NativeColorSpaceManager$",
            "OH_PictureMetadata.*",
            "Image_.*",
            "OH_Drawing.*",
            "OH_Udmf.*",
            "OH_Uds.*",
            "Udmf_.*",
        ],
        dynamic_library: vec!["ace_ndk.z"],
        extra: "\n\n#[allow(unused_imports)]\nuse napi_sys_ohos::*;\nuse ohos_arkui_input_sys::*;\nuse ohos_image_native_sys::*;\nuse ohos_native_drawing_sys::*;\nuse ohos_udmf_sys::*;",
    }
});
