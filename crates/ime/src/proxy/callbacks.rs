use std::sync::{Arc, LazyLock, RwLock};

use ohos_input_method_sys::InputMethod_TextEditorProxy;
use rustc_hash::FxHashMap;

use crate::{
    private_command::PrivateCommand, Action, Direction, EnterKey, KeyboardStatus, Selection,
    TextConfig,
};

type Callback<T> = Arc<T>;
type SetPreviewTextCallback = Callback<dyn Fn(String, i32, i32) + Send + Sync>;

#[derive(Default)]
pub(crate) struct IMECallbacks {
    pub(crate) delete_backward: Option<Callback<dyn Fn(i32) + Send + Sync>>,
    pub(crate) insert_text: Option<Callback<dyn Fn(String) + Send + Sync>>,
    pub(crate) delete_forward: Option<Callback<dyn Fn(i32) + Send + Sync>>,
    pub(crate) finish_text_preview: Option<Callback<dyn Fn() + Send + Sync>>,
    pub(crate) get_left_text_of_cursor: Option<Callback<dyn Fn(i32) -> String + Send + Sync>>,
    pub(crate) get_right_text_of_cursor: Option<Callback<dyn Fn(i32) -> String + Send + Sync>>,
    pub(crate) get_text_config: Option<Callback<dyn Fn(TextConfig) + Send + Sync>>,
    pub(crate) get_text_index_at_cursor: Option<Callback<dyn Fn() -> i32 + Send + Sync>>,
    pub(crate) handle_extend_action: Option<Callback<dyn Fn(Action) + Send + Sync>>,
    pub(crate) handle_set_selection: Option<Callback<dyn Fn(Selection) + Send + Sync>>,
    pub(crate) move_cursor: Option<Callback<dyn Fn(Direction) + Send + Sync>>,
    pub(crate) receive_private_command: Option<Callback<dyn Fn(Vec<PrivateCommand>) + Send + Sync>>,
    pub(crate) send_enter_key: Option<Callback<dyn Fn(EnterKey) + Send + Sync>>,
    pub(crate) send_keyboard_status: Option<Callback<dyn Fn(KeyboardStatus) + Send + Sync>>,
    pub(crate) set_preview_text: Option<SetPreviewTextCallback>,
}

pub(crate) type SharedCallbacks = Arc<RwLock<IMECallbacks>>;

static CALLBACKS_BY_EDITOR: LazyLock<RwLock<FxHashMap<usize, SharedCallbacks>>> =
    LazyLock::new(|| RwLock::new(FxHashMap::default()));

pub(crate) fn register_callbacks(
    editor: *mut InputMethod_TextEditorProxy,
    callbacks: SharedCallbacks,
) {
    let mut registry = CALLBACKS_BY_EDITOR
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    registry.insert(editor as usize, callbacks);
}

pub(crate) fn unregister_callbacks(editor: *mut InputMethod_TextEditorProxy) {
    let mut registry = CALLBACKS_BY_EDITOR
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    registry.remove(&(editor as usize));
}

pub(crate) fn callbacks_for(editor: *mut InputMethod_TextEditorProxy) -> Option<SharedCallbacks> {
    CALLBACKS_BY_EDITOR
        .read()
        .ok()?
        .get(&(editor as usize))
        .cloned()
}
