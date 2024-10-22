use ohos_hilogs_sys::OH_LOG_Print;
use std::ffi::CString;

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

#[derive(Default)]
pub struct LogOptions<'a> {
    // 打印的domain 一般直接使用 0x0000
    pub domain: Option<u32>,
    /// 打印的标签 默认为 hilog_rs
    pub tag: Option<&'a str>,
}

macro_rules! log_factory {
    ($level: ident,$level_enum: expr) => {
        pub fn $level<T: AsRef<str>>(info: T, options: Option<LogOptions>) {
            let option_result = options.unwrap_or(LogOptions {
                tag: None,
                domain: None,
            });

            let tag_result = option_result.tag.unwrap_or("hilog_rs");
            let domain = option_result.domain.unwrap_or(0x0000);

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
    ($info: expr) => {
        ohos_hilog_binding::debug($info, None);
    };
    ($info: expr,$option: expr) => {
        ohos_hilog_binding::debug($info, Some($option));
    };
}

#[macro_export]
macro_rules! hilog_info {
    ($info: expr) => {
        ohos_hilog_binding::info($info, None);
    };
    ($info: expr,$option: expr) => {
        ohos_hilog_binding::info($info, Some($option));
    };
}

#[macro_export]
macro_rules! hilog_warn {
    ($info: expr) => {
        ohos_hilog_binding::warn($info, None);
    };
    ($info: expr,$option: expr) => {
        ohos_hilog_binding::warn($info, Some($option));
    };
}

#[macro_export]
macro_rules! hilog_error {
    ($info: expr) => {
        ohos_hilog_binding::error($info, None);
    };
    ($info: expr,$option: expr) => {
        ohos_hilog_binding::error($info, Some($option));
    };
}

#[macro_export]
macro_rules! hilog_fatal {
    ($info: expr) => {
        ohos_hilog_binding::fatal($info, None);
    };
    ($info: expr,$option: expr) => {
        ohos_hilog_binding::fatal($info, Some($option));
    };
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
                        error("Hilog forwarder failed to read stdin/stderr: {e:?}", None);
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
