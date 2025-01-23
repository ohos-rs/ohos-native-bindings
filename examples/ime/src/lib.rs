use std::sync::{LazyLock, Mutex};

use napi_derive_ohos::napi;
use ohos_hilog_binding::hilog_info;
use ohos_ime_binding::IME;

static IME_INSTANCE: LazyLock<Mutex<Option<IME>>> = LazyLock::new(|| Mutex::new(None));

#[napi]
pub fn add_ime() -> () {
    let ime = IME::new(Default::default());

    ime.insert_text(|s| hilog_info!(format!("insert_text: {}", s)));

    let mut guard = IME_INSTANCE.lock().unwrap();
    *guard = Some(ime);
}

#[napi]
pub fn show() -> () {
    let mut guard = IME_INSTANCE.lock().unwrap();
    if let Some(ime) = guard.as_mut() {
        ime.show_keyboard();
    }
}
