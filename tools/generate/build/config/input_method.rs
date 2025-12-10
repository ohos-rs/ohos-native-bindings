use once_cell::sync::Lazy;

use super::SysConfig;

pub const INPUT_METHOD: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-input-method-sys",
    headers: vec![
        "inputmethod/inputmethod_controller_capi.h",
        "inputmethod/inputmethod_attach_options_capi.h",
        "inputmethod/inputmethod_cursor_info_capi.h",
        "inputmethod/inputmethod_inputmethod_proxy_capi.h",
        "inputmethod/inputmethod_private_command_capi.h",
        "inputmethod/inputmethod_text_avoid_info_capi.h",
        "inputmethod/inputmethod_text_config_capi.h",
        "inputmethod/inputmethod_text_editor_proxy_capi.h",
        "inputmethod/inputmethod_types_capi.h",
    ],
    white_list: vec!["OH.*", "Input.*"],
    block_list: vec![],
    dynamic_library: vec!["ohinputmethod"],
    extra: "",
});
