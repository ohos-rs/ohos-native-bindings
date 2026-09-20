use once_cell::sync::Lazy;

use super::SysConfig;

/// Multimodal InputKit types and functions shared with consumers such as
/// Window Manager.
pub const INPUT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-input-sys",
    headers: vec!["multimodalinput/oh_input_manager.h"],
    white_list: vec!["Input_.*", "InputEvent_.*", "OH_Input_.*"],
    // PixelMap is owned by the native image binding. Keeping one canonical
    // opaque type lets cursor APIs interoperate without pointer casts.
    block_list: vec!["OH_PixelmapNative"],
    dynamic_library: vec!["ohinput"],
    extra: "\n\npub use ohos_image_native_sys::OH_PixelmapNative;",
});
