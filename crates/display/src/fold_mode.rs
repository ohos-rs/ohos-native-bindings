use ohos_display_sys::{
    NativeDisplayManager_FoldDisplayMode,
    NativeDisplayManager_FoldDisplayMode_DISPLAY_MANAGER_FOLD_DISPLAY_MODE_COORDINATION,
    NativeDisplayManager_FoldDisplayMode_DISPLAY_MANAGER_FOLD_DISPLAY_MODE_FULL,
    NativeDisplayManager_FoldDisplayMode_DISPLAY_MANAGER_FOLD_DISPLAY_MODE_MAIN,
    NativeDisplayManager_FoldDisplayMode_DISPLAY_MANAGER_FOLD_DISPLAY_MODE_SUB,
    NativeDisplayManager_FoldDisplayMode_DISPLAY_MANAGER_FOLD_DISPLAY_MODE_UNKNOWN,
};

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    NativeDisplayManager_FoldDisplayMode,
    "NativeDisplayManager_FoldDisplayMode_DISPLAY_MANAGER_FOLD_DISPLAY_MODE_"
)]
pub enum FoldMode {
    Unknown,
    Main,
    Sub,
    Full,
    Coordination,
}
