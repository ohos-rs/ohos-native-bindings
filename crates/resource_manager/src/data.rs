use ohos_resource_manager_sys::{
    ScreenDensity_SCREEN_LDPI, ScreenDensity_SCREEN_MDPI, ScreenDensity_SCREEN_SDPI,
    ScreenDensity_SCREEN_XLDPI, ScreenDensity_SCREEN_XXLDPI, ScreenDensity_SCREEN_XXXLDPI,
};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum ScreenDensity {
    #[default]
    Current,
    ScreenSDPI,
    ScreenMDPI,
    ScreenLDPI,
    ScreenXLDPI,
    ScreenXXLDPI,
    ScreenXXXLDPI,
}

impl From<ScreenDensity> for ohos_resource_manager_sys::ScreenDensity {
    fn from(value: ScreenDensity) -> Self {
        match value {
            ScreenDensity::Current => 0,
            ScreenDensity::ScreenMDPI => ScreenDensity_SCREEN_MDPI,
            ScreenDensity::ScreenSDPI => ScreenDensity_SCREEN_SDPI,
            ScreenDensity::ScreenLDPI => ScreenDensity_SCREEN_LDPI,
            ScreenDensity::ScreenXLDPI => ScreenDensity_SCREEN_XLDPI,
            ScreenDensity::ScreenXXLDPI => ScreenDensity_SCREEN_XXLDPI,
            ScreenDensity::ScreenXXXLDPI => ScreenDensity_SCREEN_XXXLDPI,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum IconType {
    #[default]
    Current,
    Theme,
    Dynamic,
}
