use ohos_display_sys::{
    NativeDisplayManager_Orientation, NativeDisplayManager_Orientation_DISPLAY_MANAGER_LANDSCAPE,
    NativeDisplayManager_Orientation_DISPLAY_MANAGER_LANDSCAPE_INVERTED,
    NativeDisplayManager_Orientation_DISPLAY_MANAGER_PORTRAIT,
    NativeDisplayManager_Orientation_DISPLAY_MANAGER_PORTRAIT_INVERTED,
};

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    NativeDisplayManager_Orientation,
    "NativeDisplayManager_Orientation_DISPLAY_MANAGER_"
)]
pub enum Orientation {
    Portrait,
    Landscape,
    PortraitInverted,
    LandscapeInverted,
}
