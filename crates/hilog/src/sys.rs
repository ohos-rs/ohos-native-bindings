use std::ffi::c_char;

/// 打印日志类型 第三方应用仅有此类型
#[repr(C)]
pub enum LogType {
    LogApp,
}

/// 日志等级
#[repr(C)]
pub enum LogLevel {
    LogDebug = 3,
    LogInfo,
    LogWarn,
    LogError,
    LogFatal,
}

extern "C" {
    // 接受format之后的&str参数
    pub fn OH_LOG_Print(
        type_: LogType,
        level: LogLevel,
        domain: u32,
        tag: *const c_char,
        fmt: *const c_char,
    );
}
