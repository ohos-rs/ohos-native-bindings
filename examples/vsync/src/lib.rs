use std::sync::LazyLock;

use napi_derive_ohos::napi;
use ohos_hilog_binding::hilog_info;
use ohos_vsync_binding::Vsync;

static VSYNC: LazyLock<Vsync> = LazyLock::new(|| Vsync::new("vsync"));

#[napi]
pub fn handle_vsync() {
    VSYNC.on_frame_once(|s| {
        hilog_info!("vsync: {}", s);
    });
}

#[napi]
pub fn handle_vsync_with_self() {
    let a = 1;
    VSYNC.on_frame(move |s| {
        hilog_info!("vsync: {} {}", s, a);
    });
}
