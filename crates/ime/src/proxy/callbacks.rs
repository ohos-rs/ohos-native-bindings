use std::sync::{LazyLock, RwLock};

use crate::{
    private_command::PrivateCommand, Action, Direction, EnterKey, KeyboardStatus, Selection,
    TextConfig,
};

#[derive(Default)]
pub struct IMECallbacks {
    pub(crate) delete_backward: Option<Box<dyn Fn(i32) -> ()>>,
    pub(crate) insert_text: Option<Box<dyn Fn(String) -> ()>>,
    pub(crate) delete_forward: Option<Box<dyn Fn(i32) -> ()>>,
    pub(crate) finish_text_preview: Option<Box<dyn Fn() -> ()>>,
    pub(crate) get_left_text_of_cursor: Option<Box<dyn Fn(i32) -> String>>,
    pub(crate) get_right_text_of_cursor: Option<Box<dyn Fn(i32) -> String>>,
    pub(crate) get_text_config: Option<Box<dyn Fn(TextConfig) -> ()>>,
    pub(crate) get_text_index_at_cursor: Option<Box<dyn Fn() -> i32>>,
    pub(crate) handle_extend_action: Option<Box<dyn Fn(Action) -> ()>>,
    pub(crate) handle_set_selection: Option<Box<dyn Fn(Selection) -> ()>>,
    pub(crate) move_cursor: Option<Box<dyn Fn(Direction) -> ()>>,
    pub(crate) receive_private_command: Option<Box<dyn Fn(Vec<PrivateCommand>) -> ()>>,
    pub(crate) send_enter_key: Option<Box<dyn Fn(EnterKey) -> ()>>,
    pub(crate) send_keyboard_status: Option<Box<dyn Fn(KeyboardStatus) -> ()>>,
    pub(crate) set_preview_text: Option<Box<dyn Fn(String, i32, i32) -> ()>>,
}

unsafe impl Sync for IMECallbacks {}
unsafe impl Send for IMECallbacks {}

pub static OHOS_RS_IME_CALLBACKS: LazyLock<RwLock<IMECallbacks>> =
    LazyLock::new(|| RwLock::new(IMECallbacks::default()));
