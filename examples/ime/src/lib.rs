use napi_derive_ohos::napi;
use ohos_hilog_binding::hilog_info;
use ohos_ime_binding::IME;

#[napi]
pub fn add_ime() -> () {
    let ime = IME::new(Default::default());

    ime.insert_text(|s| {
        hilog_info!(format!("ohos-rs {}", s));
    });
}
