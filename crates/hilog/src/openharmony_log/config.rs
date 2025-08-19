use crate::LogLevel;

use super::FormatFn;
use log::{Level, LevelFilter, Record};
use std::ffi::{c_char, CString};
use std::fmt;

/// Filter for android logger.
#[derive(Default)]
pub struct Config {
    pub(crate) log_level: Option<LevelFilter>,
    filter: Option<env_filter::Filter>,
    pub(crate) tag: Option<CString>,
    pub(crate) custom_format: Option<FormatFn>,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("log_level", &self.log_level)
            .field("filter", &self.filter)
            .field("tag", &self.tag)
            .field(
                "custom_format",
                match &self.custom_format {
                    Some(_) => &"Some(_)",
                    None => &"None",
                },
            )
            .finish()
    }
}

fn openharmony_log_priority_from_level(level: Level) -> LogLevel {
    match level {
        Level::Warn => LogLevel::LogWarn,
        Level::Info => LogLevel::LogInfo,
        Level::Debug => LogLevel::LogDebug,
        Level::Error => LogLevel::LogError,
        Level::Trace => LogLevel::LogInfo,
    }
}

/// Asks OpenHarmony liblog if a message with given `tag` and `priority` should be logged, using
/// `default_prio` as the level filter in case no system- or process-wide overrides are set.
fn openharmony_is_loggable_len(prio: LogLevel, tag: &str) -> bool {
    // SAFETY: tag points to a valid string tag.len() bytes long.
    unsafe {
        ohos_hilogs_sys::OH_LOG_IsLoggable(
            ohos_hilogs_sys::LOG_DOMAIN,
            tag.as_ptr() as *const c_char,
            prio.into(),
        )
    }
}

fn openharmony_is_loggable(tag: &str, record_level: Level) -> bool {
    let prio = openharmony_log_priority_from_level(record_level);
    openharmony_is_loggable_len(prio, tag)
}

impl Config {
    /// Changes the maximum log level.
    ///
    /// Note, that `Trace` is the maximum level, because it provides the
    /// maximum amount of detail in the emitted logs.
    ///
    /// If `Off` level is provided, then nothing is logged at all.
    ///
    /// [`log::max_level()`] is considered as the default level.
    pub fn with_max_level(mut self, level: LevelFilter) -> Self {
        self.log_level = Some(level);
        self
    }

    pub(crate) fn filter_matches(&self, record: &Record) -> bool {
        if let Some(ref filter) = self.filter {
            filter.matches(record)
        } else {
            true
        }
    }

    pub(crate) fn is_loggable(&self, tag: &str, level: Level) -> bool {
        openharmony_is_loggable(tag, level)
    }

    pub fn with_filter(mut self, filter: env_filter::Filter) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn with_tag<S: Into<Vec<u8>>>(mut self, tag: S) -> Self {
        self.tag = Some(CString::new(tag).expect("Can't convert tag to CString"));
        self
    }

    /// Sets the format function for formatting the log output.
    /// ```
    /// # use android_logger::Config;
    /// android_logger::init_once(
    ///     Config::default()
    ///         .with_max_level(log::LevelFilter::Trace)
    ///         .format(|f, record| write!(f, "my_app: {}", record.args()))
    /// )
    /// ```
    pub fn format<F>(mut self, format: F) -> Self
    where
        F: Fn(&mut dyn fmt::Write, &Record) -> fmt::Result + Sync + Send + 'static,
    {
        self.custom_format = Some(Box::new(format));
        self
    }
}
