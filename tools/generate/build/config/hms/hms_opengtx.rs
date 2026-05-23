use once_cell::sync::Lazy;

use super::super::SysConfig;

pub const OPENGTX: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "hms-opengtx-sys",
    headers: vec!["graphics_game_sdk/opengtx_base.h"],
    white_list: vec!["HMS_OpenGTX.*", "OpenGTX_*"],
    block_list: vec![],
    dynamic_library: vec!["opengtx"],
    extra: "",
});
