use ohos_display_sys::{
    NativeDisplayManager_Rotation, NativeDisplayManager_Rotation_DISPLAY_MANAGER_ROTATION_0,
    NativeDisplayManager_Rotation_DISPLAY_MANAGER_ROTATION_180,
    NativeDisplayManager_Rotation_DISPLAY_MANAGER_ROTATION_270,
    NativeDisplayManager_Rotation_DISPLAY_MANAGER_ROTATION_90,
};

use enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    NativeDisplayManager_Rotation,
    "NativeDisplayManager_Rotation_DISPLAY_MANAGER_"
)]
pub enum Rotation {
    Rotation0,
    Rotation90,
    Rotation180,
    Rotation270,
}
