use ohos_hilogs_sys::OH_LOG_Print;
use std::{
    ffi::CString,
    sync::{LazyLock, RwLock},
};

#[cfg(feature = "log")]
mod openharmony_log;

#[cfg(feature = "log")]
pub mod log {
    pub use crate::openharmony_log::*;
}

#[cfg(feature = "redirect")]
use std::{
    ffi::CStr,
    fs::File,
    io::{BufRead as _, BufReader, Result},
    os::fd::{FromRawFd as _, RawFd},
};

pub enum LogType {
    LogApp,
}

impl From<LogType> for ohos_hilogs_sys::LogType {
    fn from(value: LogType) -> Self {
        match value {
            LogType::LogApp => ohos_hilogs_sys::LogType_LOG_APP,
        }
    }
}

static GLOBAL_OPTIONS: LazyLock<RwLock<LogOptions>> =
    LazyLock::new(|| RwLock::new(LogOptions::default()));

/// set global options for hilog
pub fn set_global_options(options: LogOptions) {
    let mut global_options = GLOBAL_OPTIONS.write().unwrap();
    *global_options = options;
}

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    LogDebug,
    LogInfo,
    LogWarn,
    LogError,
    LogFatal,
}

impl From<LogLevel> for ohos_hilogs_sys::LogLevel {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::LogDebug => ohos_hilogs_sys::LogLevel_LOG_DEBUG,
            LogLevel::LogInfo => ohos_hilogs_sys::LogLevel_LOG_INFO,
            LogLevel::LogWarn => ohos_hilogs_sys::LogLevel_LOG_WARN,
            LogLevel::LogError => ohos_hilogs_sys::LogLevel_LOG_ERROR,
            LogLevel::LogFatal => ohos_hilogs_sys::LogLevel_LOG_FATAL,
        }
    }
}

pub struct LogOptions {
    // hilog print domain default is 0x0000
    pub domain: u32,
    /// hilog print tag default is `hilog_rs`
    pub tag: &'static str,
}

impl Default for LogOptions {
    fn default() -> Self {
        LogOptions {
            domain: 0x0000,
            tag: "hilog_rs",
        }
    }
}

pub struct Hilog {
    options: RwLock<LogOptions>,
}

unsafe impl Send for Hilog {}
unsafe impl Sync for Hilog {}

impl Hilog {
    pub fn new() -> Hilog {
        Hilog {
            options: RwLock::new(Default::default()),
        }
    }

    pub fn with_options(&self, options: LogOptions) {
        let mut inner_options = self.options.write().unwrap();
        *inner_options = options;
    }

    pub fn debug<T: AsRef<str>>(&self, info: T) {
        let option_result = self.options.read().unwrap();
        let tag = CString::new(option_result.tag).unwrap();
        let content = CString::new(info.as_ref()).unwrap();
        unsafe {
            OH_LOG_Print(
                LogType::LogApp.into(),
                LogLevel::LogDebug.into(),
                option_result.domain,
                tag.as_ptr(),
                content.as_ptr(),
            );
        }
    }

    pub fn info<T: AsRef<str>>(&self, info: T) {
        let option_result = self.options.read().unwrap();
        let tag = CString::new(option_result.tag).unwrap();
        let content = CString::new(info.as_ref()).unwrap();
        unsafe {
            OH_LOG_Print(
                LogType::LogApp.into(),
                LogLevel::LogInfo.into(),
                option_result.domain,
                tag.as_ptr(),
                content.as_ptr(),
            );
        }
    }

    pub fn warn<T: AsRef<str>>(&self, info: T) {
        let option_result = self.options.read().unwrap();
        let tag = CString::new(option_result.tag).unwrap();
        let content = CString::new(info.as_ref()).unwrap();
        unsafe {
            OH_LOG_Print(
                LogType::LogApp.into(),
                LogLevel::LogWarn.into(),
                option_result.domain,
                tag.as_ptr(),
                content.as_ptr(),
            );
        }
    }

    pub fn error<T: AsRef<str>>(&self, info: T) {
        let option_result = self.options.read().unwrap();
        let tag = CString::new(option_result.tag).unwrap();
        let content = CString::new(info.as_ref()).unwrap();
        unsafe {
            OH_LOG_Print(
                LogType::LogApp.into(),
                LogLevel::LogError.into(),
                option_result.domain,
                tag.as_ptr(),
                content.as_ptr(),
            );
        }
    }

    pub fn fatal<T: AsRef<str>>(&self, info: T) {
        let option_result = self.options.read().unwrap();
        let tag = CString::new(option_result.tag).unwrap();
        let content = CString::new(info.as_ref()).unwrap();
        unsafe {
            OH_LOG_Print(
                LogType::LogApp.into(),
                LogLevel::LogFatal.into(),
                option_result.domain,
                tag.as_ptr(),
                content.as_ptr(),
            );
        }
    }
}

macro_rules! log_factory {
    ($level: ident,$level_enum: expr) => {
        pub fn $level<T: AsRef<str>>(info: T) {
            let option_result = GLOBAL_OPTIONS.read().unwrap();
            let tag_result = option_result.tag;
            let domain = option_result.domain;

            let tag = CString::new(tag_result).unwrap();
            let content = CString::new(info.as_ref()).unwrap();

            unsafe {
                OH_LOG_Print(
                    LogType::LogApp.into(),
                    $level_enum,
                    domain,
                    tag.as_ptr(),
                    content.as_ptr(),
                );
            }
        }
    };
}

log_factory!(debug, LogLevel::LogDebug.into());
log_factory!(info, LogLevel::LogInfo.into());
log_factory!(warn, LogLevel::LogWarn.into());
log_factory!(error, LogLevel::LogError.into());
log_factory!(fatal, LogLevel::LogFatal.into());

#[macro_export]
macro_rules! hilog_debug {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        ohos_hilog_binding::debug(format!("{}", format_args!($fmt $(, $($arg)*)?)))
    };
    (format!($($arg:tt)*)) => {
        ohos_hilog_binding::debug(format!($($arg)*))
    };
    ($expr:expr) => {
        ohos_hilog_binding::debug(format!("{}", $expr))
    }
}

#[macro_export]
macro_rules! hilog_info {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        ohos_hilog_binding::info(format!("{}", format_args!($fmt $(, $($arg)*)?)))
    };
    (format!($($arg:tt)*)) => {
        ohos_hilog_binding::info(format!($($arg)*))
    };
    ($expr:expr) => {
        ohos_hilog_binding::info(format!("{}", $expr))
    }
}

#[macro_export]
macro_rules! hilog_warn {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        ohos_hilog_binding::warn(format!("{}", format_args!($fmt $(, $($arg)*)?)))
    };
    (format!($($arg:tt)*)) => {
        ohos_hilog_binding::warn(format!($($arg)*))
    };
    ($expr:expr) => {
        ohos_hilog_binding::warn(format!("{}", $expr))
    }
}

#[macro_export]
macro_rules! hilog_error {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        ohos_hilog_binding::error(format!("{}", format_args!($fmt $(, $($arg)*)?)))
    };
    (format!($($arg:tt)*)) => {
        ohos_hilog_binding::error(format!($($arg)*))
    };
    ($expr:expr) => {
        ohos_hilog_binding::error(format!("{}", $expr))
    }
}

#[macro_export]
macro_rules! hilog_fatal {
    ($fmt:literal $(, $($arg:tt)*)?) => {
        ohos_hilog_binding::fatal(format!("{}", format_args!($fmt $(, $($arg)*)?)))
    };
    (format!($($arg:tt)*)) => {
        ohos_hilog_binding::fatal(format!($($arg)*))
    };
    ($expr:expr) => {
        ohos_hilog_binding::fatal(format!("{}", $expr))
    }
}

#[cfg(feature = "redirect")]
pub fn forward_stdio_to_hilog() -> std::thread::JoinHandle<Result<()>> {
    // XXX: make this stdout/stderr redirection an optional / opt-in feature?...
    // these code base with android-activity
    // https://github.com/rust-mobile/android-activity/blob/main/android-activity/src/util.rs#L39
    // Use MIT LICENSE
    let file = unsafe {
        let mut logpipe: [RawFd; 2] = Default::default();
        libc::pipe2(logpipe.as_mut_ptr(), libc::O_CLOEXEC);
        libc::dup2(logpipe[1], libc::STDOUT_FILENO);
        libc::dup2(logpipe[1], libc::STDERR_FILENO);
        libc::close(logpipe[1]);

        File::from_raw_fd(logpipe[0])
    };

    std::thread::Builder::new()
        .name("stdio-to-hilog".to_string())
        .spawn(move || -> Result<()> {
            let tag = CStr::from_bytes_with_nul(b"RustStdoutStderr\0").unwrap();
            let mut reader = BufReader::new(file);
            let mut buffer = String::new();
            loop {
                buffer.clear();
                let len = match reader.read_line(&mut buffer) {
                    Ok(len) => len,
                    Err(e) => {
                        error(format!(
                            "Hilog forwarder failed to read stdin/stderr: {e:?}"
                        ));
                        break Err(e);
                    }
                };
                if len == 0 {
                    break Ok(());
                } else if let Ok(msg) = CString::new(buffer.clone()) {
                    unsafe {
                        OH_LOG_Print(
                            LogType::LogApp.into(),
                            LogLevel::LogInfo.into(),
                            0x0000,
                            tag.as_ptr().cast(),
                            msg.as_ptr().cast(),
                        )
                    };
                }
            }
        })
        .expect("Failed to start stdout/stderr to hilog forwarder thread")
}

// TODO: can't run but can compile
#[cfg(test)]
mod test {
    use crate as ohos_hilog_binding;
    use crate::Hilog;

    #[test]
    fn test_hilog() {
        let hilog = Hilog::new();
        hilog.debug("test debug");
        hilog.info("test info");
        hilog.warn("test warn");
    }

    #[test]
    fn test_hilog_macro() {
        hilog_debug!("test debug");
        hilog_info!("test info");
        hilog_warn!("test warn");
        hilog_error!("test error");
        hilog_fatal!("test fatal");

        hilog_debug!(format!("test debug {}", 1));
        hilog_info!(format!("test info {}", 1));
        hilog_warn!(format!("test warn {}", 1));
        hilog_error!(format!("test error {}", 1));
        hilog_fatal!(format!("test fatal {}", 1));

        let a = 1;
        hilog_debug!("test debug {a}");
        hilog_info!("test info {a}");
        hilog_warn!("test warn {a}");
        hilog_error!("test error {a}");
        hilog_fatal!("test fatal {a}");
    }
}
