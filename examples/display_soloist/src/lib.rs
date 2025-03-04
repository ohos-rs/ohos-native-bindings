use napi_derive_ohos::napi;
use ohos_display_soloist_binding::DisplaySoloist;
use ohos_hilog_binding::hilog_info;
use std::sync::LazyLock;

static DISPLAY_SOLOIST: LazyLock<DisplaySoloist> = LazyLock::new(|| DisplaySoloist::new(false));

#[napi]
pub fn start_display_soloist() {
    DISPLAY_SOLOIST.on_frame(|ts, tts| {
        hilog_info!("display_soloist: ts: {}, tts: {}", ts, tts);
    });
}

#[napi]
pub fn stop_display_soloist() {
    DISPLAY_SOLOIST.stop();
}
