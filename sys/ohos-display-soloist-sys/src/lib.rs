/* automatically generated by rust-bindgen 0.65.1 */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_DisplaySoloist {
    _unused: [u8; 0],
}
#[doc = " @brief Defines the native displaySoloist callback.\n\n @param timestamp Indicates the current timestamp.\n @param targetTimestamp Indicates the target timestamp.\n @param data Indicates the pointer to user data.\n @since 12\n @version 1.0"]
pub type OH_DisplaySoloist_FrameCallback = ::std::option::Option<
    unsafe extern "C" fn(
        timestamp: ::std::os::raw::c_longlong,
        targetTimestamp: ::std::os::raw::c_longlong,
        data: *mut ::std::os::raw::c_void,
    ),
>;
#[doc = " @brief Defines the expected frame rate range struct.\n\n @since 12\n @version 1.0"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DisplaySoloist_ExpectedRateRange {
    #[doc = " The minimum frame rate of dynamical callback rate range."]
    pub min: i32,
    #[doc = " The maximum frame rate of dynamical callback rate range."]
    pub max: i32,
    #[doc = " The expected frame rate of dynamical callback rate range."]
    pub expected: i32,
}
extern "C" {
    #[doc = " @brief Creates a <b>OH_DisplaySoloist</b> instance.\\n\n\n @param useExclusiveThread Indicates whether the vsync run in a exclusive thread.\n @return Returns the pointer to the <b>OH_DisplaySoloist</b> instance created if the execution is successful.\n if nullptr is returned, the creation fails.\n the possible cause of the failure is that the available memory is empty.\n @since 12\n @version 1.0"]
    pub fn OH_DisplaySoloist_Create(useExclusiveThread: bool) -> *mut OH_DisplaySoloist;
}
extern "C" {
    #[doc = " @brief Destroys a <b>OH_DisplaySoloist</b> instance and reclaims the memory occupied by the object.\n\n @param displaySoloist Indicates the pointer to a native displaySoloist.\n @return Returns int32_t, returns 0 if the execution is successful, returns -1 if displaySoloist is incorrect.\n @since 12\n @version 1.0"]
    pub fn OH_DisplaySoloist_Destroy(displaySoloist: *mut OH_DisplaySoloist) -> i32;
}
extern "C" {
    #[doc = " @brief Start to request next vsync with callback.\n\n @param displaySoloist Indicates the pointer to a native displaySoloist.\n @param callback Indicates the OH_DisplaySoloist_FrameCallback which will be called when next vsync coming.\n @param data Indicates data whick will be used in callback.\n @return Returns int32_t, returns 0 if the execution is successful.\n returns -1 if displaySoloist or callback is incorrect.\n @since 12\n @version 1.0"]
    pub fn OH_DisplaySoloist_Start(
        displaySoloist: *mut OH_DisplaySoloist,
        callback: OH_DisplaySoloist_FrameCallback,
        data: *mut ::std::os::raw::c_void,
    ) -> i32;
}
extern "C" {
    #[doc = " @brief Stop to request next vsync with callback.\n\n @param displaySoloist Indicates the pointer to a native displaySoloist.\n @return Returns int32_t, returns 0 if the execution is successful, returns -1 if displaySoloist is incorrect.\n @since 12\n @version 1.0"]
    pub fn OH_DisplaySoloist_Stop(displaySoloist: *mut OH_DisplaySoloist) -> i32;
}
extern "C" {
    #[doc = " @brief Set vsync expected frame rate range.\n\n @param displaySoloist Indicates the pointer to a native displaySoloist.\n @param range Indicates the pointer to an expected rate range.\n @return Returns int32_t, returns 0 if the execution is successful\n returns -1 if displaySoloist or range is incorrect.\n @since 12\n @version 1.0"]
    pub fn OH_DisplaySoloist_SetExpectedFrameRateRange(
        displaySoloist: *mut OH_DisplaySoloist,
        range: *mut DisplaySoloist_ExpectedRateRange,
    ) -> i32;
}
