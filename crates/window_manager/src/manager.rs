#[cfg(feature = "api-21")]
use std::ffi::CStr;
#[cfg(feature = "api-17")]
use std::ptr;

#[cfg(feature = "api-21")]
use ohos_native_window_manager_sys::{
    OH_PixelmapNative, OH_WindowManager_GetAllMainWindowInfo,
    OH_WindowManager_GetMainWindowSnapshot, OH_WindowManager_ReleaseAllMainWindowInfo,
    OH_WindowManager_ReleaseMainWindowSnapshot, WindowManager_MainWindowInfo,
    WindowManager_WindowSnapshotConfig,
};
#[cfg(feature = "api-17")]
use ohos_native_window_manager_sys::{
    OH_WindowManager_GetAllWindowLayoutInfoList, OH_WindowManager_ReleaseAllWindowLayoutInfoList,
};

#[cfg(feature = "api-17")]
use crate::error::{check, Error, Result};
#[cfg(feature = "api-21")]
use crate::types::MainWindowInfo;
#[cfg(feature = "api-17")]
use crate::types::Rect;

#[cfg(feature = "api-21")]
pub type RawPixelMap = OH_PixelmapNative;
#[cfg(feature = "api-21")]
pub type MainWindowSnapshotCallback =
    unsafe extern "C" fn(snapshot_list: *mut *const RawPixelMap, snapshot_count: usize);

#[cfg(feature = "api-21")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowSnapshotConfig {
    pub use_cache: bool,
}

#[cfg(feature = "api-21")]
impl Default for WindowSnapshotConfig {
    fn default() -> Self {
        Self { use_cache: true }
    }
}

#[cfg(feature = "api-21")]
impl From<WindowSnapshotConfig> for WindowManager_WindowSnapshotConfig {
    fn from(value: WindowSnapshotConfig) -> Self {
        Self {
            useCache: value.use_cache,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WindowManager;

impl WindowManager {
    #[cfg(feature = "api-17")]
    pub fn visible_window_layouts(display_id: i64) -> Result<Vec<Rect>> {
        let mut raw = ptr::null_mut();
        let mut len = 0;
        check(unsafe {
            OH_WindowManager_GetAllWindowLayoutInfoList(display_id, &mut raw, &mut len)
        })?;

        if len > 0 && raw.is_null() {
            return Err(Error::UnexpectedNull);
        }

        let layouts = if len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(raw, len) }
                .iter()
                .copied()
                .map(Rect::from)
                .collect()
        };
        if !raw.is_null() {
            unsafe { OH_WindowManager_ReleaseAllWindowLayoutInfoList(raw) };
        }
        Ok(layouts)
    }

    #[cfg(feature = "api-21")]
    pub fn main_windows() -> Result<Vec<MainWindowInfo>> {
        let mut raw = ptr::null_mut();
        let mut len = 0;
        check(unsafe { OH_WindowManager_GetAllMainWindowInfo(&mut raw, &mut len) })?;

        if len > 0 && raw.is_null() {
            return Err(Error::UnexpectedNull);
        }

        let windows = if len == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(raw, len) }
                .iter()
                .map(main_window_info_from_raw)
                .collect()
        };
        if !raw.is_null() {
            unsafe { OH_WindowManager_ReleaseAllMainWindowInfo(raw) };
        }
        Ok(windows)
    }

    /// Requests snapshots for the supplied main-window IDs.
    ///
    /// The native API invokes `callback` asynchronously. The callback must
    /// release the returned native list according to the platform contract.
    #[cfg(feature = "api-21")]
    pub fn request_main_window_snapshots(
        window_ids: &mut [i32],
        config: WindowSnapshotConfig,
        callback: MainWindowSnapshotCallback,
    ) -> Result<()> {
        check(unsafe {
            OH_WindowManager_GetMainWindowSnapshot(
                window_ids.as_mut_ptr(),
                window_ids.len(),
                config.into(),
                Some(callback),
            )
        })
    }

    /// Releases a snapshot list returned to a native snapshot callback.
    ///
    /// # Safety
    ///
    /// `snapshot_list` must be the exact pointer returned by
    /// `OH_WindowManager_GetMainWindowSnapshot`, and it must not have been
    /// released previously.
    #[cfg(feature = "api-21")]
    pub unsafe fn release_main_window_snapshots(snapshot_list: *const RawPixelMap) {
        unsafe { OH_WindowManager_ReleaseMainWindowSnapshot(snapshot_list) };
    }
}

#[cfg(feature = "api-21")]
fn main_window_info_from_raw(raw: &WindowManager_MainWindowInfo) -> MainWindowInfo {
    let label = if raw.label.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(raw.label) }
                .to_string_lossy()
                .into_owned(),
        )
    };
    MainWindowInfo {
        display_id: raw.displayId,
        window_id: raw.windowId,
        showing: raw.showing,
        label,
    }
}

#[cfg(all(test, feature = "api-21"))]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn copies_main_window_label() {
        let label = CString::new("main window").unwrap();
        let raw = WindowManager_MainWindowInfo {
            displayId: 7,
            windowId: 8,
            showing: true,
            label: label.as_ptr(),
        };

        assert_eq!(
            main_window_info_from_raw(&raw),
            MainWindowInfo {
                display_id: 7,
                window_id: 8,
                showing: true,
                label: Some("main window".to_string()),
            }
        );
    }

    #[test]
    fn accepts_missing_main_window_label() {
        let raw = WindowManager_MainWindowInfo {
            displayId: 1,
            windowId: 2,
            showing: false,
            label: ptr::null(),
        };

        assert_eq!(main_window_info_from_raw(&raw).label, None);
    }

    #[test]
    fn snapshot_config_defaults_to_using_cache() {
        assert!(WindowSnapshotConfig::default().use_cache);
    }
}
