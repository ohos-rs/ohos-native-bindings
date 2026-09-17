use ohos_window_manager_sys::{
    Input_KeyEvent, OH_NativeWindowManager_KeyEventFilter,
    OH_NativeWindowManager_RegisterKeyEventFilter, OH_NativeWindowManager_UnregisterKeyEventFilter,
};

#[cfg(feature = "api-15")]
use std::ffi::CString;
#[cfg(feature = "api-15")]
use std::mem::MaybeUninit;
#[cfg(feature = "api-20")]
use std::ptr::NonNull;

#[cfg(feature = "api-15")]
use ohos_image_native_binding::PixelMapNativeHandle;
#[cfg(feature = "api-20")]
use ohos_window_manager_sys::OH_WindowManager_InjectTouchEvent;
#[cfg(feature = "api-15")]
use ohos_window_manager_sys::{
    Input_MouseEvent, Input_TouchEvent, OH_NativeWindowManager_MouseEventFilter,
    OH_NativeWindowManager_RegisterMouseEventFilter,
    OH_NativeWindowManager_RegisterTouchEventFilter, OH_NativeWindowManager_TouchEventFilter,
    OH_NativeWindowManager_UnregisterMouseEventFilter,
    OH_NativeWindowManager_UnregisterTouchEventFilter, OH_WindowManager_GetWindowAvoidArea,
    OH_WindowManager_GetWindowProperties, OH_WindowManager_IsWindowShown,
    OH_WindowManager_SetWindowBackgroundColor, OH_WindowManager_SetWindowBrightness,
    OH_WindowManager_SetWindowFocusable, OH_WindowManager_SetWindowKeepScreenOn,
    OH_WindowManager_SetWindowNavigationBarEnabled, OH_WindowManager_SetWindowPrivacyMode,
    OH_WindowManager_SetWindowStatusBarColor, OH_WindowManager_SetWindowStatusBarEnabled,
    OH_WindowManager_SetWindowTouchable, OH_WindowManager_ShowWindow, OH_WindowManager_Snapshot,
    WindowManager_AvoidArea, WindowManager_WindowProperties,
};
#[cfg(feature = "api-26")]
use ohos_window_manager_sys::{
    OH_NativeWindowManager_GetKeyEventFilter, OH_NativeWindowManager_GetMouseEventFilter,
    OH_NativeWindowManager_GetTouchEventFilter,
    OH_WindowManager_RegisterFrameMetricsMeasuredCallback,
    OH_WindowManager_UnregisterFrameMetricsMeasuredCallback,
};
#[cfg(feature = "api-22")]
use ohos_window_manager_sys::{OH_WindowManager_LockCursor, OH_WindowManager_UnlockCursor};

#[cfg(feature = "api-15")]
use crate::error::{check, Error};
use crate::error::{check_status, Result};
#[cfg(feature = "api-15")]
use crate::types::{AvoidArea, AvoidAreaType, WindowProperties};
#[cfg(feature = "api-26")]
use crate::FrameMetricsMeasuredCallback;

pub type KeyEventFilter = unsafe extern "C" fn(*mut Input_KeyEvent) -> bool;
#[cfg(feature = "api-15")]
pub type MouseEventFilter = unsafe extern "C" fn(*mut Input_MouseEvent) -> bool;
#[cfg(feature = "api-15")]
pub type TouchEventFilter = unsafe extern "C" fn(*mut Input_TouchEvent) -> bool;

/// A borrowed handle to a window managed by OpenHarmony.
///
/// This type does not own or destroy the native window. Its ID must come from
/// the ArkTS window properties or another trusted platform API.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Window {
    id: i32,
}

impl Window {
    pub const fn from_id(id: i32) -> Self {
        Self { id }
    }

    pub const fn id(self) -> i32 {
        self.id
    }

    pub fn register_key_event_filter(self, filter: KeyEventFilter) -> Result<()> {
        let status = unsafe {
            OH_NativeWindowManager_RegisterKeyEventFilter(
                self.id,
                Some(filter) as OH_NativeWindowManager_KeyEventFilter,
            )
        };
        check_status(status)
    }

    pub fn unregister_key_event_filter(self) -> Result<()> {
        check_status(unsafe { OH_NativeWindowManager_UnregisterKeyEventFilter(self.id) })
    }

    #[cfg(feature = "api-15")]
    pub fn set_status_bar_enabled(self, enabled: bool, animate: bool) -> Result<()> {
        check(unsafe { OH_WindowManager_SetWindowStatusBarEnabled(self.id, enabled, animate) })
    }

    #[cfg(feature = "api-15")]
    pub fn set_status_bar_color(self, argb: u32) -> Result<()> {
        check(unsafe { OH_WindowManager_SetWindowStatusBarColor(self.id, argb as i32) })
    }

    #[cfg(feature = "api-15")]
    pub fn set_navigation_bar_enabled(self, enabled: bool, animate: bool) -> Result<()> {
        check(unsafe { OH_WindowManager_SetWindowNavigationBarEnabled(self.id, enabled, animate) })
    }

    #[cfg(feature = "api-15")]
    pub fn avoid_area(self, area_type: AvoidAreaType) -> Result<AvoidArea> {
        let mut raw = MaybeUninit::<WindowManager_AvoidArea>::uninit();
        check(unsafe {
            OH_WindowManager_GetWindowAvoidArea(self.id, area_type.into(), raw.as_mut_ptr())
        })?;
        Ok(unsafe { raw.assume_init() }.into())
    }

    #[cfg(feature = "api-15")]
    pub fn is_shown(self) -> Result<bool> {
        let mut shown = false;
        check(unsafe { OH_WindowManager_IsWindowShown(self.id, &mut shown) })?;
        Ok(shown)
    }

    #[cfg(feature = "api-15")]
    pub fn show(self) -> Result<()> {
        check(unsafe { OH_WindowManager_ShowWindow(self.id) })
    }

    #[cfg(feature = "api-15")]
    pub fn set_touchable(self, touchable: bool) -> Result<()> {
        check(unsafe { OH_WindowManager_SetWindowTouchable(self.id, touchable) })
    }

    #[cfg(feature = "api-15")]
    pub fn set_focusable(self, focusable: bool) -> Result<()> {
        check(unsafe { OH_WindowManager_SetWindowFocusable(self.id, focusable) })
    }

    #[cfg(feature = "api-15")]
    pub fn set_background_color(self, color: &str) -> Result<()> {
        let color = c_string(color)?;
        check(unsafe { OH_WindowManager_SetWindowBackgroundColor(self.id, color.as_ptr()) })
    }

    #[cfg(feature = "api-15")]
    pub fn set_brightness(self, brightness: f32) -> Result<()> {
        check(unsafe { OH_WindowManager_SetWindowBrightness(self.id, brightness) })
    }

    #[cfg(feature = "api-15")]
    pub fn set_keep_screen_on(self, keep_screen_on: bool) -> Result<()> {
        check(unsafe { OH_WindowManager_SetWindowKeepScreenOn(self.id, keep_screen_on) })
    }

    #[cfg(feature = "api-15")]
    pub fn set_privacy_mode(self, privacy: bool) -> Result<()> {
        check(unsafe { OH_WindowManager_SetWindowPrivacyMode(self.id, privacy) })
    }

    #[cfg(feature = "api-15")]
    pub fn properties(self) -> Result<WindowProperties> {
        let mut raw = MaybeUninit::<WindowManager_WindowProperties>::uninit();
        check(unsafe { OH_WindowManager_GetWindowProperties(self.id, raw.as_mut_ptr()) })?;
        Ok(unsafe { raw.assume_init() }.into())
    }

    #[cfg(feature = "api-15")]
    pub fn snapshot(self, pixel_map: PixelMapNativeHandle) -> Result<()> {
        check(unsafe { OH_WindowManager_Snapshot(self.id, pixel_map.as_raw().cast()) })
    }

    #[cfg(feature = "api-15")]
    pub fn register_mouse_event_filter(self, filter: MouseEventFilter) -> Result<()> {
        let status = unsafe {
            OH_NativeWindowManager_RegisterMouseEventFilter(
                self.id,
                Some(filter) as OH_NativeWindowManager_MouseEventFilter,
            )
        };
        check_status(status)
    }

    #[cfg(feature = "api-15")]
    pub fn unregister_mouse_event_filter(self) -> Result<()> {
        check_status(unsafe { OH_NativeWindowManager_UnregisterMouseEventFilter(self.id) })
    }

    #[cfg(feature = "api-15")]
    pub fn register_touch_event_filter(self, filter: TouchEventFilter) -> Result<()> {
        let status = unsafe {
            OH_NativeWindowManager_RegisterTouchEventFilter(
                self.id,
                Some(filter) as OH_NativeWindowManager_TouchEventFilter,
            )
        };
        check_status(status)
    }

    #[cfg(feature = "api-15")]
    pub fn unregister_touch_event_filter(self) -> Result<()> {
        check_status(unsafe { OH_NativeWindowManager_UnregisterTouchEventFilter(self.id) })
    }

    /// Injects a native touch event into this window.
    ///
    /// # Safety
    ///
    /// `event` must point to a live `Input_TouchEvent` created by the
    /// multimodal input API and must remain valid for the duration of the call.
    #[cfg(feature = "api-20")]
    pub unsafe fn inject_touch_event(
        self,
        event: NonNull<Input_TouchEvent>,
        window_x: i32,
        window_y: i32,
    ) -> Result<()> {
        check(unsafe {
            OH_WindowManager_InjectTouchEvent(self.id, event.as_ptr(), window_x, window_y)
        })
    }

    #[cfg(feature = "api-22")]
    pub fn lock_cursor(self, follow_movement: bool) -> Result<()> {
        check(unsafe { OH_WindowManager_LockCursor(self.id, follow_movement) })
    }

    #[cfg(feature = "api-22")]
    pub fn unlock_cursor(self) -> Result<()> {
        check(unsafe { OH_WindowManager_UnlockCursor(self.id) })
    }

    #[cfg(feature = "api-26")]
    pub fn key_event_filter(self) -> Result<Option<KeyEventFilter>> {
        let mut filter: OH_NativeWindowManager_KeyEventFilter = None;
        check_status(unsafe { OH_NativeWindowManager_GetKeyEventFilter(self.id, &mut filter) })?;
        Ok(filter)
    }

    #[cfg(feature = "api-26")]
    pub fn mouse_event_filter(self) -> Result<Option<MouseEventFilter>> {
        let mut filter: OH_NativeWindowManager_MouseEventFilter = None;
        check_status(unsafe { OH_NativeWindowManager_GetMouseEventFilter(self.id, &mut filter) })?;
        Ok(filter)
    }

    #[cfg(feature = "api-26")]
    pub fn touch_event_filter(self) -> Result<Option<TouchEventFilter>> {
        let mut filter: OH_NativeWindowManager_TouchEventFilter = None;
        check_status(unsafe { OH_NativeWindowManager_GetTouchEventFilter(self.id, &mut filter) })?;
        Ok(filter)
    }

    #[cfg(feature = "api-26")]
    pub fn register_frame_metrics_measured_callback(
        self,
        callback: FrameMetricsMeasuredCallback,
    ) -> Result<()> {
        check(unsafe {
            OH_WindowManager_RegisterFrameMetricsMeasuredCallback(self.id, Some(callback))
        })
    }

    #[cfg(feature = "api-26")]
    pub fn unregister_frame_metrics_measured_callback(
        self,
        callback: FrameMetricsMeasuredCallback,
    ) -> Result<()> {
        check(unsafe {
            OH_WindowManager_UnregisterFrameMetricsMeasuredCallback(self.id, Some(callback))
        })
    }
}

#[cfg(feature = "api-15")]
fn c_string(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| Error::InteriorNul)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_window_id() {
        let window = Window::from_id(42);
        assert_eq!(window.id(), 42);
    }

    #[cfg(feature = "api-15")]
    #[test]
    fn validates_strings_before_ffi() {
        assert_eq!(c_string("#ff00ff").unwrap().to_bytes(), b"#ff00ff");
        assert_eq!(c_string("#ff\0ff"), Err(Error::InteriorNul));
    }
}
