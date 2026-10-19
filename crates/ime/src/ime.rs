use std::marker::PhantomData;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, RwLock, Weak};

use ohos_input_method_sys::{
    InputMethod_ErrorCode_IME_ERR_DETACHED, InputMethod_ErrorCode_IME_ERR_NULL_POINTER,
    InputMethod_InputMethodProxy, OH_InputMethodController_Attach, OH_InputMethodController_Detach,
    OH_InputMethodProxy_HideKeyboard, OH_InputMethodProxy_NotifyConfigurationChange,
    OH_InputMethodProxy_NotifyCursorUpdate, OH_InputMethodProxy_NotifySelectionChange,
    OH_InputMethodProxy_ShowKeyboard,
};

use crate::proxy::{register_callbacks, unregister_callbacks, IMECallbacks, SharedCallbacks};
use crate::session::NativeSessions;
use crate::{
    Action, AttachOptions, Cursor, Direction, EnterKey, ImeError, ImeResult, InputType,
    KeyboardStatus, Rect, Selection, TextConfig, TextEditor, TextState,
};

static NEXT_SESSION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone)]
pub struct IME {
    inner: Arc<IMEInner>,
    main_thread: PhantomData<Rc<()>>,
}

struct IMEInner {
    id: u64,
    native: Mutex<NativeEditor>,
    attachment_serial: Mutex<()>,
    pending_attachment: Mutex<Option<Weak<AttachmentState>>>,
    show_on_attach: bool,
    callbacks: SharedCallbacks,
    needs_attach: AtomicBool,
    configuration: Arc<RwLock<EditorConfiguration>>,
    text_state: Arc<RwLock<TextState>>,
    text_notified: AtomicBool,
    #[cfg(feature = "api-22")]
    callbacks_in_main_thread: bool,
}

struct NativeEditor {
    raw: Option<NonNull<InputMethod_InputMethodProxy>>,
    editor: Option<TextEditor>,
    generation: u64,
}

// SAFETY: these opaque native handles are only accessed under their Mutex and
// the process-wide NativeSessions guard. Their Rust callbacks use Send + Sync
// closures and synchronized snapshots, without borrowing ArkUI or N-API state.
unsafe impl Send for NativeEditor {}

#[derive(Clone, Copy, PartialEq)]
enum AttachmentStatus {
    Pending,
    Cancelled,
    Committed,
}

struct AttachmentState {
    status: Mutex<AttachmentStatus>,
    changed: Condvar,
}

impl AttachmentState {
    fn transition(&self, status: AttachmentStatus) -> bool {
        let mut current = self
            .status
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if *current != AttachmentStatus::Pending {
            return false;
        }
        *current = status;
        self.changed.notify_one();
        true
    }

    fn cancelled(&self) -> bool {
        *self
            .status
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            == AttachmentStatus::Cancelled
    }
}

/// An IPC-thread-callback editor's connection work, runnable on a worker.
/// The worker stays responsible for cleanup until the UI accepts or cancels it.
pub struct ImeAttachment {
    inner: Arc<IMEInner>,
    state: Arc<AttachmentState>,
}

/// Dropping an uncommitted request signals cancellation without native IPC.
pub struct ImeAttachmentCancellation {
    state: Arc<AttachmentState>,
}

impl ImeAttachment {
    /// Report the result before waiting for the UI's commit or cancellation.
    /// `completed` must deliver the result without waiting for this worker to exit.
    pub fn run(self, show_keyboard: bool, completed: impl FnOnce(ImeResult<bool>)) {
        let _serial = self
            .inner
            .attachment_serial
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.state.cancelled() {
            completed(Ok(false));
            return;
        }
        let ime = IME {
            inner: self.inner.clone(),
            main_thread: PhantomData,
        };
        let mut result = ime.try_attach();
        if result.is_ok() && show_keyboard && !self.state.cancelled() {
            result = ime.show_attached();
        }
        if self.state.cancelled() {
            completed(self.cleanup().map(|()| false));
            return;
        }
        if let Err(error) = result {
            if let Err(cleanup_error) = self.cleanup() {
                log::warn!("IME attachment failure cleanup failed: {cleanup_error}");
            }
            completed(Err(error));
            return;
        }
        completed(Ok(true));
        let mut status = self
            .state
            .status
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        while *status == AttachmentStatus::Pending {
            status = self
                .state
                .changed
                .wait(status)
                .unwrap_or_else(std::sync::PoisonError::into_inner);
        }
        let cancelled = *status == AttachmentStatus::Cancelled;
        drop(status);
        if cancelled {
            if let Err(error) = self.cleanup() {
                log::warn!("IME attachment cancellation cleanup failed: {error}");
            }
        }
    }

    fn cleanup(&self) -> ImeResult<()> {
        self.inner.detach(&mut NativeSessions::wait_for_cleanup())
    }
}

impl ImeAttachmentCancellation {
    /// Accept a completed connection only if its focus request is still current.
    pub fn commit(&self) -> bool {
        self.state.transition(AttachmentStatus::Committed)
    }

    pub fn cancel(&self) {
        self.state.transition(AttachmentStatus::Cancelled);
    }
}

impl Drop for ImeAttachmentCancellation {
    fn drop(&mut self) {
        self.cancel();
    }
}

#[derive(Clone, Copy)]
struct EditorConfiguration {
    window_id: i32,
    enter_key: EnterKey,
    input_type: InputType,
    cursor: Option<Rect>,
    preview_supported: bool,
    consume_key_events: bool,
}

impl PartialEq for IME {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for IME {}

impl std::hash::Hash for IME {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(Arc::as_ptr(&self.inner), state);
    }
}

impl IME {
    pub fn new(option: AttachOptions) -> Self {
        Self::with_callback_thread(option, false, 0)
    }

    /// Bind this editor to the actual OHOS Window ID from getWindowProperties().id.
    /// Framework-local IDs and native rendering surface handles are not window IDs.
    pub fn new_for_window(option: AttachOptions, window_id: i32) -> Self {
        Self::with_callback_thread(option, false, window_id)
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
                config.set_enter_key(configuration.enter_key);
                config.set_input_type(configuration.input_type);
                config.set_preview_text_supported(configuration.preview_supported);
                if let Err(error) = config.set_consume_key_events(configuration.consume_key_events)
                {
                    log::warn!("Cannot configure native editor key handling: {error}");
                }
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
            inner: Arc::new(IMEInner {
                id: next_session_id(),
                native: Mutex::new(NativeEditor {
                    raw: None,
                    editor: None,
                    generation: 0,
                }),
                attachment_serial: Mutex::new(()),
                pending_attachment: Mutex::new(None),
                show_on_attach: option.is_showing_keyboard(),
                callbacks: Arc::new(RwLock::new(callbacks)),
                needs_attach: AtomicBool::new(true),
                configuration,
                text_state,
                text_notified: AtomicBool::new(false),
                #[cfg(feature = "api-22")]
                callbacks_in_main_thread,
            }),
            main_thread: PhantomData,
        }
    }

    /// Prepare background attachment using the platform's IPC callback thread.
    /// The ordinary IME handle remains confined to its owning UI thread.
    pub fn prepare_background_attachment(
        &self,
    ) -> ImeResult<(ImeAttachment, ImeAttachmentCancellation)> {
        #[cfg(feature = "api-22")]
        if self.inner.callbacks_in_main_thread {
            return Err(ImeError::new(
                "background-main-thread-callbacks",
                ohos_input_method_sys::InputMethod_ErrorCode_IME_ERR_IMCLIENT,
            ));
        }
        let state = Arc::new(AttachmentState {
            status: Mutex::new(AttachmentStatus::Pending),
            changed: Condvar::new(),
        });
        let previous = self
            .inner
            .pending_attachment
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .replace(Arc::downgrade(&state))
            .and_then(|state| state.upgrade());
        if let Some(previous) = previous {
            previous.transition(AttachmentStatus::Cancelled);
        }
        Ok((
            ImeAttachment {
                inner: self.inner.clone(),
                state: state.clone(),
            },
            ImeAttachmentCancellation { state },
        ))
    }

    /// Declare that this editor handles unconsumed hardware letters, characters
    /// and function keys. Set this before attaching the editor. Returns false
    /// when the OS predates the optional API-26 attribute.
    pub fn set_consume_key_events(&self, enabled: bool) -> bool {
        self.inner
            .configuration
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .consume_key_events = enabled;
        TextConfig::supports_key_event_configuration()
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
        let sessions = NativeSessions::acquire("notify")?;
        let Some(raw) = self.active_proxy(&sessions) else {
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
        let sessions = NativeSessions::acquire("notify")?;
        let Some(raw) = self.active_proxy(&sessions) else {
            return Ok(());
        };
        let cursor = Cursor::new(rect);
        let code = unsafe { OH_InputMethodProxy_NotifyCursorUpdate(raw.as_ptr(), cursor.raw) };
        self.notification_result("update-cursor", code)
    }

    /// Publish attributes and cursor changes cached while background attachment
    /// held the native session guard. Call after accepting its completed result.
    pub fn try_refresh_configuration(&self) -> ImeResult<()> {
        let sessions = NativeSessions::acquire("notify")?;
        let Some(raw) = self.active_proxy(&sessions) else {
            return Ok(());
        };
        let configuration = *self
            .inner
            .configuration
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let code = unsafe {
            OH_InputMethodProxy_NotifyConfigurationChange(
                raw.as_ptr(),
                configuration.enter_key.into(),
                configuration.input_type.into(),
            )
        };
        self.notification_result("update-configuration", code)?;
        if let Some(rect) = configuration.cursor {
            let cursor = Cursor::new(rect);
            let code = unsafe { OH_InputMethodProxy_NotifyCursorUpdate(raw.as_ptr(), cursor.raw) };
            self.notification_result("update-cursor", code)?;
        }
        Ok(())
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
                // A busy session or failed notification must leave this snapshot retryable.
                self.inner.text_notified.store(false, Ordering::Release);
                true
            }
        };
        if !changed && self.inner.text_notified.load(Ordering::Acquire) {
            return Ok(());
        }
        let sessions = NativeSessions::acquire("notify")?;
        let Some(raw) = self.active_proxy(&sessions) else {
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
        self.inner.text_notified.store(true, Ordering::Release);
        Ok(())
    }

    fn active_proxy(
        &self,
        sessions: &NativeSessions,
    ) -> Option<NonNull<InputMethod_InputMethodProxy>> {
        (sessions.active_id == self.inner.id && !self.inner.needs_attach.load(Ordering::Acquire))
            .then(|| self.inner.native().raw)
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

    /// Attach this editor, replacing any other `IME` session previously
    /// attached through this binding.
    pub fn try_attach(&self) -> ImeResult<()> {
        let mut sessions = NativeSessions::acquire("attach")?;
        if self.active_proxy(&sessions).is_some() {
            return Ok(());
        }

        // Keep the same callback table across stale-session recovery. Neither
        // NOT_EDITABLE nor DETACHED proves that OnInputStop has finished with it.
        let mut native = self.inner.native();
        if native.editor.is_none() {
            let editor = TextEditor::new();
            #[cfg(feature = "api-22")]
            if self.inner.callbacks_in_main_thread {
                editor.set_callback_in_main_thread(true)?;
            }
            register_callbacks(editor.raw, self.inner.callbacks.clone());
            native.editor = Some(editor);
        }
        let Some(editor) = native.editor.as_ref().map(|editor| editor.raw) else {
            return Err(ImeError::new(
                "attach",
                InputMethod_ErrorCode_IME_ERR_NULL_POINTER,
            ));
        };
        native.raw.take();
        self.invalidate_session();
        native.generation = sessions.generation;
        // Even a failed attach may replace the NDK's global proxy. Never let an
        // older Rust session use its previous handle after this attempt.
        sessions.active_id = 0;
        let mut raw: *mut InputMethod_InputMethodProxy = ptr::null_mut();
        let options = AttachOptions::new(self.inner.show_on_attach);
        let code = unsafe { OH_InputMethodController_Attach(editor, options.raw, &mut raw) };
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

        sessions.complete_transition(self.inner.id);
        native.generation = sessions.generation;
        native.raw = Some(raw);
        self.inner.needs_attach.store(false, Ordering::Release);
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
        let sessions = NativeSessions::acquire("hide-keyboard")?;
        let Some(raw) = self.active_proxy(&sessions) else {
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

    /// End this session without detaching an editor which replaced it. If the
    /// native cleanup fails, callbacks are disabled and their allocation is
    /// retained until a later successful native attach/detach.
    pub fn try_detach(&self) -> ImeResult<()> {
        let mut sessions = NativeSessions::acquire("detach")?;
        self.inner.detach(&mut sessions)
    }

    pub fn detach(&self) {
        let _ = self.try_detach();
    }

    fn show_attached(&self) -> ImeResult<()> {
        let sessions = NativeSessions::acquire("show-keyboard")?;
        let Some(raw) = self.active_proxy(&sessions) else {
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
        self.inner.needs_attach.store(true, Ordering::Release);
        self.inner.text_notified.store(false, Ordering::Release);
    }
}

impl IMEInner {
    fn native(&self) -> MutexGuard<'_, NativeEditor> {
        self.native
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn detach(&self, sessions: &mut NativeSessions) -> ImeResult<()> {
        self.needs_attach.store(true, Ordering::Release);
        self.text_notified.store(false, Ordering::Release);
        let mut native = self.native();
        let raw = native.raw.take();
        let mut result = Ok(());
        if sessions.active_id == self.id {
            sessions.active_id = 0;
            if let Some(raw) = raw {
                let code = unsafe { OH_InputMethodController_Detach(raw.as_ptr()) };
                if code == 0 {
                    sessions.complete_transition(0);
                } else {
                    result = Err(ImeError::new("detach", code));
                }
            }
        }
        if let Some(editor) = native.editor.take() {
            unregister_callbacks(editor.raw);
            if native.generation < sessions.generation {
                drop(editor);
            } else {
                NativeSessions::retire(editor);
            }
        }
        result
    }
}

impl Drop for IMEInner {
    fn drop(&mut self) {
        if let Ok(mut sessions) = NativeSessions::acquire("detach") {
            if let Err(error) = self.detach(&mut sessions) {
                log::warn!("Cannot detach dropped IME: {error}");
            }
        } else if let Some(editor) = self
            .native
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .editor
            .take()
        {
            // A synchronous native callback may drop a different Rust session.
            // Its pointer cannot be freed until the in-flight transition ends.
            unregister_callbacks(editor.raw);
            NativeSessions::retire(editor);
        }
    }
}

fn next_session_id() -> u64 {
    loop {
        let id = NEXT_SESSION_ID.fetch_add(1, Ordering::Relaxed);
        if id != 0 {
            return id;
        }
    }
}
