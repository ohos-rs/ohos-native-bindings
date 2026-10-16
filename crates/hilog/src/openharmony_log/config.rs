use crate::LogLevel;

use super::FormatFn;
use log::{Level, LevelFilter, Record};
use std::ffi::{CStr, CString};
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

/// Checks the system log policy with a null-terminated tag.
fn openharmony_is_loggable(tag: &CStr, record_level: Level) -> bool {
    let prio = openharmony_log_priority_from_level(record_level);
    // SAFETY: CStr provides a null-terminated tag that remains alive for the call.
    unsafe {
        ohos_hilogs_sys::OH_LOG_IsLoggable(ohos_hilogs_sys::LOG_DOMAIN, tag.as_ptr(), prio.into())
    }
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
        if let Some(tag) = &self.tag {
            return openharmony_is_loggable(tag, level);
        }
        CString::new(tag).is_ok_and(|tag| openharmony_is_loggable(&tag, level))
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

#[cfg(test)]
// Clippy also lints OHOS's thread_local! expansion despite the const initializers.
#[cfg_attr(target_env = "ohos", allow(clippy::missing_const_for_thread_local))]
mod tests {
    use super::Config;
    use log::Level;
    use std::cell::{Cell, RefCell};
    use std::ffi::{c_char, CStr};

    #[derive(Debug, PartialEq)]
    struct LogQuery {
        domain: u32,
        tag: Vec<u8>,
        level: u32,
        tag_ptr: usize,
    }

    thread_local! {
        static QUERIES: RefCell<Vec<LogQuery>> = const { RefCell::new(Vec::new()) };
        static LOGGABLE: Cell<bool> = const { Cell::new(true) };
    }

    // Interpose only the query in this test binary; production still links the
    // system HiLog implementation. Capture the actual C ABI arguments.
    #[unsafe(export_name = "OH_LOG_IsLoggable")]
    unsafe extern "C" fn capture_log_query(domain: u32, tag: *const c_char, level: u32) -> bool {
        // SAFETY: production passes CStr; substring fixtures also keep a later
        // terminator in the same allocation so the old implementation fails safely.
        let bytes = unsafe { CStr::from_ptr(tag) }.to_bytes().to_vec();
        QUERIES.with_borrow_mut(|queries| {
            queries.push(LogQuery {
                domain,
                tag: bytes,
                level,
                tag_ptr: tag as usize,
            });
        });
        LOGGABLE.get()
    }

    #[test]
    fn loggable_terminates_target_substrings() {
        for target in ["crate::module", "应用::模块", ""] {
            let backing = format!("{target}NOT_PART_OF_TAG\0");
            let tag = &backing[..target.len()];
            assert!(Config::default().is_loggable(tag, Level::Info));
            let queries = QUERIES.take();
            assert_eq!(queries.len(), 1);
            assert_eq!(queries[0].tag, target.as_bytes());
            assert_eq!(queries[0].domain, ohos_hilogs_sys::LOG_DOMAIN);
            assert_eq!(queries[0].level, ohos_hilogs_sys::LogLevel_LOG_INFO);
        }
    }

    #[test]
    fn loggable_reuses_configured_tag() {
        let config = Config::default().with_tag("configured");
        assert!(config.is_loggable("different-target\0", Level::Warn));
        let queries = QUERIES.take();
        assert_eq!(queries.len(), 1);
        assert_eq!(queries[0].tag, b"configured");
        assert_eq!(
            queries[0].tag_ptr,
            config.tag.as_ref().unwrap().as_ptr() as usize
        );
        assert_eq!(queries[0].level, ohos_hilogs_sys::LogLevel_LOG_WARN);
    }

    #[test]
    fn loggable_rejects_interior_nul_without_querying_system() {
        assert!(!Config::default().is_loggable("truncated\0target", Level::Info));
        assert!(QUERIES.take().is_empty());
    }

    #[test]
    fn loggable_preserves_levels_and_system_result() {
        let config = Config::default().with_tag("configured");
        for (level, expected) in [
            (Level::Error, ohos_hilogs_sys::LogLevel_LOG_ERROR),
            (Level::Warn, ohos_hilogs_sys::LogLevel_LOG_WARN),
            (Level::Info, ohos_hilogs_sys::LogLevel_LOG_INFO),
            (Level::Debug, ohos_hilogs_sys::LogLevel_LOG_DEBUG),
            (Level::Trace, ohos_hilogs_sys::LogLevel_LOG_INFO),
        ] {
            for allowed in [false, true] {
                LOGGABLE.set(allowed);
                assert_eq!(config.is_loggable("target\0", level), allowed);
                let queries = QUERIES.take();
                assert_eq!(queries.len(), 1);
                assert_eq!(queries[0].level, expected);
            }
        }
    }
}
