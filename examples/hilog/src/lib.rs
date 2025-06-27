use napi_derive_ohos::napi;

use log::{debug, error, LevelFilter};
use ohos_hilog_binding::log::Config;

#[napi]
pub fn log() {
    let _ = ohos_hilog_binding::forward_stdio_to_hilog();
    println!("hello");
    ohos_hilog_binding::hilog_info!("hello");
}

#[napi]
pub fn info() {
    ohos_hilog_binding::log::init_once(Config::default().with_max_level(LevelFilter::Info));

    debug!("this is a debug {}", "message");
    error!("this is printed by default");
}
