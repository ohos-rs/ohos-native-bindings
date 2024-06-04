/* automatically generated by rust-bindgen 0.65.1 */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub const __GNUC_VA_LIST: u32 = 1;
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const LOG_DOMAIN: u32 = 0;
pub type va_list = [u64; 4usize];
pub type __gnuc_va_list = [u64; 4usize];
pub const LogType_LOG_APP: LogType = 0;
pub type LogType = ::std::os::raw::c_uint;
pub const LogLevel_LOG_DEBUG: LogLevel = 3;
pub const LogLevel_LOG_INFO: LogLevel = 4;
pub const LogLevel_LOG_WARN: LogLevel = 5;
pub const LogLevel_LOG_ERROR: LogLevel = 6;
pub const LogLevel_LOG_FATAL: LogLevel = 7;
pub type LogLevel = ::std::os::raw::c_uint;
extern "C" {
    pub fn OH_LOG_Print(
        type_: LogType,
        level: LogLevel,
        domain: ::std::os::raw::c_uint,
        tag: *const ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn OH_LOG_IsLoggable(
        domain: ::std::os::raw::c_uint,
        tag: *const ::std::os::raw::c_char,
        level: LogLevel,
    ) -> bool;
}
pub type LogCallback = ::std::option::Option<
    unsafe extern "C" fn(
        type_: LogType,
        level: LogLevel,
        domain: ::std::os::raw::c_uint,
        tag: *const ::std::os::raw::c_char,
        msg: *const ::std::os::raw::c_char,
    ),
>;
extern "C" {
    pub fn OH_LOG_SetCallback(callback: LogCallback);
}