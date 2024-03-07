use std::ffi::CString;
use sys::{LogLevel, LogType, OH_LOG_Print};

mod sys;

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
                    LogType::LogApp,
                    $level_enum,
                    domain,
                    tag.as_ptr(),
                    content.as_ptr(),
                );
            }
        }
    };
}

log_factory!(debug, LogLevel::LogDebug);
log_factory!(info, LogLevel::LogInfo);
log_factory!(warn, LogLevel::LogWarn);
log_factory!(error, LogLevel::LogError);
log_factory!(fatal, LogLevel::LogFatal);

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
