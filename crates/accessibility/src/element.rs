use std::{ffi::CString, marker::PhantomData, ptr::NonNull};

use ohos_accessibility_sys::*;

use crate::{
    error::{check, AccessibilityError, Result},
    AccessibleRect, ActionType, EventType, GridInfo, GridItemInfo, RangeInfo,
};

/// An ArkUI accessibility element.
///
/// Values returned by [`ElementInfoList::add`] are owned by ArkUI. Values
/// created by [`ElementInfo::new`] are destroyed automatically.
pub struct ElementInfo<'a> {
    raw: NonNull<ArkUI_AccessibilityElementInfo>,
    owned: bool,
    _marker: PhantomData<&'a mut ArkUI_AccessibilityElementInfo>,
}

impl ElementInfo<'static> {
    pub fn new() -> Result<Self> {
        let raw = unsafe { OH_ArkUI_CreateAccessibilityElementInfo() };
        let raw = NonNull::new(raw)
            .ok_or(AccessibilityError::NullHandle("accessibility element info"))?;
        Ok(Self {
            raw,
            owned: true,
            _marker: PhantomData,
        })
    }
}

impl ElementInfo<'_> {
    pub(crate) unsafe fn from_borrowed(raw: *mut ArkUI_AccessibilityElementInfo) -> Result<Self> {
        let raw = NonNull::new(raw)
            .ok_or(AccessibilityError::NullHandle("accessibility element info"))?;
        Ok(Self {
            raw,
            owned: false,
            _marker: PhantomData,
        })
    }

    pub fn as_raw(&self) -> *mut ArkUI_AccessibilityElementInfo {
        self.raw.as_ptr()
    }

    pub fn set_element_id(&mut self, value: i32) -> Result<&mut Self> {
        check(unsafe { OH_ArkUI_AccessibilityElementInfoSetElementId(self.as_raw(), value) })?;
        Ok(self)
    }

    pub fn set_parent_id(&mut self, value: i32) -> Result<&mut Self> {
        check(unsafe { OH_ArkUI_AccessibilityElementInfoSetParentId(self.as_raw(), value) })?;
        Ok(self)
    }

    pub fn set_component_type(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(value, OH_ArkUI_AccessibilityElementInfoSetComponentType)
    }

    pub fn set_contents(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(value, OH_ArkUI_AccessibilityElementInfoSetContents)
    }

    pub fn set_hint_text(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(value, OH_ArkUI_AccessibilityElementInfoSetHintText)
    }

    pub fn set_accessibility_text(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(value, OH_ArkUI_AccessibilityElementInfoSetAccessibilityText)
    }

    pub fn set_accessibility_description(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(
            value,
            OH_ArkUI_AccessibilityElementInfoSetAccessibilityDescription,
        )
    }

    #[cfg(feature = "api-24")]
    pub fn set_component_identifier(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(
            value,
            OH_ArkUI_AccessibilityElementInfoSetComponentIdentifier,
        )
    }

    pub fn set_child_node_ids(&mut self, ids: &[i64]) -> Result<&mut Self> {
        let count = i32::try_from(ids.len()).map_err(|_| AccessibilityError::BadParameter)?;
        let mut ids = ids.to_vec();
        check(unsafe {
            OH_ArkUI_AccessibilityElementInfoSetChildNodeIds(self.as_raw(), count, ids.as_mut_ptr())
        })?;
        Ok(self)
    }

    pub fn set_operation_actions(&mut self, actions: &[(ActionType, &str)]) -> Result<&mut Self> {
        let count = i32::try_from(actions.len()).map_err(|_| AccessibilityError::BadParameter)?;
        let descriptions = actions
            .iter()
            .map(|(_, description)| CString::new(*description))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let mut raw_actions = actions
            .iter()
            .zip(&descriptions)
            .map(|((action, _), description)| ArkUI_AccessibleAction {
                actionType: action.as_raw(),
                description: description.as_ptr(),
            })
            .collect::<Vec<_>>();
        check(unsafe {
            OH_ArkUI_AccessibilityElementInfoSetOperationActions(
                self.as_raw(),
                count,
                raw_actions.as_mut_ptr(),
            )
        })?;
        Ok(self)
    }

    pub fn set_screen_rect(&mut self, value: AccessibleRect) -> Result<&mut Self> {
        let mut value = ArkUI_AccessibleRect::from(value);
        check(unsafe {
            OH_ArkUI_AccessibilityElementInfoSetScreenRect(self.as_raw(), &mut value)
        })?;
        Ok(self)
    }

    pub fn set_checkable(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetCheckable)
    }

    pub fn set_checked(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetChecked)
    }

    pub fn set_focusable(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetFocusable)
    }

    pub fn set_focused(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetFocused)
    }

    pub fn set_visible(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetVisible)
    }

    pub fn set_accessibility_focused(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(
            value,
            OH_ArkUI_AccessibilityElementInfoSetAccessibilityFocused,
        )
    }

    pub fn set_selected(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetSelected)
    }

    pub fn set_clickable(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetClickable)
    }

    pub fn set_long_clickable(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetLongClickable)
    }

    pub fn set_enabled(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetEnabled)
    }

    pub fn set_password(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetIsPassword)
    }

    pub fn set_scrollable(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetScrollable)
    }

    pub fn set_editable(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetEditable)
    }

    pub fn set_is_hint(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(value, OH_ArkUI_AccessibilityElementInfoSetIsHint)
    }

    pub fn set_range_info(&mut self, value: RangeInfo) -> Result<&mut Self> {
        let mut value = ArkUI_AccessibleRangeInfo::from(value);
        check(unsafe { OH_ArkUI_AccessibilityElementInfoSetRangeInfo(self.as_raw(), &mut value) })?;
        Ok(self)
    }

    pub fn set_grid_info(&mut self, value: GridInfo) -> Result<&mut Self> {
        let mut value = ArkUI_AccessibleGridInfo::from(value);
        check(unsafe { OH_ArkUI_AccessibilityElementInfoSetGridInfo(self.as_raw(), &mut value) })?;
        Ok(self)
    }

    pub fn set_grid_item_info(&mut self, value: GridItemInfo) -> Result<&mut Self> {
        let mut value = ArkUI_AccessibleGridItemInfo::from(value);
        check(unsafe {
            OH_ArkUI_AccessibilityElementInfoSetGridItemInfo(self.as_raw(), &mut value)
        })?;
        Ok(self)
    }

    pub fn set_selected_text_start(&mut self, value: i32) -> Result<&mut Self> {
        self.set_i32(value, OH_ArkUI_AccessibilityElementInfoSetSelectedTextStart)
    }

    pub fn set_selected_text_end(&mut self, value: i32) -> Result<&mut Self> {
        self.set_i32(value, OH_ArkUI_AccessibilityElementInfoSetSelectedTextEnd)
    }

    pub fn set_current_item_index(&mut self, value: i32) -> Result<&mut Self> {
        self.set_i32(value, OH_ArkUI_AccessibilityElementInfoSetCurrentItemIndex)
    }

    pub fn set_start_item_index(&mut self, value: i32) -> Result<&mut Self> {
        self.set_i32(value, OH_ArkUI_AccessibilityElementInfoSetStartItemIndex)
    }

    pub fn set_end_item_index(&mut self, value: i32) -> Result<&mut Self> {
        self.set_i32(value, OH_ArkUI_AccessibilityElementInfoSetEndItemIndex)
    }

    pub fn set_item_count(&mut self, value: i32) -> Result<&mut Self> {
        self.set_i32(value, OH_ArkUI_AccessibilityElementInfoSetItemCount)
    }

    pub fn set_accessibility_offset(&mut self, value: i32) -> Result<&mut Self> {
        self.set_i32(
            value,
            OH_ArkUI_AccessibilityElementInfoSetAccessibilityOffset,
        )
    }

    pub fn set_accessibility_group(&mut self, value: bool) -> Result<&mut Self> {
        self.set_bool(
            value,
            OH_ArkUI_AccessibilityElementInfoSetAccessibilityGroup,
        )
    }

    pub fn set_accessibility_level(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(
            value,
            OH_ArkUI_AccessibilityElementInfoSetAccessibilityLevel,
        )
    }

    pub fn set_z_index(&mut self, value: i32) -> Result<&mut Self> {
        self.set_i32(value, OH_ArkUI_AccessibilityElementInfoSetZIndex)
    }

    pub fn set_opacity(&mut self, value: f32) -> Result<&mut Self> {
        check(unsafe {
            OH_ArkUI_AccessibilityElementInfoSetAccessibilityOpacity(self.as_raw(), value)
        })?;
        Ok(self)
    }

    pub fn set_background_color(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(value, OH_ArkUI_AccessibilityElementInfoSetBackgroundColor)
    }

    pub fn set_background_image(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(value, OH_ArkUI_AccessibilityElementInfoSetBackgroundImage)
    }

    pub fn set_blur(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(value, OH_ArkUI_AccessibilityElementInfoSetBlur)
    }

    pub fn set_hit_test_behavior(&mut self, value: &str) -> Result<&mut Self> {
        self.set_string(value, OH_ArkUI_AccessibilityElementInfoSetHitTestBehavior)
    }

    fn set_bool(
        &mut self,
        value: bool,
        setter: unsafe extern "C" fn(*mut ArkUI_AccessibilityElementInfo, bool) -> i32,
    ) -> Result<&mut Self> {
        check(unsafe { setter(self.as_raw(), value) })?;
        Ok(self)
    }

    fn set_i32(
        &mut self,
        value: i32,
        setter: unsafe extern "C" fn(*mut ArkUI_AccessibilityElementInfo, i32) -> i32,
    ) -> Result<&mut Self> {
        check(unsafe { setter(self.as_raw(), value) })?;
        Ok(self)
    }

    fn set_string(
        &mut self,
        value: &str,
        setter: unsafe extern "C" fn(
            *mut ArkUI_AccessibilityElementInfo,
            *const std::os::raw::c_char,
        ) -> i32,
    ) -> Result<&mut Self> {
        let value = CString::new(value)?;
        check(unsafe { setter(self.as_raw(), value.as_ptr()) })?;
        Ok(self)
    }
}

impl Drop for ElementInfo<'_> {
    fn drop(&mut self) {
        if self.owned {
            unsafe { OH_ArkUI_DestoryAccessibilityElementInfo(self.as_raw()) }
        }
    }
}

pub struct ElementInfoList<'a> {
    raw: NonNull<ArkUI_AccessibilityElementInfoList>,
    _marker: PhantomData<&'a mut ArkUI_AccessibilityElementInfoList>,
}

impl ElementInfoList<'_> {
    pub(crate) unsafe fn from_raw(raw: *mut ArkUI_AccessibilityElementInfoList) -> Result<Self> {
        let raw = NonNull::new(raw).ok_or(AccessibilityError::NullHandle(
            "accessibility element info list",
        ))?;
        Ok(Self {
            raw,
            _marker: PhantomData,
        })
    }

    pub fn add(&mut self) -> Result<ElementInfo<'_>> {
        let raw = unsafe { OH_ArkUI_AddAndGetAccessibilityElementInfo(self.raw.as_ptr()) };
        unsafe { ElementInfo::from_borrowed(raw) }
    }
}

pub struct EventInfo {
    raw: NonNull<ArkUI_AccessibilityEventInfo>,
}

impl EventInfo {
    pub fn new() -> Result<Self> {
        let raw = unsafe { OH_ArkUI_CreateAccessibilityEventInfo() };
        NonNull::new(raw)
            .map(|raw| Self { raw })
            .ok_or(AccessibilityError::NullHandle("accessibility event info"))
    }

    pub fn as_raw(&self) -> *mut ArkUI_AccessibilityEventInfo {
        self.raw.as_ptr()
    }

    pub fn set_event_type(&mut self, value: EventType) -> Result<&mut Self> {
        check(unsafe { OH_ArkUI_AccessibilityEventSetEventType(self.as_raw(), value.as_raw()) })?;
        Ok(self)
    }

    pub fn set_announced_text(&mut self, value: &str) -> Result<&mut Self> {
        let value = CString::new(value)?;
        check(unsafe {
            OH_ArkUI_AccessibilityEventSetTextAnnouncedForAccessibility(
                self.as_raw(),
                value.as_ptr(),
            )
        })?;
        Ok(self)
    }

    pub fn set_request_focus_id(&mut self, value: i32) -> Result<&mut Self> {
        check(unsafe { OH_ArkUI_AccessibilityEventSetRequestFocusId(self.as_raw(), value) })?;
        Ok(self)
    }

    pub fn set_element_info(&mut self, value: &ElementInfo<'_>) -> Result<&mut Self> {
        check(unsafe { OH_ArkUI_AccessibilityEventSetElementInfo(self.as_raw(), value.as_raw()) })?;
        Ok(self)
    }
}

impl Drop for EventInfo {
    fn drop(&mut self) {
        unsafe { OH_ArkUI_DestoryAccessibilityEventInfo(self.as_raw()) }
    }
}
