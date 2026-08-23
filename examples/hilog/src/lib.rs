use std::sync::LazyLock;

use napi_derive_ohos::napi;
use ohos_hilog_binding::{Hilog, LogOptions};

static LOGGER: LazyLock<Hilog> = LazyLock::new(Hilog::new);

#[napi]
pub fn info() {
    ohos_hilog_binding::info("info log from global logger");
}

#[napi]
pub fn log() {
    log::info!("log crate -> hilog redirect (tag RustStdoutStderr / hilog_rs)");
}

#[napi]
pub fn debug() {
    ohos_hilog_binding::debug("debug log from global logger");
}

#[napi]
pub fn warn() {
    ohos_hilog_binding::warn("warn log from global logger");
}

#[napi]
pub fn error() {
    ohos_hilog_binding::error("error log from global logger");
}

/// Log at every level through an instance logger with a custom tag/domain.
#[napi]
pub fn custom_tag_logs() {
    LOGGER.with_options(LogOptions {
        domain: 0x0D1A,
        tag: "demo_hilog_tag",
    });
    LOGGER.debug("instance debug");
    LOGGER.info("instance info");
    LOGGER.warn("instance warn");
    LOGGER.error("instance error");
}

/// Update the global logger tag/domain used by the free functions.
#[napi]
pub fn set_global_tag(tag: String) {
    // LogOptions::tag is &'static str; leak the boxed str so it lives forever.
    let tag: &'static str = Box::leak(tag.into_boxed_str());
    ohos_hilog_binding::set_global_options(LogOptions {
        domain: 0x0D1A,
        tag,
    });
    ohos_hilog_binding::info("global options updated — subsequent logs use the new tag");
}
