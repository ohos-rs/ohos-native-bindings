/* automatically generated by rust-bindgen 0.65.1 */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_DisplaySoloist {
    _unused: [u8; 0],
}
pub type OH_DisplaySoloist_FrameCallback = ::std::option::Option<
    unsafe extern "C" fn(
        timestamp: ::std::os::raw::c_longlong,
        targetTimestamp: ::std::os::raw::c_longlong,
        data: *mut ::std::os::raw::c_void,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DisplaySoloist_ExpectedRateRange {
    pub min: i32,
    pub max: i32,
    pub expected: i32,
}
extern "C" {
    pub fn OH_DisplaySoloist_Create(useExclusiveThread: bool) -> *mut OH_DisplaySoloist;
}
extern "C" {
    pub fn OH_DisplaySoloist_Destroy(displaySoloist: *mut OH_DisplaySoloist) -> i32;
}
extern "C" {
    pub fn OH_DisplaySoloist_Start(
        displaySoloist: *mut OH_DisplaySoloist,
        callback: OH_DisplaySoloist_FrameCallback,
        data: *mut ::std::os::raw::c_void,
    ) -> i32;
}
extern "C" {
    pub fn OH_DisplaySoloist_Stop(displaySoloist: *mut OH_DisplaySoloist) -> i32;
}
extern "C" {
    pub fn OH_DisplaySoloist_SetExpectedFrameRateRange(
        displaySoloist: *mut OH_DisplaySoloist,
        range: *mut DisplaySoloist_ExpectedRateRange,
    ) -> i32;
}
