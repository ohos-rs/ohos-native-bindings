use std::cell::RefCell;

use napi_derive_ohos::napi;
use ohos_hilog_binding::hilog_info;
use ohos_ime_binding::{AttachOptions, IME};

thread_local! {
    static IME_INSTANCE: RefCell<Option<IME>> = const { RefCell::new(None) };
}

/// Create the IME with show_keyboard=false and register every callback the
/// binding exposes.
#[napi]
pub fn add_ime() {
    let ime = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        IME::new_with_main_thread_callbacks(Default::default())
    })) {
        Ok(ime) => ime,
        Err(_) => return,
    };

    ime.insert_text(|s| hilog_info!(format!("insert_text: {}", s)));
    ime.pre_edit(|s, start, end| hilog_info!(format!("pre_edit: {} [{},{}]", s, start, end)));
    ime.on_status_change(|status| hilog_info!(format!("on_status_change: {:?}", status)));
    ime.on_delete(|forward| hilog_info!(format!("on_delete: {}", forward)));
    ime.on_backspace(|forward| hilog_info!(format!("on_backspace: {}", forward)));
    ime.on_enter(|key| hilog_info!(format!("on_enter: {:?}", key)));
    ime.on_preview(|s, start, end| hilog_info!(format!("on_preview: {} [{},{}]", s, start, end)));
    ime.on_finish_preview(|| hilog_info!("on_finish_preview"));

    IME_INSTANCE.with(|instance| instance.replace(Some(ime)));
}

/// Create the IME showing the keyboard immediately on attach.
#[napi]
pub fn add_ime_show_keyboard() {
    let ime = IME::new_with_main_thread_callbacks(AttachOptions::new(true));
    ime.insert_text(|s| hilog_info!(format!("insert_text: {}", s)));
    IME_INSTANCE.with(|instance| instance.replace(Some(ime)));
}

#[napi]
pub fn show() {
    with_ime(IME::show_keyboard);
}

#[napi]
pub fn detach() {
    with_ime(IME::detach);
}

#[napi]
pub fn attach() {
    with_ime(IME::attach);
}

#[napi]
pub fn hide() {
    with_ime(IME::hide_keyboard);
}

/// Regression probe for native option ownership: dropping an `IME` clone must
/// not detach the original or destroy its attach options.
#[napi]
pub fn clone_drop_reopen() {
    with_ime(|ime| {
        drop(ime.clone());
        ime.detach();
        ime.show_keyboard();
    });
}

/// Regression probe for the terminal lifecycle: show, hide without detaching,
/// then show the same native input-method session again.
#[napi]
pub fn show_hide_show() {
    with_ime(|ime| {
        ime.show_keyboard();
        ime.hide_keyboard();
        ime.show_keyboard();
    });
}

fn with_ime(callback: impl FnOnce(&IME)) {
    IME_INSTANCE.with(|instance| {
        if let Some(ime) = instance.borrow().as_ref() {
            callback(ime);
        }
    });
}
