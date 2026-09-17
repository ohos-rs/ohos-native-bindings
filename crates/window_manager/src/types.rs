#[cfg(feature = "api-15")]
use ohos_enum_derive::EnumFrom;
#[cfg(feature = "api-15")]
use ohos_window_manager_sys::{
    WindowManager_AvoidArea, WindowManager_AvoidAreaType,
    WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_CUTOUT,
    WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_KEYBOARD,
    WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_NAVIGATION_INDICATOR,
    WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_SYSTEM,
    WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_SYSTEM_GESTURE, WindowManager_Rect,
    WindowManager_WindowProperties, WindowManager_WindowType,
    WindowManager_WindowType_WINDOW_MANAGER_WINDOW_TYPE_APP,
    WindowManager_WindowType_WINDOW_MANAGER_WINDOW_TYPE_DIALOG,
    WindowManager_WindowType_WINDOW_MANAGER_WINDOW_TYPE_FLOAT,
    WindowManager_WindowType_WINDOW_MANAGER_WINDOW_TYPE_MAIN,
};

#[cfg(feature = "api-15")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[cfg(feature = "api-15")]
impl From<WindowManager_Rect> for Rect {
    fn from(value: WindowManager_Rect) -> Self {
        Self {
            x: value.posX,
            y: value.posY,
            width: value.width,
            height: value.height,
        }
    }
}

#[cfg(feature = "api-15")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    WindowManager_AvoidAreaType,
    "WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_"
)]
pub enum AvoidAreaType {
    System,
    Cutout,
    SystemGesture,
    Keyboard,
    NavigationIndicator,
}

#[cfg(feature = "api-15")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AvoidArea {
    pub top: Rect,
    pub left: Rect,
    pub right: Rect,
    pub bottom: Rect,
}

#[cfg(feature = "api-15")]
impl From<WindowManager_AvoidArea> for AvoidArea {
    fn from(value: WindowManager_AvoidArea) -> Self {
        Self {
            top: value.topRect.into(),
            left: value.leftRect.into(),
            right: value.rightRect.into(),
            bottom: value.bottomRect.into(),
        }
    }
}

#[cfg(feature = "api-15")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    WindowManager_WindowType,
    "WindowManager_WindowType_WINDOW_MANAGER_WINDOW_TYPE_"
)]
pub enum WindowType {
    App,
    Main,
    Float,
    Dialog,
}

#[cfg(feature = "api-15")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowProperties {
    pub window_rect: Rect,
    pub drawable_rect: Rect,
    pub window_type: Option<WindowType>,
    pub is_full_screen: bool,
    pub is_layout_full_screen: bool,
    pub focusable: bool,
    pub touchable: bool,
    pub brightness: f32,
    pub keep_screen_on: bool,
    pub privacy_mode: bool,
    pub transparent: bool,
    pub id: u32,
    pub display_id: u32,
}

#[cfg(feature = "api-15")]
impl From<WindowManager_WindowProperties> for WindowProperties {
    fn from(value: WindowManager_WindowProperties) -> Self {
        Self {
            window_rect: value.windowRect.into(),
            drawable_rect: value.drawableRect.into(),
            window_type: WindowType::try_from_raw(value.type_),
            is_full_screen: value.isFullScreen,
            is_layout_full_screen: value.isLayoutFullScreen,
            focusable: value.focusable,
            touchable: value.touchable,
            brightness: value.brightness,
            keep_screen_on: value.isKeepScreenOn,
            privacy_mode: value.isPrivacyMode,
            transparent: value.isTransparent,
            id: value.id,
            display_id: value.displayId,
        }
    }
}

#[cfg(feature = "api-21")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MainWindowInfo {
    pub display_id: u64,
    pub window_id: i32,
    pub showing: bool,
    pub label: Option<String>,
}

#[cfg(all(test, feature = "api-15"))]
mod tests {
    use super::*;

    fn raw_rect(x: i32, y: i32, width: u32, height: u32) -> WindowManager_Rect {
        WindowManager_Rect {
            posX: x,
            posY: y,
            width,
            height,
        }
    }

    #[test]
    fn converts_rect_and_avoid_area() {
        let area = AvoidArea::from(WindowManager_AvoidArea {
            topRect: raw_rect(1, 2, 3, 4),
            leftRect: raw_rect(5, 6, 7, 8),
            rightRect: raw_rect(9, 10, 11, 12),
            bottomRect: raw_rect(13, 14, 15, 16),
        });

        assert_eq!(
            area.top,
            Rect {
                x: 1,
                y: 2,
                width: 3,
                height: 4
            }
        );
        assert_eq!(
            area.bottom,
            Rect {
                x: 13,
                y: 14,
                width: 15,
                height: 16
            }
        );
    }

    #[test]
    fn converts_window_types_from_sys_values() {
        let cases = [
            (
                WindowType::App,
                WindowManager_WindowType_WINDOW_MANAGER_WINDOW_TYPE_APP,
            ),
            (
                WindowType::Main,
                WindowManager_WindowType_WINDOW_MANAGER_WINDOW_TYPE_MAIN,
            ),
            (
                WindowType::Float,
                WindowManager_WindowType_WINDOW_MANAGER_WINDOW_TYPE_FLOAT,
            ),
            (
                WindowType::Dialog,
                WindowManager_WindowType_WINDOW_MANAGER_WINDOW_TYPE_DIALOG,
            ),
        ];

        for (window_type, raw) in cases {
            assert_eq!(WindowManager_WindowType::from(window_type), raw);
            assert_eq!(WindowType::try_from_raw(raw), Some(window_type));
        }
        assert_eq!(WindowType::try_from_raw(u32::MAX), None);
    }

    #[test]
    fn converts_avoid_area_types_to_sys_values() {
        let cases = [
            (
                AvoidAreaType::System,
                WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_SYSTEM,
            ),
            (
                AvoidAreaType::Cutout,
                WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_CUTOUT,
            ),
            (
                AvoidAreaType::SystemGesture,
                WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_SYSTEM_GESTURE,
            ),
            (
                AvoidAreaType::Keyboard,
                WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_KEYBOARD,
            ),
            (
                AvoidAreaType::NavigationIndicator,
                WindowManager_AvoidAreaType_WINDOW_MANAGER_AVOID_AREA_TYPE_NAVIGATION_INDICATOR,
            ),
        ];

        for (area_type, raw) in cases {
            assert_eq!(WindowManager_AvoidAreaType::from(area_type), raw);
            assert_eq!(AvoidAreaType::try_from_raw(raw), Some(area_type));
        }
    }
}
