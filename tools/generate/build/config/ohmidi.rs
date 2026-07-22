use once_cell::sync::Lazy;

use crate::SysConfig;

pub const OHMIDI: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-midi-sys",
    headers: vec!["ohmidi/native_midi.h", "ohmidi/native_midi_base.h"],
    white_list: vec!["OH_MIDI.*"],
    block_list: vec![],
    dynamic_library: vec!["ohmidi"],
    extra: "",
});
