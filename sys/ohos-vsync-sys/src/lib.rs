/* automatically generated by rust-bindgen 0.65.1 */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub const __BYTE_ORDER: u32 = 1234;
pub const __LONG_MAX: u64 = 9223372036854775807;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __USE_TIME_BITS64: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST64_MAX: i32 = -1;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INTMAX_MIN: i64 = -9223372036854775808;
pub const INTMAX_MAX: u64 = 9223372036854775807;
pub const UINTMAX_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const INT_FAST16_MIN: i32 = -2147483648;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST16_MAX: u32 = 2147483647;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const UINT_FAST16_MAX: u32 = 4294967295;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIZE_MAX: i32 = -1;
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
pub type int_fast8_t = i8;
pub type int_fast64_t = i64;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast16_t = i32;
pub type int_fast32_t = i32;
pub type uint_fast16_t = u32;
pub type uint_fast32_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeVSync {
    _unused: [u8; 0],
}
pub type OH_NativeVSync_FrameCallback = ::std::option::Option<
    unsafe extern "C" fn(timestamp: ::std::os::raw::c_longlong, data: *mut ::std::os::raw::c_void),
>;
extern "C" {
    #[doc = " @brief Creates a <b>NativeVsync</b> instance.\\n\n A new <b>NativeVsync</b> instance is created each time this function is called.\n\n @syscap SystemCapability.Graphic.Graphic2D.NativeVsync\n @param name Indicates the vsync connection name.\n @param length Indicates the name's length.\n @return Returns the pointer to the <b>NativeVsync</b> instance created.\n @since 9\n @version 1.0"]
    pub fn OH_NativeVSync_Create(
        name: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_uint,
    ) -> *mut OH_NativeVSync;
}
extern "C" {
    #[doc = " @brief Delete the NativeVsync instance.\n\n @syscap SystemCapability.Graphic.Graphic2D.NativeVsync\n @param nativeVsync Indicates the pointer to a <b>NativeVsync</b> instance.\n @since 9\n @version 1.0"]
    pub fn OH_NativeVSync_Destroy(nativeVsync: *mut OH_NativeVSync);
}
extern "C" {
    #[doc = " @brief Creates a <b>NativeVsync</b> instance.\\n\n A new <b>NativeVsync</b> instance is created each time this function is called.\n\n @syscap SystemCapability.Graphic.Graphic2D.NativeVsync\n @param windowID Indicates the id of the associated window.\n @param name Indicates the vsync connection name.\n @param length Indicates the name's length.\n @return Returns the pointer to the <b>NativeVsync</b> instance created.\n @since 14\n @version 1.0"]
    pub fn OH_NativeVSync_Create_ForAssociatedWindow(
        windowID: u64,
        name: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_uint,
    ) -> *mut OH_NativeVSync;
}
extern "C" {
    #[doc = " @brief Request next vsync with callback.\n If you call this interface multiple times in one frame, it will only call the last callback.\n\n @syscap SystemCapability.Graphic.Graphic2D.NativeVsync\n @param nativeVsync Indicates the pointer to a NativeVsync.\n @param callback Indicates the OH_NativeVSync_FrameCallback which will be called when next vsync coming.\n @param data Indicates data which will be used in callback.\n @return {@link NATIVE_ERROR_OK} 0 - Success.\n     {@link NATIVE_ERROR_INVALID_ARGUMENTS} 40001000 - the parameter nativeVsync is NULL or callback is NULL.\n     {@link NATIVE_ERROR_BINDER_ERROR} 50401000 - ipc send failed.\n @since 9\n @version 1.0"]
    pub fn OH_NativeVSync_RequestFrame(
        nativeVsync: *mut OH_NativeVSync,
        callback: OH_NativeVSync_FrameCallback,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " @brief Request next vsync with callback.\n If this function is called multiple times in one vsync period, all these callbacks and dataset will be called.\n\n @syscap SystemCapability.Graphic.Graphic2D.NativeVsync\n @param nativeVsync Indicates the pointer to a NativeVsync.\n @param callback Indicates the OH_NativeVSync_FrameCallback which will be called when next vsync coming.\n @param data Indicates data which will be used in callback.\n @return {@link NATIVE_ERROR_OK} 0 - Success.\n     {@link NATIVE_ERROR_INVALID_ARGUMENTS} 40001000 - the parameter nativeVsync is NULL or callback is NULL.\n     {@link NATIVE_ERROR_BINDER_ERROR} 50401000 - ipc send failed.\n @since 12\n @version 1.0"]
    pub fn OH_NativeVSync_RequestFrameWithMultiCallback(
        nativeVsync: *mut OH_NativeVSync,
        callback: OH_NativeVSync_FrameCallback,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " @brief Get vsync period.\n\n @syscap SystemCapability.Graphic.Graphic2D.NativeVsync\n @param nativeVsync Indicates the pointer to a NativeVsync.\n @param period Indicates the vsync period.\n @return Returns int32_t, return value == 0, success, otherwise, failed.\n @since 10\n @version 1.0"]
    pub fn OH_NativeVSync_GetPeriod(
        nativeVsync: *mut OH_NativeVSync,
        period: *mut ::std::os::raw::c_longlong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " @brief Enables DVSync to improve the smoothness of self-drawing animations.\n DVSync, short for Decoupled VSync, is a frame timing management policy that is decoupled from the hardware's VSync.\n DVSync drives the early rendering of upcoming animation frames by sending VSync signals with future timestamps.\n These frames are stored in a frame buffer queue. This helps DVSync reduce potential frame drop and therefore\n enhances the smoothness of animations.\n DVSync requires free self-drawing frame buffers to store these pre-rendered animation frames.\n Therefore, you must ensure that at least one free frame buffer is available. Otherwise, do not enable DVSync.\n After DVSync is enabled, you must correctly respond to the early VSync signals and request the subsequent VSync\n after the animation frame associated with the previous VSync is complete. In addition, the self-drawing frames must\n carry timestamps that align with VSync.\n After the animation ends, disable DVSync.\n Only phones and tablets support DVSync.\n On a platform that does not support DVSync or if another application has enabled DVSync, the attempt to enable it\n will not take effect, and the application still receives normal VSync signals.\n\n @syscap SystemCapability.Graphic.Graphic2D.NativeVsync\n @param nativeVsync Indicates the pointer to a NativeVsync.\n @param enable Whether to enable DVSync.The value true means to enable DVSync, and false means the opposite.\n @return {@link NATIVE_ERROR_OK} 0 - Success.\n     {@link NATIVE_ERROR_INVALID_ARGUMENTS} 40001000 - the parameter nativeVsync is NULL.\n     {@link NATIVE_ERROR_BINDER_ERROR} 50401000 - ipc send failed.\n @since 14\n @version 1.0"]
    pub fn OH_NativeVSync_DVSyncSwitch(
        nativeVsync: *mut OH_NativeVSync,
        enable: bool,
    ) -> ::std::os::raw::c_int;
}
