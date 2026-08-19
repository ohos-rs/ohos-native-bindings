use std::sync::{LazyLock, Mutex};

use napi_derive_ohos::napi;
use ohos_hilog_binding::hilog_info;
use ohos_ime_binding::{AttachOptions, IME};

static IME_INSTANCE: LazyLock<Mutex<Option<IME>>> = LazyLock::new(|| Mutex::new(None));

/// Create the IME with show_keyboard=false and register every callback the
/// binding exposes.
#[napi]
pub fn add_ime() {
    let ime = IME::new(Default::default());

    ime.insert_text(|s| hilog_info!(format!("insert_text: {}", s)));
    ime.pre_edit(|s, start, end| hilog_info!(format!("pre_edit: {} [{},{}]", s, start, end)));
    ime.on_status_change(|status| hilog_info!(format!("on_status_change: {:?}", status)));
    ime.on_delete(|forward| hilog_info!(format!("on_delete: {}", forward)));
    ime.on_backspace(|forward| hilog_info!(format!("on_backspace: {}", forward)));
    ime.on_enter(|key| hilog_info!(format!("on_enter: {:?}", key)));
    ime.on_preview(|s, start, end| hilog_info!(format!("on_preview: {} [{},{}]", s, start, end)));
    ime.on_finish_preview(|| hilog_info!("on_finish_preview"));

    let mut guard = IME_INSTANCE.lock().unwrap();
    *guard = Some(ime);
}

/// Create the IME showing the keyboard immediately on attach.
#[napi]
pub fn add_ime_show_keyboard() {
    let ime = IME::new(AttachOptions::new(true));
    ime.insert_text(|s| hilog_info!(format!("insert_text: {}", s)));
    let mut guard = IME_INSTANCE.lock().unwrap();
    *guard = Some(ime);
}

#[napi]
pub fn show() {
    let mut guard = IME_INSTANCE.lock().unwrap();
    if let Some(ime) = guard.as_mut() {
        ime.show_keyboard();
    }
}

#[napi]
pub fn detach() {
    let mut guard = IME_INSTANCE.lock().unwrap();
    if let Some(ime) = guard.as_mut() {
        ime.detach();
    }
}

#[napi]
pub fn attach() {
    let mut guard = IME_INSTANCE.lock().unwrap();
    if let Some(ime) = guard.as_mut() {
        ime.attach();
    }
}

#[napi]
pub fn hide() {
    let mut guard = IME_INSTANCE.lock().unwrap();
    if let Some(ime) = guard.as_mut() {
        ime.hide_keyboard();
    }
}
