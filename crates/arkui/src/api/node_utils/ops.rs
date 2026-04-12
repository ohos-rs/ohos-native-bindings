//! Module api::node_utils::ops wrappers and related types.

#[cfg(any(feature = "api-13", feature = "api-14", feature = "api-15"))]
use std::os::raw::c_char;
use std::os::raw::c_void;
#[cfg(feature = "api-22")]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

#[cfg(any(feature = "api-14", feature = "api-15"))]
use ohos_arkui_input_binding::sys::ArkUI_NodeHandle;
use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_IntOffset, ArkUI_IntSize, OH_ArkUI_GetContextByNode, OH_ArkUI_List_CloseAllSwipeActions,
    OH_ArkUI_NodeUtils_GetLayoutPosition, OH_ArkUI_NodeUtils_GetLayoutPositionInScreen,
    OH_ArkUI_NodeUtils_GetLayoutPositionInWindow, OH_ArkUI_NodeUtils_GetLayoutSize,
    OH_ArkUI_NodeUtils_GetPositionWithTranslateInScreen,
    OH_ArkUI_NodeUtils_GetPositionWithTranslateInWindow,
    OH_ArkUI_RegisterSystemColorModeChangeEvent, OH_ArkUI_RegisterSystemFontStyleChangeEvent,
    OH_ArkUI_UnregisterSystemColorModeChangeEvent, OH_ArkUI_UnregisterSystemFontStyleChangeEvent,
};
#[cfg(any(feature = "api-14", feature = "api-15"))]
use std::ptr::NonNull;

#[cfg(feature = "api-18")]
use ohos_arkui_sys::OH_ArkUI_NodeUtils_MoveTo;
#[cfg(feature = "api-14")]
use ohos_arkui_sys::{
    ArkUI_ActiveChildrenInfo, ArkUI_CustomProperty, OH_ArkUI_ActiveChildrenInfo_Destroy,
    OH_ArkUI_ActiveChildrenInfo_GetCount, OH_ArkUI_ActiveChildrenInfo_GetNodeByIndex,
    OH_ArkUI_CustomProperty_Destroy, OH_ArkUI_CustomProperty_GetStringValue,
    OH_ArkUI_NodeUtils_GetActiveChildrenInfo, OH_ArkUI_NodeUtils_GetCurrentPageRootNode,
    OH_ArkUI_NodeUtils_GetCustomProperty, OH_ArkUI_NodeUtils_GetNodeType,
    OH_ArkUI_NodeUtils_GetParentInPageTree, OH_ArkUI_NodeUtils_IsCreatedByNDK,
};
#[cfg(feature = "api-15")]
use ohos_arkui_sys::{
    ArkUI_CrossLanguageOption, ArkUI_HostWindowInfo, ArkUI_SnapshotOptions,
    OH_ArkUI_CrossLanguageOption_Create, OH_ArkUI_CrossLanguageOption_Destroy,
    OH_ArkUI_CrossLanguageOption_GetAttributeSettingStatus,
    OH_ArkUI_CrossLanguageOption_SetAttributeSettingStatus, OH_ArkUI_GetNodeSnapshot,
    OH_ArkUI_HostWindowInfo_Destroy, OH_ArkUI_HostWindowInfo_GetName,
    OH_ArkUI_NodeUtils_GetAttachedNodeHandleById, OH_ArkUI_NodeUtils_GetChildWithExpandMode,
    OH_ArkUI_NodeUtils_GetCrossLanguageOption, OH_ArkUI_NodeUtils_GetFirstChildIndexWithoutExpand,
    OH_ArkUI_NodeUtils_GetLastChildIndexWithoutExpand, OH_ArkUI_NodeUtils_GetPositionToParent,
    OH_ArkUI_NodeUtils_GetWindowInfo, OH_ArkUI_NodeUtils_SetCrossLanguageOption,
    OH_ArkUI_RegisterDrawCallbackOnNodeHandle, OH_ArkUI_RegisterLayoutCallbackOnNodeHandle,
    OH_ArkUI_UnregisterDrawCallbackOnNodeHandle, OH_ArkUI_UnregisterLayoutCallbackOnNodeHandle,
};
#[cfg(feature = "api-21")]
use ohos_arkui_sys::{
    ArkUI_NodeEvent, OH_ArkUI_NativeModule_InvalidateAttributes,
    OH_ArkUI_NativeModule_RegisterCommonEvent,
    OH_ArkUI_NativeModule_RegisterCommonVisibleAreaApproximateChangeEvent,
    OH_ArkUI_NativeModule_UnregisterCommonEvent,
    OH_ArkUI_NativeModule_UnregisterCommonVisibleAreaApproximateChangeEvent,
};
#[cfg(feature = "api-20")]
use ohos_arkui_sys::{
    OH_ArkUI_AddSupportedUIStates, OH_ArkUI_NodeUtils_GetLayoutPositionInGlobalDisplay,
    OH_ArkUI_NodeUtils_GetNodeHandleByUniqueId, OH_ArkUI_NodeUtils_GetNodeUniqueId,
    OH_ArkUI_RemoveSupportedUIStates, OH_ArkUI_RunTaskInScope, OH_ArkUI_SetForceDarkConfig,
};
#[cfg(feature = "api-22")]
use ohos_arkui_sys::{
    OH_ArkUI_NativeModule_AdoptChild, OH_ArkUI_NativeModule_RemoveAdoptedChild,
    OH_ArkUI_PostAsyncUITask, OH_ArkUI_PostUITask, OH_ArkUI_PostUITaskAndWait,
};
#[cfg(feature = "api-13")]
use ohos_arkui_sys::{
    OH_ArkUI_NodeUtils_AddCustomProperty, OH_ArkUI_NodeUtils_RemoveCustomProperty,
};
#[cfg(feature = "api-15")]
use ohos_image_native_binding::PixelMapNativeHandle;

use ohos_arkui_sys::{ArkUI_ContextHandle, ArkUI_SystemColorMode, ArkUI_SystemFontStyleEvent};

use crate::{
    check_arkui_status, ArkUIError, ArkUIHandle, ArkUINode, ArkUIResult, NavDestinationState,
    RouterPageState,
};

#[cfg(feature = "api-21")]
use super::types::NodeEventRef;
use super::types::SystemFontStyleEventRef;

#[cfg(feature = "api-15")]
struct NodePersistentVoidCallbackContext {
    callback: Box<dyn Fn()>,
}

struct NodeOneShotVoidCallbackContext {
    callback: Box<dyn Fn()>,
}

#[cfg(feature = "api-20")]
struct NodeSupportedUIStatesCallbackContext {
    callback: Box<dyn Fn(i32)>,
}

#[cfg(feature = "api-21")]
struct NodeEventCallbackContext {
    callback: Box<dyn Fn(NodeEventRef)>,
}

struct NodeSystemColorModeCallbackContext {
    callback: Box<dyn Fn(ArkUI_SystemColorMode)>,
}

struct NodeSystemFontStyleCallbackContext {
    callback: Box<dyn Fn(SystemFontStyleEventRef)>,
}

#[cfg(feature = "api-20")]
struct NodeForceDarkCallbackContext {
    callback: Box<dyn Fn(u32) -> u32>,
}

#[cfg(any(feature = "api-18", feature = "api-20"))]
struct NodeTimestampFrameCallbackContext {
    callback: Box<dyn Fn(u64, u32)>,
}

#[cfg(feature = "api-15")]
type NodePersistentVoidCallbackMap = HashMap<usize, usize>;
#[cfg(feature = "api-20")]
type NodeSupportedUIStatesCallbackMap = HashMap<usize, NodeSupportedUIStateRegistration>;
#[cfg(feature = "api-21")]
type NodeCommonEventCallbackMap = HashMap<(usize, usize), usize>;
#[cfg(feature = "api-21")]
type NodeVisibleAreaEventCallbackMap = HashMap<usize, usize>;
type NodeSystemColorModeCallbackMap = HashMap<usize, usize>;
type NodeSystemFontStyleCallbackMap = HashMap<usize, usize>;
#[cfg(feature = "api-20")]
type NodeForceDarkCallbackSlots = [Option<usize>; FORCE_DARK_CALLBACK_SLOT_COUNT];
#[cfg(feature = "api-20")]
type NodeForceDarkCallbackRegistrationMap = HashMap<(usize, usize), usize>;

#[cfg(feature = "api-20")]
struct NodeSupportedUIStateRegistration {
    callback: usize,
    ui_states: i32,
}

#[cfg(feature = "api-15")]
static LAYOUT_CALLBACK_CONTEXTS: OnceLock<Mutex<NodePersistentVoidCallbackMap>> = OnceLock::new();
#[cfg(feature = "api-15")]
static DRAW_CALLBACK_CONTEXTS: OnceLock<Mutex<NodePersistentVoidCallbackMap>> = OnceLock::new();
#[cfg(feature = "api-20")]
static SUPPORTED_UI_STATES_CALLBACK_CONTEXTS: OnceLock<Mutex<NodeSupportedUIStatesCallbackMap>> =
    OnceLock::new();
#[cfg(feature = "api-21")]
static COMMON_EVENT_CALLBACK_CONTEXTS: OnceLock<Mutex<NodeCommonEventCallbackMap>> =
    OnceLock::new();
#[cfg(feature = "api-21")]
static VISIBLE_AREA_EVENT_CALLBACK_CONTEXTS: OnceLock<Mutex<NodeVisibleAreaEventCallbackMap>> =
    OnceLock::new();
static SYSTEM_COLOR_MODE_CALLBACK_CONTEXTS: OnceLock<Mutex<NodeSystemColorModeCallbackMap>> =
    OnceLock::new();
static SYSTEM_FONT_STYLE_CALLBACK_CONTEXTS: OnceLock<Mutex<NodeSystemFontStyleCallbackMap>> =
    OnceLock::new();
#[cfg(feature = "api-20")]
static FORCE_DARK_CALLBACK_SLOTS: OnceLock<Mutex<NodeForceDarkCallbackSlots>> = OnceLock::new();
#[cfg(feature = "api-20")]
static FORCE_DARK_CALLBACK_REGISTRATIONS: OnceLock<Mutex<NodeForceDarkCallbackRegistrationMap>> =
    OnceLock::new();

#[cfg(feature = "api-20")]
const FORCE_DARK_CALLBACK_SLOT_COUNT: usize = 16;
#[cfg(feature = "api-20")]
type ForceDarkInvertCallback = unsafe extern "C" fn(color: u32) -> u32;

#[cfg(feature = "api-15")]
fn layout_callback_contexts() -> &'static Mutex<NodePersistentVoidCallbackMap> {
    LAYOUT_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(feature = "api-15")]
fn draw_callback_contexts() -> &'static Mutex<NodePersistentVoidCallbackMap> {
    DRAW_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(feature = "api-20")]
fn supported_ui_states_callback_contexts() -> &'static Mutex<NodeSupportedUIStatesCallbackMap> {
    SUPPORTED_UI_STATES_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(feature = "api-21")]
fn common_event_callback_contexts() -> &'static Mutex<NodeCommonEventCallbackMap> {
    COMMON_EVENT_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(feature = "api-21")]
fn visible_area_event_callback_contexts() -> &'static Mutex<NodeVisibleAreaEventCallbackMap> {
    VISIBLE_AREA_EVENT_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn system_color_mode_callback_contexts() -> &'static Mutex<NodeSystemColorModeCallbackMap> {
    SYSTEM_COLOR_MODE_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn system_font_style_callback_contexts() -> &'static Mutex<NodeSystemFontStyleCallbackMap> {
    SYSTEM_FONT_STYLE_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(feature = "api-20")]
fn force_dark_callback_slots() -> &'static Mutex<NodeForceDarkCallbackSlots> {
    FORCE_DARK_CALLBACK_SLOTS.get_or_init(|| Mutex::new([None; FORCE_DARK_CALLBACK_SLOT_COUNT]))
}

#[cfg(feature = "api-20")]
fn force_dark_callback_registrations() -> &'static Mutex<NodeForceDarkCallbackRegistrationMap> {
    FORCE_DARK_CALLBACK_REGISTRATIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(feature = "api-15")]
unsafe extern "C" fn node_persistent_void_callback_trampoline(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut NodePersistentVoidCallbackContext) };
    (callback.callback)();
}

unsafe extern "C" fn node_one_shot_void_callback_trampoline(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { Box::from_raw(user_data as *mut NodeOneShotVoidCallbackContext) };
    (callback.callback)();
}

#[cfg(feature = "api-20")]
unsafe extern "C" fn node_supported_ui_states_callback_trampoline(
    current_states: i32,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut NodeSupportedUIStatesCallbackContext) };
    (callback.callback)(current_states);
}

#[cfg(feature = "api-21")]
unsafe extern "C" fn node_event_callback_trampoline(event: *mut ArkUI_NodeEvent) {
    let Some(event) = NodeEventRef::from_raw(event) else {
        return;
    };
    let user_data = event.user_data();
    let Some(user_data) = user_data else {
        return;
    };
    let callback = unsafe { &*(user_data.as_ptr() as *mut NodeEventCallbackContext) };
    (callback.callback)(event);
}

unsafe extern "C" fn node_system_color_mode_callback_trampoline(
    color_mode: ArkUI_SystemColorMode,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut NodeSystemColorModeCallbackContext) };
    (callback.callback)(color_mode);
}

unsafe extern "C" fn node_system_font_style_callback_trampoline(
    event: *mut ArkUI_SystemFontStyleEvent,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let Some(event) = SystemFontStyleEventRef::from_raw(event) else {
        return;
    };
    let callback = unsafe { &*(user_data as *mut NodeSystemFontStyleCallbackContext) };
    (callback.callback)(event);
}

#[cfg(feature = "api-20")]
fn force_dark_slot_callback(slot: usize, color: u32) -> u32 {
    let slots = match force_dark_callback_slots().lock() {
        Ok(slots) => slots,
        Err(poisoned) => poisoned.into_inner(),
    };
    let Some(callback) = slots.get(slot).and_then(|slot_callback| *slot_callback) else {
        return color;
    };
    let callback = unsafe { &*(callback as *mut NodeForceDarkCallbackContext) };
    (callback.callback)(color)
}

#[cfg(feature = "api-20")]
macro_rules! define_force_dark_callback_trampoline {
    ($name:ident, $slot:expr) => {
        unsafe extern "C" fn $name(color: u32) -> u32 {
            force_dark_slot_callback($slot, color)
        }
    };
}

#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_0, 0);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_1, 1);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_2, 2);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_3, 3);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_4, 4);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_5, 5);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_6, 6);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_7, 7);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_8, 8);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_9, 9);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_10, 10);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_11, 11);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_12, 12);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_13, 13);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_14, 14);
#[cfg(feature = "api-20")]
define_force_dark_callback_trampoline!(force_dark_callback_trampoline_15, 15);

#[cfg(feature = "api-20")]
const FORCE_DARK_CALLBACK_TRAMPOLINES: [Option<ForceDarkInvertCallback>;
    FORCE_DARK_CALLBACK_SLOT_COUNT] = [
    Some(force_dark_callback_trampoline_0),
    Some(force_dark_callback_trampoline_1),
    Some(force_dark_callback_trampoline_2),
    Some(force_dark_callback_trampoline_3),
    Some(force_dark_callback_trampoline_4),
    Some(force_dark_callback_trampoline_5),
    Some(force_dark_callback_trampoline_6),
    Some(force_dark_callback_trampoline_7),
    Some(force_dark_callback_trampoline_8),
    Some(force_dark_callback_trampoline_9),
    Some(force_dark_callback_trampoline_10),
    Some(force_dark_callback_trampoline_11),
    Some(force_dark_callback_trampoline_12),
    Some(force_dark_callback_trampoline_13),
    Some(force_dark_callback_trampoline_14),
    Some(force_dark_callback_trampoline_15),
];

#[cfg(feature = "api-20")]
fn reserve_force_dark_callback_slot(
    callback: *mut NodeForceDarkCallbackContext,
) -> ArkUIResult<usize> {
    let mut slots = match force_dark_callback_slots().lock() {
        Ok(slots) => slots,
        Err(poisoned) => poisoned.into_inner(),
    };
    for (index, slot) in slots.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(callback as usize);
            return Ok(index);
        }
    }
    Err(ArkUIError::new(
        ArkUIErrorCode::ParamInvalid,
        "force dark callback slots exceeded limit",
    ))
}

#[cfg(feature = "api-20")]
fn take_force_dark_callback_slot(slot_index: usize) -> Option<usize> {
    let mut slots = match force_dark_callback_slots().lock() {
        Ok(slots) => slots,
        Err(poisoned) => poisoned.into_inner(),
    };
    slots.get_mut(slot_index).and_then(|slot| slot.take())
}

#[cfg(feature = "api-20")]
fn remove_force_dark_callback_registration(
    ui_context: ArkUI_ContextHandle,
    node_type: crate::ArkUINodeType,
) {
    let key = (ui_context as usize, node_type as usize);
    let slot = {
        let mut registrations = match force_dark_callback_registrations().lock() {
            Ok(registrations) => registrations,
            Err(poisoned) => poisoned.into_inner(),
        };
        registrations.remove(&key)
    };
    let Some(slot) = slot else {
        return;
    };
    let Some(callback) = take_force_dark_callback_slot(slot) else {
        return;
    };
    unsafe {
        drop(Box::from_raw(callback as *mut NodeForceDarkCallbackContext));
    }
}

#[cfg(feature = "api-20")]
unsafe extern "C" fn node_idle_callback_trampoline(
    nano_time_left: u64,
    frame_count: u32,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { Box::from_raw(user_data as *mut NodeTimestampFrameCallbackContext) };
    (callback.callback)(nano_time_left, frame_count);
}

#[cfg(feature = "api-18")]
unsafe extern "C" fn node_frame_callback_trampoline(
    nano_timestamp: u64,
    frame_count: u32,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { Box::from_raw(user_data as *mut NodeTimestampFrameCallbackContext) };
    (callback.callback)(nano_timestamp, frame_count);
}

#[cfg(feature = "api-22")]
struct NodeAsyncTaskCallbackContext {
    async_ui_task: Box<dyn Fn()>,
    on_finish: Option<Box<dyn Fn()>>,
    pending: AtomicUsize,
}

#[cfg(feature = "api-22")]
unsafe extern "C" fn node_async_ui_task_trampoline(task_data: *mut c_void) {
    if task_data.is_null() {
        return;
    }
    let callback = unsafe { &*(task_data as *mut NodeAsyncTaskCallbackContext) };
    (callback.async_ui_task)();
    if callback.pending.fetch_sub(1, Ordering::AcqRel) == 1 {
        unsafe {
            drop(Box::from_raw(
                task_data as *mut NodeAsyncTaskCallbackContext,
            ));
        }
    }
}

#[cfg(feature = "api-22")]
unsafe extern "C" fn node_async_ui_task_on_finish_trampoline(task_data: *mut c_void) {
    if task_data.is_null() {
        return;
    }
    let callback = unsafe { &*(task_data as *mut NodeAsyncTaskCallbackContext) };
    if let Some(on_finish) = callback.on_finish.as_ref() {
        on_finish();
    }
    if callback.pending.fetch_sub(1, Ordering::AcqRel) == 1 {
        unsafe {
            drop(Box::from_raw(
                task_data as *mut NodeAsyncTaskCallbackContext,
            ));
        }
    }
}

#[cfg(feature = "api-14")]
/// Wrapper for custom property values returned by node-utils APIs.
pub struct CustomProperty {
    raw: NonNull<ArkUI_CustomProperty>,
}

#[cfg(feature = "api-14")]
impl CustomProperty {
    pub(crate) fn from_raw(raw: *mut ArkUI_CustomProperty) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn get(node: ArkUI_NodeHandle, name: *const c_char) -> ArkUIResult<Option<Self>> {
        let mut handle = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetCustomProperty(
                node,
                name,
                &mut handle
            ))
        }?;
        Ok(Self::from_raw(handle))
    }

    fn raw(&self) -> *mut ArkUI_CustomProperty {
        self.raw.as_ptr()
    }

    pub fn string_value(&self) -> Option<String> {
        let value = unsafe { OH_ArkUI_CustomProperty_GetStringValue(self.raw()) };
        if value.is_null() {
            None
        } else {
            Some(
                unsafe { std::ffi::CStr::from_ptr(value) }
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }
}

#[cfg(feature = "api-14")]
impl Drop for CustomProperty {
    fn drop(&mut self) {
        unsafe { OH_ArkUI_CustomProperty_Destroy(self.raw()) }
    }
}

#[cfg(feature = "api-14")]
/// Wrapper for active-children info of a lazy list/container node.
pub struct ActiveChildrenInfo {
    raw: NonNull<ArkUI_ActiveChildrenInfo>,
}

#[cfg(feature = "api-14")]
impl ActiveChildrenInfo {
    pub(crate) fn from_raw(raw: *mut ArkUI_ActiveChildrenInfo) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn get(node: ArkUI_NodeHandle) -> ArkUIResult<Option<Self>> {
        let mut handle = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetActiveChildrenInfo(node, &mut handle))
        }?;
        Ok(Self::from_raw(handle))
    }

    fn raw(&self) -> *mut ArkUI_ActiveChildrenInfo {
        self.raw.as_ptr()
    }

    pub fn count(&self) -> i32 {
        unsafe { OH_ArkUI_ActiveChildrenInfo_GetCount(self.raw()) }
    }

    pub fn node_by_index(&self, index: i32) -> Option<ArkUINode> {
        let node = unsafe { OH_ArkUI_ActiveChildrenInfo_GetNodeByIndex(self.raw(), index) };
        ArkUINode::from_raw_handle(node)
    }
}

#[cfg(feature = "api-14")]
impl Drop for ActiveChildrenInfo {
    fn drop(&mut self) {
        unsafe { OH_ArkUI_ActiveChildrenInfo_Destroy(self.raw()) }
    }
}

#[cfg(feature = "api-15")]
/// Host window metadata associated with a node.
pub struct HostWindowInfo {
    raw: NonNull<ArkUI_HostWindowInfo>,
}

#[cfg(feature = "api-15")]
impl HostWindowInfo {
    pub(crate) fn from_raw(raw: *mut ArkUI_HostWindowInfo) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn get(node: ArkUI_NodeHandle) -> ArkUIResult<Option<Self>> {
        let mut info = std::ptr::null_mut();
        unsafe { check_arkui_status!(OH_ArkUI_NodeUtils_GetWindowInfo(node, &mut info)) }?;
        Ok(Self::from_raw(info))
    }

    fn raw(&self) -> *mut ArkUI_HostWindowInfo {
        self.raw.as_ptr()
    }

    pub fn name(&self) -> Option<String> {
        let name = unsafe { OH_ArkUI_HostWindowInfo_GetName(self.raw()) };
        if name.is_null() {
            None
        } else {
            Some(
                unsafe { std::ffi::CStr::from_ptr(name) }
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }
}

#[cfg(feature = "api-15")]
impl Drop for HostWindowInfo {
    fn drop(&mut self) {
        unsafe { OH_ArkUI_HostWindowInfo_Destroy(self.raw()) }
    }
}

#[cfg(feature = "api-15")]
/// Options controlling cross-language attribute setting behavior.
pub struct CrossLanguageOption {
    raw: NonNull<ArkUI_CrossLanguageOption>,
}

#[cfg(feature = "api-15")]
impl CrossLanguageOption {
    pub fn new() -> Option<Self> {
        let option = unsafe { OH_ArkUI_CrossLanguageOption_Create() };
        NonNull::new(option).map(|raw| Self { raw })
    }

    fn raw(&self) -> *mut ArkUI_CrossLanguageOption {
        self.raw.as_ptr()
    }

    pub fn set_attribute_setting_status(&mut self, enabled: bool) {
        unsafe { OH_ArkUI_CrossLanguageOption_SetAttributeSettingStatus(self.raw(), enabled) }
    }

    pub fn attribute_setting_status(&self) -> bool {
        unsafe { OH_ArkUI_CrossLanguageOption_GetAttributeSettingStatus(self.raw()) }
    }
}

#[cfg(feature = "api-15")]
impl Drop for CrossLanguageOption {
    fn drop(&mut self) {
        unsafe { OH_ArkUI_CrossLanguageOption_Destroy(self.raw()) }
    }
}

#[cfg(feature = "api-15")]
/// Snapshot option wrapper used when capturing node images.
pub struct SnapshotOptions {
    raw: NonNull<ArkUI_SnapshotOptions>,
}

#[cfg(feature = "api-15")]
impl SnapshotOptions {
    pub fn new() -> ArkUIResult<Self> {
        let options = unsafe { ohos_arkui_sys::OH_ArkUI_CreateSnapshotOptions() };
        NonNull::new(options)
            .map(|raw| Self { raw })
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_CreateSnapshotOptions returned null",
                )
            })
    }

    fn raw(&self) -> *mut ArkUI_SnapshotOptions {
        self.raw.as_ptr()
    }

    pub fn set_scale(&mut self, scale: f32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_SnapshotOptions_SetScale(
                self.raw(),
                scale
            ))
        }
    }
}

#[cfg(feature = "api-15")]
impl Drop for SnapshotOptions {
    fn drop(&mut self) {
        unsafe { ohos_arkui_sys::OH_ArkUI_DestroySnapshotOptions(self.raw()) }
    }
}

impl ArkUIHandle {
    pub(crate) fn context_by_node(&self, node: &ArkUINode) -> ArkUIResult<ArkUI_ContextHandle> {
        let _ = self.raw();
        let context = unsafe { OH_ArkUI_GetContextByNode(node.raw()) };
        if context.is_null() {
            Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_GetContextByNode returned null",
            ))
        } else {
            Ok(context)
        }
    }

    #[cfg(feature = "api-22")]
    pub(crate) fn post_ui_task<T: Fn() + 'static>(
        &self,
        node: &ArkUINode,
        task: T,
    ) -> ArkUIResult<()> {
        let context = self.context_by_node(node)?;
        let callback = Box::into_raw(Box::new(NodeOneShotVoidCallbackContext {
            callback: Box::new(task),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_PostUITask(
                context,
                callback.cast(),
                Some(node_one_shot_void_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        Ok(())
    }

    #[cfg(feature = "api-22")]
    pub(crate) fn post_ui_task_and_wait<T: Fn() + 'static>(
        &self,
        node: &ArkUINode,
        task: T,
    ) -> ArkUIResult<()> {
        let context = self.context_by_node(node)?;
        let callback = Box::into_raw(Box::new(NodeOneShotVoidCallbackContext {
            callback: Box::new(task),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_PostUITaskAndWait(
                context,
                callback.cast(),
                Some(node_one_shot_void_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        Ok(())
    }

    pub(crate) fn list_close_all_swipe_actions<T: Fn() + 'static>(
        &self,
        node: &ArkUINode,
        on_finish: T,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let callback = Box::into_raw(Box::new(NodeOneShotVoidCallbackContext {
            callback: Box::new(on_finish),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_List_CloseAllSwipeActions(
                node.raw(),
                callback.cast(),
                Some(node_one_shot_void_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        Ok(())
    }

    pub(crate) fn register_system_color_mode_change_event<
        T: Fn(ArkUI_SystemColorMode) + 'static,
    >(
        &self,
        node: &ArkUINode,
        on_color_mode_change: T,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let callback = Box::into_raw(Box::new(NodeSystemColorModeCallbackContext {
            callback: Box::new(on_color_mode_change),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_RegisterSystemColorModeChangeEvent(
                node.raw(),
                callback.cast(),
                Some(node_system_color_mode_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match system_color_mode_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(node.raw() as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(
                    old as *mut NodeSystemColorModeCallbackContext,
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn unregister_system_color_mode_change_event(&self, node: &ArkUINode) {
        let _ = self.raw();
        unsafe { OH_ArkUI_UnregisterSystemColorModeChangeEvent(node.raw()) }
        let mut callbacks = match system_color_mode_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(node.raw() as usize)) {
            unsafe {
                drop(Box::from_raw(
                    callback as *mut NodeSystemColorModeCallbackContext,
                ));
            }
        }
    }

    pub(crate) fn register_system_font_style_change_event<
        T: Fn(SystemFontStyleEventRef) + 'static,
    >(
        &self,
        node: &ArkUINode,
        on_font_style_change: T,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let callback = Box::into_raw(Box::new(NodeSystemFontStyleCallbackContext {
            callback: Box::new(on_font_style_change),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_RegisterSystemFontStyleChangeEvent(
                node.raw(),
                callback.cast(),
                Some(node_system_font_style_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match system_font_style_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(node.raw() as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(
                    old as *mut NodeSystemFontStyleCallbackContext,
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn unregister_system_font_style_change_event(&self, node: &ArkUINode) {
        let _ = self.raw();
        unsafe { OH_ArkUI_UnregisterSystemFontStyleChangeEvent(node.raw()) }
        let mut callbacks = match system_font_style_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(node.raw() as usize)) {
            unsafe {
                drop(Box::from_raw(
                    callback as *mut NodeSystemFontStyleCallbackContext,
                ));
            }
        }
    }
}

impl ArkUIHandle {
    pub(crate) fn get_layout_size(&self, node: &ArkUINode) -> ArkUIResult<ArkUI_IntSize> {
        let _ = self.raw();
        let mut size: ArkUI_IntSize = unsafe { std::mem::zeroed() };
        unsafe { check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutSize(node.raw(), &mut size)) }?;
        Ok(size)
    }

    pub(crate) fn get_layout_position(&self, node: &ArkUINode) -> ArkUIResult<ArkUI_IntOffset> {
        let _ = self.raw();
        let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutPosition(
                node.raw(),
                &mut offset
            ))
        }?;
        Ok(offset)
    }

    pub(crate) fn get_layout_position_in_window(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<ArkUI_IntOffset> {
        let _ = self.raw();
        let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutPositionInWindow(
                node.raw(),
                &mut offset
            ))
        }?;
        Ok(offset)
    }

    pub(crate) fn get_layout_position_in_screen(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<ArkUI_IntOffset> {
        let _ = self.raw();
        let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutPositionInScreen(
                node.raw(),
                &mut offset
            ))
        }?;
        Ok(offset)
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn get_layout_position_in_global_display(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<ArkUI_IntOffset> {
        let _ = self.raw();
        let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutPositionInGlobalDisplay(
                node.raw(),
                &mut offset
            ))
        }?;
        Ok(offset)
    }

    pub(crate) fn get_position_with_translate_in_window(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<ArkUI_IntOffset> {
        let _ = self.raw();
        let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetPositionWithTranslateInWindow(
                node.raw(),
                &mut offset
            ))
        }?;
        Ok(offset)
    }

    pub(crate) fn get_position_with_translate_in_screen(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<ArkUI_IntOffset> {
        let _ = self.raw();
        let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetPositionWithTranslateInScreen(
                node.raw(),
                &mut offset
            ))
        }?;
        Ok(offset)
    }

    #[cfg(feature = "api-13")]
    pub(crate) fn add_custom_property(
        &self,
        node: &ArkUINode,
        name: *const c_char,
        value: *const c_char,
    ) {
        let _ = self.raw();
        unsafe { OH_ArkUI_NodeUtils_AddCustomProperty(node.raw(), name, value) }
    }

    #[cfg(feature = "api-13")]
    pub(crate) fn remove_custom_property(&self, node: &ArkUINode, name: *const c_char) {
        let _ = self.raw();
        unsafe { OH_ArkUI_NodeUtils_RemoveCustomProperty(node.raw(), name) }
    }

    #[cfg(feature = "api-14")]
    pub(crate) fn get_custom_property(
        &self,
        node: &ArkUINode,
        name: *const c_char,
    ) -> ArkUIResult<Option<CustomProperty>> {
        let _ = self.raw();
        CustomProperty::get(node.raw(), name)
    }

    #[cfg(feature = "api-14")]
    pub(crate) fn get_active_children_info(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<Option<ActiveChildrenInfo>> {
        let _ = self.raw();
        ActiveChildrenInfo::get(node.raw())
    }

    #[cfg(feature = "api-14")]
    pub(crate) fn get_parent_in_page_tree(&self, node: &ArkUINode) -> Option<ArkUINode> {
        let _ = self.raw();
        let parent = unsafe { OH_ArkUI_NodeUtils_GetParentInPageTree(node.raw()) };
        ArkUINode::from_raw_handle(parent)
    }

    #[cfg(feature = "api-14")]
    pub(crate) fn get_current_page_root_node(&self, node: &ArkUINode) -> Option<ArkUINode> {
        let _ = self.raw();
        let root = unsafe { OH_ArkUI_NodeUtils_GetCurrentPageRootNode(node.raw()) };
        ArkUINode::from_raw_handle(root)
    }

    #[cfg(feature = "api-14")]
    pub(crate) fn is_created_by_ndk(&self, node: &ArkUINode) -> bool {
        let _ = self.raw();
        unsafe { OH_ArkUI_NodeUtils_IsCreatedByNDK(node.raw()) }
    }

    #[cfg(feature = "api-14")]
    pub(crate) fn get_node_type(&self, node: &ArkUINode) -> Option<crate::ArkUINodeType> {
        let _ = self.raw();
        let raw = unsafe { OH_ArkUI_NodeUtils_GetNodeType(node.raw()) };
        if raw < 0 {
            None
        } else {
            crate::ArkUINodeType::try_from_raw(raw as ohos_arkui_sys::ArkUI_NodeType)
        }
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn get_window_info(&self, node: &ArkUINode) -> ArkUIResult<Option<HostWindowInfo>> {
        let _ = self.raw();
        HostWindowInfo::get(node.raw())
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn get_first_child_index_without_expand(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<u32> {
        let _ = self.raw();
        let mut index = 0u32;
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetFirstChildIndexWithoutExpand(
                node.raw(),
                &mut index
            ))
        }?;
        Ok(index)
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn get_last_child_index_without_expand(&self, node: &ArkUINode) -> ArkUIResult<u32> {
        let _ = self.raw();
        let mut index = 0u32;
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetLastChildIndexWithoutExpand(
                node.raw(),
                &mut index
            ))
        }?;
        Ok(index)
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn get_child_with_expand_mode(
        &self,
        node: &ArkUINode,
        position: i32,
        expand_mode: u32,
    ) -> ArkUIResult<Option<ArkUINode>> {
        let _ = self.raw();
        let mut subnode = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetChildWithExpandMode(
                node.raw(),
                position,
                &mut subnode,
                expand_mode
            ))
        }?;
        Ok(ArkUINode::from_raw_handle(subnode))
    }

    #[cfg(feature = "api-18")]
    pub(crate) fn move_to(
        &self,
        node: &ArkUINode,
        target_parent: &ArkUINode,
        index: i32,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_MoveTo(
                node.raw(),
                target_parent.raw(),
                index
            ))
        }
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn set_cross_language_option(
        &self,
        node: &ArkUINode,
        option: &CrossLanguageOption,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_SetCrossLanguageOption(
                node.raw(),
                option.raw()
            ))
        }
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn get_cross_language_option(
        &self,
        node: &ArkUINode,
        option: &mut CrossLanguageOption,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetCrossLanguageOption(
                node.raw(),
                option.raw()
            ))
        }
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn unregister_layout_callback_on_node_handle(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(OH_ArkUI_UnregisterLayoutCallbackOnNodeHandle(node.raw())) }?;
        let mut callbacks = match layout_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(node.raw() as usize)) {
            unsafe {
                drop(Box::from_raw(
                    callback as *mut NodePersistentVoidCallbackContext,
                ));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn unregister_draw_callback_on_node_handle(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(OH_ArkUI_UnregisterDrawCallbackOnNodeHandle(node.raw())) }?;
        let mut callbacks = match draw_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(node.raw() as usize)) {
            unsafe {
                drop(Box::from_raw(
                    callback as *mut NodePersistentVoidCallbackContext,
                ));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn register_layout_callback_on_node_handle<T: Fn() + 'static>(
        &self,
        node: &ArkUINode,
        on_layout_completed: T,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let _ = self.unregister_layout_callback_on_node_handle(node);
        let callback = Box::into_raw(Box::new(NodePersistentVoidCallbackContext {
            callback: Box::new(on_layout_completed),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_RegisterLayoutCallbackOnNodeHandle(
                node.raw(),
                callback.cast(),
                Some(node_persistent_void_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match layout_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(node.raw() as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(old as *mut NodePersistentVoidCallbackContext));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn register_draw_callback_on_node_handle<T: Fn() + 'static>(
        &self,
        node: &ArkUINode,
        on_draw_completed: T,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let _ = self.unregister_draw_callback_on_node_handle(node);
        let callback = Box::into_raw(Box::new(NodePersistentVoidCallbackContext {
            callback: Box::new(on_draw_completed),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_RegisterDrawCallbackOnNodeHandle(
                node.raw(),
                callback.cast(),
                Some(node_persistent_void_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match draw_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(node.raw() as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(old as *mut NodePersistentVoidCallbackContext));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn get_position_to_parent(&self, node: &ArkUINode) -> ArkUIResult<ArkUI_IntOffset> {
        let _ = self.raw();
        let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetPositionToParent(
                node.raw(),
                &mut offset
            ))
        }?;
        Ok(offset)
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn get_attached_node_handle_by_id(
        &self,
        id: *const c_char,
    ) -> ArkUIResult<Option<ArkUINode>> {
        let _ = self.raw();
        let mut node = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetAttachedNodeHandleById(id, &mut node))
        }?;
        Ok(ArkUINode::from_raw_handle(node))
    }

    #[cfg(feature = "api-15")]
    pub(crate) fn get_node_snapshot(
        &self,
        node: &ArkUINode,
        snapshot_options: Option<&SnapshotOptions>,
    ) -> ArkUIResult<Option<PixelMapNativeHandle>> {
        let _ = self.raw();
        let mut pixelmap = std::ptr::null_mut();
        let snapshot_options = snapshot_options
            .map(SnapshotOptions::raw)
            .unwrap_or(std::ptr::null_mut());
        unsafe {
            check_arkui_status!(OH_ArkUI_GetNodeSnapshot(
                node.raw(),
                snapshot_options,
                &mut pixelmap
            ))
        }?;
        Ok(PixelMapNativeHandle::from_raw(pixelmap.cast()))
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn get_node_unique_id(&self, node: &ArkUINode) -> ArkUIResult<i32> {
        let _ = self.raw();
        let mut unique_id = -1;
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetNodeUniqueId(
                node.raw(),
                &mut unique_id
            ))
        }?;
        Ok(unique_id)
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn add_supported_ui_states<T: Fn(i32) + 'static>(
        &self,
        node: &ArkUINode,
        ui_states: i32,
        states_change_handler: T,
        exclude_inner: bool,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let callback = Box::into_raw(Box::new(NodeSupportedUIStatesCallbackContext {
            callback: Box::new(states_change_handler),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_AddSupportedUIStates(
                node.raw(),
                ui_states,
                Some(node_supported_ui_states_callback_trampoline),
                exclude_inner,
                callback.cast()
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match supported_ui_states_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(
            node.raw() as usize,
            NodeSupportedUIStateRegistration {
                callback: callback as usize,
                ui_states,
            },
        ) {
            unsafe {
                drop(Box::from_raw(
                    old.callback as *mut NodeSupportedUIStatesCallbackContext,
                ));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn remove_supported_ui_states(
        &self,
        node: &ArkUINode,
        ui_states: i32,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(OH_ArkUI_RemoveSupportedUIStates(node.raw(), ui_states)) }?;
        let key = node.raw() as usize;
        let mut callbacks = match supported_ui_states_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        let should_remove = if let Some(registration) = callbacks.get_mut(&key) {
            registration.ui_states &= !ui_states;
            registration.ui_states == 0
        } else {
            false
        };
        if should_remove {
            if let Some(registration) = callbacks.remove(&key) {
                unsafe {
                    drop(Box::from_raw(
                        registration.callback as *mut NodeSupportedUIStatesCallbackContext,
                    ));
                }
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn run_task_in_scope<T: Fn() + 'static>(
        &self,
        node: &ArkUINode,
        callback: T,
    ) -> ArkUIResult<()> {
        let context = self.context_by_node(node)?;
        let callback = Box::into_raw(Box::new(NodeOneShotVoidCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_RunTaskInScope(
                context,
                callback.cast(),
                Some(node_one_shot_void_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        Ok(())
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn get_node_handle_by_unique_id(
        &self,
        unique_id: u32,
    ) -> ArkUIResult<Option<ArkUINode>> {
        let _ = self.raw();
        let mut node = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetNodeHandleByUniqueId(
                unique_id, &mut node
            ))
        }?;
        Ok(ArkUINode::from_raw_handle(node))
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn clear_force_dark_config(
        &self,
        node: &ArkUINode,
        force_dark: bool,
        node_type: crate::ArkUINodeType,
    ) -> ArkUIResult<()> {
        let context = self.context_by_node(node)?;
        unsafe {
            check_arkui_status!(OH_ArkUI_SetForceDarkConfig(
                context,
                force_dark,
                node_type.into(),
                None
            ))
        }?;
        remove_force_dark_callback_registration(context, node_type);
        Ok(())
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn set_force_dark_config_with_color_invert<T: Fn(u32) -> u32 + 'static>(
        &self,
        node: &ArkUINode,
        force_dark: bool,
        node_type: crate::ArkUINodeType,
        color_invert: T,
    ) -> ArkUIResult<()> {
        let context = self.context_by_node(node)?;
        let callback = Box::into_raw(Box::new(NodeForceDarkCallbackContext {
            callback: Box::new(color_invert),
        }));
        let slot = match reserve_force_dark_callback_slot(callback) {
            Ok(slot) => slot,
            Err(err) => {
                unsafe {
                    drop(Box::from_raw(callback));
                }
                return Err(err);
            }
        };
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_SetForceDarkConfig(
                context,
                force_dark,
                node_type.into(),
                FORCE_DARK_CALLBACK_TRAMPOLINES[slot]
            ))
        };
        if let Err(err) = result {
            let callback = take_force_dark_callback_slot(slot);
            if let Some(callback) = callback {
                unsafe {
                    drop(Box::from_raw(callback as *mut NodeForceDarkCallbackContext));
                }
            }
            return Err(err);
        }
        let old_slot = {
            let key = (context as usize, node_type as usize);
            let mut registrations = match force_dark_callback_registrations().lock() {
                Ok(registrations) => registrations,
                Err(poisoned) => poisoned.into_inner(),
            };
            registrations.insert(key, slot)
        };
        if let Some(old_slot) = old_slot {
            let callback = take_force_dark_callback_slot(old_slot);
            if let Some(callback) = callback {
                unsafe {
                    drop(Box::from_raw(callback as *mut NodeForceDarkCallbackContext));
                }
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-21")]
    pub(crate) fn invalidate_attributes(&self, node: &ArkUINode) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(OH_ArkUI_NativeModule_InvalidateAttributes(node.raw())) }
    }

    #[cfg(feature = "api-21")]
    pub(crate) fn register_common_event<T: Fn(NodeEventRef) + 'static>(
        &self,
        node: &ArkUINode,
        event_type: crate::NodeEventType,
        callback: T,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let callback = Box::into_raw(Box::new(NodeEventCallbackContext {
            callback: Box::new(callback),
        }));
        let raw_event_type: ohos_arkui_sys::ArkUI_NodeEventType = event_type.into();
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_NativeModule_RegisterCommonEvent(
                node.raw(),
                raw_event_type,
                callback.cast(),
                Some(node_event_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match common_event_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(
            (node.raw() as usize, raw_event_type as usize),
            callback as usize,
        ) {
            unsafe {
                drop(Box::from_raw(old as *mut NodeEventCallbackContext));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-21")]
    pub(crate) fn unregister_common_event(
        &self,
        node: &ArkUINode,
        event_type: crate::NodeEventType,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let raw_event_type: ohos_arkui_sys::ArkUI_NodeEventType = event_type.into();
        unsafe {
            check_arkui_status!(OH_ArkUI_NativeModule_UnregisterCommonEvent(
                node.raw(),
                raw_event_type
            ))
        }?;
        let mut callbacks = match common_event_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(node.raw() as usize, raw_event_type as usize)) {
            unsafe {
                drop(Box::from_raw(callback as *mut NodeEventCallbackContext));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-21")]
    pub(crate) fn register_common_visible_area_approximate_change_event<
        T: Fn(NodeEventRef) + 'static,
    >(
        &self,
        node: &ArkUINode,
        ratios: &mut [f32],
        expected_update_interval: f32,
        callback: T,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        let callback = Box::into_raw(Box::new(NodeEventCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(
                OH_ArkUI_NativeModule_RegisterCommonVisibleAreaApproximateChangeEvent(
                    node.raw(),
                    ratios.as_mut_ptr(),
                    ratios.len() as i32,
                    expected_update_interval,
                    callback.cast(),
                    Some(node_event_callback_trampoline)
                )
            )
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match visible_area_event_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(node.raw() as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(old as *mut NodeEventCallbackContext));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-21")]
    pub(crate) fn unregister_common_visible_area_approximate_change_event(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe {
            check_arkui_status!(
                OH_ArkUI_NativeModule_UnregisterCommonVisibleAreaApproximateChangeEvent(node.raw())
            )
        }?;
        let mut callbacks = match visible_area_event_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(node.raw() as usize)) {
            unsafe {
                drop(Box::from_raw(callback as *mut NodeEventCallbackContext));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-22")]
    pub(crate) fn adopt_child(&self, node: &ArkUINode, child: &ArkUINode) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(OH_ArkUI_NativeModule_AdoptChild(node.raw(), child.raw())) }
    }

    #[cfg(feature = "api-22")]
    pub(crate) fn remove_adopted_child(
        &self,
        node: &ArkUINode,
        child: &ArkUINode,
    ) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe {
            check_arkui_status!(OH_ArkUI_NativeModule_RemoveAdoptedChild(
                node.raw(),
                child.raw()
            ))
        }
    }

    #[cfg(feature = "api-22")]
    pub(crate) fn post_async_ui_task<T: Fn() + 'static>(
        &self,
        node: &ArkUINode,
        async_ui_task: T,
    ) -> ArkUIResult<()> {
        let context = self.context_by_node(node)?;
        let callback = Box::into_raw(Box::new(NodeAsyncTaskCallbackContext {
            async_ui_task: Box::new(async_ui_task),
            on_finish: None,
            pending: AtomicUsize::new(1),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_PostAsyncUITask(
                context,
                callback.cast(),
                Some(node_async_ui_task_trampoline),
                None
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        Ok(())
    }

    #[cfg(feature = "api-22")]
    pub(crate) fn post_async_ui_task_with_on_finish<T: Fn() + 'static, F: Fn() + 'static>(
        &self,
        node: &ArkUINode,
        async_ui_task: T,
        on_finish: F,
    ) -> ArkUIResult<()> {
        let context = self.context_by_node(node)?;
        let callback = Box::into_raw(Box::new(NodeAsyncTaskCallbackContext {
            async_ui_task: Box::new(async_ui_task),
            on_finish: Some(Box::new(on_finish)),
            pending: AtomicUsize::new(2),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_PostAsyncUITask(
                context,
                callback.cast(),
                Some(node_async_ui_task_trampoline),
                Some(node_async_ui_task_on_finish_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        Ok(())
    }

    pub(crate) fn get_navigation_id(&self, node: &ArkUINode) -> ArkUIResult<String> {
        let _ = self.raw();
        read_buffer_string(|buffer, buffer_size, write_length| unsafe {
            ohos_arkui_sys::OH_ArkUI_GetNavigationId(node.raw(), buffer, buffer_size, write_length)
        })
    }

    pub(crate) fn get_nav_destination_name(&self, node: &ArkUINode) -> ArkUIResult<String> {
        let _ = self.raw();
        read_buffer_string(|buffer, buffer_size, write_length| unsafe {
            ohos_arkui_sys::OH_ArkUI_GetNavDestinationName(
                node.raw(),
                buffer,
                buffer_size,
                write_length,
            )
        })
    }

    pub(crate) fn get_nav_stack_length(&self, node: &ArkUINode) -> ArkUIResult<i32> {
        let _ = self.raw();
        let mut length = 0;
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetNavStackLength(
                node.raw(),
                &mut length
            ))
        }?;
        Ok(length)
    }

    pub(crate) fn get_nav_destination_name_by_index(
        &self,
        node: &ArkUINode,
        index: i32,
    ) -> ArkUIResult<String> {
        let _ = self.raw();
        read_buffer_string(|buffer, buffer_size, write_length| unsafe {
            ohos_arkui_sys::OH_ArkUI_GetNavDestinationNameByIndex(
                node.raw(),
                index,
                buffer,
                buffer_size,
                write_length,
            )
        })
    }

    pub(crate) fn get_nav_destination_id(&self, node: &ArkUINode) -> ArkUIResult<String> {
        let _ = self.raw();
        read_buffer_string(|buffer, buffer_size, write_length| unsafe {
            ohos_arkui_sys::OH_ArkUI_GetNavDestinationId(
                node.raw(),
                buffer,
                buffer_size,
                write_length,
            )
        })
    }

    pub(crate) fn get_nav_destination_state(
        &self,
        node: &ArkUINode,
    ) -> ArkUIResult<NavDestinationState> {
        let _ = self.raw();
        let mut state: ohos_arkui_sys::ArkUI_NavDestinationState = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetNavDestinationState(
                node.raw(),
                &mut state
            ))
        }?;
        Ok(state.into())
    }

    pub(crate) fn get_nav_destination_index(&self, node: &ArkUINode) -> ArkUIResult<i32> {
        let _ = self.raw();
        let mut index = 0;
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetNavDestinationIndex(
                node.raw(),
                &mut index
            ))
        }?;
        Ok(index)
    }

    #[cfg(feature = "napi")]
    pub(crate) fn get_nav_destination_param(&self, node: &ArkUINode) -> napi_sys_ohos::napi_value {
        let _ = self.raw();
        unsafe { ohos_arkui_sys::OH_ArkUI_GetNavDestinationParam(node.raw()) }
    }

    pub(crate) fn get_router_page_index(&self, node: &ArkUINode) -> ArkUIResult<i32> {
        let _ = self.raw();
        let mut index = 0;
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetRouterPageIndex(
                node.raw(),
                &mut index
            ))
        }?;
        Ok(index)
    }

    pub(crate) fn get_router_page_name(&self, node: &ArkUINode) -> ArkUIResult<String> {
        let _ = self.raw();
        read_buffer_string(|buffer, buffer_size, write_length| unsafe {
            ohos_arkui_sys::OH_ArkUI_GetRouterPageName(
                node.raw(),
                buffer,
                buffer_size,
                write_length,
            )
        })
    }

    pub(crate) fn get_router_page_path(&self, node: &ArkUINode) -> ArkUIResult<String> {
        let _ = self.raw();
        read_buffer_string(|buffer, buffer_size, write_length| unsafe {
            ohos_arkui_sys::OH_ArkUI_GetRouterPagePath(
                node.raw(),
                buffer,
                buffer_size,
                write_length,
            )
        })
    }

    pub(crate) fn get_router_page_state(&self, node: &ArkUINode) -> ArkUIResult<RouterPageState> {
        let _ = self.raw();
        let mut state: ohos_arkui_sys::ArkUI_RouterPageState = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetRouterPageState(
                node.raw(),
                &mut state
            ))
        }?;
        Ok(state.into())
    }

    pub(crate) fn get_router_page_id(&self, node: &ArkUINode) -> ArkUIResult<String> {
        let _ = self.raw();
        read_buffer_string(|buffer, buffer_size, write_length| unsafe {
            ohos_arkui_sys::OH_ArkUI_GetRouterPageId(node.raw(), buffer, buffer_size, write_length)
        })
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn post_idle_callback<T: Fn(u64, u32) + 'static>(
        &self,
        node: &ArkUINode,
        callback: T,
    ) -> ArkUIResult<()> {
        let context = self.context_by_node(node)?;
        let callback = Box::into_raw(Box::new(NodeTimestampFrameCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_PostIdleCallback(
                context,
                callback.cast(),
                Some(node_idle_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        Ok(())
    }

    #[cfg(feature = "api-22")]
    pub(crate) fn swiper_finish_animation(&self, node: &ArkUINode) -> ArkUIResult<()> {
        let _ = self.raw();
        unsafe { check_arkui_status!(ohos_arkui_sys::OH_ArkUI_Swiper_FinishAnimation(node.raw())) }
    }
}

#[cfg(feature = "api-21")]
impl ArkUINode {
    /// Force ArkUI to invalidate the attribute cache for this node.
    pub fn invalidate_attributes(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NativeModule_InvalidateAttributes(self.raw())) }
    }
}

#[cfg(feature = "api-18")]
impl ArkUINode {
    /// Register a one-shot callback during the next ArkUI frame for this node's UI context.
    pub fn post_frame_callback<T: Fn(u64, u32) + 'static>(&self, callback: T) -> ArkUIResult<()> {
        let context = unsafe { OH_ArkUI_GetContextByNode(self.raw()) };
        if context.is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_GetContextByNode returned null",
            ));
        }

        let callback = Box::into_raw(Box::new(NodeTimestampFrameCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_PostFrameCallback(
                context,
                callback.cast(),
                Some(node_frame_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        Ok(())
    }
}

#[cfg(feature = "api-20")]
impl ArkUINode {
    /// Register a one-shot callback at the end of the next idle frame for this node's UI context.
    pub fn post_idle_callback<T: Fn(u64, u32) + 'static>(&self, callback: T) -> ArkUIResult<()> {
        let context = unsafe { OH_ArkUI_GetContextByNode(self.raw()) };
        if context.is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_GetContextByNode returned null",
            ));
        }

        let callback = Box::into_raw(Box::new(NodeTimestampFrameCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_PostIdleCallback(
                context,
                callback.cast(),
                Some(node_idle_callback_trampoline)
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        Ok(())
    }

    /// Add supported UI state tracking for this node.
    pub fn add_supported_ui_states<T: Fn(i32) + 'static>(
        &self,
        ui_states: i32,
        states_change_handler: T,
        exclude_inner: bool,
    ) -> ArkUIResult<()> {
        let callback = Box::into_raw(Box::new(NodeSupportedUIStatesCallbackContext {
            callback: Box::new(states_change_handler),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_AddSupportedUIStates(
                self.raw(),
                ui_states,
                Some(node_supported_ui_states_callback_trampoline),
                exclude_inner,
                callback.cast()
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match supported_ui_states_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(
            self.raw() as usize,
            NodeSupportedUIStateRegistration {
                callback: callback as usize,
                ui_states,
            },
        ) {
            unsafe {
                drop(Box::from_raw(
                    old.callback as *mut NodeSupportedUIStatesCallbackContext,
                ));
            }
        }
        Ok(())
    }

    /// Remove supported UI state tracking from this node.
    pub fn remove_supported_ui_states(&self, ui_states: i32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_RemoveSupportedUIStates(self.raw(), ui_states)) }?;
        let key = self.raw() as usize;
        let mut callbacks = match supported_ui_states_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        let should_remove = if let Some(registration) = callbacks.get_mut(&key) {
            registration.ui_states &= !ui_states;
            registration.ui_states == 0
        } else {
            false
        };
        if should_remove {
            if let Some(registration) = callbacks.remove(&key) {
                unsafe {
                    drop(Box::from_raw(
                        registration.callback as *mut NodeSupportedUIStatesCallbackContext,
                    ));
                }
            }
        }
        Ok(())
    }
}

fn read_buffer_string<F>(mut reader: F) -> ArkUIResult<String>
where
    F: FnMut(*mut std::os::raw::c_char, i32, *mut i32) -> u32,
{
    let mut write_length = 0;
    let mut buffer = vec![0u8; 256];
    let mut status = reader(
        buffer.as_mut_ptr().cast(),
        buffer.len() as i32,
        &mut write_length,
    );
    if write_length > buffer.len() as i32 {
        buffer.resize(write_length as usize, 0);
        status = reader(
            buffer.as_mut_ptr().cast(),
            buffer.len() as i32,
            &mut write_length,
        );
    }
    check_arkui_status!(status)?;
    let mut end = (write_length as usize).min(buffer.len());
    if end == 0 {
        end = buffer.iter().position(|v| *v == 0).unwrap_or(0);
    } else if buffer.get(end.saturating_sub(1)).copied() == Some(0) {
        end -= 1;
    }
    Ok(String::from_utf8_lossy(&buffer[..end]).into_owned())
}
