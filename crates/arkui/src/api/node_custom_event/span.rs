//! Module api::node_custom_event::span wrappers and related types.

use std::ptr::NonNull;

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_CustomSpanDrawInfo, ArkUI_CustomSpanMeasureInfo, ArkUI_CustomSpanMetrics,
    OH_ArkUI_CustomSpanDrawInfo_Create, OH_ArkUI_CustomSpanDrawInfo_Dispose,
    OH_ArkUI_CustomSpanDrawInfo_GetBaseline, OH_ArkUI_CustomSpanDrawInfo_GetLineBottom,
    OH_ArkUI_CustomSpanDrawInfo_GetLineTop, OH_ArkUI_CustomSpanDrawInfo_GetXOffset,
    OH_ArkUI_CustomSpanMeasureInfo_Create, OH_ArkUI_CustomSpanMeasureInfo_Dispose,
    OH_ArkUI_CustomSpanMeasureInfo_GetFontSize, OH_ArkUI_CustomSpanMetrics_Create,
    OH_ArkUI_CustomSpanMetrics_Dispose, OH_ArkUI_CustomSpanMetrics_SetHeight,
    OH_ArkUI_CustomSpanMetrics_SetWidth,
};

use crate::{check_arkui_status, ArkUIError, ArkUIResult};

/// Measurement input for custom text span callbacks.
pub struct CustomSpanMeasureInfo {
    raw: NonNull<ArkUI_CustomSpanMeasureInfo>,
}

impl CustomSpanMeasureInfo {
    /// Allocates a new measure-info object.
    pub fn new() -> ArkUIResult<Self> {
        let info = unsafe { OH_ArkUI_CustomSpanMeasureInfo_Create() };
        let info = NonNull::new(info).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CustomSpanMeasureInfo_Create returned null",
            )
        })?;
        Ok(Self::from_non_null(info))
    }

    fn from_non_null(raw: NonNull<ArkUI_CustomSpanMeasureInfo>) -> Self {
        Self { raw }
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_CustomSpanMeasureInfo) -> Self {
        Self::from_non_null(
            NonNull::new(raw)
                .unwrap_or_else(|| panic!("ArkUI_CustomSpanMeasureInfo pointer is null")),
        )
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_CustomSpanMeasureInfo {
        self.raw.as_ptr()
    }

    /// Releases the underlying native object.
    pub fn dispose(self) {
        unsafe { OH_ArkUI_CustomSpanMeasureInfo_Dispose(self.raw()) }
    }

    /// Returns current font size provided by layout engine.
    pub fn font_size(&self) -> f32 {
        unsafe { OH_ArkUI_CustomSpanMeasureInfo_GetFontSize(self.raw()) }
    }
}

/// Output metrics for custom text span measurement.
pub struct CustomSpanMetrics {
    raw: NonNull<ArkUI_CustomSpanMetrics>,
}

impl CustomSpanMetrics {
    /// Allocates a new custom-span metrics object.
    pub fn new() -> ArkUIResult<Self> {
        let metrics = unsafe { OH_ArkUI_CustomSpanMetrics_Create() };
        let metrics = NonNull::new(metrics).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CustomSpanMetrics_Create returned null",
            )
        })?;
        Ok(Self::from_non_null(metrics))
    }

    fn from_non_null(raw: NonNull<ArkUI_CustomSpanMetrics>) -> Self {
        Self { raw }
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_CustomSpanMetrics) -> Self {
        Self::from_non_null(
            NonNull::new(raw).unwrap_or_else(|| panic!("ArkUI_CustomSpanMetrics pointer is null")),
        )
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_CustomSpanMetrics {
        self.raw.as_ptr()
    }

    /// Releases the underlying native object.
    pub fn dispose(self) {
        unsafe { OH_ArkUI_CustomSpanMetrics_Dispose(self.raw()) }
    }

    /// Sets measured span width.
    pub fn set_width(&mut self, width: f32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_CustomSpanMetrics_SetWidth(self.raw(), width)) }
    }

    /// Sets measured span height.
    pub fn set_height(&mut self, height: f32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_CustomSpanMetrics_SetHeight(self.raw(), height)) }
    }
}

/// Drawing info for custom text span rendering callbacks.
pub struct CustomSpanDrawInfo {
    raw: NonNull<ArkUI_CustomSpanDrawInfo>,
}

impl CustomSpanDrawInfo {
    /// Allocates a new draw-info object.
    pub fn new() -> ArkUIResult<Self> {
        let info = unsafe { OH_ArkUI_CustomSpanDrawInfo_Create() };
        let info = NonNull::new(info).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CustomSpanDrawInfo_Create returned null",
            )
        })?;
        Ok(Self::from_non_null(info))
    }

    fn from_non_null(raw: NonNull<ArkUI_CustomSpanDrawInfo>) -> Self {
        Self { raw }
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_CustomSpanDrawInfo) -> Self {
        Self::from_non_null(
            NonNull::new(raw).unwrap_or_else(|| panic!("ArkUI_CustomSpanDrawInfo pointer is null")),
        )
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_CustomSpanDrawInfo {
        self.raw.as_ptr()
    }

    /// Releases the underlying native object.
    pub fn dispose(self) {
        unsafe { OH_ArkUI_CustomSpanDrawInfo_Dispose(self.raw()) }
    }

    /// Horizontal offset where this span should be drawn.
    pub fn x_offset(&self) -> f32 {
        unsafe { OH_ArkUI_CustomSpanDrawInfo_GetXOffset(self.raw()) }
    }

    /// Top line boundary for the span.
    pub fn line_top(&self) -> f32 {
        unsafe { OH_ArkUI_CustomSpanDrawInfo_GetLineTop(self.raw()) }
    }

    /// Bottom line boundary for the span.
    pub fn line_bottom(&self) -> f32 {
        unsafe { OH_ArkUI_CustomSpanDrawInfo_GetLineBottom(self.raw()) }
    }

    /// Baseline position for text alignment.
    pub fn baseline(&self) -> f32 {
        unsafe { OH_ArkUI_CustomSpanDrawInfo_GetBaseline(self.raw()) }
    }
}
