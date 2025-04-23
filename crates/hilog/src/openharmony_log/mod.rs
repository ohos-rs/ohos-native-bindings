// Copyright 2016 The android_logger Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.


//! A logger which writes to openharmony output.
//!
//! ## Example
//!
//! ```
//! #[macro_use] extern crate log;
//! extern crate ohos_hilog_binding;
//!
//! use log::LevelFilter;
//! use ohos_hilog_binding::log::Config;
//!
//! /// Android code may not have obvious "main", this is just an example.
//! fn main() {
//!     ohos_hilog_binding::init_once(
//!         Config::default().with_max_level(LevelFilter::Trace),
//!     );
//!
//!     debug!("this is a debug {}", "message");
//!     error!("this is printed by default");
//! }
//! ```
//!
//! ## Example with module path filter
//!
//! It is possible to limit log messages to output from a specific crate,
//! and override the logcat tag name (by default, the crate name is used):
//!
//! ```
//! #[macro_use] extern crate log;
//! extern crate ohos_hilog_binding;
//!
//! use log::LevelFilter;
//! use ohos_hilog_binding::log::{Config,FilterBuilder};
//!
//! fn main() {
//!     ohos_hilog_binding::log::init_once(
//!         Config::default()
//!             .with_max_level(LevelFilter::Trace)
//!             .with_tag("mytag")
//!             .with_filter(FilterBuilder::new().parse("debug,hello::crate=trace").build()),
//!     );
//!
//!     // ..
//! }
//! ```
//!
//! ## Example with a custom log formatter
//!
//! ```
//! use ohos_hilog_binding::log::Config;
//!
//! ohos_hilog_binding::log::init_once(
//!     Config::default()
//!         .with_max_level(log::LevelFilter::Trace)
//!         .format(|f, record| write!(f, "my_app: {}", record.args()))
//! )
//! ```

use log::{Log, Metadata, Record};
use std::ffi::{c_char, CStr, CString};
use std::fmt;
use std::mem::MaybeUninit;
use std::sync::OnceLock;

use arrays::{fill_tag_bytes, uninit_array};
pub use config::Config;
pub use env_filter::{Builder as FilterBuilder, Filter};
use platform_log_writer::PlatformLogWriter;

pub(crate) type FormatFn = Box<dyn Fn(&mut dyn fmt::Write, &Record) -> fmt::Result + Sync + Send>;

pub mod arrays;
pub mod config;
pub mod platform_log_writer;

/// Outputs log to OpenHarmony system.
fn openharmony_log(prio: crate::LogLevel, tag: &CStr, msg: &CStr) {
    unsafe {
        ohos_hilogs_sys::OH_LOG_Print(
            ohos_hilogs_sys::LogType_LOG_APP,
            prio.into(),
            ohos_hilogs_sys::LOG_DOMAIN,
            tag.as_ptr() as *const c_char,
            msg.as_ptr() as *const c_char,
        );
    };
}

/// Underlying openharmony logger backend
#[derive(Debug, Default)]
pub struct OpenHarmonyLogger {
    config: OnceLock<Config>,
}

impl OpenHarmonyLogger {
    /// Create new logger instance from config
    pub fn new(config: Config) -> OpenHarmonyLogger {
        OpenHarmonyLogger {
            config: OnceLock::from(config),
        }
    }

    fn config(&self) -> &Config {
        self.config.get_or_init(Config::default)
    }
}

static OPENHARMONY_LOGGER: OnceLock<OpenHarmonyLogger> = OnceLock::new();

/// Maximum length of a tag that does not require allocation.
///
/// Tags configured explicitly in [Config] will not cause an extra allocation. When the tag is
/// derived from the module path, paths longer than this limit will trigger an allocation for each
/// log statement.
///
/// The terminating nullbyte does not count towards this limit.
pub(crate) const LOGGING_TAG_MAX_LEN: usize = 127;
pub(crate) const LOGGING_MSG_MAX_LEN: usize = 4000;

impl Log for OpenHarmonyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.config()
            .is_loggable(metadata.target(), metadata.level())
    }

    fn log(&self, record: &Record) {
        let config = self.config();

        if !self.enabled(record.metadata()) {
            return;
        }

        // this also checks the level, but only if a filter was
        // installed.
        if !config.filter_matches(record) {
            return;
        }

        // Temporary storage for null-terminating record.module_path() if it's needed.
        // Tags too long to fit here cause allocation.
        let mut tag_bytes: [MaybeUninit<u8>; LOGGING_TAG_MAX_LEN + 1] = uninit_array();
        // In case we end up allocating, keep the CString alive.
        let _owned_tag;

        let module_path = record.module_path().unwrap_or_default();

        let tag = if let Some(tag) = &config.tag {
            tag
        } else if module_path.len() < tag_bytes.len() {
            fill_tag_bytes(&mut tag_bytes, module_path.as_bytes())
        } else {
            // Tag longer than available stack buffer; allocate.
            _owned_tag = CString::new(module_path.as_bytes())
                .expect("record.module_path() shouldn't contain nullbytes");
            _owned_tag.as_ref()
        };

        // message must not exceed LOGGING_MSG_MAX_LEN
        // therefore split log message into multiple log calls
        let mut writer = PlatformLogWriter::new(record.level(), tag);

        // If a custom tag is used, add the module path to the message.
        // Use PlatformLogWriter to output chunks if they exceed max size.
        let _ = match (&config.tag, &config.custom_format) {
            (_, Some(format)) => format(&mut writer, record),
            (Some(_), _) => fmt::write(
                &mut writer,
                format_args!("{}: {}", module_path, *record.args()),
            ),
            _ => fmt::write(&mut writer, *record.args()),
        };

        // output the remaining message (this would usually be the most common case)
        writer.flush();
    }

    fn flush(&self) {}
}

/// Send a log record to Android logging backend.
///
/// This action does not require initialization. However, without initialization it
/// will use the default filter, which allows all logs.
pub fn log(record: &Record) {
    OPENHARMONY_LOGGER
        .get_or_init(OpenHarmonyLogger::default)
        .log(record)
}

/// Initializes the global logger with an android logger.
///
/// This can be called many times, but will only initialize logging once,
/// and will not replace any other previously initialized logger.
///
/// It is ok to call this at the activity creation, and it will be
/// repeatedly called on every lifecycle restart (i.e. screen rotation).
pub fn init_once(config: Config) {
    let log_level = config.log_level;
    let logger = OPENHARMONY_LOGGER.get_or_init(|| OpenHarmonyLogger::new(config));

    if let Err(err) = log::set_logger(logger) {
        log::debug!("openharmony_logger: log::set_logger failed: {}", err);
    } else if let Some(level) = log_level {
        log::set_max_level(level);
    }
}
