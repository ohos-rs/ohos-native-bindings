use std::cell::{Cell, RefCell};
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use ohos_input_method_sys::{
    InputMethod_ErrorCode_IME_ERR_DETACHED, InputMethod_ErrorCode_IME_ERR_NULL_POINTER,
    InputMethod_InputMethodProxy, OH_InputMethodController_Attach, OH_InputMethodController_Detach,
    OH_InputMethodProxy_HideKeyboard, OH_InputMethodProxy_NotifyConfigurationChange,
    OH_InputMethodProxy_NotifyCursorUpdate, OH_InputMethodProxy_NotifySelectionChange,
    OH_InputMethodProxy_ShowKeyboard,
};

use crate::proxy::{register_callbacks, unregister_callbacks, IMECallbacks, SharedCallbacks};
use crate::{
    Action, AttachOptions, Cursor, Direction, EnterKey, ImeError, ImeResult, InputType,
    KeyboardStatus, Rect, Selection, TextEditor, TextState,
};

#[derive(Clone)]
pub struct IME {
    inner: Rc<IMEInner>,
}

struct IMEInner {
    raw: RefCell<Option<NonNull<InputMethod_InputMethodProxy>>>,
    option: AttachOptions,
    text_editor: RefCell<Option<TextEditor>>,
    callbacks: SharedCallbacks,
    needs_attach: Cell<bool>,
    configuration: Arc<RwLock<EditorConfiguration>>,
    text_state: Arc<RwLock<TextState>>,
    text_notified: Cell<bool>,
    #[cfg(feature = "api-22")]
    callbacks_in_main_thread: bool,
}

#[derive(Clone, Copy)]
struct EditorConfiguration {
    window_id: i32,
    enter_key: EnterKey,
    input_type: InputType,
    cursor: Option<Rect>,
    preview_supported: bool,
    #[cfg(feature = "api-26")]
    consume_key_events: bool,
}

impl PartialEq for IME {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for IME {}

impl std::hash::Hash for IME {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(Rc::as_ptr(&self.inner), state);
    }
}

impl IME {
    pub fn new(option: AttachOptions) -> Self {
        Self::with_callback_thread(option, false, 0)
    }

    /// Creates an editor for the actual native Window ID. Zero selects the
    /// main window. Rendering surface IDs are not native Window IDs.
    pub fn new_for_window(option: AttachOptions, window_id: i32) -> Self {
        Self::with_callback_thread(option, false, window_id)
    }

    /// Sets the native Window ID used by the next attachment.
    pub fn set_window_id(&self, window_id: i32) {
        self.inner
            .configuration
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .window_id = window_id;
    }

    /// Sets API-26 hardware-key consumption for the next attachment.
    ///
    /// Requires an API-26 runtime. Native configuration errors are logged
    /// because the SDK's configuration callback has no error return channel.
    #[cfg(feature = "api-26")]
    pub fn set_consume_key_events(&self, enabled: bool) {
        self.inner
            .configuration
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .consume_key_events = enabled;
    }

    /// Create an IME whose TextEditor callbacks are dispatched on the main
    /// thread instead of the platform-default IPC thread.
    #[cfg(feature = "api-22")]
    pub fn new_with_main_thread_callbacks(option: AttachOptions) -> Self {
        Self::with_callback_thread(option, true, 0)
    }

    fn with_callback_thread(
        option: AttachOptions,
        callbacks_in_main_thread: bool,
        window_id: i32,
    ) -> Self {
        #[cfg(not(feature = "api-22"))]
        let _ = callbacks_in_main_thread;
        let configuration = Arc::new(RwLock::new(EditorConfiguration {
            window_id,
            enter_key: EnterKey::Unspecified,
            input_type: InputType::Text,
            cursor: None,
            preview_supported: false,
            #[cfg(feature = "api-26")]
            consume_key_events: false,
        }));
        let callback_configuration = configuration.clone();
        let text_state = Arc::new(RwLock::new(TextState::default()));
        let left_state = text_state.clone();
        let right_state = text_state.clone();
        let cursor_state = text_state.clone();
        let config_state = text_state.clone();
        let callbacks = IMECallbacks {
            get_left_text_of_cursor: Some(Arc::new(move |number| {
                left_state
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .before_cursor(number)
            })),
            get_right_text_of_cursor: Some(Arc::new(move |number| {
                right_state
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .after_cursor(number)
            })),
            get_text_index_at_cursor: Some(Arc::new(move || {
                cursor_state
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .cursor()
                    .min(i32::MAX as usize) as i32
            })),
            get_text_config: Some(Arc::new(move |config| {
                let configuration = *callback_configuration
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                config.set_window_id(configuration.window_id);
                #[cfg(feature = "api-26")]
                if let Err(error) = config.set_consume_key_events(configuration.consume_key_events)
                {
                    log::error!("Cannot configure native editor key consumption: {error}");
                }
                config.set_enter_key(configuration.enter_key);
                config.set_input_type(configuration.input_type);
                config.set_preview_text_supported(configuration.preview_supported);
                let state = config_state
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                config.set_selection(Selection {
                    start: state.selection.start.min(i32::MAX as usize) as i32,
                    end: state.selection.end.min(i32::MAX as usize) as i32,
                });
                if let Some(rect) = configuration.cursor {
                    config.cursor().set_rect(rect);
                }
            })),
            ..Default::default()
        };
        Self {
            inner: Rc::new(IMEInner {
                raw: RefCell::new(None),
                option,
                text_editor: RefCell::new(None),
                callbacks: Arc::new(RwLock::new(callbacks)),
                needs_attach: Cell::new(true),
                configuration,
                text_state,
                text_notified: Cell::new(false),
                #[cfg(feature = "api-22")]
                callbacks_in_main_thread,
            }),
        }
    }

    pub fn insert_text<T>(&self, callback: T)
    where
        T: Fn(String) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.insert_text = Some(Arc::new(callback)));
    }

    pub fn pre_edit<T>(&self, callback: T)
    where
        T: Fn(String, i32, i32) + Send + Sync + 'static,
    {
        self.inner
            .configuration
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .preview_supported = true;
        self.update_callbacks(|callbacks| callbacks.set_preview_text = Some(Arc::new(callback)));
    }

    pub fn on_status_change<T>(&self, callback: T)
    where
        T: Fn(KeyboardStatus) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| {
            callbacks.send_keyboard_status = Some(Arc::new(callback));
        });
    }

    pub fn on_delete<T>(&self, callback: T)
    where
        T: Fn(i32) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.delete_backward = Some(Arc::new(callback)));
    }

    pub fn on_backspace<T>(&self, callback: T)
    where
        T: Fn(i32) + Send + Sync + 'static,
    {
        self.on_delete(callback);
    }

    /// Receive deletion toward the end of the document, in UTF-16 units.
    pub fn on_delete_forward<T>(&self, callback: T)
    where
        T: Fn(i32) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.delete_forward = Some(Arc::new(callback)));
    }

    /// Receive cursor movement requested by the system input method.
    pub fn on_move_cursor<T>(&self, callback: T)
    where
        T: Fn(Direction) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.move_cursor = Some(Arc::new(callback)));
    }

    /// Receive the system's requested selection in UTF-16 units.
    pub fn on_set_selection<T>(&self, callback: T)
    where
        T: Fn(Selection) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| {
            callbacks.handle_set_selection = Some(Arc::new(callback))
        });
    }

    /// Receive system Select All, Cut, Copy and Paste commands.
    pub fn on_extend_action<T>(&self, callback: T)
    where
        T: Fn(Action) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| {
            callbacks.handle_extend_action = Some(Arc::new(callback))
        });
    }

    pub fn on_enter<T>(&self, callback: T)
    where
        T: Fn(EnterKey) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.send_enter_key = Some(Arc::new(callback)));
    }

    pub fn on_preview<T>(&self, callback: T)
    where
        T: Fn(String, i32, i32) + Send + Sync + 'static,
    {
        self.pre_edit(callback);
    }

    pub fn on_finish_preview<T>(&self, callback: T)
    where
        T: Fn() + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| {
            callbacks.finish_text_preview = Some(Arc::new(callback));
        });
    }

    /// Retain the editor attributes for the next attach and update this session
    /// immediately when it is active. An inactive editor never changes the
    /// attributes of another editor which replaced it.
    pub fn try_update_configuration(
        &self,
        enter_key: EnterKey,
        input_type: InputType,
    ) -> ImeResult<()> {
        {
            let mut configuration = self
                .inner
                .configuration
                .write()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            configuration.enter_key = enter_key;
            configuration.input_type = input_type;
        }
        let Some(raw) = self.active_proxy() else {
            return Ok(());
        };
        let code = unsafe {
            OH_InputMethodProxy_NotifyConfigurationChange(
                raw.as_ptr(),
                enter_key.into(),
                input_type.into(),
            )
        };
        self.notification_result("update-configuration", code)
    }

    /// Retain the cursor rectangle in absolute physical screen coordinates.
    /// The current session receives the update if this editor is active.
    pub fn try_update_cursor(&self, rect: Rect) -> ImeResult<()> {
        self.inner
            .configuration
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .cursor = Some(rect);
        let Some(raw) = self.active_proxy() else {
            return Ok(());
        };
        let cursor = Cursor::new(rect);
        let code = unsafe { OH_InputMethodProxy_NotifyCursorUpdate(raw.as_ptr(), cursor.raw) };
        self.notification_result("update-cursor", code)
    }

    /// Cache text for IPC-thread queries and notify the active native editor.
    /// The SDK accepts at most 8K UTF-16 units of whole input text. Larger or
    /// offset snapshots answer surrounding-text queries but are not passed as
    /// misleading whole-document selection notifications.
    pub fn try_update_text_state(&self, state: TextState) -> ImeResult<()> {
        let changed = {
            let mut previous = self
                .inner
                .text_state
                .write()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if *previous == state {
                false
            } else {
                *previous = state;
                self.inner.text_notified.set(false);
                true
            }
        };
        if !changed && self.inner.text_notified.get() {
            return Ok(());
        }
        let Some(raw) = self.active_proxy() else {
            return Ok(());
        };
        let state = self
            .inner
            .text_state
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let mut text: Vec<u16> = state.text.encode_utf16().collect();
        if state.offset != 0
            || text.len() > 8192
            || state.selection.end > text.len()
            || state.selection.start > state.selection.end
        {
            return Ok(());
        }
        let code = unsafe {
            OH_InputMethodProxy_NotifySelectionChange(
                raw.as_ptr(),
                text.as_mut_ptr(),
                text.len(),
                state.selection.start as i32,
                state.selection.end as i32,
            )
        };
        self.notification_result("update-selection", code)?;
        self.inner.text_notified.set(true);
        Ok(())
    }

    fn active_proxy(&self) -> Option<NonNull<InputMethod_InputMethodProxy>> {
        (!self.inner.needs_attach.get())
            .then(|| *self.inner.raw.borrow())
            .flatten()
    }

    fn notification_result(&self, operation: &'static str, code: u32) -> ImeResult<()> {
        if code == 0 {
            return Ok(());
        }
        let error = ImeError::new(operation, code);
        if error.is_stale_session() {
            self.invalidate_session();
        }
        Err(error)
    }

    /// Attaches this editor synchronously using the native controller.
    ///
    /// The host serializes window ownership and native operations. This binding
    /// neither queues work nor tracks which window owns the process-wide IME.
    pub fn try_attach(&self) -> ImeResult<()> {
        if self.active_proxy().is_some() {
            return Ok(());
        }

        // Keep the same callback table across stale-session recovery. Neither
        // NOT_EDITABLE nor DETACHED proves that OnInputStop has finished with it.
        if self.inner.text_editor.borrow().is_none() {
            let editor = TextEditor::new();
            #[cfg(feature = "api-22")]
            if self.inner.callbacks_in_main_thread {
                editor.set_callback_in_main_thread(true)?;
            }
            register_callbacks(editor.raw, self.inner.callbacks.clone());
            self.inner.text_editor.replace(Some(editor));
        }
        let editor = self.inner.text_editor.borrow().as_ref().unwrap().raw;
        self.inner.raw.borrow_mut().take();
        self.invalidate_session();
        let mut raw: *mut InputMethod_InputMethodProxy = ptr::null_mut();
        let code =
            unsafe { OH_InputMethodController_Attach(editor, self.inner.option.raw, &mut raw) };
        if code != 0 {
            // The NDK can retain the callback table even when attach fails.
            return Err(ImeError::new("attach", code));
        }
        let Some(raw) = NonNull::new(raw) else {
            return Err(ImeError::new(
                "attach",
                InputMethod_ErrorCode_IME_ERR_NULL_POINTER,
            ));
        };

        self.inner.raw.replace(Some(raw));
        self.inner.needs_attach.set(false);
        Ok(())
    }

    pub fn attach(&self) {
        let _ = self.try_attach();
    }

    /// Show the keyboard, recovering once if HarmonyOS invalidated the proxy
    /// because another editor was attached or the Ability left foreground.
    pub fn try_show_keyboard(&self) -> ImeResult<()> {
        self.try_attach()?;
        match self.show_attached() {
            Ok(()) => Ok(()),
            Err(error) if error.is_stale_session() => {
                self.invalidate_session();
                self.try_attach()?;
                let result = self.show_attached();
                if result.as_ref().is_err_and(|error| error.is_stale_session()) {
                    self.invalidate_session();
                }
                result
            }
            Err(error) => Err(error),
        }
    }

    pub fn show_keyboard(&self) {
        let _ = self.try_show_keyboard();
    }

    /// Hide the keyboard without ending this editor session.
    pub fn try_hide_keyboard(&self) -> ImeResult<()> {
        let Some(raw) = self.active_proxy() else {
            // Inactive/stale editors must not call through an invalid proxy.
            self.invalidate_session();
            return Ok(());
        };
        let code = unsafe { OH_InputMethodProxy_HideKeyboard(raw.as_ptr()) };
        if code == 0 {
            Ok(())
        } else {
            let error = ImeError::new("hide-keyboard", code);
            if error.is_stale_session() {
                self.invalidate_session();
            }
            Err(error)
        }
    }

    pub fn hide_keyboard(&self) {
        let _ = self.try_hide_keyboard();
    }

    /// Detaches this editor synchronously. On failure its callback table is
    /// retained so the host can retry. Call before transferring IME ownership.
    pub fn try_detach(&self) -> ImeResult<()> {
        self.inner.detach()
    }

    pub fn detach(&self) {
        let _ = self.try_detach();
    }

    /// Shows the keyboard through the attached proxy, without attaching or
    /// recovering a stale connection. The host owns lifecycle recovery.
    pub fn show_attached(&self) -> ImeResult<()> {
        let Some(raw) = self.active_proxy() else {
            return Err(ImeError::new(
                "show-keyboard",
                InputMethod_ErrorCode_IME_ERR_DETACHED,
            ));
        };
        let code = unsafe { OH_InputMethodProxy_ShowKeyboard(raw.as_ptr()) };
        if code == 0 {
            Ok(())
        } else {
            Err(ImeError::new("show-keyboard", code))
        }
    }

    fn update_callbacks(&self, update: impl FnOnce(&mut IMECallbacks)) {
        let mut callbacks = self
            .inner
            .callbacks
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        update(&mut callbacks);
    }

    fn invalidate_session(&self) {
        self.inner.needs_attach.set(true);
        self.inner.text_notified.set(false);
    }
}

impl IMEInner {
    fn detach(&self) -> ImeResult<()> {
        self.needs_attach.set(true);
        self.text_notified.set(false);
        let Some(raw) = *self.raw.borrow() else {
            return Ok(());
        };
        // SAFETY: this proxy was returned by the native Attach operation.
        let code = unsafe { OH_InputMethodController_Detach(raw.as_ptr()) };
        if code != 0 {
            return Err(ImeError::new("detach", code));
        }
        self.raw.borrow_mut().take();
        if let Some(editor) = self.text_editor.borrow_mut().take() {
            unregister_callbacks(editor.raw);
            drop(editor);
        }
        Ok(())
    }
}

impl Drop for IMEInner {
    fn drop(&mut self) {
        if let Err(error) = self.detach() {
            log::error!("Cannot detach dropped IME: {error}");
        }
        if let Some(editor) = self.text_editor.get_mut().take() {
            // A failed attach/detach is not a native lifetime boundary. Retire
            // callbacks but keep the table alive rather than risk a native UAF.
            unregister_callbacks(editor.raw);
            log::error!("Retaining native editor after unsuccessful IME cleanup");
            std::mem::forget(editor);
        }
    }
}
