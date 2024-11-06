use once_cell::sync::Lazy;

use crate::SysConfig;

pub const ABILITY: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-ability-access-control-sys",
    headers: vec!["accesstoken/ability_access_control.h"],
    white_list: vec!["OH_.*"],
    block_list: vec![],
    extra: "",
});
