//! Module api::attribute_option::list_and_layout wrappers and related types.

use std::{os::raw::c_void, ptr::NonNull};

#[cfg(feature = "api-20")]
use std::os::raw::c_char;
#[cfg(feature = "api-20")]
use std::sync::{Mutex, OnceLock};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::*;

use super::base::{c_char_ptr_to_string, non_null_or_panic, with_cstring};
use crate::{check_arkui_status, ArkUIError, ArkUIResult};

struct ListItemSwipeActionVoidCallbackContext {
    callback: Box<dyn Fn()>,
}

struct ListItemSwipeActionStateCallbackContext {
    callback: Box<dyn Fn(crate::ListItemSwipeActionState)>,
}

#[derive(Default)]
struct ListItemSwipeActionItemCallbacks {
    on_enter_action_area: Option<*mut ListItemSwipeActionVoidCallbackContext>,
    on_action: Option<*mut ListItemSwipeActionVoidCallbackContext>,
    on_exit_action_area: Option<*mut ListItemSwipeActionVoidCallbackContext>,
    on_state_change: Option<*mut ListItemSwipeActionStateCallbackContext>,
}

/// List-item swipe action item descriptor.
pub struct ListItemSwipeActionItem {
    raw: NonNull<ArkUI_ListItemSwipeActionItem>,
    callbacks: ListItemSwipeActionItemCallbacks,
}

impl ListItemSwipeActionItem {
    pub fn new() -> ArkUIResult<Self> {
        let item = unsafe { OH_ArkUI_ListItemSwipeActionItem_Create() };
        NonNull::new(item)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_ListItemSwipeActionItem_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_ListItemSwipeActionItem {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_ListItemSwipeActionItem) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_ListItemSwipeActionItem"),
            callbacks: ListItemSwipeActionItemCallbacks::default(),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_ListItemSwipeActionItem {
        self.raw.as_ptr()
    }

    pub fn dispose(mut self) {
        self.clear_on_enter_action_area();
        self.clear_on_action();
        self.clear_on_exit_action_area();
        self.clear_on_state_change();
        unsafe { OH_ArkUI_ListItemSwipeActionItem_Dispose(self.raw()) }
    }

    pub fn set_content(&mut self, node: &crate::ArkUINode) {
        unsafe { OH_ArkUI_ListItemSwipeActionItem_SetContent(self.raw(), node.raw()) }
    }

    pub fn set_action_area_distance(&mut self, distance: f32) {
        unsafe { OH_ArkUI_ListItemSwipeActionItem_SetActionAreaDistance(self.raw(), distance) }
    }

    pub fn get_action_area_distance(&self) -> f32 {
        unsafe { OH_ArkUI_ListItemSwipeActionItem_GetActionAreaDistance(self.raw()) }
    }

    pub fn set_on_enter_action_area<T: Fn() + 'static>(&mut self, callback: T) {
        self.clear_on_enter_action_area();
        let callback = Box::into_raw(Box::new(ListItemSwipeActionVoidCallbackContext {
            callback: Box::new(callback),
        }));
        unsafe {
            OH_ArkUI_ListItemSwipeActionItem_SetOnEnterActionAreaWithUserData(
                self.raw(),
                callback.cast(),
                Some(list_item_swipe_action_item_void_callback_trampoline),
            )
        };
        self.callbacks.on_enter_action_area = Some(callback);
    }

    pub fn clear_on_enter_action_area(&mut self) {
        unsafe {
            OH_ArkUI_ListItemSwipeActionItem_SetOnEnterActionAreaWithUserData(
                self.raw(),
                std::ptr::null_mut(),
                None,
            )
        };
        if let Some(callback) = self.callbacks.on_enter_action_area.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
    }

    pub fn set_on_action<T: Fn() + 'static>(&mut self, callback: T) {
        self.clear_on_action();
        let callback = Box::into_raw(Box::new(ListItemSwipeActionVoidCallbackContext {
            callback: Box::new(callback),
        }));
        unsafe {
            OH_ArkUI_ListItemSwipeActionItem_SetOnActionWithUserData(
                self.raw(),
                callback.cast(),
                Some(list_item_swipe_action_item_void_callback_trampoline),
            )
        };
        self.callbacks.on_action = Some(callback);
    }

    pub fn clear_on_action(&mut self) {
        unsafe {
            OH_ArkUI_ListItemSwipeActionItem_SetOnActionWithUserData(
                self.raw(),
                std::ptr::null_mut(),
                None,
            )
        };
        if let Some(callback) = self.callbacks.on_action.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
    }

    pub fn set_on_exit_action_area<T: Fn() + 'static>(&mut self, callback: T) {
        self.clear_on_exit_action_area();
        let callback = Box::into_raw(Box::new(ListItemSwipeActionVoidCallbackContext {
            callback: Box::new(callback),
        }));
        unsafe {
            OH_ArkUI_ListItemSwipeActionItem_SetOnExitActionAreaWithUserData(
                self.raw(),
                callback.cast(),
                Some(list_item_swipe_action_item_void_callback_trampoline),
            )
        };
        self.callbacks.on_exit_action_area = Some(callback);
    }

    pub fn clear_on_exit_action_area(&mut self) {
        unsafe {
            OH_ArkUI_ListItemSwipeActionItem_SetOnExitActionAreaWithUserData(
                self.raw(),
                std::ptr::null_mut(),
                None,
            )
        };
        if let Some(callback) = self.callbacks.on_exit_action_area.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
    }

    pub fn set_on_state_change<T: Fn(crate::ListItemSwipeActionState) + 'static>(
        &mut self,
        callback: T,
    ) {
        self.clear_on_state_change();
        let callback = Box::into_raw(Box::new(ListItemSwipeActionStateCallbackContext {
            callback: Box::new(callback),
        }));
        unsafe {
            OH_ArkUI_ListItemSwipeActionItem_SetOnStateChangeWithUserData(
                self.raw(),
                callback.cast(),
                Some(list_item_swipe_action_item_state_callback_trampoline),
            )
        };
        self.callbacks.on_state_change = Some(callback);
    }

    pub fn clear_on_state_change(&mut self) {
        unsafe {
            OH_ArkUI_ListItemSwipeActionItem_SetOnStateChangeWithUserData(
                self.raw(),
                std::ptr::null_mut(),
                None,
            )
        };
        if let Some(callback) = self.callbacks.on_state_change.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
    }
}

unsafe extern "C" fn list_item_swipe_action_item_void_callback_trampoline(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }

    let callback = unsafe { &*(user_data as *mut ListItemSwipeActionVoidCallbackContext) };
    (callback.callback)();
}

unsafe extern "C" fn list_item_swipe_action_item_state_callback_trampoline(
    swipe_action_state: ArkUI_ListItemSwipeActionState,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }

    let callback = unsafe { &*(user_data as *mut ListItemSwipeActionStateCallbackContext) };
    (callback.callback)(swipe_action_state.into());
}

struct NodeAdapterEventReceiverCallbackContext {
    callback: Box<dyn Fn(&mut NodeAdapterEvent)>,
}

/// Node-adapter callback event wrapper.
pub struct NodeAdapterEvent {
    raw: NonNull<ArkUI_NodeAdapterEvent>,
}

impl NodeAdapterEvent {
    pub(crate) fn from_raw(raw: *mut ArkUI_NodeAdapterEvent) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_NodeAdapterEvent"),
        }
    }

    fn raw(&self) -> *mut ArkUI_NodeAdapterEvent {
        self.raw.as_ptr()
    }

    pub fn event_type(&self) -> crate::NodeAdapterEventType {
        unsafe { OH_ArkUI_NodeAdapterEvent_GetType(self.raw()).into() }
    }

    pub fn removed_node(&self) -> Option<crate::ArkUINode> {
        crate::ArkUINode::from_raw_handle(unsafe {
            OH_ArkUI_NodeAdapterEvent_GetRemovedNode(self.raw())
        })
    }

    pub fn item_index(&self) -> u32 {
        unsafe { OH_ArkUI_NodeAdapterEvent_GetItemIndex(self.raw()) }
    }

    pub fn host_node(&self) -> Option<crate::ArkUINode> {
        crate::ArkUINode::from_raw_handle(unsafe {
            OH_ArkUI_NodeAdapterEvent_GetHostNode(self.raw())
        })
    }

    pub fn set_item(&mut self, node: &crate::ArkUINode) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NodeAdapterEvent_SetItem(self.raw(), node.raw())) }
    }

    pub fn set_node_id(&mut self, id: i32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NodeAdapterEvent_SetNodeId(self.raw(), id)) }
    }
}

/// Node-adapter wrapper for lazy data-driven list/grid.
pub struct NodeAdapter {
    raw: NonNull<ArkUI_NodeAdapter>,
    event_receiver: Option<*mut NodeAdapterEventReceiverCallbackContext>,
}

impl NodeAdapter {
    pub fn new() -> ArkUIResult<Self> {
        let handle = unsafe { OH_ArkUI_NodeAdapter_Create() };
        NonNull::new(handle)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_NodeAdapter_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> ArkUI_NodeAdapterHandle {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: ArkUI_NodeAdapterHandle) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_NodeAdapter"),
            event_receiver: None,
        }
    }

    pub(crate) fn into_raw(self) -> ArkUI_NodeAdapterHandle {
        self.raw.as_ptr()
    }

    pub fn dispose(mut self) {
        self.unregister_event_receiver();
        unsafe { OH_ArkUI_NodeAdapter_Dispose(self.raw()) }
    }

    pub fn set_total_node_count(&mut self, size: u32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NodeAdapter_SetTotalNodeCount(self.raw(), size)) }
    }

    pub fn get_total_node_count(&self) -> u32 {
        unsafe { OH_ArkUI_NodeAdapter_GetTotalNodeCount(self.raw()) }
    }

    pub fn register_event_receiver<T: Fn(&mut NodeAdapterEvent) + 'static>(
        &mut self,
        receiver: T,
    ) -> ArkUIResult<()> {
        self.unregister_event_receiver();
        let receiver = Box::into_raw(Box::new(NodeAdapterEventReceiverCallbackContext {
            callback: Box::new(receiver),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_NodeAdapter_RegisterEventReceiver(
                self.raw(),
                receiver.cast(),
                Some(node_adapter_event_receiver_callback_trampoline),
            ))
        };

        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(receiver));
            }
            return Err(err);
        }

        self.event_receiver = Some(receiver);
        Ok(())
    }

    pub fn unregister_event_receiver(&mut self) {
        unsafe { OH_ArkUI_NodeAdapter_UnregisterEventReceiver(self.raw()) }
        if let Some(receiver) = self.event_receiver.take() {
            unsafe {
                drop(Box::from_raw(receiver));
            }
        }
    }

    pub fn reload_all_items(&mut self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NodeAdapter_ReloadAllItems(self.raw())) }
    }

    pub fn reload_item(&mut self, start_position: u32, item_count: u32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeAdapter_ReloadItem(
                self.raw(),
                start_position,
                item_count
            ))
        }
    }

    pub fn remove_item(&mut self, start_position: u32, item_count: u32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeAdapter_RemoveItem(
                self.raw(),
                start_position,
                item_count
            ))
        }
    }

    pub fn insert_item(&mut self, start_position: u32, item_count: u32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeAdapter_InsertItem(
                self.raw(),
                start_position,
                item_count
            ))
        }
    }

    pub fn move_item(&mut self, from: u32, to: u32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NodeAdapter_MoveItem(self.raw(), from, to)) }
    }

    pub fn get_all_items(&self) -> ArkUIResult<Vec<crate::ArkUINode>> {
        let mut items = std::ptr::null_mut();
        let mut size = 0;
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeAdapter_GetAllItems(
                self.raw(),
                &mut items,
                &mut size
            ))
        }?;
        if items.is_null() || size == 0 {
            return Ok(Vec::new());
        }
        let items = unsafe { std::slice::from_raw_parts(items, size as usize) };
        Ok(items
            .iter()
            .copied()
            .filter_map(crate::ArkUINode::from_raw_handle)
            .collect())
    }
}

unsafe extern "C" fn node_adapter_event_receiver_callback_trampoline(
    event: *mut ArkUI_NodeAdapterEvent,
) {
    let user_data = unsafe { OH_ArkUI_NodeAdapterEvent_GetUserData(event) };
    if user_data.is_null() {
        return;
    }

    let callback = unsafe { &*(user_data as *mut NodeAdapterEventReceiverCallbackContext) };
    let mut event = NodeAdapterEvent::from_raw(event);
    (callback.callback)(&mut event);
}

struct ListItemSwipeOffsetCallbackContext {
    callback: Box<dyn Fn(f32)>,
}

/// List-item swipe action option wrapper.
pub struct ListItemSwipeActionOption {
    raw: NonNull<ArkUI_ListItemSwipeActionOption>,
    on_offset_change: Option<*mut ListItemSwipeOffsetCallbackContext>,
}

impl ListItemSwipeActionOption {
    pub fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_ListItemSwipeActionOption_Create() };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_ListItemSwipeActionOption_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_ListItemSwipeActionOption {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_ListItemSwipeActionOption) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_ListItemSwipeActionOption"),
            on_offset_change: None,
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_ListItemSwipeActionOption {
        self.raw.as_ptr()
    }

    pub fn dispose(mut self) {
        self.clear_on_offset_change();
        unsafe { OH_ArkUI_ListItemSwipeActionOption_Dispose(self.raw()) }
    }

    pub fn set_start(&mut self, item: &ListItemSwipeActionItem) {
        unsafe { OH_ArkUI_ListItemSwipeActionOption_SetStart(self.raw(), item.raw()) }
    }

    pub fn set_end(&mut self, item: &ListItemSwipeActionItem) {
        unsafe { OH_ArkUI_ListItemSwipeActionOption_SetEnd(self.raw(), item.raw()) }
    }

    pub fn set_edge_effect(&mut self, edge_effect: crate::ListItemSwipeEdgeEffect) {
        unsafe { OH_ArkUI_ListItemSwipeActionOption_SetEdgeEffect(self.raw(), edge_effect.into()) }
    }

    pub fn get_edge_effect(&self) -> crate::ListItemSwipeEdgeEffect {
        unsafe { (OH_ArkUI_ListItemSwipeActionOption_GetEdgeEffect(self.raw()) as u32).into() }
    }

    pub fn set_on_offset_change<T: Fn(f32) + 'static>(&mut self, callback: T) {
        self.clear_on_offset_change();
        let callback = Box::into_raw(Box::new(ListItemSwipeOffsetCallbackContext {
            callback: Box::new(callback),
        }));
        unsafe {
            OH_ArkUI_ListItemSwipeActionOption_SetOnOffsetChangeWithUserData(
                self.raw(),
                callback.cast(),
                Some(list_item_swipe_action_option_offset_callback_trampoline),
            )
        };
        self.on_offset_change = Some(callback);
    }

    pub fn clear_on_offset_change(&mut self) {
        unsafe {
            OH_ArkUI_ListItemSwipeActionOption_SetOnOffsetChangeWithUserData(
                self.raw(),
                std::ptr::null_mut(),
                None,
            )
        };
        if let Some(callback) = self.on_offset_change.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
    }
}

unsafe extern "C" fn list_item_swipe_action_option_offset_callback_trampoline(
    offset: f32,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }

    let callback = unsafe { &*(user_data as *mut ListItemSwipeOffsetCallbackContext) };
    (callback.callback)(offset);
}

/// Main-axis size array wrapper for list children.
pub struct ListChildrenMainSize {
    raw: NonNull<ArkUI_ListChildrenMainSize>,
}

impl ListChildrenMainSize {
    pub fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_ListChildrenMainSizeOption_Create() };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_ListChildrenMainSizeOption_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_ListChildrenMainSize {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_ListChildrenMainSize) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_ListChildrenMainSize"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_ListChildrenMainSize {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_ListChildrenMainSizeOption_Dispose(self.raw()) }
    }

    pub fn set_default_main_size(&mut self, default_main_size: f32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_ListChildrenMainSizeOption_SetDefaultMainSize(
                self.raw(),
                default_main_size
            ))
        }
    }

    pub fn get_default_main_size(&self) -> f32 {
        unsafe { OH_ArkUI_ListChildrenMainSizeOption_GetDefaultMainSize(self.raw()) }
    }

    pub fn resize(&mut self, total_size: i32) {
        unsafe { OH_ArkUI_ListChildrenMainSizeOption_Resize(self.raw(), total_size) }
    }

    pub fn splice(&mut self, index: i32, delete_count: i32, add_count: i32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_ListChildrenMainSizeOption_Splice(
                self.raw(),
                index,
                delete_count,
                add_count
            ))
        }
    }

    pub fn update_size(&mut self, index: i32, main_size: f32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_ListChildrenMainSizeOption_UpdateSize(
                self.raw(),
                index,
                main_size
            ))
        }
    }

    pub fn get_main_size(&self, index: i32) -> f32 {
        unsafe { OH_ArkUI_ListChildrenMainSizeOption_GetMainSize(self.raw(), index) }
    }
}

/// Accessibility state wrapper.
pub struct AccessibilityState {
    raw: NonNull<ArkUI_AccessibilityState>,
}

impl AccessibilityState {
    pub fn new() -> ArkUIResult<Self> {
        let state = unsafe { OH_ArkUI_AccessibilityState_Create() };
        NonNull::new(state)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_AccessibilityState_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_AccessibilityState {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_AccessibilityState) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_AccessibilityState"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_AccessibilityState {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_AccessibilityState_Dispose(self.raw()) }
    }

    pub fn set_disabled(&mut self, is_disabled: i32) {
        unsafe { OH_ArkUI_AccessibilityState_SetDisabled(self.raw(), is_disabled) }
    }

    pub fn is_disabled(&self) -> i32 {
        unsafe { OH_ArkUI_AccessibilityState_IsDisabled(self.raw()) }
    }

    pub fn set_selected(&mut self, is_selected: i32) {
        unsafe { OH_ArkUI_AccessibilityState_SetSelected(self.raw(), is_selected) }
    }

    pub fn is_selected(&self) -> i32 {
        unsafe { OH_ArkUI_AccessibilityState_IsSelected(self.raw()) }
    }

    pub fn set_checked_state(&mut self, checked_state: i32) {
        unsafe { OH_ArkUI_AccessibilityState_SetCheckedState(self.raw(), checked_state) }
    }

    pub fn get_checked_state(&self) -> i32 {
        unsafe { OH_ArkUI_AccessibilityState_GetCheckedState(self.raw()) }
    }
}

/// Guideline option wrapper for relative layout.
pub struct GuidelineOption {
    raw: NonNull<ArkUI_GuidelineOption>,
}

impl GuidelineOption {
    pub fn new(size: i32) -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_GuidelineOption_Create(size) };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_GuidelineOption_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_GuidelineOption {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_GuidelineOption) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_GuidelineOption"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_GuidelineOption {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_GuidelineOption_Dispose(self.raw()) }
    }

    pub fn set_id(&mut self, value: &str, index: i32) -> ArkUIResult<()> {
        with_cstring(value, |value_ptr| unsafe {
            OH_ArkUI_GuidelineOption_SetId(self.raw(), value_ptr, index)
        })
    }

    pub fn set_direction(&mut self, value: crate::Axis, index: i32) {
        unsafe { OH_ArkUI_GuidelineOption_SetDirection(self.raw(), value.into(), index) }
    }

    pub fn set_position_start(&mut self, value: f32, index: i32) {
        unsafe { OH_ArkUI_GuidelineOption_SetPositionStart(self.raw(), value, index) }
    }

    pub fn set_position_end(&mut self, value: f32, index: i32) {
        unsafe { OH_ArkUI_GuidelineOption_SetPositionEnd(self.raw(), value, index) }
    }

    pub fn get_id(&self, index: i32) -> Option<String> {
        c_char_ptr_to_string(unsafe { OH_ArkUI_GuidelineOption_GetId(self.raw(), index) })
    }

    pub fn get_direction(&self, index: i32) -> crate::Axis {
        unsafe { OH_ArkUI_GuidelineOption_GetDirection(self.raw(), index).into() }
    }

    pub fn get_position_start(&self, index: i32) -> f32 {
        unsafe { OH_ArkUI_GuidelineOption_GetPositionStart(self.raw(), index) }
    }

    pub fn get_position_end(&self, index: i32) -> f32 {
        unsafe { OH_ArkUI_GuidelineOption_GetPositionEnd(self.raw(), index) }
    }
}

/// Barrier option wrapper for relative layout.
pub struct BarrierOption {
    raw: NonNull<ArkUI_BarrierOption>,
}

impl BarrierOption {
    pub fn new(size: i32) -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_BarrierOption_Create(size) };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_BarrierOption_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_BarrierOption {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_BarrierOption) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_BarrierOption"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_BarrierOption {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_BarrierOption_Dispose(self.raw()) }
    }

    pub fn set_id(&mut self, value: &str, index: i32) -> ArkUIResult<()> {
        with_cstring(value, |value_ptr| unsafe {
            OH_ArkUI_BarrierOption_SetId(self.raw(), value_ptr, index)
        })
    }

    pub fn set_direction(&mut self, value: crate::BarrierDirection, index: i32) {
        unsafe { OH_ArkUI_BarrierOption_SetDirection(self.raw(), value.into(), index) }
    }

    pub fn set_referenced_id(&mut self, value: &str, index: i32) -> ArkUIResult<()> {
        with_cstring(value, |value_ptr| unsafe {
            OH_ArkUI_BarrierOption_SetReferencedId(self.raw(), value_ptr, index)
        })
    }

    pub fn get_id(&self, index: i32) -> Option<String> {
        c_char_ptr_to_string(unsafe { OH_ArkUI_BarrierOption_GetId(self.raw(), index) })
    }

    pub fn get_direction(&self, index: i32) -> crate::BarrierDirection {
        unsafe { OH_ArkUI_BarrierOption_GetDirection(self.raw(), index).into() }
    }

    pub fn get_referenced_id(&self, index: i32, referenced_index: i32) -> Option<String> {
        c_char_ptr_to_string(unsafe {
            OH_ArkUI_BarrierOption_GetReferencedId(self.raw(), index, referenced_index)
        })
    }

    pub fn get_referenced_id_size(&self, index: i32) -> i32 {
        unsafe { OH_ArkUI_BarrierOption_GetReferencedIdSize(self.raw(), index) }
    }
}

#[cfg(feature = "api-15")]
/// Linear-progress style option wrapper.
pub struct ProgressLinearStyleOption {
    raw: NonNull<ArkUI_ProgressLinearStyleOption>,
}

#[cfg(feature = "api-15")]
impl ProgressLinearStyleOption {
    pub fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_ProgressLinearStyleOption_Create() };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_ProgressLinearStyleOption_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_ProgressLinearStyleOption {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_ProgressLinearStyleOption) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_ProgressLinearStyleOption"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_ProgressLinearStyleOption {
        self.raw.as_ptr()
    }

    pub fn destroy(self) {
        unsafe { OH_ArkUI_ProgressLinearStyleOption_Destroy(self.raw()) }
    }

    pub fn set_scan_effect_enabled(&mut self, enabled: bool) {
        unsafe { OH_ArkUI_ProgressLinearStyleOption_SetScanEffectEnabled(self.raw(), enabled) }
    }

    pub fn set_smooth_effect_enabled(&mut self, enabled: bool) {
        unsafe { OH_ArkUI_ProgressLinearStyleOption_SetSmoothEffectEnabled(self.raw(), enabled) }
    }

    pub fn set_stroke_width(&mut self, stroke_width: f32) {
        unsafe { OH_ArkUI_ProgressLinearStyleOption_SetStrokeWidth(self.raw(), stroke_width) }
    }

    pub fn set_stroke_radius(&mut self, stroke_radius: f32) {
        unsafe { OH_ArkUI_ProgressLinearStyleOption_SetStrokeRadius(self.raw(), stroke_radius) }
    }

    pub fn get_scan_effect_enabled(&self) -> bool {
        unsafe { OH_ArkUI_ProgressLinearStyleOption_GetScanEffectEnabled(self.raw()) }
    }

    pub fn get_smooth_effect_enabled(&self) -> bool {
        unsafe { OH_ArkUI_ProgressLinearStyleOption_GetSmoothEffectEnabled(self.raw()) }
    }

    pub fn get_stroke_width(&self) -> f32 {
        unsafe { OH_ArkUI_ProgressLinearStyleOption_GetStrokeWidth(self.raw()) }
    }

    pub fn get_stroke_radius(&self) -> f32 {
        unsafe { OH_ArkUI_ProgressLinearStyleOption_GetStrokeRadius(self.raw()) }
    }
}

#[cfg(feature = "api-21")]
/// Position edge values used by side/edge-based APIs.
pub struct PositionEdges {
    raw: NonNull<ArkUI_PositionEdges>,
}

#[cfg(feature = "api-21")]
impl PositionEdges {
    pub fn new() -> ArkUIResult<Self> {
        let edges = unsafe { OH_ArkUI_PositionEdges_Create() };
        NonNull::new(edges)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_PositionEdges_Create returned null",
                )
            })
    }

    pub fn copy(&self) -> ArkUIResult<Self> {
        let copied = unsafe { OH_ArkUI_PositionEdges_Copy(self.raw()) };
        NonNull::new(copied)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_PositionEdges_Copy returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_PositionEdges {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_PositionEdges) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_PositionEdges"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_PositionEdges {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_PositionEdges_Dispose(self.raw()) }
    }

    pub fn set_top(&mut self, value: f32) {
        unsafe { OH_ArkUI_PositionEdges_SetTop(self.raw(), value) }
    }

    pub fn get_top(&self) -> ArkUIResult<f32> {
        let mut value = 0.0;
        unsafe { check_arkui_status!(OH_ArkUI_PositionEdges_GetTop(self.raw(), &mut value)) }?;
        Ok(value)
    }

    pub fn set_left(&mut self, value: f32) {
        unsafe { OH_ArkUI_PositionEdges_SetLeft(self.raw(), value) }
    }

    pub fn get_left(&self) -> ArkUIResult<f32> {
        let mut value = 0.0;
        unsafe { check_arkui_status!(OH_ArkUI_PositionEdges_GetLeft(self.raw(), &mut value)) }?;
        Ok(value)
    }

    pub fn set_bottom(&mut self, value: f32) {
        unsafe { OH_ArkUI_PositionEdges_SetBottom(self.raw(), value) }
    }

    pub fn get_bottom(&self) -> ArkUIResult<f32> {
        let mut value = 0.0;
        unsafe { check_arkui_status!(OH_ArkUI_PositionEdges_GetBottom(self.raw(), &mut value)) }?;
        Ok(value)
    }

    pub fn set_right(&mut self, value: f32) {
        unsafe { OH_ArkUI_PositionEdges_SetRight(self.raw(), value) }
    }

    pub fn get_right(&self) -> ArkUIResult<f32> {
        let mut value = 0.0;
        unsafe { check_arkui_status!(OH_ArkUI_PositionEdges_GetRight(self.raw(), &mut value)) }?;
        Ok(value)
    }
}

#[cfg(feature = "api-21")]
/// Pixel round-policy wrapper.
pub struct PixelRoundPolicy {
    raw: NonNull<ArkUI_PixelRoundPolicy>,
}

#[cfg(feature = "api-21")]
impl PixelRoundPolicy {
    pub fn new() -> ArkUIResult<Self> {
        let policy = unsafe { OH_ArkUI_PixelRoundPolicy_Create() };
        NonNull::new(policy)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_PixelRoundPolicy_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_PixelRoundPolicy {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_PixelRoundPolicy) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_PixelRoundPolicy"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_PixelRoundPolicy {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_PixelRoundPolicy_Dispose(self.raw()) }
    }

    pub fn set_top(&mut self, value: crate::PixelRoundCalcPolicy) {
        unsafe { OH_ArkUI_PixelRoundPolicy_SetTop(self.raw(), value.into()) }
    }

    pub fn get_top(&self) -> ArkUIResult<crate::PixelRoundCalcPolicy> {
        let mut value = ArkUI_PixelRoundCalcPolicy_ARKUI_PIXELROUNDCALCPOLICY_NOFORCEROUND;
        unsafe { check_arkui_status!(OH_ArkUI_PixelRoundPolicy_GetTop(self.raw(), &mut value)) }?;
        Ok(value.into())
    }

    pub fn set_start(&mut self, value: crate::PixelRoundCalcPolicy) {
        unsafe { OH_ArkUI_PixelRoundPolicy_SetStart(self.raw(), value.into()) }
    }

    pub fn get_start(&self) -> ArkUIResult<crate::PixelRoundCalcPolicy> {
        let mut value = ArkUI_PixelRoundCalcPolicy_ARKUI_PIXELROUNDCALCPOLICY_NOFORCEROUND;
        unsafe { check_arkui_status!(OH_ArkUI_PixelRoundPolicy_GetStart(self.raw(), &mut value)) }?;
        Ok(value.into())
    }

    pub fn set_bottom(&mut self, value: crate::PixelRoundCalcPolicy) {
        unsafe { OH_ArkUI_PixelRoundPolicy_SetBottom(self.raw(), value.into()) }
    }

    pub fn get_bottom(&self) -> ArkUIResult<crate::PixelRoundCalcPolicy> {
        let mut value = ArkUI_PixelRoundCalcPolicy_ARKUI_PIXELROUNDCALCPOLICY_NOFORCEROUND;
        unsafe {
            check_arkui_status!(OH_ArkUI_PixelRoundPolicy_GetBottom(self.raw(), &mut value))
        }?;
        Ok(value.into())
    }

    pub fn set_end(&mut self, value: crate::PixelRoundCalcPolicy) {
        unsafe { OH_ArkUI_PixelRoundPolicy_SetEnd(self.raw(), value.into()) }
    }

    pub fn get_end(&self) -> ArkUIResult<crate::PixelRoundCalcPolicy> {
        let mut value = ArkUI_PixelRoundCalcPolicy_ARKUI_PIXELROUNDCALCPOLICY_NOFORCEROUND;
        unsafe { check_arkui_status!(OH_ArkUI_PixelRoundPolicy_GetEnd(self.raw(), &mut value)) }?;
        Ok(value.into())
    }
}

#[cfg(feature = "api-22")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Grid item size value object.
pub struct GridItemSize {
    pub row_span: u32,
    pub column_span: u32,
}

#[cfg(feature = "api-22")]
impl From<ArkUI_GridItemSize> for GridItemSize {
    fn from(value: ArkUI_GridItemSize) -> Self {
        Self {
            row_span: value.rowSpan,
            column_span: value.columnSpan,
        }
    }
}

#[cfg(feature = "api-22")]
impl From<GridItemSize> for ArkUI_GridItemSize {
    fn from(value: GridItemSize) -> Self {
        Self {
            rowSpan: value.row_span,
            columnSpan: value.column_span,
        }
    }
}

#[cfg(feature = "api-22")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Grid item rectangle value object.
pub struct GridItemRect {
    pub row_start: u32,
    pub column_start: u32,
    pub row_span: u32,
    pub column_span: u32,
}

#[cfg(feature = "api-22")]
impl From<ArkUI_GridItemRect> for GridItemRect {
    fn from(value: ArkUI_GridItemRect) -> Self {
        Self {
            row_start: value.rowStart,
            column_start: value.columnStart,
            row_span: value.rowSpan,
            column_span: value.columnSpan,
        }
    }
}

#[cfg(feature = "api-22")]
impl From<GridItemRect> for ArkUI_GridItemRect {
    fn from(value: GridItemRect) -> Self {
        Self {
            rowStart: value.row_start,
            columnStart: value.column_start,
            rowSpan: value.row_span,
            columnSpan: value.column_span,
        }
    }
}

#[cfg(feature = "api-22")]
struct GridLayoutIrregularSizeCallbackContext {
    callback: Box<dyn Fn(i32) -> GridItemSize>,
}

#[cfg(feature = "api-22")]
struct GridLayoutRectCallbackContext {
    callback: Box<dyn Fn(i32) -> GridItemRect>,
}

#[cfg(feature = "api-22")]
/// Grid layout options wrapper.
pub struct GridLayoutOptions {
    raw: NonNull<ArkUI_GridLayoutOptions>,
    irregular_size_callback: Option<*mut GridLayoutIrregularSizeCallbackContext>,
    rect_callback: Option<*mut GridLayoutRectCallbackContext>,
}

#[cfg(feature = "api-22")]
impl GridLayoutOptions {
    pub fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_GridLayoutOptions_Create() };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_GridLayoutOptions_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_GridLayoutOptions {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_GridLayoutOptions) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_GridLayoutOptions"),
            irregular_size_callback: None,
            rect_callback: None,
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_GridLayoutOptions {
        self.raw.as_ptr()
    }

    pub fn dispose(mut self) {
        self.clear_get_irregular_size_by_index_callback();
        self.clear_get_rect_by_index_callback();
        unsafe { OH_ArkUI_GridLayoutOptions_Dispose(self.raw()) }
    }

    pub fn set_irregular_indexes(&mut self, irregular_indexes: &mut [u32]) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_GridLayoutOptions_SetIrregularIndexes(
                self.raw(),
                irregular_indexes.as_mut_ptr(),
                irregular_indexes.len() as i32
            ))
        }
    }

    pub fn get_irregular_indexes(&self, irregular_indexes: &mut [u32]) -> ArkUIResult<i32> {
        let mut size = irregular_indexes.len() as i32;
        unsafe {
            check_arkui_status!(OH_ArkUI_GridLayoutOptions_GetIrregularIndexes(
                self.raw(),
                irregular_indexes.as_mut_ptr(),
                &mut size
            ))
        }?;
        Ok(size)
    }

    pub fn register_get_irregular_size_by_index_callback<T: Fn(i32) -> GridItemSize + 'static>(
        &mut self,
        callback: T,
    ) {
        self.clear_get_irregular_size_by_index_callback();
        let callback = Box::into_raw(Box::new(GridLayoutIrregularSizeCallbackContext {
            callback: Box::new(callback),
        }));
        unsafe {
            OH_ArkUI_GridLayoutOptions_RegisterGetIrregularSizeByIndexCallback(
                self.raw(),
                callback.cast(),
                Some(grid_layout_irregular_size_callback_trampoline),
            )
        };
        self.irregular_size_callback = Some(callback);
    }

    pub fn clear_get_irregular_size_by_index_callback(&mut self) {
        unsafe {
            OH_ArkUI_GridLayoutOptions_RegisterGetIrregularSizeByIndexCallback(
                self.raw(),
                std::ptr::null_mut(),
                None,
            )
        };
        if let Some(callback) = self.irregular_size_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
    }

    pub fn register_get_rect_by_index_callback<T: Fn(i32) -> GridItemRect + 'static>(
        &mut self,
        callback: T,
    ) {
        self.clear_get_rect_by_index_callback();
        let callback = Box::into_raw(Box::new(GridLayoutRectCallbackContext {
            callback: Box::new(callback),
        }));
        unsafe {
            OH_ArkUI_GridLayoutOptions_RegisterGetRectByIndexCallback(
                self.raw(),
                callback.cast(),
                Some(grid_layout_rect_callback_trampoline),
            )
        };
        self.rect_callback = Some(callback);
    }

    pub fn clear_get_rect_by_index_callback(&mut self) {
        unsafe {
            OH_ArkUI_GridLayoutOptions_RegisterGetRectByIndexCallback(
                self.raw(),
                std::ptr::null_mut(),
                None,
            )
        };
        if let Some(callback) = self.rect_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
    }
}

#[cfg(feature = "api-22")]
unsafe extern "C" fn grid_layout_irregular_size_callback_trampoline(
    item_index: i32,
    user_data: *mut c_void,
) -> ArkUI_GridItemSize {
    if user_data.is_null() {
        return unsafe { std::mem::zeroed() };
    }

    let callback = unsafe { &*(user_data as *mut GridLayoutIrregularSizeCallbackContext) };
    (callback.callback)(item_index).into()
}

#[cfg(feature = "api-22")]
unsafe extern "C" fn grid_layout_rect_callback_trampoline(
    item_index: i32,
    user_data: *mut c_void,
) -> ArkUI_GridItemRect {
    if user_data.is_null() {
        return unsafe { std::mem::zeroed() };
    }

    let callback = unsafe { &*(user_data as *mut GridLayoutRectCallbackContext) };
    (callback.callback)(item_index).into()
}

#[cfg(feature = "api-22")]
/// Counter display configuration for text input/area.
pub struct ShowCounterConfig {
    raw: NonNull<ArkUI_ShowCounterConfig>,
}

#[cfg(feature = "api-22")]
impl ShowCounterConfig {
    pub fn new() -> ArkUIResult<Self> {
        let config = unsafe { OH_ArkUI_ShowCounterConfig_Create() };
        NonNull::new(config)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_ShowCounterConfig_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_ShowCounterConfig {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_ShowCounterConfig) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_ShowCounterConfig"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_ShowCounterConfig {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_ShowCounterConfig_Dispose(self.raw()) }
    }

    pub fn set_counter_text_color(&mut self, color: u32) {
        unsafe { OH_ArkUI_ShowCounterConfig_SetCounterTextColor(self.raw(), color) }
    }

    pub fn set_counter_text_overflow_color(&mut self, color: u32) {
        unsafe { OH_ArkUI_ShowCounterConfig_SetCounterTextOverflowColor(self.raw(), color) }
    }

    pub fn get_counter_text_color(&self) -> u32 {
        unsafe { OH_ArkUI_ShowCounterConfig_GetCounterTextColor(self.raw()) }
    }

    pub fn get_counter_text_overflow_color(&self) -> u32 {
        unsafe { OH_ArkUI_ShowCounterConfig_GetCounterTextOverflowColor(self.raw()) }
    }
}

#[cfg(feature = "api-17")]
/// Options for visible-area change event registration.
pub struct VisibleAreaEventOptions {
    raw: NonNull<ArkUI_VisibleAreaEventOptions>,
}

#[cfg(feature = "api-17")]
impl VisibleAreaEventOptions {
    pub fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_VisibleAreaEventOptions_Create() };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_VisibleAreaEventOptions_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_VisibleAreaEventOptions {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_VisibleAreaEventOptions) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_VisibleAreaEventOptions"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_VisibleAreaEventOptions {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_VisibleAreaEventOptions_Dispose(self.raw()) }
    }

    pub fn set_ratios(&mut self, ratios: &mut [f32]) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_VisibleAreaEventOptions_SetRatios(
                self.raw(),
                ratios.as_mut_ptr(),
                ratios.len() as i32
            ))
        }
    }

    pub fn get_ratios(&self, ratios: &mut [f32]) -> ArkUIResult<i32> {
        let mut size = ratios.len() as i32;
        unsafe {
            check_arkui_status!(OH_ArkUI_VisibleAreaEventOptions_GetRatios(
                self.raw(),
                ratios.as_mut_ptr(),
                &mut size
            ))
        }?;
        Ok(size)
    }

    pub fn set_expected_update_interval(&mut self, interval: i32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_VisibleAreaEventOptions_SetExpectedUpdateInterval(
                self.raw(),
                interval
            ))
        }
    }

    pub fn get_expected_update_interval(&self) -> i32 {
        unsafe { OH_ArkUI_VisibleAreaEventOptions_GetExpectedUpdateInterval(self.raw()) }
    }

    #[cfg(feature = "api-22")]
    pub fn set_measure_from_viewport(&mut self, measure_from_viewport: bool) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_VisibleAreaEventOptions_SetMeasureFromViewport(
                self.raw(),
                measure_from_viewport
            ))
        }
    }

    #[cfg(feature = "api-22")]
    pub fn get_measure_from_viewport(&self) -> bool {
        unsafe { OH_ArkUI_VisibleAreaEventOptions_GetMeasureFromViewport(self.raw()) }
    }
}

#[cfg(feature = "api-19")]
/// Range-content array wrapper for text picker.
pub struct TextPickerRangeContentArray {
    raw: NonNull<ArkUI_TextPickerRangeContentArray>,
}

#[cfg(feature = "api-19")]
impl TextPickerRangeContentArray {
    pub fn new(length: i32) -> ArkUIResult<Self> {
        let handle = unsafe { OH_ArkUI_TextPickerRangeContentArray_Create(length) };
        NonNull::new(handle)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_TextPickerRangeContentArray_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_TextPickerRangeContentArray {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_TextPickerRangeContentArray) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_TextPickerRangeContentArray"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_TextPickerRangeContentArray {
        self.raw.as_ptr()
    }

    pub fn destroy(self) {
        unsafe { OH_ArkUI_TextPickerRangeContentArray_Destroy(self.raw()) }
    }

    pub fn set_icon_at_index(&mut self, icon: &str, index: i32) -> ArkUIResult<()> {
        with_cstring(icon, |icon_ptr| unsafe {
            OH_ArkUI_TextPickerRangeContentArray_SetIconAtIndex(
                self.raw(),
                icon_ptr.cast_mut(),
                index,
            )
        })
    }

    pub fn set_text_at_index(&mut self, text: &str, index: i32) -> ArkUIResult<()> {
        with_cstring(text, |text_ptr| unsafe {
            OH_ArkUI_TextPickerRangeContentArray_SetTextAtIndex(
                self.raw(),
                text_ptr.cast_mut(),
                index,
            )
        })
    }
}

#[cfg(feature = "api-19")]
/// Range-content array wrapper for cascade text picker.
pub struct TextCascadePickerRangeContentArray {
    raw: NonNull<ArkUI_TextCascadePickerRangeContentArray>,
}

#[cfg(feature = "api-19")]
impl TextCascadePickerRangeContentArray {
    pub fn new(length: i32) -> ArkUIResult<Self> {
        let handle = unsafe { OH_ArkUI_TextCascadePickerRangeContentArray_Create(length) };
        NonNull::new(handle)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_TextCascadePickerRangeContentArray_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_TextCascadePickerRangeContentArray {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_TextCascadePickerRangeContentArray) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_TextCascadePickerRangeContentArray"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_TextCascadePickerRangeContentArray {
        self.raw.as_ptr()
    }

    pub fn destroy(self) {
        unsafe { OH_ArkUI_TextCascadePickerRangeContentArray_Destroy(self.raw()) }
    }

    pub fn set_text_at_index(&mut self, text: &str, index: i32) -> ArkUIResult<()> {
        with_cstring(text, |text_ptr| unsafe {
            OH_ArkUI_TextCascadePickerRangeContentArray_SetTextAtIndex(
                self.raw(),
                text_ptr.cast_mut(),
                index,
            )
        })
    }

    pub fn set_child_at_index(&mut self, child: &TextCascadePickerRangeContentArray, index: i32) {
        unsafe {
            OH_ArkUI_TextCascadePickerRangeContentArray_SetChildAtIndex(
                self.raw(),
                child.raw(),
                index,
            )
        }
    }
}

#[cfg(feature = "api-20")]
/// Embedded component option wrapper.
pub struct EmbeddedComponentOption {
    raw: NonNull<ArkUI_EmbeddedComponentOption>,
    on_error_callback: Option<EmbeddedComponentOnErrorCallbackRegistration>,
    on_terminated_callback: Option<EmbeddedComponentOnTerminatedCallbackRegistration>,
}

#[cfg(feature = "api-20")]
impl EmbeddedComponentOption {
    pub fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_EmbeddedComponentOption_Create() };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_EmbeddedComponentOption_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_EmbeddedComponentOption {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_EmbeddedComponentOption) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_EmbeddedComponentOption"),
            on_error_callback: None,
            on_terminated_callback: None,
        }
    }

    pub(crate) fn into_raw(mut self) -> *mut ArkUI_EmbeddedComponentOption {
        self.clear_on_error();
        self.clear_on_terminated();
        self.raw.as_ptr()
    }

    pub fn dispose(mut self) {
        self.clear_on_error();
        self.clear_on_terminated();
        unsafe { OH_ArkUI_EmbeddedComponentOption_Dispose(self.raw()) }
    }

    pub fn set_on_error<T: Fn(i32, Option<String>, Option<String>) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_error();

        let callback = non_null_or_panic(
            Box::into_raw(Box::new(EmbeddedComponentOnErrorCallbackContext {
                callback: Box::new(callback),
            })),
            "EmbeddedComponentOnErrorCallbackContext",
        );
        let slot = {
            let mut slots = match embedded_component_on_error_callback_slots().lock() {
                Ok(slots) => slots,
                Err(poisoned) => poisoned.into_inner(),
            };
            let reserved = reserve_embedded_component_callback_slot(
                &mut slots,
                callback.as_ptr() as usize,
                "EmbeddedComponentOption on_error",
            );
            if let Err(err) = reserved {
                unsafe {
                    drop(Box::from_raw(callback.as_ptr()));
                }
                return Err(err);
            }
            reserved?
        };
        unsafe {
            OH_ArkUI_EmbeddedComponentOption_SetOnError(
                self.raw(),
                EMBEDDED_COMPONENT_ON_ERROR_CALLBACK_TRAMPOLINES[slot],
            )
        };
        self.on_error_callback =
            Some(EmbeddedComponentOnErrorCallbackRegistration { slot, callback });
        Ok(())
    }

    pub fn clear_on_error(&mut self) {
        unsafe { OH_ArkUI_EmbeddedComponentOption_SetOnError(self.raw(), None) };
        if let Some(registration) = self.on_error_callback.take() {
            release_embedded_component_callback_slot(
                embedded_component_on_error_callback_slots(),
                registration.slot,
            );
            unsafe {
                drop(Box::from_raw(registration.callback.as_ptr()));
            }
        }
    }

    pub fn set_on_terminated<T: Fn(i32, Option<EmbeddedComponentWantRef>) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_terminated();

        let callback = non_null_or_panic(
            Box::into_raw(Box::new(EmbeddedComponentOnTerminatedCallbackContext {
                callback: Box::new(callback),
            })),
            "EmbeddedComponentOnTerminatedCallbackContext",
        );
        let slot = {
            let mut slots = match embedded_component_on_terminated_callback_slots().lock() {
                Ok(slots) => slots,
                Err(poisoned) => poisoned.into_inner(),
            };
            let reserved = reserve_embedded_component_callback_slot(
                &mut slots,
                callback.as_ptr() as usize,
                "EmbeddedComponentOption on_terminated",
            );
            if let Err(err) = reserved {
                unsafe {
                    drop(Box::from_raw(callback.as_ptr()));
                }
                return Err(err);
            }
            reserved?
        };
        unsafe {
            OH_ArkUI_EmbeddedComponentOption_SetOnTerminated(
                self.raw(),
                EMBEDDED_COMPONENT_ON_TERMINATED_CALLBACK_TRAMPOLINES[slot],
            )
        };
        self.on_terminated_callback =
            Some(EmbeddedComponentOnTerminatedCallbackRegistration { slot, callback });
        Ok(())
    }

    pub fn clear_on_terminated(&mut self) {
        unsafe { OH_ArkUI_EmbeddedComponentOption_SetOnTerminated(self.raw(), None) };
        if let Some(registration) = self.on_terminated_callback.take() {
            release_embedded_component_callback_slot(
                embedded_component_on_terminated_callback_slots(),
                registration.slot,
            );
            unsafe {
                drop(Box::from_raw(registration.callback.as_ptr()));
            }
        }
    }
}

#[cfg(feature = "api-20")]
#[derive(Clone, Copy, Debug)]
/// Borrowed reference wrapper for embedded component want object.
pub struct EmbeddedComponentWantRef {
    raw: NonNull<AbilityBase_Want>,
}

#[cfg(feature = "api-20")]
impl EmbeddedComponentWantRef {
    pub(crate) fn from_raw(raw: *mut AbilityBase_Want) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn raw(&self) -> *mut AbilityBase_Want {
        self.raw.as_ptr()
    }
}

#[cfg(feature = "api-20")]
struct EmbeddedComponentOnErrorCallbackContext {
    callback: Box<dyn Fn(i32, Option<String>, Option<String>)>,
}

#[cfg(feature = "api-20")]
struct EmbeddedComponentOnTerminatedCallbackContext {
    callback: Box<dyn Fn(i32, Option<EmbeddedComponentWantRef>)>,
}

#[cfg(feature = "api-20")]
struct EmbeddedComponentOnErrorCallbackRegistration {
    slot: usize,
    callback: NonNull<EmbeddedComponentOnErrorCallbackContext>,
}

#[cfg(feature = "api-20")]
struct EmbeddedComponentOnTerminatedCallbackRegistration {
    slot: usize,
    callback: NonNull<EmbeddedComponentOnTerminatedCallbackContext>,
}

#[cfg(feature = "api-20")]
const EMBEDDED_COMPONENT_CALLBACK_SLOT_COUNT: usize = 16;

#[cfg(feature = "api-20")]
type EmbeddedComponentCallbackSlots = [Option<usize>; EMBEDDED_COMPONENT_CALLBACK_SLOT_COUNT];

#[cfg(feature = "api-20")]
static EMBEDDED_COMPONENT_ON_ERROR_CALLBACK_SLOTS: OnceLock<Mutex<EmbeddedComponentCallbackSlots>> =
    OnceLock::new();
#[cfg(feature = "api-20")]
static EMBEDDED_COMPONENT_ON_TERMINATED_CALLBACK_SLOTS: OnceLock<
    Mutex<EmbeddedComponentCallbackSlots>,
> = OnceLock::new();

#[cfg(feature = "api-20")]
fn embedded_component_on_error_callback_slots() -> &'static Mutex<EmbeddedComponentCallbackSlots> {
    EMBEDDED_COMPONENT_ON_ERROR_CALLBACK_SLOTS
        .get_or_init(|| Mutex::new([None; EMBEDDED_COMPONENT_CALLBACK_SLOT_COUNT]))
}

#[cfg(feature = "api-20")]
fn embedded_component_on_terminated_callback_slots(
) -> &'static Mutex<EmbeddedComponentCallbackSlots> {
    EMBEDDED_COMPONENT_ON_TERMINATED_CALLBACK_SLOTS
        .get_or_init(|| Mutex::new([None; EMBEDDED_COMPONENT_CALLBACK_SLOT_COUNT]))
}

#[cfg(feature = "api-20")]
fn reserve_embedded_component_callback_slot(
    slots: &mut EmbeddedComponentCallbackSlots,
    callback: usize,
    callback_name: &'static str,
) -> ArkUIResult<usize> {
    for (index, slot) in slots.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(callback);
            return Ok(index);
        }
    }
    Err(ArkUIError::new(
        ArkUIErrorCode::ParamInvalid,
        format!("{callback_name} callbacks exceed slot limit"),
    ))
}

#[cfg(feature = "api-20")]
fn release_embedded_component_callback_slot(
    slots: &Mutex<EmbeddedComponentCallbackSlots>,
    slot: usize,
) {
    let mut slots = match slots.lock() {
        Ok(slots) => slots,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(current) = slots.get_mut(slot) {
        *current = None;
    }
}

#[cfg(feature = "api-20")]
type EmbeddedComponentOnErrorCallback =
    unsafe extern "C" fn(code: i32, name: *const c_char, message: *const c_char);
#[cfg(feature = "api-20")]
type EmbeddedComponentOnTerminatedCallback =
    unsafe extern "C" fn(code: i32, want: *mut AbilityBase_Want);

#[cfg(feature = "api-20")]
macro_rules! define_embedded_component_on_error_trampoline {
    ($name:ident, $slot:expr) => {
        unsafe extern "C" fn $name(code: i32, name: *const c_char, message: *const c_char) {
            let callback = {
                let callbacks = match embedded_component_on_error_callback_slots().lock() {
                    Ok(callbacks) => callbacks,
                    Err(poisoned) => poisoned.into_inner(),
                };
                callbacks[$slot]
            };
            let Some(callback) = callback else {
                return;
            };
            let callback =
                unsafe { &*(callback as *const EmbeddedComponentOnErrorCallbackContext) };
            (callback.callback)(
                code,
                c_char_ptr_to_string(name),
                c_char_ptr_to_string(message),
            );
        }
    };
}

#[cfg(feature = "api-20")]
macro_rules! define_embedded_component_on_terminated_trampoline {
    ($name:ident, $slot:expr) => {
        unsafe extern "C" fn $name(code: i32, want: *mut AbilityBase_Want) {
            let callback = {
                let callbacks = match embedded_component_on_terminated_callback_slots().lock() {
                    Ok(callbacks) => callbacks,
                    Err(poisoned) => poisoned.into_inner(),
                };
                callbacks[$slot]
            };
            let Some(callback) = callback else {
                return;
            };
            let callback =
                unsafe { &*(callback as *const EmbeddedComponentOnTerminatedCallbackContext) };
            (callback.callback)(code, EmbeddedComponentWantRef::from_raw(want));
        }
    };
}

#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_0,
    0
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_1,
    1
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_2,
    2
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_3,
    3
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_4,
    4
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_5,
    5
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_6,
    6
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_7,
    7
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_8,
    8
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_9,
    9
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_10,
    10
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_11,
    11
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_12,
    12
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_13,
    13
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_14,
    14
);
#[cfg(feature = "api-20")]
define_embedded_component_on_error_trampoline!(
    embedded_component_on_error_callback_trampoline_15,
    15
);

#[cfg(feature = "api-20")]
const EMBEDDED_COMPONENT_ON_ERROR_CALLBACK_TRAMPOLINES: [Option<EmbeddedComponentOnErrorCallback>;
    EMBEDDED_COMPONENT_CALLBACK_SLOT_COUNT] = [
    Some(embedded_component_on_error_callback_trampoline_0),
    Some(embedded_component_on_error_callback_trampoline_1),
    Some(embedded_component_on_error_callback_trampoline_2),
    Some(embedded_component_on_error_callback_trampoline_3),
    Some(embedded_component_on_error_callback_trampoline_4),
    Some(embedded_component_on_error_callback_trampoline_5),
    Some(embedded_component_on_error_callback_trampoline_6),
    Some(embedded_component_on_error_callback_trampoline_7),
    Some(embedded_component_on_error_callback_trampoline_8),
    Some(embedded_component_on_error_callback_trampoline_9),
    Some(embedded_component_on_error_callback_trampoline_10),
    Some(embedded_component_on_error_callback_trampoline_11),
    Some(embedded_component_on_error_callback_trampoline_12),
    Some(embedded_component_on_error_callback_trampoline_13),
    Some(embedded_component_on_error_callback_trampoline_14),
    Some(embedded_component_on_error_callback_trampoline_15),
];

#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_0,
    0
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_1,
    1
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_2,
    2
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_3,
    3
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_4,
    4
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_5,
    5
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_6,
    6
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_7,
    7
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_8,
    8
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_9,
    9
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_10,
    10
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_11,
    11
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_12,
    12
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_13,
    13
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_14,
    14
);
#[cfg(feature = "api-20")]
define_embedded_component_on_terminated_trampoline!(
    embedded_component_on_terminated_callback_trampoline_15,
    15
);

#[cfg(feature = "api-20")]
const EMBEDDED_COMPONENT_ON_TERMINATED_CALLBACK_TRAMPOLINES: [Option<
    EmbeddedComponentOnTerminatedCallback,
>;
    EMBEDDED_COMPONENT_CALLBACK_SLOT_COUNT] = [
    Some(embedded_component_on_terminated_callback_trampoline_0),
    Some(embedded_component_on_terminated_callback_trampoline_1),
    Some(embedded_component_on_terminated_callback_trampoline_2),
    Some(embedded_component_on_terminated_callback_trampoline_3),
    Some(embedded_component_on_terminated_callback_trampoline_4),
    Some(embedded_component_on_terminated_callback_trampoline_5),
    Some(embedded_component_on_terminated_callback_trampoline_6),
    Some(embedded_component_on_terminated_callback_trampoline_7),
    Some(embedded_component_on_terminated_callback_trampoline_8),
    Some(embedded_component_on_terminated_callback_trampoline_9),
    Some(embedded_component_on_terminated_callback_trampoline_10),
    Some(embedded_component_on_terminated_callback_trampoline_11),
    Some(embedded_component_on_terminated_callback_trampoline_12),
    Some(embedded_component_on_terminated_callback_trampoline_13),
    Some(embedded_component_on_terminated_callback_trampoline_14),
    Some(embedded_component_on_terminated_callback_trampoline_15),
];
