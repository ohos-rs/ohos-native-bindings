use ohos_hilogs_sys::OH_LOG_Print;
use std::ffi::CString;

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
        hilog_binding::debug($info, None);
    };
    ($info: expr,$option: expr) => {
        hilog_binding::debug($info, Some($option));
    };
}

#[macro_export]
macro_rules! hilog_info {
    ($info: expr) => {
        hilog_binding::info($info, None);
    };
    ($info: expr,$option: expr) => {
        hilog_binding::info($info, Some($option));
    };
}

#[macro_export]
macro_rules! hilog_warn {
    ($info: expr) => {
        hilog_binding::warn($info, None);
    };
    ($info: expr,$option: expr) => {
        hilog_binding::warn($info, Some($option));
    };
}

#[macro_export]
macro_rules! hilog_error {
    ($info: expr) => {
        hilog_binding::error($info, None);
    };
    ($info: expr,$option: expr) => {
        hilog_binding::error($info, Some($option));
    };
}

#[macro_export]
macro_rules! hilog_fatal {
    ($info: expr) => {
        hilog_binding::fatal($info, None);
    };
    ($info: expr,$option: expr) => {
        hilog_binding::fatal($info, Some($option));
    };
}
