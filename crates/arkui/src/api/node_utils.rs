#[cfg(any(feature = "api-13", feature = "api-14", feature = "api-15"))]
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr::NonNull;
#[cfg(feature = "api-22")]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use ohos_arkui_input_binding::sys::ArkUI_NodeHandle;
use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_IntOffset, ArkUI_IntSize, OH_ArkUI_GetContextByNode, OH_ArkUI_List_CloseAllSwipeActions,
    OH_ArkUI_NodeUtils_GetLayoutPosition, OH_ArkUI_NodeUtils_GetLayoutPositionInScreen,
    OH_ArkUI_NodeUtils_GetLayoutPositionInWindow, OH_ArkUI_NodeUtils_GetLayoutSize,
    OH_ArkUI_NodeUtils_GetPositionWithTranslateInScreen,
    OH_ArkUI_NodeUtils_GetPositionWithTranslateInWindow,
    OH_ArkUI_RegisterSystemColorModeChangeEvent, OH_ArkUI_RegisterSystemFontStyleChangeEvent,
    OH_ArkUI_SystemFontStyleEvent_GetFontSizeScale,
    OH_ArkUI_SystemFontStyleEvent_GetFontWeightScale,
    OH_ArkUI_UnregisterSystemColorModeChangeEvent, OH_ArkUI_UnregisterSystemFontStyleChangeEvent,
};

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
    OH_PixelmapNative,
};
#[cfg(feature = "api-21")]
use ohos_arkui_sys::{
    ArkUI_NodeEvent, OH_ArkUI_NativeModule_InvalidateAttributes,
    OH_ArkUI_NativeModule_RegisterCommonEvent,
    OH_ArkUI_NativeModule_RegisterCommonVisibleAreaApproximateChangeEvent,
    OH_ArkUI_NativeModule_UnregisterCommonEvent,
    OH_ArkUI_NativeModule_UnregisterCommonVisibleAreaApproximateChangeEvent,
    OH_ArkUI_NodeEvent_GetUserData,
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

use ohos_arkui_sys::{ArkUI_ContextHandle, ArkUI_SystemColorMode, ArkUI_SystemFontStyleEvent};

use crate::{check_arkui_status, ArkUIError, ArkUIResult, NavDestinationState, RouterPageState};

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

#[cfg(feature = "api-21")]
#[derive(Clone, Copy, Debug)]
pub struct NodeEventRef {
    raw: NonNull<ArkUI_NodeEvent>,
}

#[cfg(feature = "api-21")]
impl NodeEventRef {
    fn from_raw(raw: *mut ArkUI_NodeEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn raw(&self) -> *mut ArkUI_NodeEvent {
        self.raw.as_ptr()
    }

    pub fn user_data(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { OH_ArkUI_NodeEvent_GetUserData(self.raw()) })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SystemFontStyleEventRef {
    raw: NonNull<ArkUI_SystemFontStyleEvent>,
}

impl SystemFontStyleEventRef {
    pub(crate) fn from_raw(raw: *mut ArkUI_SystemFontStyleEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn from_const_raw(raw: *const ArkUI_SystemFontStyleEvent) -> Option<Self> {
        Self::from_raw(raw.cast_mut())
    }

    pub fn raw(&self) -> *mut ArkUI_SystemFontStyleEvent {
        self.raw.as_ptr()
    }

    pub fn font_size_scale(&self) -> f32 {
        unsafe { OH_ArkUI_SystemFontStyleEvent_GetFontSizeScale(self.raw()) }
    }

    pub fn font_weight_scale(&self) -> f32 {
        unsafe { OH_ArkUI_SystemFontStyleEvent_GetFontWeightScale(self.raw()) }
    }
}

fn non_null_or_error<T>(ptr: *mut T, func: &'static str) -> ArkUIResult<NonNull<T>> {
    NonNull::new(ptr).ok_or_else(|| {
        ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            format!("{func} returned null"),
        )
    })
}

#[cfg(any(feature = "api-14", feature = "api-15"))]
fn c_str_to_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(
            unsafe { std::ffi::CStr::from_ptr(ptr) }
                .to_string_lossy()
                .into_owned(),
        )
    }
}

#[cfg(feature = "api-14")]
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
        c_str_to_string(value)
    }
}

#[cfg(feature = "api-14")]
impl Drop for CustomProperty {
    fn drop(&mut self) {
        unsafe { OH_ArkUI_CustomProperty_Destroy(self.raw()) }
    }
}

#[cfg(feature = "api-14")]
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

    pub fn node_by_index(&self, index: i32) -> Option<ArkUI_NodeHandle> {
        let node = unsafe { OH_ArkUI_ActiveChildrenInfo_GetNodeByIndex(self.raw(), index) };
        NonNull::new(node).map(NonNull::as_ptr)
    }
}

#[cfg(feature = "api-14")]
impl Drop for ActiveChildrenInfo {
    fn drop(&mut self) {
        unsafe { OH_ArkUI_ActiveChildrenInfo_Destroy(self.raw()) }
    }
}

#[cfg(feature = "api-15")]
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
        c_str_to_string(name)
    }
}

#[cfg(feature = "api-15")]
impl Drop for HostWindowInfo {
    fn drop(&mut self) {
        unsafe { OH_ArkUI_HostWindowInfo_Destroy(self.raw()) }
    }
}

#[cfg(feature = "api-15")]
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
pub struct SnapshotOptions {
    raw: NonNull<ArkUI_SnapshotOptions>,
}

#[cfg(feature = "api-15")]
impl SnapshotOptions {
    pub fn new() -> ArkUIResult<Self> {
        let options = unsafe { ohos_arkui_sys::OH_ArkUI_CreateSnapshotOptions() };
        non_null_or_error(options, "OH_ArkUI_CreateSnapshotOptions").map(|raw| Self { raw })
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

pub(crate) fn get_layout_size(node: ArkUI_NodeHandle) -> ArkUIResult<ArkUI_IntSize> {
    let mut size: ArkUI_IntSize = unsafe { std::mem::zeroed() };
    unsafe { check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutSize(node, &mut size)) }?;
    Ok(size)
}

pub(crate) fn get_layout_position(node: ArkUI_NodeHandle) -> ArkUIResult<ArkUI_IntOffset> {
    let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
    unsafe { check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutPosition(node, &mut offset)) }?;
    Ok(offset)
}

pub(crate) fn get_layout_position_in_window(
    node: ArkUI_NodeHandle,
) -> ArkUIResult<ArkUI_IntOffset> {
    let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutPositionInWindow(
            node,
            &mut offset
        ))
    }?;
    Ok(offset)
}

pub(crate) fn get_layout_position_in_screen(
    node: ArkUI_NodeHandle,
) -> ArkUIResult<ArkUI_IntOffset> {
    let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutPositionInScreen(
            node,
            &mut offset
        ))
    }?;
    Ok(offset)
}

#[cfg(feature = "api-20")]
pub(crate) fn get_layout_position_in_global_display(
    node: ArkUI_NodeHandle,
) -> ArkUIResult<ArkUI_IntOffset> {
    let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutPositionInGlobalDisplay(
            node,
            &mut offset
        ))
    }?;
    Ok(offset)
}

pub(crate) fn get_position_with_translate_in_window(
    node: ArkUI_NodeHandle,
) -> ArkUIResult<ArkUI_IntOffset> {
    let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_GetPositionWithTranslateInWindow(
            node,
            &mut offset
        ))
    }?;
    Ok(offset)
}

pub(crate) fn get_position_with_translate_in_screen(
    node: ArkUI_NodeHandle,
) -> ArkUIResult<ArkUI_IntOffset> {
    let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_GetPositionWithTranslateInScreen(
            node,
            &mut offset
        ))
    }?;
    Ok(offset)
}

#[cfg(feature = "api-13")]
pub(crate) fn add_custom_property(
    node: ArkUI_NodeHandle,
    name: *const c_char,
    value: *const c_char,
) {
    unsafe { OH_ArkUI_NodeUtils_AddCustomProperty(node, name, value) }
}

#[cfg(feature = "api-13")]
pub(crate) fn remove_custom_property(node: ArkUI_NodeHandle, name: *const c_char) {
    unsafe { OH_ArkUI_NodeUtils_RemoveCustomProperty(node, name) }
}

#[cfg(feature = "api-14")]
pub(crate) fn get_custom_property(
    node: ArkUI_NodeHandle,
    name: *const c_char,
) -> ArkUIResult<Option<CustomProperty>> {
    CustomProperty::get(node, name)
}

#[cfg(feature = "api-14")]
pub(crate) fn custom_property_get_string_value(handle: &CustomProperty) -> Option<String> {
    handle.string_value()
}

#[cfg(feature = "api-14")]
pub(crate) fn get_parent_in_page_tree(node: ArkUI_NodeHandle) -> Option<ArkUI_NodeHandle> {
    let parent = unsafe { OH_ArkUI_NodeUtils_GetParentInPageTree(node) };
    if parent.is_null() {
        None
    } else {
        Some(parent)
    }
}

#[cfg(feature = "api-14")]
pub(crate) fn get_active_children_info(
    node: ArkUI_NodeHandle,
) -> ArkUIResult<Option<ActiveChildrenInfo>> {
    ActiveChildrenInfo::get(node)
}

#[cfg(feature = "api-14")]
pub(crate) fn active_children_info_count(handle: &ActiveChildrenInfo) -> i32 {
    handle.count()
}

#[cfg(feature = "api-14")]
pub(crate) fn active_children_info_node_by_index(
    handle: &ActiveChildrenInfo,
    index: i32,
) -> Option<ArkUI_NodeHandle> {
    handle.node_by_index(index)
}

#[cfg(feature = "api-14")]
pub(crate) fn get_current_page_root_node(node: ArkUI_NodeHandle) -> Option<ArkUI_NodeHandle> {
    let root = unsafe { OH_ArkUI_NodeUtils_GetCurrentPageRootNode(node) };
    if root.is_null() {
        None
    } else {
        Some(root)
    }
}

#[cfg(feature = "api-14")]
pub(crate) fn is_created_by_ndk(node: ArkUI_NodeHandle) -> bool {
    unsafe { OH_ArkUI_NodeUtils_IsCreatedByNDK(node) }
}

#[cfg(feature = "api-14")]
pub(crate) fn get_node_type(node: ArkUI_NodeHandle) -> Option<crate::ArkUINodeType> {
    let raw = unsafe { OH_ArkUI_NodeUtils_GetNodeType(node) };
    if raw < 0 {
        None
    } else {
        crate::ArkUINodeType::try_from_raw(raw as ohos_arkui_sys::ArkUI_NodeType)
    }
}

#[cfg(feature = "api-15")]
pub(crate) fn get_window_info(node: ArkUI_NodeHandle) -> ArkUIResult<Option<HostWindowInfo>> {
    HostWindowInfo::get(node)
}

#[cfg(feature = "api-15")]
pub(crate) fn host_window_info_get_name(info: &HostWindowInfo) -> Option<String> {
    info.name()
}

#[cfg(feature = "api-15")]
pub(crate) fn get_first_child_index_without_expand(node: ArkUI_NodeHandle) -> ArkUIResult<u32> {
    let mut index = 0u32;
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_GetFirstChildIndexWithoutExpand(
            node, &mut index
        ))
    }?;
    Ok(index)
}

#[cfg(feature = "api-15")]
pub(crate) fn get_last_child_index_without_expand(node: ArkUI_NodeHandle) -> ArkUIResult<u32> {
    let mut index = 0u32;
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_GetLastChildIndexWithoutExpand(
            node, &mut index
        ))
    }?;
    Ok(index)
}

#[cfg(feature = "api-15")]
pub(crate) fn get_child_with_expand_mode(
    node: ArkUI_NodeHandle,
    position: i32,
    expand_mode: u32,
) -> ArkUIResult<Option<ArkUI_NodeHandle>> {
    let mut subnode = std::ptr::null_mut();
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_GetChildWithExpandMode(
            node,
            position,
            &mut subnode,
            expand_mode
        ))
    }?;
    if subnode.is_null() {
        Ok(None)
    } else {
        Ok(Some(subnode))
    }
}

#[cfg(feature = "api-18")]
pub(crate) fn move_to(
    node: ArkUI_NodeHandle,
    target_parent: ArkUI_NodeHandle,
    index: i32,
) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_NodeUtils_MoveTo(node, target_parent, index)) }
}

#[cfg(feature = "api-15")]
pub(crate) fn set_cross_language_option(
    node: ArkUI_NodeHandle,
    option: &CrossLanguageOption,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_SetCrossLanguageOption(
            node,
            option.raw()
        ))
    }
}

#[cfg(feature = "api-15")]
pub(crate) fn cross_language_option_set_attribute_setting_status(
    option: &mut CrossLanguageOption,
    enabled: bool,
) {
    option.set_attribute_setting_status(enabled)
}

#[cfg(feature = "api-15")]
pub(crate) fn cross_language_option_get_attribute_setting_status(
    option: &CrossLanguageOption,
) -> bool {
    option.attribute_setting_status()
}

#[cfg(feature = "api-15")]
pub(crate) fn get_cross_language_option(
    node: ArkUI_NodeHandle,
    option: &mut CrossLanguageOption,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_GetCrossLanguageOption(
            node,
            option.raw()
        ))
    }
}

#[cfg(feature = "api-15")]
pub(crate) fn register_layout_callback_on_node_handle<T: Fn() + 'static>(
    node: ArkUI_NodeHandle,
    on_layout_completed: T,
) -> ArkUIResult<()> {
    let _ = unregister_layout_callback_on_node_handle(node);
    let callback = Box::into_raw(Box::new(NodePersistentVoidCallbackContext {
        callback: Box::new(on_layout_completed),
    }));
    let result = unsafe {
        check_arkui_status!(OH_ArkUI_RegisterLayoutCallbackOnNodeHandle(
            node,
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
    if let Some(old) = callbacks.insert(node as usize, callback as usize) {
        unsafe {
            drop(Box::from_raw(old as *mut NodePersistentVoidCallbackContext));
        }
    }
    Ok(())
}

#[cfg(feature = "api-15")]
pub(crate) fn register_draw_callback_on_node_handle<T: Fn() + 'static>(
    node: ArkUI_NodeHandle,
    on_draw_completed: T,
) -> ArkUIResult<()> {
    let _ = unregister_draw_callback_on_node_handle(node);
    let callback = Box::into_raw(Box::new(NodePersistentVoidCallbackContext {
        callback: Box::new(on_draw_completed),
    }));
    let result = unsafe {
        check_arkui_status!(OH_ArkUI_RegisterDrawCallbackOnNodeHandle(
            node,
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
    if let Some(old) = callbacks.insert(node as usize, callback as usize) {
        unsafe {
            drop(Box::from_raw(old as *mut NodePersistentVoidCallbackContext));
        }
    }
    Ok(())
}

#[cfg(feature = "api-15")]
pub(crate) fn unregister_layout_callback_on_node_handle(node: ArkUI_NodeHandle) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_UnregisterLayoutCallbackOnNodeHandle(node)) }?;
    let mut callbacks = match layout_callback_contexts().lock() {
        Ok(callbacks) => callbacks,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(callback) = callbacks.remove(&(node as usize)) {
        unsafe {
            drop(Box::from_raw(
                callback as *mut NodePersistentVoidCallbackContext,
            ));
        }
    }
    Ok(())
}

#[cfg(feature = "api-15")]
pub(crate) fn unregister_draw_callback_on_node_handle(node: ArkUI_NodeHandle) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_UnregisterDrawCallbackOnNodeHandle(node)) }?;
    let mut callbacks = match draw_callback_contexts().lock() {
        Ok(callbacks) => callbacks,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(callback) = callbacks.remove(&(node as usize)) {
        unsafe {
            drop(Box::from_raw(
                callback as *mut NodePersistentVoidCallbackContext,
            ));
        }
    }
    Ok(())
}

#[cfg(feature = "api-15")]
pub(crate) fn get_attached_node_handle_by_id(
    id: *const c_char,
) -> ArkUIResult<Option<ArkUI_NodeHandle>> {
    let mut node = std::ptr::null_mut();
    unsafe { check_arkui_status!(OH_ArkUI_NodeUtils_GetAttachedNodeHandleById(id, &mut node)) }?;
    if node.is_null() {
        Ok(None)
    } else {
        Ok(Some(node))
    }
}

#[cfg(feature = "api-15")]
pub(crate) fn get_node_snapshot(
    node: ArkUI_NodeHandle,
    snapshot_options: Option<&SnapshotOptions>,
) -> ArkUIResult<*mut OH_PixelmapNative> {
    let mut pixelmap = std::ptr::null_mut();
    let snapshot_options = snapshot_options
        .map(SnapshotOptions::raw)
        .unwrap_or(std::ptr::null_mut());
    unsafe {
        check_arkui_status!(OH_ArkUI_GetNodeSnapshot(
            node,
            snapshot_options,
            &mut pixelmap
        ))
    }?;
    Ok(pixelmap)
}

#[cfg(feature = "api-15")]
pub(crate) fn get_position_to_parent(node: ArkUI_NodeHandle) -> ArkUIResult<ArkUI_IntOffset> {
    let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
    unsafe { check_arkui_status!(OH_ArkUI_NodeUtils_GetPositionToParent(node, &mut offset)) }?;
    Ok(offset)
}

#[cfg(feature = "api-20")]
pub(crate) fn add_supported_ui_states<T: Fn(i32) + 'static>(
    node: ArkUI_NodeHandle,
    ui_states: i32,
    states_change_handler: T,
    exclude_inner: bool,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeSupportedUIStatesCallbackContext {
        callback: Box::new(states_change_handler),
    }));
    let result = unsafe {
        check_arkui_status!(OH_ArkUI_AddSupportedUIStates(
            node,
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
        node as usize,
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
    node: ArkUI_NodeHandle,
    ui_states: i32,
) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_RemoveSupportedUIStates(node, ui_states)) }?;
    let key = node as usize;
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
    ui_context: ArkUI_ContextHandle,
    callback: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeOneShotVoidCallbackContext {
        callback: Box::new(callback),
    }));
    let result = unsafe {
        check_arkui_status!(OH_ArkUI_RunTaskInScope(
            ui_context,
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
    unique_id: u32,
) -> ArkUIResult<Option<ArkUI_NodeHandle>> {
    let mut node = std::ptr::null_mut();
    unsafe {
        check_arkui_status!(OH_ArkUI_NodeUtils_GetNodeHandleByUniqueId(
            unique_id, &mut node
        ))
    }?;
    if node.is_null() {
        Ok(None)
    } else {
        Ok(Some(node))
    }
}

#[cfg(feature = "api-20")]
pub(crate) fn get_node_unique_id(node: ArkUI_NodeHandle) -> ArkUIResult<i32> {
    let mut unique_id = -1;
    unsafe { check_arkui_status!(OH_ArkUI_NodeUtils_GetNodeUniqueId(node, &mut unique_id)) }?;
    Ok(unique_id)
}

#[cfg(feature = "api-20")]
fn set_force_dark_config_raw(
    ui_context: ArkUI_ContextHandle,
    force_dark: bool,
    node_type: crate::ArkUINodeType,
    color_invert_func: Option<ForceDarkInvertCallback>,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_SetForceDarkConfig(
            ui_context,
            force_dark,
            node_type.into(),
            color_invert_func
        ))
    }
}

#[cfg(feature = "api-20")]
pub(crate) fn clear_force_dark_config(
    ui_context: ArkUI_ContextHandle,
    force_dark: bool,
    node_type: crate::ArkUINodeType,
) -> ArkUIResult<()> {
    set_force_dark_config_raw(ui_context, force_dark, node_type, None)?;
    remove_force_dark_callback_registration(ui_context, node_type);
    Ok(())
}

#[cfg(feature = "api-20")]
pub(crate) fn set_force_dark_config_with_color_invert<T: Fn(u32) -> u32 + 'static>(
    ui_context: ArkUI_ContextHandle,
    force_dark: bool,
    node_type: crate::ArkUINodeType,
    color_invert: T,
) -> ArkUIResult<()> {
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
    let result = set_force_dark_config_raw(
        ui_context,
        force_dark,
        node_type,
        FORCE_DARK_CALLBACK_TRAMPOLINES[slot],
    );
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
        let key = (ui_context as usize, node_type as usize);
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
pub(crate) fn invalidate_attributes(node: ArkUI_NodeHandle) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_NativeModule_InvalidateAttributes(node)) }
}

#[cfg(feature = "api-21")]
pub(crate) fn register_common_event<T: Fn(NodeEventRef) + 'static>(
    node: ArkUI_NodeHandle,
    event_type: crate::NodeEventType,
    callback: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeEventCallbackContext {
        callback: Box::new(callback),
    }));
    let raw_event_type: ohos_arkui_sys::ArkUI_NodeEventType = event_type.into();
    let result = unsafe {
        check_arkui_status!(OH_ArkUI_NativeModule_RegisterCommonEvent(
            node,
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
    if let Some(old) = callbacks.insert((node as usize, raw_event_type as usize), callback as usize)
    {
        unsafe {
            drop(Box::from_raw(old as *mut NodeEventCallbackContext));
        }
    }
    Ok(())
}

#[cfg(feature = "api-21")]
pub(crate) fn unregister_common_event(
    node: ArkUI_NodeHandle,
    event_type: crate::NodeEventType,
) -> ArkUIResult<()> {
    let raw_event_type: ohos_arkui_sys::ArkUI_NodeEventType = event_type.into();
    unsafe {
        check_arkui_status!(OH_ArkUI_NativeModule_UnregisterCommonEvent(
            node,
            raw_event_type
        ))
    }?;
    let mut callbacks = match common_event_callback_contexts().lock() {
        Ok(callbacks) => callbacks,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(callback) = callbacks.remove(&(node as usize, raw_event_type as usize)) {
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
    node: ArkUI_NodeHandle,
    ratios: &mut [f32],
    expected_update_interval: f32,
    callback: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeEventCallbackContext {
        callback: Box::new(callback),
    }));
    let result = unsafe {
        check_arkui_status!(
            OH_ArkUI_NativeModule_RegisterCommonVisibleAreaApproximateChangeEvent(
                node,
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
    if let Some(old) = callbacks.insert(node as usize, callback as usize) {
        unsafe {
            drop(Box::from_raw(old as *mut NodeEventCallbackContext));
        }
    }
    Ok(())
}

#[cfg(feature = "api-21")]
pub(crate) fn unregister_common_visible_area_approximate_change_event(
    node: ArkUI_NodeHandle,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(
            OH_ArkUI_NativeModule_UnregisterCommonVisibleAreaApproximateChangeEvent(node)
        )
    }?;
    let mut callbacks = match visible_area_event_callback_contexts().lock() {
        Ok(callbacks) => callbacks,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(callback) = callbacks.remove(&(node as usize)) {
        unsafe {
            drop(Box::from_raw(callback as *mut NodeEventCallbackContext));
        }
    }
    Ok(())
}

#[cfg(feature = "api-22")]
pub(crate) fn adopt_child(node: ArkUI_NodeHandle, child: ArkUI_NodeHandle) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_NativeModule_AdoptChild(node, child)) }
}

#[cfg(feature = "api-22")]
pub(crate) fn remove_adopted_child(
    node: ArkUI_NodeHandle,
    child: ArkUI_NodeHandle,
) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_NativeModule_RemoveAdoptedChild(node, child)) }
}

#[cfg(feature = "api-22")]
pub(crate) fn post_async_ui_task<T: Fn() + 'static>(
    context: ArkUI_ContextHandle,
    async_ui_task: T,
) -> ArkUIResult<()> {
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
    context: ArkUI_ContextHandle,
    async_ui_task: T,
    on_finish: F,
) -> ArkUIResult<()> {
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

#[cfg(feature = "api-22")]
fn post_ui_task_raw(context: ArkUI_ContextHandle, task_data: *mut c_void) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_PostUITask(
            context,
            task_data,
            Some(node_one_shot_void_callback_trampoline)
        ))
    }
}

#[cfg(feature = "api-22")]
fn post_ui_task_and_wait_raw(
    context: ArkUI_ContextHandle,
    task_data: *mut c_void,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_PostUITaskAndWait(
            context,
            task_data,
            Some(node_one_shot_void_callback_trampoline)
        ))
    }
}

#[cfg(feature = "api-22")]
pub(crate) fn post_ui_task<T: Fn() + 'static>(
    context: ArkUI_ContextHandle,
    task: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeOneShotVoidCallbackContext {
        callback: Box::new(task),
    }));
    let result = post_ui_task_raw(context, callback.cast());
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
    context: ArkUI_ContextHandle,
    task: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeOneShotVoidCallbackContext {
        callback: Box::new(task),
    }));
    let result = post_ui_task_and_wait_raw(context, callback.cast());
    if let Err(err) = result {
        unsafe {
            drop(Box::from_raw(callback));
        }
        return Err(err);
    }
    Ok(())
}

fn list_close_all_swipe_actions_raw(
    node: ArkUI_NodeHandle,
    user_data: *mut c_void,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_List_CloseAllSwipeActions(
            node,
            user_data,
            Some(node_one_shot_void_callback_trampoline)
        ))
    }
}

pub(crate) fn get_context_by_node(node: ArkUI_NodeHandle) -> ArkUI_ContextHandle {
    unsafe { OH_ArkUI_GetContextByNode(node) }
}

pub(crate) fn list_close_all_swipe_actions<T: Fn() + 'static>(
    node: ArkUI_NodeHandle,
    on_finish: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeOneShotVoidCallbackContext {
        callback: Box::new(on_finish),
    }));
    let result = list_close_all_swipe_actions_raw(node, callback.cast());
    if let Err(err) = result {
        unsafe {
            drop(Box::from_raw(callback));
        }
        return Err(err);
    }
    Ok(())
}

pub(crate) fn register_system_color_mode_change_event<T: Fn(ArkUI_SystemColorMode) + 'static>(
    node: ArkUI_NodeHandle,
    on_color_mode_change: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeSystemColorModeCallbackContext {
        callback: Box::new(on_color_mode_change),
    }));
    let result = unsafe {
        check_arkui_status!(OH_ArkUI_RegisterSystemColorModeChangeEvent(
            node,
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
    if let Some(old) = callbacks.insert(node as usize, callback as usize) {
        unsafe {
            drop(Box::from_raw(
                old as *mut NodeSystemColorModeCallbackContext,
            ));
        }
    }
    Ok(())
}

pub(crate) fn unregister_system_color_mode_change_event(node: ArkUI_NodeHandle) {
    unsafe { OH_ArkUI_UnregisterSystemColorModeChangeEvent(node) }
    let mut callbacks = match system_color_mode_callback_contexts().lock() {
        Ok(callbacks) => callbacks,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(callback) = callbacks.remove(&(node as usize)) {
        unsafe {
            drop(Box::from_raw(
                callback as *mut NodeSystemColorModeCallbackContext,
            ));
        }
    }
}

pub(crate) fn register_system_font_style_change_event<T: Fn(SystemFontStyleEventRef) + 'static>(
    node: ArkUI_NodeHandle,
    on_font_style_change: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeSystemFontStyleCallbackContext {
        callback: Box::new(on_font_style_change),
    }));
    let result = unsafe {
        check_arkui_status!(OH_ArkUI_RegisterSystemFontStyleChangeEvent(
            node,
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
    if let Some(old) = callbacks.insert(node as usize, callback as usize) {
        unsafe {
            drop(Box::from_raw(
                old as *mut NodeSystemFontStyleCallbackContext,
            ));
        }
    }
    Ok(())
}

pub(crate) fn unregister_system_font_style_change_event(node: ArkUI_NodeHandle) {
    unsafe { OH_ArkUI_UnregisterSystemFontStyleChangeEvent(node) }
    let mut callbacks = match system_font_style_callback_contexts().lock() {
        Ok(callbacks) => callbacks,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(callback) = callbacks.remove(&(node as usize)) {
        unsafe {
            drop(Box::from_raw(
                callback as *mut NodeSystemFontStyleCallbackContext,
            ));
        }
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

fn ptr_or_error<T>(ptr: *mut T, func: &'static str) -> ArkUIResult<*mut T> {
    if ptr.is_null() {
        Err(ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            format!("{func} returned null"),
        ))
    } else {
        Ok(ptr)
    }
}

#[cfg(feature = "napi")]
pub(crate) fn get_drawable_descriptor_from_napi_value(
    env: napi_sys_ohos::napi_env,
    value: napi_sys_ohos::napi_value,
) -> ArkUIResult<*mut ohos_arkui_sys::ArkUI_DrawableDescriptor> {
    let mut descriptor = std::ptr::null_mut();
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetDrawableDescriptorFromNapiValue(
            env,
            value,
            &mut descriptor
        ))
    }?;
    ptr_or_error(descriptor, "OH_ArkUI_GetDrawableDescriptorFromNapiValue")
}

#[cfg(feature = "napi")]
pub(crate) fn get_drawable_descriptor_from_resource_napi_value(
    env: napi_sys_ohos::napi_env,
    value: napi_sys_ohos::napi_value,
) -> ArkUIResult<*mut ohos_arkui_sys::ArkUI_DrawableDescriptor> {
    let mut descriptor = std::ptr::null_mut();
    unsafe {
        check_arkui_status!(
            ohos_arkui_sys::OH_ArkUI_GetDrawableDescriptorFromResourceNapiValue(
                env,
                value,
                &mut descriptor
            )
        )
    }?;
    ptr_or_error(
        descriptor,
        "OH_ArkUI_GetDrawableDescriptorFromResourceNapiValue",
    )
}

pub(crate) fn get_navigation_id(node: ArkUI_NodeHandle) -> ArkUIResult<String> {
    read_buffer_string(|buffer, buffer_size, write_length| unsafe {
        ohos_arkui_sys::OH_ArkUI_GetNavigationId(node, buffer, buffer_size, write_length)
    })
}

pub(crate) fn get_nav_destination_name(node: ArkUI_NodeHandle) -> ArkUIResult<String> {
    read_buffer_string(|buffer, buffer_size, write_length| unsafe {
        ohos_arkui_sys::OH_ArkUI_GetNavDestinationName(node, buffer, buffer_size, write_length)
    })
}

pub(crate) fn get_nav_stack_length(node: ArkUI_NodeHandle) -> ArkUIResult<i32> {
    let mut length = 0;
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetNavStackLength(
            node,
            &mut length
        ))
    }?;
    Ok(length)
}

pub(crate) fn get_nav_destination_name_by_index(
    node: ArkUI_NodeHandle,
    index: i32,
) -> ArkUIResult<String> {
    read_buffer_string(|buffer, buffer_size, write_length| unsafe {
        ohos_arkui_sys::OH_ArkUI_GetNavDestinationNameByIndex(
            node,
            index,
            buffer,
            buffer_size,
            write_length,
        )
    })
}

pub(crate) fn get_nav_destination_id(node: ArkUI_NodeHandle) -> ArkUIResult<String> {
    read_buffer_string(|buffer, buffer_size, write_length| unsafe {
        ohos_arkui_sys::OH_ArkUI_GetNavDestinationId(node, buffer, buffer_size, write_length)
    })
}

pub(crate) fn get_nav_destination_state(
    node: ArkUI_NodeHandle,
) -> ArkUIResult<NavDestinationState> {
    let mut state: ohos_arkui_sys::ArkUI_NavDestinationState = unsafe { std::mem::zeroed() };
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetNavDestinationState(
            node, &mut state
        ))
    }?;
    Ok(state.into())
}

pub(crate) fn get_nav_destination_index(node: ArkUI_NodeHandle) -> ArkUIResult<i32> {
    let mut index = 0;
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetNavDestinationIndex(
            node, &mut index
        ))
    }?;
    Ok(index)
}

#[cfg(feature = "napi")]
pub(crate) fn get_nav_destination_param(node: ArkUI_NodeHandle) -> napi_sys_ohos::napi_value {
    unsafe { ohos_arkui_sys::OH_ArkUI_GetNavDestinationParam(node) }
}

pub(crate) fn get_router_page_index(node: ArkUI_NodeHandle) -> ArkUIResult<i32> {
    let mut index = 0;
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetRouterPageIndex(
            node, &mut index
        ))
    }?;
    Ok(index)
}

pub(crate) fn get_router_page_name(node: ArkUI_NodeHandle) -> ArkUIResult<String> {
    read_buffer_string(|buffer, buffer_size, write_length| unsafe {
        ohos_arkui_sys::OH_ArkUI_GetRouterPageName(node, buffer, buffer_size, write_length)
    })
}

pub(crate) fn get_router_page_path(node: ArkUI_NodeHandle) -> ArkUIResult<String> {
    read_buffer_string(|buffer, buffer_size, write_length| unsafe {
        ohos_arkui_sys::OH_ArkUI_GetRouterPagePath(node, buffer, buffer_size, write_length)
    })
}

pub(crate) fn get_router_page_state(node: ArkUI_NodeHandle) -> ArkUIResult<RouterPageState> {
    let mut state: ohos_arkui_sys::ArkUI_RouterPageState = unsafe { std::mem::zeroed() };
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetRouterPageState(
            node, &mut state
        ))
    }?;
    Ok(state.into())
}

pub(crate) fn get_router_page_id(node: ArkUI_NodeHandle) -> ArkUIResult<String> {
    read_buffer_string(|buffer, buffer_size, write_length| unsafe {
        ohos_arkui_sys::OH_ArkUI_GetRouterPageId(node, buffer, buffer_size, write_length)
    })
}

#[cfg(feature = "api-18")]
pub(crate) fn post_frame_callback<T: Fn(u64, u32) + 'static>(
    ui_context: ArkUI_ContextHandle,
    callback: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeTimestampFrameCallbackContext {
        callback: Box::new(callback),
    }));
    let result = unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_PostFrameCallback(
            ui_context,
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

#[cfg(feature = "api-20")]
#[cfg(feature = "napi")]
pub(crate) fn init_module_for_arkts_env(env: napi_sys_ohos::napi_env) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(ohos_arkui_sys::OH_ArkUI_InitModuleForArkTSEnv(env)) }
}

#[cfg(feature = "api-20")]
#[cfg(feature = "napi")]
pub(crate) fn notify_arkts_env_destroy(env: napi_sys_ohos::napi_env) {
    unsafe { ohos_arkui_sys::OH_ArkUI_NotifyArkTSEnvDestroy(env) }
}

#[cfg(feature = "api-20")]
pub(crate) fn post_idle_callback<T: Fn(u64, u32) + 'static>(
    ui_context: ArkUI_ContextHandle,
    callback: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(NodeTimestampFrameCallbackContext {
        callback: Box::new(callback),
    }));
    let result = unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_PostIdleCallback(
            ui_context,
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
pub(crate) fn swiper_finish_animation(node: ArkUI_NodeHandle) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(ohos_arkui_sys::OH_ArkUI_Swiper_FinishAnimation(node)) }
}
