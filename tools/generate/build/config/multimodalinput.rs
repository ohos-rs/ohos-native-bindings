use once_cell::sync::Lazy;

use super::SysConfig;

pub const MULTIMODAL_INPUT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-input-sys",
    headers: vec![
        "multimodalinput/oh_axis_type.h",
        "multimodalinput/oh_key_code.h",
        "multimodalinput/oh_pointer_style.h",
        "multimodalinput/oh_input_manager.h",
    ],
    // All symbols are `OH_Input_*` (functions), `Input_*` (types, enums, callbacks) or
    // `InputEvent_*` (axis enums). Keep it this narrow: oh_input_manager.h pulls in
    // info/application_target_sdk_version.h, whose `get/set_application_target_sdk_version`
    // live in libc and whose `OH_API_VERSION_*` macros would be caught by a bare `OH_.*`.
    white_list: vec!["OH_Input_.*", "Input_.*", "InputEvent_.*"],
    // OH_PixelmapNative is only forward-declared here; take the real type from
    // ohos-image-native-sys so custom-cursor APIs stay callable across crates.
    block_list: vec!["OH_PixelmapNative"],
    dynamic_library: vec!["ohinput"],
    extra: "\n\n#[allow(unused_imports)]\nuse ohos_image_native_sys::*;",
});
