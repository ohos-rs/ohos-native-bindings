use std::marker::PhantomData;
use std::ptr::NonNull;

use ohos_native_window_manager_sys::{
    OH_WindowManager_FrameMetrics, OH_WindowManager_FrameMetrics_GetInputHandlingDuration,
    OH_WindowManager_FrameMetrics_GetLayoutMeasureDuration,
    OH_WindowManager_FrameMetrics_GetVsyncTimestamp,
    OH_WindowManager_FrameMetrics_IsFirstDrawFrame,
};

use crate::error::{check, Result};

pub type RawFrameMetrics = OH_WindowManager_FrameMetrics;
pub type FrameMetricsMeasuredCallback =
    unsafe extern "C" fn(window_id: i32, metrics: *const RawFrameMetrics);

#[derive(Clone, Copy, Debug)]
pub struct FrameMetrics<'a> {
    raw: NonNull<RawFrameMetrics>,
    marker: PhantomData<&'a RawFrameMetrics>,
}

impl<'a> FrameMetrics<'a> {
    /// Creates a borrowed metrics view from a native callback pointer.
    ///
    /// # Safety
    ///
    /// `raw` must remain valid and immutable for the returned view's lifetime.
    pub unsafe fn from_raw(raw: *const RawFrameMetrics) -> Option<Self> {
        NonNull::new(raw.cast_mut()).map(|raw| Self {
            raw,
            marker: PhantomData,
        })
    }

    pub fn is_first_draw_frame(self) -> Result<bool> {
        let mut value = false;
        check(unsafe {
            OH_WindowManager_FrameMetrics_IsFirstDrawFrame(self.as_raw(), &mut value)
        })?;
        Ok(value)
    }

    pub fn input_handling_duration(self) -> Result<u64> {
        self.read_duration(OH_WindowManager_FrameMetrics_GetInputHandlingDuration)
    }

    pub fn layout_measure_duration(self) -> Result<u64> {
        self.read_duration(OH_WindowManager_FrameMetrics_GetLayoutMeasureDuration)
    }

    pub fn vsync_timestamp(self) -> Result<u64> {
        self.read_duration(OH_WindowManager_FrameMetrics_GetVsyncTimestamp)
    }

    pub const fn as_raw(self) -> *const RawFrameMetrics {
        self.raw.as_ptr()
    }

    fn read_duration(
        self,
        getter: unsafe extern "C" fn(*const RawFrameMetrics, *mut u64) -> i32,
    ) -> Result<u64> {
        let mut value = 0;
        check(unsafe { getter(self.as_raw(), &mut value) })?;
        Ok(value)
    }
}
