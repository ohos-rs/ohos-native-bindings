use std::sync::LazyLock;

use napi_derive_ohos::napi;
use ohos_display_soloist_binding::{DisplaySoloist, DisplaySoloist_ExpectedRateRange};
use ohos_hilog_binding::hilog_info;

static DISPLAY_SOLOIST: LazyLock<DisplaySoloist> = LazyLock::new(|| DisplaySoloist::new(false));

/// Soloist running its vsync callbacks on a dedicated thread.
static EXCLUSIVE_SOLOIST: LazyLock<DisplaySoloist> = LazyLock::new(|| DisplaySoloist::new(true));

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

/// Start callbacks on the exclusive-thread soloist instance.
#[napi]
pub fn start_exclusive_soloist() {
    EXCLUSIVE_SOLOIST.on_frame(|ts, tts| {
        hilog_info!("display_soloist(exclusive): ts: {}, tts: {}", ts, tts);
    });
}

#[napi]
pub fn stop_exclusive_soloist() {
    EXCLUSIVE_SOLOIST.stop();
}

/// Constrain the shared soloist's expected frame rate range.
#[napi]
pub fn set_frame_rate(min: i32, max: i32, expected: i32) {
    DISPLAY_SOLOIST.set_frame_rate(DisplaySoloist_ExpectedRateRange { min, max, expected });
    hilog_info!("display_soloist: frame rate set to [{min}, {expected}, {max}]");
}
