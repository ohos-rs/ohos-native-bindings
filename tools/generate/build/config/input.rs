use once_cell::sync::Lazy;

use super::SysConfig;

pub const MULTI_MODAL_INPUT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-input-sys",
    headers: vec![
        "multimodalinput/oh_axis_type.h",
        "multimodalinput/oh_input_manager.h",
        "multimodalinput/oh_key_code.h",
    ],
    white_list: vec!["Input.*", "OH_.*"],
    block_list: vec![],
    dynamic_library: vec!["ohinput"],
    extra: "",
});
