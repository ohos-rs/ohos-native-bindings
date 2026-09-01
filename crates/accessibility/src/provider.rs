#[cfg(feature = "api-15")]
use std::collections::HashMap;
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    os::raw::c_char,
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::NonNull,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, OnceLock, RwLock,
    },
};

use ohos_accessibility_sys::*;
#[cfg(feature = "api-23")]
use ohos_arkui_input_sys::ArkUI_NodeHandle;

use crate::{
    error::{check, AccessibilityError, Result},
    ActionType, ElementInfo, ElementInfoList, EventInfo, FocusMoveDirection, FocusType, SearchMode,
};

pub type EventCompletionCallback = unsafe extern "C" fn(error_code: i32);

/// A provider owned by an ArkUI custom node or XComponent.
///
/// The lifetime is tied to the native owner from which the provider was
/// obtained. The provider itself does not release the native object.
#[derive(Clone, Copy)]
pub struct Provider<'a> {
    raw: NonNull<ArkUI_AccessibilityProvider>,
    _owner: PhantomData<&'a ()>,
}

impl<'a> Provider<'a> {
    /// Wrap a provider borrowed from ArkUI.
    ///
    /// # Safety
    ///
    /// `raw` must be a valid provider and remain alive for `'a`.
    pub unsafe fn from_raw(raw: *mut ArkUI_AccessibilityProvider) -> Result<Self> {
        let raw =
            NonNull::new(raw).ok_or(AccessibilityError::NullHandle("accessibility provider"))?;
        Ok(Self {
            raw,
            _owner: PhantomData,
        })
    }

    /// Obtain the provider associated with an `ARKUI_NODE_CUSTOM` handle.
    ///
    /// # Safety
    ///
    /// `node` must point to a live ArkUI node handle for all of `'a`.
    #[cfg(feature = "api-23")]
    pub unsafe fn from_node_handle(node: *mut ArkUI_NodeHandle) -> Result<Self> {
        if node.is_null() {
            return Err(AccessibilityError::NullHandle("ArkUI node"));
        }
        let mut provider = std::ptr::null_mut();
        check(unsafe {
            OH_ArkUI_NativeModule_GetNativeAccessibilityProvider(node, &mut provider)
        })?;
        unsafe { Self::from_raw(provider) }
    }

    pub fn as_raw(self) -> *mut ArkUI_AccessibilityProvider {
        self.raw.as_ptr()
    }

    /// Register the official single-instance callback table.
    ///
    /// ArkUI has no unregister function. Dropping the returned registration
    /// removes the Rust handler, so late native callbacks fail safely.
    pub fn register_callbacks<C>(self, callbacks: C) -> Result<ProviderRegistration<'a>>
    where
        C: ProviderCallbacks,
    {
        let token = next_token();
        let entry = CallbackEntry {
            token,
            callbacks: Arc::new(callbacks),
        };
        {
            let mut slot = single_registry()
                .write()
                .map_err(|_| AccessibilityError::LockPoisoned)?;
            if slot.is_some() {
                return Err(AccessibilityError::AlreadyRegistered);
            }
            *slot = Some(entry);
        }

        let result = unsafe {
            OH_ArkUI_AccessibilityProviderRegisterCallback(self.as_raw(), single_callback_table())
        };
        if let Err(error) = check(result) {
            remove_single(token);
            return Err(error);
        }

        Ok(ProviderRegistration {
            provider: self,
            key: RegistrationKey::Single,
            token,
            instance_id: None,
        })
    }

    /// Register the official multi-instance callback table.
    #[cfg(feature = "api-15")]
    pub fn register_callbacks_with_instance<C>(
        self,
        instance_id: &str,
        callbacks: C,
    ) -> Result<ProviderRegistration<'a>>
    where
        C: ProviderCallbacks,
    {
        let instance_id = CString::new(instance_id)?;
        let key = instance_id.as_bytes().to_vec();
        let token = next_token();
        let entry = CallbackEntry {
            token,
            callbacks: Arc::new(callbacks),
        };
        {
            let mut registry = instance_registry()
                .write()
                .map_err(|_| AccessibilityError::LockPoisoned)?;
            if registry.contains_key(&key) {
                return Err(AccessibilityError::AlreadyRegistered);
            }
            registry.insert(key.clone(), entry);
        }

        // ArkUI exposes no unregister operation and does not document whether
        // it copies this pointer. Keep one process-lifetime allocation so a
        // late native callback can still read the instance ID safely after the
        // Rust registration has been dropped.
        let instance_id: &'static CStr = Box::leak(Box::new(instance_id)).as_c_str();
        let result = unsafe {
            OH_ArkUI_AccessibilityProviderRegisterCallbackWithInstance(
                instance_id.as_ptr(),
                self.as_raw(),
                instance_callback_table(),
            )
        };
        if let Err(error) = check(result) {
            remove_instance(&key, token);
            return Err(error);
        }

        Ok(ProviderRegistration {
            provider: self,
            key: RegistrationKey::Instance(key),
            token,
            instance_id: Some(instance_id),
        })
    }

    /// Proactively report an event to ArkUI.
    ///
    /// The official API copies the event during this call; the event wrapper
    /// may therefore be dropped after this method returns.
    pub fn send_event(self, event: &EventInfo, callback: Option<EventCompletionCallback>) {
        unsafe { OH_ArkUI_SendAccessibilityAsyncEvent(self.as_raw(), event.as_raw(), callback) }
    }
}

pub struct ProviderRegistration<'a> {
    provider: Provider<'a>,
    key: RegistrationKey,
    token: u64,
    instance_id: Option<&'static CStr>,
}

impl ProviderRegistration<'_> {
    pub fn provider(&self) -> Provider<'_> {
        self.provider
    }

    pub fn instance_id(&self) -> Option<&CStr> {
        self.instance_id
    }
}

impl Drop for ProviderRegistration<'_> {
    fn drop(&mut self) {
        match &self.key {
            RegistrationKey::Single => remove_single(self.token),
            #[cfg(feature = "api-15")]
            RegistrationKey::Instance(key) => remove_instance(key, self.token),
        }
    }
}

enum RegistrationKey {
    Single,
    #[cfg(feature = "api-15")]
    Instance(Vec<u8>),
}

/// Borrowed action arguments supplied by ArkUI.
pub struct ActionArguments<'a> {
    raw: Option<NonNull<ArkUI_AccessibilityActionArguments>>,
    _marker: PhantomData<&'a ArkUI_AccessibilityActionArguments>,
}

impl ActionArguments<'_> {
    unsafe fn from_raw(raw: *mut ArkUI_AccessibilityActionArguments) -> Self {
        Self {
            raw: NonNull::new(raw),
            _marker: PhantomData,
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<&CStr>> {
        let Some(raw) = self.raw else {
            return Ok(None);
        };
        let key = CString::new(key)?;
        let mut value = std::ptr::null_mut();
        let result = unsafe {
            OH_ArkUI_FindAccessibilityActionArgumentByKey(raw.as_ptr(), key.as_ptr(), &mut value)
        };
        if result == ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_FAILED {
            return Ok(None);
        }
        check(result)?;
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(unsafe { CStr::from_ptr(value) }))
        }
    }
}

/// Callback surface recommended by the ArkUI native accessibility API.
///
/// Methods default to `Unsupported`, allowing adapters to opt into only the
/// capabilities they can implement.
pub trait ProviderCallbacks: Send + Sync + 'static {
    fn find_node_infos_by_id(
        &self,
        _element_id: i64,
        _mode: SearchMode,
        _request_id: i32,
        _elements: &mut ElementInfoList<'_>,
    ) -> Result<()> {
        Err(AccessibilityError::Unsupported)
    }

    fn find_node_infos_by_text(
        &self,
        _element_id: i64,
        _text: &CStr,
        _request_id: i32,
        _elements: &mut ElementInfoList<'_>,
    ) -> Result<()> {
        Err(AccessibilityError::Unsupported)
    }

    fn find_focused_node(
        &self,
        _element_id: i64,
        _focus_type: FocusType,
        _request_id: i32,
        _element: &mut ElementInfo<'_>,
    ) -> Result<()> {
        Err(AccessibilityError::Unsupported)
    }

    fn find_next_focus_node(
        &self,
        _element_id: i64,
        _direction: FocusMoveDirection,
        _request_id: i32,
        _element: &mut ElementInfo<'_>,
    ) -> Result<()> {
        Err(AccessibilityError::Unsupported)
    }

    fn execute_action(
        &self,
        _element_id: i64,
        _action: ActionType,
        _arguments: &ActionArguments<'_>,
        _request_id: i32,
    ) -> Result<()> {
        Err(AccessibilityError::Unsupported)
    }

    fn clear_focused_node(&self) -> Result<()> {
        Err(AccessibilityError::Unsupported)
    }

    fn cursor_position(&self, _element_id: i64, _request_id: i32) -> Result<i32> {
        Err(AccessibilityError::Unsupported)
    }
}

#[derive(Clone)]
struct CallbackEntry {
    token: u64,
    callbacks: Arc<dyn ProviderCallbacks>,
}

static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);
static SINGLE_REGISTRY: OnceLock<RwLock<Option<CallbackEntry>>> = OnceLock::new();
#[cfg(feature = "api-15")]
static INSTANCE_REGISTRY: OnceLock<RwLock<HashMap<Vec<u8>, CallbackEntry>>> = OnceLock::new();
static SINGLE_CALLBACK_TABLE: OnceLock<usize> = OnceLock::new();
#[cfg(feature = "api-15")]
static INSTANCE_CALLBACK_TABLE: OnceLock<usize> = OnceLock::new();

fn next_token() -> u64 {
    NEXT_TOKEN.fetch_add(1, Ordering::Relaxed)
}

fn single_registry() -> &'static RwLock<Option<CallbackEntry>> {
    SINGLE_REGISTRY.get_or_init(|| RwLock::new(None))
}

#[cfg(feature = "api-15")]
fn instance_registry() -> &'static RwLock<HashMap<Vec<u8>, CallbackEntry>> {
    INSTANCE_REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

fn remove_single(token: u64) {
    if let Ok(mut slot) = single_registry().write() {
        if slot.as_ref().is_some_and(|entry| entry.token == token) {
            *slot = None;
        }
    }
}

#[cfg(feature = "api-15")]
fn remove_instance(key: &[u8], token: u64) {
    if let Ok(mut registry) = instance_registry().write() {
        if registry.get(key).is_some_and(|entry| entry.token == token) {
            registry.remove(key);
        }
    }
}

fn single_handler() -> Result<Arc<dyn ProviderCallbacks>> {
    single_registry()
        .read()
        .map_err(|_| AccessibilityError::LockPoisoned)?
        .as_ref()
        .map(|entry| entry.callbacks.clone())
        .ok_or(AccessibilityError::Failed)
}

#[cfg(feature = "api-15")]
unsafe fn instance_handler(instance_id: *const c_char) -> Result<Arc<dyn ProviderCallbacks>> {
    if instance_id.is_null() {
        return Err(AccessibilityError::BadParameter);
    }
    let key = unsafe { CStr::from_ptr(instance_id) }.to_bytes();
    instance_registry()
        .read()
        .map_err(|_| AccessibilityError::LockPoisoned)?
        .get(key)
        .map(|entry| entry.callbacks.clone())
        .ok_or(AccessibilityError::Failed)
}

fn invoke(callback: impl FnOnce() -> Result<()>) -> i32 {
    match catch_unwind(AssertUnwindSafe(callback)) {
        Ok(Ok(())) => ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_SUCCESSFUL,
        Ok(Err(error)) => error.into_code(),
        Err(_) => AccessibilityError::CallbackPanicked.into_code(),
    }
}

fn single_callback_table() -> *mut ArkUI_AccessibilityProviderCallbacks {
    *SINGLE_CALLBACK_TABLE.get_or_init(|| {
        Box::into_raw(Box::new(ArkUI_AccessibilityProviderCallbacks {
            findAccessibilityNodeInfosById: Some(single_find_by_id),
            findAccessibilityNodeInfosByText: Some(single_find_by_text),
            findFocusedAccessibilityNode: Some(single_find_focused),
            findNextFocusAccessibilityNode: Some(single_find_next_focus),
            executeAccessibilityAction: Some(single_execute_action),
            clearFocusedFocusAccessibilityNode: Some(single_clear_focus),
            getAccessibilityNodeCursorPosition: Some(single_cursor_position),
        })) as usize
    }) as _
}

#[cfg(feature = "api-15")]
fn instance_callback_table() -> *mut ArkUI_AccessibilityProviderCallbacksWithInstance {
    *INSTANCE_CALLBACK_TABLE.get_or_init(|| {
        Box::into_raw(Box::new(ArkUI_AccessibilityProviderCallbacksWithInstance {
            findAccessibilityNodeInfosById: Some(instance_find_by_id),
            findAccessibilityNodeInfosByText: Some(instance_find_by_text),
            findFocusedAccessibilityNode: Some(instance_find_focused),
            findNextFocusAccessibilityNode: Some(instance_find_next_focus),
            executeAccessibilityAction: Some(instance_execute_action),
            clearFocusedFocusAccessibilityNode: Some(instance_clear_focus),
            getAccessibilityNodeCursorPosition: Some(instance_cursor_position),
        })) as usize
    }) as _
}

unsafe extern "C" fn single_find_by_id(
    element_id: i64,
    mode: ArkUI_AccessibilitySearchMode,
    request_id: i32,
    elements: *mut ArkUI_AccessibilityElementInfoList,
) -> i32 {
    invoke(|| {
        let handler = single_handler()?;
        let mut elements = unsafe { ElementInfoList::from_raw(elements)? };
        handler.find_node_infos_by_id(
            element_id,
            SearchMode::from_raw(mode),
            request_id,
            &mut elements,
        )
    })
}

unsafe extern "C" fn single_find_by_text(
    element_id: i64,
    text: *const c_char,
    request_id: i32,
    elements: *mut ArkUI_AccessibilityElementInfoList,
) -> i32 {
    invoke(|| {
        if text.is_null() {
            return Err(AccessibilityError::BadParameter);
        }
        let handler = single_handler()?;
        let text = unsafe { CStr::from_ptr(text) };
        let mut elements = unsafe { ElementInfoList::from_raw(elements)? };
        handler.find_node_infos_by_text(element_id, text, request_id, &mut elements)
    })
}

unsafe extern "C" fn single_find_focused(
    element_id: i64,
    focus_type: ArkUI_AccessibilityFocusType,
    request_id: i32,
    element: *mut ArkUI_AccessibilityElementInfo,
) -> i32 {
    invoke(|| {
        let handler = single_handler()?;
        let focus_type =
            FocusType::try_from_raw(focus_type).ok_or(AccessibilityError::BadParameter)?;
        let mut element = unsafe { ElementInfo::from_borrowed(element)? };
        handler.find_focused_node(element_id, focus_type, request_id, &mut element)
    })
}

unsafe extern "C" fn single_find_next_focus(
    element_id: i64,
    direction: ArkUI_AccessibilityFocusMoveDirection,
    request_id: i32,
    element: *mut ArkUI_AccessibilityElementInfo,
) -> i32 {
    invoke(|| {
        let handler = single_handler()?;
        let direction =
            FocusMoveDirection::try_from_raw(direction).ok_or(AccessibilityError::BadParameter)?;
        let mut element = unsafe { ElementInfo::from_borrowed(element)? };
        handler.find_next_focus_node(element_id, direction, request_id, &mut element)
    })
}

unsafe extern "C" fn single_execute_action(
    element_id: i64,
    action: ArkUI_Accessibility_ActionType,
    arguments: *mut ArkUI_AccessibilityActionArguments,
    request_id: i32,
) -> i32 {
    invoke(|| {
        let handler = single_handler()?;
        let action = ActionType::try_from_raw(action).ok_or(AccessibilityError::BadParameter)?;
        let arguments = unsafe { ActionArguments::from_raw(arguments) };
        handler.execute_action(element_id, action, &arguments, request_id)
    })
}

unsafe extern "C" fn single_clear_focus() -> i32 {
    invoke(|| single_handler()?.clear_focused_node())
}

unsafe extern "C" fn single_cursor_position(
    element_id: i64,
    request_id: i32,
    index: *mut i32,
) -> i32 {
    invoke(|| {
        if index.is_null() {
            return Err(AccessibilityError::BadParameter);
        }
        let value = single_handler()?.cursor_position(element_id, request_id)?;
        unsafe { *index = value };
        Ok(())
    })
}

#[cfg(feature = "api-15")]
unsafe extern "C" fn instance_find_by_id(
    instance_id: *const c_char,
    element_id: i64,
    mode: ArkUI_AccessibilitySearchMode,
    request_id: i32,
    elements: *mut ArkUI_AccessibilityElementInfoList,
) -> i32 {
    invoke(|| {
        let handler = unsafe { instance_handler(instance_id)? };
        let mut elements = unsafe { ElementInfoList::from_raw(elements)? };
        handler.find_node_infos_by_id(
            element_id,
            SearchMode::from_raw(mode),
            request_id,
            &mut elements,
        )
    })
}

#[cfg(feature = "api-15")]
unsafe extern "C" fn instance_find_by_text(
    instance_id: *const c_char,
    element_id: i64,
    text: *const c_char,
    request_id: i32,
    elements: *mut ArkUI_AccessibilityElementInfoList,
) -> i32 {
    invoke(|| {
        if text.is_null() {
            return Err(AccessibilityError::BadParameter);
        }
        let handler = unsafe { instance_handler(instance_id)? };
        let text = unsafe { CStr::from_ptr(text) };
        let mut elements = unsafe { ElementInfoList::from_raw(elements)? };
        handler.find_node_infos_by_text(element_id, text, request_id, &mut elements)
    })
}

#[cfg(feature = "api-15")]
unsafe extern "C" fn instance_find_focused(
    instance_id: *const c_char,
    element_id: i64,
    focus_type: ArkUI_AccessibilityFocusType,
    request_id: i32,
    element: *mut ArkUI_AccessibilityElementInfo,
) -> i32 {
    invoke(|| {
        let handler = unsafe { instance_handler(instance_id)? };
        let focus_type =
            FocusType::try_from_raw(focus_type).ok_or(AccessibilityError::BadParameter)?;
        let mut element = unsafe { ElementInfo::from_borrowed(element)? };
        handler.find_focused_node(element_id, focus_type, request_id, &mut element)
    })
}

#[cfg(feature = "api-15")]
unsafe extern "C" fn instance_find_next_focus(
    instance_id: *const c_char,
    element_id: i64,
    direction: ArkUI_AccessibilityFocusMoveDirection,
    request_id: i32,
    element: *mut ArkUI_AccessibilityElementInfo,
) -> i32 {
    invoke(|| {
        let handler = unsafe { instance_handler(instance_id)? };
        let direction =
            FocusMoveDirection::try_from_raw(direction).ok_or(AccessibilityError::BadParameter)?;
        let mut element = unsafe { ElementInfo::from_borrowed(element)? };
        handler.find_next_focus_node(element_id, direction, request_id, &mut element)
    })
}

#[cfg(feature = "api-15")]
unsafe extern "C" fn instance_execute_action(
    instance_id: *const c_char,
    element_id: i64,
    action: ArkUI_Accessibility_ActionType,
    arguments: *mut ArkUI_AccessibilityActionArguments,
    request_id: i32,
) -> i32 {
    invoke(|| {
        let handler = unsafe { instance_handler(instance_id)? };
        let action = ActionType::try_from_raw(action).ok_or(AccessibilityError::BadParameter)?;
        let arguments = unsafe { ActionArguments::from_raw(arguments) };
        handler.execute_action(element_id, action, &arguments, request_id)
    })
}

#[cfg(feature = "api-15")]
unsafe extern "C" fn instance_clear_focus(instance_id: *const c_char) -> i32 {
    invoke(|| unsafe { instance_handler(instance_id)? }.clear_focused_node())
}

#[cfg(feature = "api-15")]
unsafe extern "C" fn instance_cursor_position(
    instance_id: *const c_char,
    element_id: i64,
    request_id: i32,
    index: *mut i32,
) -> i32 {
    invoke(|| {
        if index.is_null() {
            return Err(AccessibilityError::BadParameter);
        }
        let value =
            unsafe { instance_handler(instance_id)? }.cursor_position(element_id, request_id)?;
        unsafe { *index = value };
        Ok(())
    })
}
