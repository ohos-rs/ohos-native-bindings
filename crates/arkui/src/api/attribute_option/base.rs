//! Module api::attribute_option::base wrappers and related types.

use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
    ptr::NonNull,
};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::*;

use crate::check_arkui_status;
use crate::{ArkUIError, ArkUIResult};

pub(in crate::api::attribute_option) fn non_null_or_panic<T>(
    ptr: *mut T,
    name: &'static str,
) -> NonNull<T> {
    NonNull::new(ptr).unwrap_or_else(|| panic!("{name} pointer is null"))
}

pub(in crate::api::attribute_option) fn c_char_ptr_to_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() })
    }
}

pub(in crate::api::attribute_option) fn with_optional_cstring<F>(
    value: Option<&str>,
    f: F,
) -> ArkUIResult<()>
where
    F: FnOnce(*const c_char),
{
    let owned = if let Some(v) = value {
        Some(CString::new(v).map_err(|_| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "string contains interior NUL bytes",
            )
        })?)
    } else {
        None
    };
    let ptr = owned
        .as_ref()
        .map_or(std::ptr::null::<c_char>(), |v| v.as_ptr());
    f(ptr);
    Ok(())
}

pub(in crate::api::attribute_option) fn with_cstring<F>(value: &str, f: F) -> ArkUIResult<()>
where
    F: FnOnce(*const c_char),
{
    let value = CString::new(value).map_err(|_| {
        ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            "string contains interior NUL bytes",
        )
    })?;
    f(value.as_ptr());
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
/// Margin value object used by layout-related attributes.
pub struct Margin {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl From<ArkUI_Margin> for Margin {
    fn from(value: ArkUI_Margin) -> Self {
        Self {
            top: value.top,
            right: value.right,
            bottom: value.bottom,
            left: value.left,
        }
    }
}

impl From<Margin> for ArkUI_Margin {
    fn from(value: Margin) -> Self {
        Self {
            top: value.top,
            right: value.right,
            bottom: value.bottom,
            left: value.left,
        }
    }
}

/// Layout constraint wrapper for max/min size rules.
pub struct LayoutConstraint {
    raw: NonNull<ArkUI_LayoutConstraint>,
}

impl LayoutConstraint {
    pub fn new() -> ArkUIResult<Self> {
        let constraint = unsafe { OH_ArkUI_LayoutConstraint_Create() };
        NonNull::new(constraint)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_LayoutConstraint_Create returned null",
                )
            })
    }

    pub fn copy(&self) -> ArkUIResult<Self> {
        let copied = unsafe { OH_ArkUI_LayoutConstraint_Copy(self.raw()) };
        NonNull::new(copied)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_LayoutConstraint_Copy returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_LayoutConstraint {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_LayoutConstraint) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_LayoutConstraint"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_LayoutConstraint {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        let _ = unsafe { OH_ArkUI_LayoutConstraint_Dispose(self.raw()) };
    }

    pub fn get_max_width(&self) -> i32 {
        unsafe { OH_ArkUI_LayoutConstraint_GetMaxWidth(self.raw()) }
    }

    pub fn get_min_width(&self) -> i32 {
        unsafe { OH_ArkUI_LayoutConstraint_GetMinWidth(self.raw()) }
    }

    pub fn get_max_height(&self) -> i32 {
        unsafe { OH_ArkUI_LayoutConstraint_GetMaxHeight(self.raw()) }
    }

    pub fn get_min_height(&self) -> i32 {
        unsafe { OH_ArkUI_LayoutConstraint_GetMinHeight(self.raw()) }
    }

    pub fn get_percent_reference_width(&self) -> i32 {
        unsafe { OH_ArkUI_LayoutConstraint_GetPercentReferenceWidth(self.raw()) }
    }

    pub fn get_percent_reference_height(&self) -> i32 {
        unsafe { OH_ArkUI_LayoutConstraint_GetPercentReferenceHeight(self.raw()) }
    }

    pub fn set_max_width(&mut self, value: i32) {
        unsafe { OH_ArkUI_LayoutConstraint_SetMaxWidth(self.raw(), value) }
    }

    pub fn set_min_width(&mut self, value: i32) {
        unsafe { OH_ArkUI_LayoutConstraint_SetMinWidth(self.raw(), value) }
    }

    pub fn set_max_height(&mut self, value: i32) {
        unsafe { OH_ArkUI_LayoutConstraint_SetMaxHeight(self.raw(), value) }
    }

    pub fn set_min_height(&mut self, value: i32) {
        unsafe { OH_ArkUI_LayoutConstraint_SetMinHeight(self.raw(), value) }
    }

    pub fn set_percent_reference_width(&mut self, value: i32) {
        unsafe { OH_ArkUI_LayoutConstraint_SetPercentReferenceWidth(self.raw(), value) }
    }

    pub fn set_percent_reference_height(&mut self, value: i32) {
        unsafe { OH_ArkUI_LayoutConstraint_SetPercentReferenceHeight(self.raw(), value) }
    }
}

/// Alignment-rule option wrapper for relative container layout.
pub struct AlignmentRuleOption {
    raw: NonNull<ArkUI_AlignmentRuleOption>,
}

impl AlignmentRuleOption {
    pub fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_AlignmentRuleOption_Create() };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_AlignmentRuleOption_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_AlignmentRuleOption {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_AlignmentRuleOption) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_AlignmentRuleOption"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_AlignmentRuleOption {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_AlignmentRuleOption_Dispose(self.raw()) }
    }

    pub fn set_start(
        &mut self,
        id: Option<&str>,
        alignment: crate::HorizontalAlignment,
    ) -> ArkUIResult<()> {
        with_optional_cstring(id, |id_ptr| unsafe {
            OH_ArkUI_AlignmentRuleOption_SetStart(self.raw(), id_ptr, alignment.into())
        })
    }

    pub fn set_end(
        &mut self,
        id: Option<&str>,
        alignment: crate::HorizontalAlignment,
    ) -> ArkUIResult<()> {
        with_optional_cstring(id, |id_ptr| unsafe {
            OH_ArkUI_AlignmentRuleOption_SetEnd(self.raw(), id_ptr, alignment.into())
        })
    }

    pub fn set_center_horizontal(
        &mut self,
        id: Option<&str>,
        alignment: crate::HorizontalAlignment,
    ) -> ArkUIResult<()> {
        with_optional_cstring(id, |id_ptr| unsafe {
            OH_ArkUI_AlignmentRuleOption_SetCenterHorizontal(self.raw(), id_ptr, alignment.into())
        })
    }

    pub fn set_top(
        &mut self,
        id: Option<&str>,
        alignment: crate::VerticalAlignment,
    ) -> ArkUIResult<()> {
        with_optional_cstring(id, |id_ptr| unsafe {
            OH_ArkUI_AlignmentRuleOption_SetTop(self.raw(), id_ptr, alignment.into())
        })
    }

    pub fn set_bottom(
        &mut self,
        id: Option<&str>,
        alignment: crate::VerticalAlignment,
    ) -> ArkUIResult<()> {
        with_optional_cstring(id, |id_ptr| unsafe {
            OH_ArkUI_AlignmentRuleOption_SetBottom(self.raw(), id_ptr, alignment.into())
        })
    }

    pub fn set_center_vertical(
        &mut self,
        id: Option<&str>,
        alignment: crate::VerticalAlignment,
    ) -> ArkUIResult<()> {
        with_optional_cstring(id, |id_ptr| unsafe {
            OH_ArkUI_AlignmentRuleOption_SetCenterVertical(self.raw(), id_ptr, alignment.into())
        })
    }

    pub fn set_bias_horizontal(&mut self, bias: f32) {
        unsafe { OH_ArkUI_AlignmentRuleOption_SetBiasHorizontal(self.raw(), bias) }
    }

    pub fn set_bias_vertical(&mut self, bias: f32) {
        unsafe { OH_ArkUI_AlignmentRuleOption_SetBiasVertical(self.raw(), bias) }
    }

    pub fn get_start_id(&self) -> Option<String> {
        c_char_ptr_to_string(unsafe { OH_ArkUI_AlignmentRuleOption_GetStartId(self.raw()) })
    }

    pub fn get_start_alignment(&self) -> crate::HorizontalAlignment {
        unsafe { OH_ArkUI_AlignmentRuleOption_GetStartAlignment(self.raw()).into() }
    }

    pub fn get_end_id(&self) -> Option<String> {
        c_char_ptr_to_string(unsafe { OH_ArkUI_AlignmentRuleOption_GetEndId(self.raw()) })
    }

    pub fn get_end_alignment(&self) -> crate::HorizontalAlignment {
        unsafe { OH_ArkUI_AlignmentRuleOption_GetEndAlignment(self.raw()).into() }
    }

    pub fn get_center_id_horizontal(&self) -> Option<String> {
        c_char_ptr_to_string(unsafe {
            OH_ArkUI_AlignmentRuleOption_GetCenterIdHorizontal(self.raw())
        })
    }

    pub fn get_center_alignment_horizontal(&self) -> crate::HorizontalAlignment {
        unsafe { OH_ArkUI_AlignmentRuleOption_GetCenterAlignmentHorizontal(self.raw()).into() }
    }

    pub fn get_top_id(&self) -> Option<String> {
        c_char_ptr_to_string(unsafe { OH_ArkUI_AlignmentRuleOption_GetTopId(self.raw()) })
    }

    pub fn get_top_alignment(&self) -> crate::VerticalAlignment {
        unsafe { OH_ArkUI_AlignmentRuleOption_GetTopAlignment(self.raw()).into() }
    }

    pub fn get_bottom_id(&self) -> Option<String> {
        c_char_ptr_to_string(unsafe { OH_ArkUI_AlignmentRuleOption_GetBottomId(self.raw()) })
    }

    pub fn get_bottom_alignment(&self) -> crate::VerticalAlignment {
        unsafe { OH_ArkUI_AlignmentRuleOption_GetBottomAlignment(self.raw()).into() }
    }

    pub fn get_center_id_vertical(&self) -> Option<String> {
        c_char_ptr_to_string(unsafe {
            OH_ArkUI_AlignmentRuleOption_GetCenterIdVertical(self.raw())
        })
    }

    pub fn get_center_alignment_vertical(&self) -> crate::VerticalAlignment {
        unsafe { OH_ArkUI_AlignmentRuleOption_GetCenterAlignmentVertical(self.raw()).into() }
    }

    pub fn get_bias_horizontal(&self) -> f32 {
        unsafe { OH_ArkUI_AlignmentRuleOption_GetBiasHorizontal(self.raw()) }
    }

    pub fn get_bias_vertical(&self) -> f32 {
        unsafe { OH_ArkUI_AlignmentRuleOption_GetBiasVertical(self.raw()) }
    }
}

/// Accessibility value wrapper exposed by ArkUI attributes.
pub struct AccessibilityValue {
    raw: NonNull<ArkUI_AccessibilityValue>,
}

impl AccessibilityValue {
    pub fn new() -> ArkUIResult<Self> {
        let value = unsafe { OH_ArkUI_AccessibilityValue_Create() };
        NonNull::new(value)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_AccessibilityValue_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_AccessibilityValue {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_AccessibilityValue) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_AccessibilityValue"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_AccessibilityValue {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_AccessibilityValue_Dispose(self.raw()) }
    }

    pub fn set_min(&mut self, min: i32) {
        unsafe { OH_ArkUI_AccessibilityValue_SetMin(self.raw(), min) }
    }

    pub fn get_min(&self) -> i32 {
        unsafe { OH_ArkUI_AccessibilityValue_GetMin(self.raw()) }
    }

    pub fn set_max(&mut self, max: i32) {
        unsafe { OH_ArkUI_AccessibilityValue_SetMax(self.raw(), max) }
    }

    pub fn get_max(&self) -> i32 {
        unsafe { OH_ArkUI_AccessibilityValue_GetMax(self.raw()) }
    }

    pub fn set_current(&mut self, current: i32) {
        unsafe { OH_ArkUI_AccessibilityValue_SetCurrent(self.raw(), current) }
    }

    pub fn get_current(&self) -> i32 {
        unsafe { OH_ArkUI_AccessibilityValue_GetCurrent(self.raw()) }
    }

    #[cfg(feature = "api-18")]
    pub fn set_range_min(&mut self, range_min: i32) {
        unsafe { OH_ArkUI_AccessibilityValue_SetRangeMin(self.raw(), range_min) }
    }

    #[cfg(feature = "api-18")]
    pub fn get_range_min(&self) -> i32 {
        unsafe { OH_ArkUI_AccessibilityValue_GetRangeMin(self.raw()) }
    }

    #[cfg(feature = "api-18")]
    pub fn set_range_max(&mut self, range_max: i32) {
        unsafe { OH_ArkUI_AccessibilityValue_SetRangeMax(self.raw(), range_max) }
    }

    #[cfg(feature = "api-18")]
    pub fn get_range_max(&self) -> i32 {
        unsafe { OH_ArkUI_AccessibilityValue_GetRangeMax(self.raw()) }
    }

    #[cfg(feature = "api-18")]
    pub fn set_range_current(&mut self, range_current: i32) {
        unsafe { OH_ArkUI_AccessibilityValue_SetRangeCurrent(self.raw(), range_current) }
    }

    #[cfg(feature = "api-18")]
    pub fn get_range_current(&self) -> i32 {
        unsafe { OH_ArkUI_AccessibilityValue_GetRangeCurrent(self.raw()) }
    }

    pub fn set_text(&mut self, text: &str) -> ArkUIResult<()> {
        with_cstring(text, |text_ptr| unsafe {
            OH_ArkUI_AccessibilityValue_SetText(self.raw(), text_ptr)
        })
    }

    pub fn get_text(&self) -> Option<String> {
        c_char_ptr_to_string(unsafe { OH_ArkUI_AccessibilityValue_GetText(self.raw()) })
    }
}

struct WaterFlowMainSizeCallbackContext {
    callback: Box<dyn Fn(i32) -> f32>,
}

/// Section option wrapper for water-flow components.
pub struct WaterFlowSectionOption {
    raw: NonNull<ArkUI_WaterFlowSectionOption>,
    main_size_callbacks: HashMap<i32, *mut WaterFlowMainSizeCallbackContext>,
}

impl WaterFlowSectionOption {
    pub fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_WaterFlowSectionOption_Create() };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_WaterFlowSectionOption_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_WaterFlowSectionOption {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_WaterFlowSectionOption) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_WaterFlowSectionOption"),
            main_size_callbacks: HashMap::new(),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_WaterFlowSectionOption {
        self.raw.as_ptr()
    }

    pub fn dispose(mut self) {
        self.clear_get_item_main_size_callback_by_index_all();
        unsafe { OH_ArkUI_WaterFlowSectionOption_Dispose(self.raw()) }
    }

    pub fn set_size(&mut self, size: i32) {
        unsafe { OH_ArkUI_WaterFlowSectionOption_SetSize(self.raw(), size) }
    }

    pub fn get_size(&self) -> i32 {
        unsafe { OH_ArkUI_WaterFlowSectionOption_GetSize(self.raw()) }
    }

    pub fn set_item_count(&mut self, index: i32, item_count: i32) {
        unsafe { OH_ArkUI_WaterFlowSectionOption_SetItemCount(self.raw(), index, item_count) }
    }

    pub fn get_item_count(&self, index: i32) -> i32 {
        unsafe { OH_ArkUI_WaterFlowSectionOption_GetItemCount(self.raw(), index) }
    }

    pub fn register_get_item_main_size_callback_by_index<T: Fn(i32) -> f32 + 'static>(
        &mut self,
        index: i32,
        callback: T,
    ) {
        self.clear_get_item_main_size_callback_by_index(index);
        let callback = Box::into_raw(Box::new(WaterFlowMainSizeCallbackContext {
            callback: Box::new(callback),
        }));
        unsafe {
            OH_ArkUI_WaterFlowSectionOption_RegisterGetItemMainSizeCallbackByIndexWithUserData(
                self.raw(),
                index,
                callback.cast(),
                Some(water_flow_main_size_callback_trampoline),
            );
        }
        self.main_size_callbacks.insert(index, callback);
    }

    pub fn clear_get_item_main_size_callback_by_index(&mut self, index: i32) {
        unsafe {
            OH_ArkUI_WaterFlowSectionOption_RegisterGetItemMainSizeCallbackByIndexWithUserData(
                self.raw(),
                index,
                std::ptr::null_mut(),
                None,
            );
        }
        if let Some(callback) = self.main_size_callbacks.remove(&index) {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
    }

    pub fn clear_get_item_main_size_callback_by_index_all(&mut self) {
        let indexes: Vec<i32> = self.main_size_callbacks.keys().copied().collect();
        for index in indexes {
            self.clear_get_item_main_size_callback_by_index(index);
        }
    }

    pub fn set_cross_count(&mut self, index: i32, cross_count: i32) {
        unsafe { OH_ArkUI_WaterFlowSectionOption_SetCrossCount(self.raw(), index, cross_count) }
    }

    pub fn get_cross_count(&self, index: i32) -> i32 {
        unsafe { OH_ArkUI_WaterFlowSectionOption_GetCrossCount(self.raw(), index) }
    }

    pub fn set_column_gap(&mut self, index: i32, column_gap: f32) {
        unsafe { OH_ArkUI_WaterFlowSectionOption_SetColumnGap(self.raw(), index, column_gap) }
    }

    pub fn get_column_gap(&self, index: i32) -> f32 {
        unsafe { OH_ArkUI_WaterFlowSectionOption_GetColumnGap(self.raw(), index) }
    }

    pub fn set_row_gap(&mut self, index: i32, row_gap: f32) {
        unsafe { OH_ArkUI_WaterFlowSectionOption_SetRowGap(self.raw(), index, row_gap) }
    }

    pub fn get_row_gap(&self, index: i32) -> f32 {
        unsafe { OH_ArkUI_WaterFlowSectionOption_GetRowGap(self.raw(), index) }
    }

    pub fn set_margin(
        &mut self,
        index: i32,
        margin_top: f32,
        margin_right: f32,
        margin_bottom: f32,
        margin_left: f32,
    ) {
        unsafe {
            OH_ArkUI_WaterFlowSectionOption_SetMargin(
                self.raw(),
                index,
                margin_top,
                margin_right,
                margin_bottom,
                margin_left,
            )
        }
    }

    pub fn get_margin(&self, index: i32) -> Margin {
        unsafe { OH_ArkUI_WaterFlowSectionOption_GetMargin(self.raw(), index) }.into()
    }
}

unsafe extern "C" fn water_flow_main_size_callback_trampoline(
    item_index: i32,
    user_data: *mut c_void,
) -> f32 {
    if user_data.is_null() {
        return 0.0;
    }

    let callback = unsafe { &*(user_data as *mut WaterFlowMainSizeCallbackContext) };
    (callback.callback)(item_index)
}

pub use ohos_image_native_binding::PixelMapNativeHandle;

/// Wrapper for drawable descriptor handles.
pub struct DrawableDescriptor {
    raw: NonNull<ArkUI_DrawableDescriptor>,
}

impl DrawableDescriptor {
    pub fn from_pixel_map(pixel_map: PixelMapNativeHandle) -> ArkUIResult<Self> {
        let descriptor =
            unsafe { OH_ArkUI_DrawableDescriptor_CreateFromPixelMap(pixel_map.as_raw().cast()) };
        NonNull::new(descriptor)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_DrawableDescriptor_CreateFromPixelMap returned null",
                )
            })
    }

    pub fn from_animated_pixel_map(pixel_map_array: &[PixelMapNativeHandle]) -> ArkUIResult<Self> {
        let mut raw_pixel_map_array: Vec<OH_PixelmapNativeHandle> = pixel_map_array
            .iter()
            .map(|pixel_map| pixel_map.as_raw().cast())
            .collect();
        let array_ptr = if raw_pixel_map_array.is_empty() {
            std::ptr::null_mut()
        } else {
            raw_pixel_map_array.as_mut_ptr()
        };
        let descriptor = unsafe {
            OH_ArkUI_DrawableDescriptor_CreateFromAnimatedPixelMap(
                array_ptr,
                raw_pixel_map_array.len() as i32,
            )
        };
        NonNull::new(descriptor)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_DrawableDescriptor_CreateFromAnimatedPixelMap returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_DrawableDescriptor {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_DrawableDescriptor) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_DrawableDescriptor"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_DrawableDescriptor {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_DrawableDescriptor_Dispose(self.raw()) }
    }

    pub fn get_static_pixel_map(&self) -> Option<PixelMapNativeHandle> {
        PixelMapNativeHandle::from_raw(
            unsafe { OH_ArkUI_DrawableDescriptor_GetStaticPixelMap(self.raw()) }.cast(),
        )
    }

    pub fn get_animated_pixel_maps(&self) -> Vec<PixelMapNativeHandle> {
        let size = self.get_animated_pixel_map_array_size();
        if size <= 0 {
            return Vec::new();
        }
        let ptr = unsafe { OH_ArkUI_DrawableDescriptor_GetAnimatedPixelMapArray(self.raw()) };
        if ptr.is_null() {
            return Vec::new();
        }
        let size = size as usize;
        unsafe { std::slice::from_raw_parts(ptr, size) }
            .iter()
            .filter_map(|pixel_map| PixelMapNativeHandle::from_raw((*pixel_map).cast()))
            .collect()
    }

    pub fn get_animated_pixel_map_array_size(&self) -> i32 {
        unsafe { OH_ArkUI_DrawableDescriptor_GetAnimatedPixelMapArraySize(self.raw()) }
    }

    pub fn set_animation_duration(&mut self, duration: i32) {
        unsafe { OH_ArkUI_DrawableDescriptor_SetAnimationDuration(self.raw(), duration) }
    }

    pub fn get_animation_duration(&self) -> i32 {
        unsafe { OH_ArkUI_DrawableDescriptor_GetAnimationDuration(self.raw()) }
    }

    pub fn set_animation_iteration(&mut self, iteration: i32) {
        unsafe { OH_ArkUI_DrawableDescriptor_SetAnimationIteration(self.raw(), iteration) }
    }

    pub fn get_animation_iteration(&self) -> i32 {
        unsafe { OH_ArkUI_DrawableDescriptor_GetAnimationIteration(self.raw()) }
    }

    #[cfg(feature = "api-22")]
    pub fn set_animation_frame_durations(&mut self, durations: &mut [u32]) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DrawableDescriptor_SetAnimationFrameDurations(
                self.raw(),
                durations.as_mut_ptr(),
                durations.len()
            ))
        }
    }

    #[cfg(feature = "api-22")]
    pub fn get_animation_frame_durations(&self, durations: &mut [u32]) -> ArkUIResult<usize> {
        let mut size = durations.len();
        unsafe {
            check_arkui_status!(OH_ArkUI_DrawableDescriptor_GetAnimationFrameDurations(
                self.raw(),
                durations.as_mut_ptr(),
                &mut size
            ))
        }?;
        Ok(size)
    }

    #[cfg(feature = "api-22")]
    pub fn set_animation_auto_play(&mut self, auto_play: u32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_DrawableDescriptor_SetAnimationAutoPlay(
                self.raw(),
                auto_play
            ))
        }
    }

    #[cfg(feature = "api-22")]
    pub fn get_animation_auto_play(&self) -> ArkUIResult<u32> {
        let mut auto_play = 0;
        unsafe {
            check_arkui_status!(OH_ArkUI_DrawableDescriptor_GetAnimationAutoPlay(
                self.raw(),
                &mut auto_play
            ))
        }?;
        Ok(auto_play)
    }

    #[cfg(feature = "api-22")]
    pub fn create_animation_controller(
        &self,
        node: &crate::ArkUINode,
    ) -> ArkUIResult<DrawableDescriptorAnimationController> {
        let mut controller = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(OH_ArkUI_DrawableDescriptor_CreateAnimationController(
                self.raw(),
                node.raw(),
                &mut controller
            ))
        }?;
        NonNull::new(controller)
            .map(|raw| DrawableDescriptorAnimationController::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_DrawableDescriptor_CreateAnimationController returned null",
                )
            })
    }
}

#[cfg(feature = "api-22")]
/// Controller wrapper for drawable descriptor animations.
pub struct DrawableDescriptorAnimationController {
    raw: NonNull<ArkUI_DrawableDescriptor_AnimationController>,
}

#[cfg(feature = "api-22")]
impl DrawableDescriptorAnimationController {
    pub(crate) fn raw(&self) -> *mut ArkUI_DrawableDescriptor_AnimationController {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_DrawableDescriptor_AnimationController) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_DrawableDescriptor_AnimationController"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_DrawableDescriptor_AnimationController {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_DrawableDescriptor_DisposeAnimationController(self.raw()) }
    }

    pub fn start_animation(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DrawableDescriptor_StartAnimation(self.raw())) }
    }

    pub fn stop_animation(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DrawableDescriptor_StopAnimation(self.raw())) }
    }

    pub fn resume_animation(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DrawableDescriptor_ResumeAnimation(self.raw())) }
    }

    pub fn pause_animation(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_DrawableDescriptor_PauseAnimation(self.raw())) }
    }

    pub fn get_animation_status(&self) -> ArkUIResult<DrawableDescriptor_AnimationStatus> {
        let mut status =
            DrawableDescriptor_AnimationStatus_DRAWABLE_DESCRIPTOR_ANIMATION_STATUS_INITIAL;
        unsafe {
            check_arkui_status!(OH_ArkUI_DrawableDescriptor_GetAnimationStatus(
                self.raw(),
                &mut status
            ))
        }?;
        Ok(status)
    }
}

/// Swiper indicator option wrapper.
pub struct SwiperIndicator {
    raw: NonNull<ArkUI_SwiperIndicator>,
}

impl SwiperIndicator {
    pub fn new(type_: crate::SwiperIndicatorType) -> ArkUIResult<Self> {
        let indicator = unsafe { OH_ArkUI_SwiperIndicator_Create(type_.into()) };
        NonNull::new(indicator)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_SwiperIndicator_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_SwiperIndicator {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_SwiperIndicator) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_SwiperIndicator"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_SwiperIndicator {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_SwiperIndicator_Dispose(self.raw()) }
    }

    pub fn set_start_position(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetStartPosition(self.raw(), value) }
    }

    pub fn get_start_position(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetStartPosition(self.raw()) }
    }

    pub fn set_top_position(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetTopPosition(self.raw(), value) }
    }

    pub fn get_top_position(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetTopPosition(self.raw()) }
    }

    pub fn set_end_position(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetEndPosition(self.raw(), value) }
    }

    pub fn get_end_position(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetEndPosition(self.raw()) }
    }

    pub fn set_bottom_position(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetBottomPosition(self.raw(), value) }
    }

    pub fn get_bottom_position(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetBottomPosition(self.raw()) }
    }

    pub fn set_item_width(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetItemWidth(self.raw(), value) }
    }

    pub fn get_item_width(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetItemWidth(self.raw()) }
    }

    pub fn set_item_height(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetItemHeight(self.raw(), value) }
    }

    pub fn get_item_height(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetItemHeight(self.raw()) }
    }

    pub fn set_selected_item_width(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetSelectedItemWidth(self.raw(), value) }
    }

    pub fn get_selected_item_width(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetSelectedItemWidth(self.raw()) }
    }

    pub fn set_selected_item_height(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetSelectedItemHeight(self.raw(), value) }
    }

    pub fn get_selected_item_height(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetSelectedItemHeight(self.raw()) }
    }

    pub fn set_mask(&mut self, mask: i32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetMask(self.raw(), mask) }
    }

    pub fn get_mask(&self) -> i32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetMask(self.raw()) }
    }

    pub fn set_color(&mut self, color: u32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetColor(self.raw(), color) }
    }

    pub fn get_color(&self) -> u32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetColor(self.raw()) }
    }

    pub fn set_selected_color(&mut self, color: u32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetSelectedColor(self.raw(), color) }
    }

    pub fn get_selected_color(&self) -> u32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetSelectedColor(self.raw()) }
    }

    pub fn set_max_display_count(&mut self, max_display_count: i32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_SwiperIndicator_SetMaxDisplayCount(
                self.raw(),
                max_display_count
            ))
        }
    }

    pub fn get_max_display_count(&self) -> i32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetMaxDisplayCount(self.raw()) }
    }

    #[cfg(feature = "api-19")]
    pub fn set_ignore_size_of_bottom(&mut self, ignore_size: i32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetIgnoreSizeOfBottom(self.raw(), ignore_size) }
    }

    #[cfg(feature = "api-19")]
    pub fn get_ignore_size_of_bottom(&self) -> i32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetIgnoreSizeOfBottom(self.raw()) }
    }

    #[cfg(feature = "api-19")]
    pub fn set_space(&mut self, space: f32) {
        unsafe { OH_ArkUI_SwiperIndicator_SetSpace(self.raw(), space) }
    }

    #[cfg(feature = "api-19")]
    pub fn get_space(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperIndicator_GetSpace(self.raw()) }
    }
}

#[cfg(feature = "api-19")]
/// Digit-style swiper indicator option wrapper.
pub struct SwiperDigitIndicator {
    raw: NonNull<ArkUI_SwiperDigitIndicator>,
}

#[cfg(feature = "api-19")]
impl SwiperDigitIndicator {
    pub fn new() -> ArkUIResult<Self> {
        let indicator = unsafe { OH_ArkUI_SwiperDigitIndicator_Create() };
        NonNull::new(indicator)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_SwiperDigitIndicator_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_SwiperDigitIndicator {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_SwiperDigitIndicator) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_SwiperDigitIndicator"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_SwiperDigitIndicator {
        self.raw.as_ptr()
    }

    pub fn destroy(self) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_Destroy(self.raw()) }
    }

    pub fn set_start_position(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetStartPosition(self.raw(), value) }
    }

    pub fn get_start_position(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetStartPosition(self.raw()) }
    }

    pub fn set_top_position(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetTopPosition(self.raw(), value) }
    }

    pub fn get_top_position(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetTopPosition(self.raw()) }
    }

    pub fn set_end_position(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetEndPosition(self.raw(), value) }
    }

    pub fn get_end_position(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetEndPosition(self.raw()) }
    }

    pub fn set_bottom_position(&mut self, value: f32) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetBottomPosition(self.raw(), value) }
    }

    pub fn get_bottom_position(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetBottomPosition(self.raw()) }
    }

    pub fn set_font_color(&mut self, color: u32) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetFontColor(self.raw(), color) }
    }

    pub fn get_font_color(&self) -> u32 {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetFontColor(self.raw()) }
    }

    pub fn set_selected_font_color(&mut self, color: u32) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetSelectedFontColor(self.raw(), color) }
    }

    pub fn get_selected_font_color(&self) -> u32 {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetSelectedFontColor(self.raw()) }
    }

    pub fn set_font_size(&mut self, size: f32) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetFontSize(self.raw(), size) }
    }

    pub fn get_font_size(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetFontSize(self.raw()) }
    }

    pub fn set_selected_font_size(&mut self, size: f32) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetSelectedFontSize(self.raw(), size) }
    }

    pub fn get_selected_font_size(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetSelectedFontSize(self.raw()) }
    }

    pub fn set_font_weight(&mut self, weight: crate::FontWeight) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetFontWeight(self.raw(), weight.into()) }
    }

    pub fn get_font_weight(&self) -> crate::FontWeight {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetFontWeight(self.raw()).into() }
    }

    pub fn set_selected_font_weight(&mut self, weight: crate::FontWeight) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetSelectedFontWeight(self.raw(), weight.into()) }
    }

    pub fn get_selected_font_weight(&self) -> crate::FontWeight {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetSelectedFontWeight(self.raw()).into() }
    }

    pub fn set_ignore_size_of_bottom(&mut self, ignore_size: i32) {
        unsafe { OH_ArkUI_SwiperDigitIndicator_SetIgnoreSizeOfBottom(self.raw(), ignore_size) }
    }

    pub fn get_ignore_size_of_bottom(&self) -> i32 {
        unsafe { OH_ArkUI_SwiperDigitIndicator_GetIgnoreSizeOfBottom(self.raw()) }
    }
}

#[cfg(feature = "api-19")]
/// Arrow style option wrapper for swiper controls.
pub struct SwiperArrowStyle {
    raw: NonNull<ArkUI_SwiperArrowStyle>,
}

#[cfg(feature = "api-19")]
impl SwiperArrowStyle {
    pub fn new() -> ArkUIResult<Self> {
        let style = unsafe { OH_ArkUI_SwiperArrowStyle_Create() };
        NonNull::new(style)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_SwiperArrowStyle_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_SwiperArrowStyle {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_SwiperArrowStyle) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_SwiperArrowStyle"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_SwiperArrowStyle {
        self.raw.as_ptr()
    }

    pub fn destroy(self) {
        unsafe { OH_ArkUI_SwiperArrowStyle_Destroy(self.raw()) }
    }

    pub fn set_show_background(&mut self, show: i32) {
        unsafe { OH_ArkUI_SwiperArrowStyle_SetShowBackground(self.raw(), show) }
    }

    pub fn get_show_background(&self) -> i32 {
        unsafe { OH_ArkUI_SwiperArrowStyle_GetShowBackground(self.raw()) }
    }

    pub fn set_show_sidebar_middle(&mut self, show: i32) {
        unsafe { OH_ArkUI_SwiperArrowStyle_SetShowSidebarMiddle(self.raw(), show) }
    }

    pub fn get_show_sidebar_middle(&self) -> i32 {
        unsafe { OH_ArkUI_SwiperArrowStyle_GetShowSidebarMiddle(self.raw()) }
    }

    pub fn set_background_size(&mut self, size: f32) {
        unsafe { OH_ArkUI_SwiperArrowStyle_SetBackgroundSize(self.raw(), size) }
    }

    pub fn get_background_size(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperArrowStyle_GetBackgroundSize(self.raw()) }
    }

    pub fn set_background_color(&mut self, color: u32) {
        unsafe { OH_ArkUI_SwiperArrowStyle_SetBackgroundColor(self.raw(), color) }
    }

    pub fn get_background_color(&self) -> u32 {
        unsafe { OH_ArkUI_SwiperArrowStyle_GetBackgroundColor(self.raw()) }
    }

    pub fn set_arrow_size(&mut self, size: f32) {
        unsafe { OH_ArkUI_SwiperArrowStyle_SetArrowSize(self.raw(), size) }
    }

    pub fn get_arrow_size(&self) -> f32 {
        unsafe { OH_ArkUI_SwiperArrowStyle_GetArrowSize(self.raw()) }
    }

    pub fn set_arrow_color(&mut self, color: u32) {
        unsafe { OH_ArkUI_SwiperArrowStyle_SetArrowColor(self.raw(), color) }
    }

    pub fn get_arrow_color(&self) -> u32 {
        unsafe { OH_ArkUI_SwiperArrowStyle_GetArrowColor(self.raw()) }
    }
}

/// Frame descriptor used by image animator options.
pub struct ImageAnimatorFrameInfo {
    raw: NonNull<ArkUI_ImageAnimatorFrameInfo>,
}

impl ImageAnimatorFrameInfo {
    pub fn from_string(src: &str) -> ArkUIResult<Self> {
        let src = CString::new(src).map_err(|_| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "string contains interior NUL bytes",
            )
        })?;
        let info = unsafe {
            OH_ArkUI_ImageAnimatorFrameInfo_CreateFromString(src.as_ptr() as *mut c_char)
        };
        NonNull::new(info)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_ImageAnimatorFrameInfo_CreateFromString returned null",
                )
            })
    }

    pub fn from_drawable_descriptor(drawable: &DrawableDescriptor) -> ArkUIResult<Self> {
        let info =
            unsafe { OH_ArkUI_ImageAnimatorFrameInfo_CreateFromDrawableDescriptor(drawable.raw()) };
        NonNull::new(info)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_ImageAnimatorFrameInfo_CreateFromDrawableDescriptor returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_ImageAnimatorFrameInfo {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_ImageAnimatorFrameInfo) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_ImageAnimatorFrameInfo"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_ImageAnimatorFrameInfo {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_Dispose(self.raw()) }
    }

    pub fn set_width(&mut self, width: i32) {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_SetWidth(self.raw(), width) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_GetWidth(self.raw()) }
    }

    pub fn set_height(&mut self, height: i32) {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_SetHeight(self.raw(), height) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_GetHeight(self.raw()) }
    }

    pub fn set_top(&mut self, top: i32) {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_SetTop(self.raw(), top) }
    }

    pub fn get_top(&self) -> i32 {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_GetTop(self.raw()) }
    }

    pub fn set_left(&mut self, left: i32) {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_SetLeft(self.raw(), left) }
    }

    pub fn get_left(&self) -> i32 {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_GetLeft(self.raw()) }
    }

    pub fn set_duration(&mut self, duration: i32) {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_SetDuration(self.raw(), duration) }
    }

    pub fn get_duration(&self) -> i32 {
        unsafe { OH_ArkUI_ImageAnimatorFrameInfo_GetDuration(self.raw()) }
    }
}
