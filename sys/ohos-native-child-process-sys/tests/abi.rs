//! Compile-only checks: `cargo check --tests` requires no device or native linker.
use ohos_native_child_process_sys as sys;
use std::ffi::{c_char, c_int};
use std::mem::{align_of, size_of};

const _: unsafe extern "C" fn(*const c_char, sys::OH_Ability_OnNativeChildProcessStarted) -> c_int =
    sys::OH_Ability_CreateNativeChildProcess;
const _: sys::OH_Ability_OnNativeChildProcessStarted =
    None::<unsafe extern "C" fn(c_int, *mut sys::OHIPCRemoteProxy)>;
const _: () = {
    assert!(size_of::<sys::Ability_NativeChildProcess_ErrCode>() == 4);
    assert!(align_of::<sys::Ability_NativeChildProcess_ErrCode>() == 4);
    assert!(size_of::<sys::OH_Ability_OnNativeChildProcessStarted>() == size_of::<*const ()>());
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_NO_ERROR == 0);
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_INVALID_PARAM == 401);
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_NOT_SUPPORTED == 801);
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_INTERNAL == 16000050);
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_BUSY == 16010001);
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_TIMEOUT == 16010002);
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_SERVICE_ERROR == 16010003);
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_MULTI_PROCESS_DISABLED == 16010004);
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_ALREADY_IN_CHILD == 16010005);
    assert!(
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_MAX_CHILD_PROCESSES_REACHED == 16010006
    );
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_LIB_LOADING_FAILED == 16010007);
    assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_CONNECTION_FAILED == 16010008);
};

#[cfg(feature = "api-13")]
mod api13 {
    use super::{c_char, sys};
    use std::mem::{align_of, offset_of, size_of};

    const _: unsafe extern "C" fn(
        *const c_char,
        sys::NativeChildProcess_Args,
        sys::NativeChildProcess_Options,
        *mut i32,
    ) -> sys::Ability_NativeChildProcess_ErrCode = sys::OH_Ability_StartNativeChildProcess;

    const _: () = {
        let pointer = size_of::<*const ()>();
        assert!(size_of::<sys::NativeChildProcess_IsolationMode>() == 4);
        assert!(sys::NativeChildProcess_IsolationMode_NCP_ISOLATION_MODE_NORMAL == 0);
        assert!(sys::NativeChildProcess_IsolationMode_NCP_ISOLATION_MODE_ISOLATED == 1);
        assert!(size_of::<sys::NativeChildProcess_Fd>() == 3 * pointer);
        assert!(align_of::<sys::NativeChildProcess_Fd>() == pointer);
        assert!(offset_of!(sys::NativeChildProcess_Fd, fdName) == 0);
        assert!(offset_of!(sys::NativeChildProcess_Fd, fd) == pointer);
        assert!(offset_of!(sys::NativeChildProcess_Fd, next) == 2 * pointer);
        assert!(size_of::<sys::NativeChildProcess_FdList>() == pointer);
        assert!(align_of::<sys::NativeChildProcess_FdList>() == pointer);
        assert!(offset_of!(sys::NativeChildProcess_FdList, head) == 0);
        assert!(size_of::<sys::NativeChildProcess_Args>() == 2 * pointer);
        assert!(align_of::<sys::NativeChildProcess_Args>() == pointer);
        assert!(offset_of!(sys::NativeChildProcess_Args, entryParams) == 0);
        assert!(offset_of!(sys::NativeChildProcess_Args, fdList) == pointer);
        assert!(size_of::<sys::NativeChildProcess_Options>() == 16);
        assert!(align_of::<sys::NativeChildProcess_Options>() == 8);
        assert!(offset_of!(sys::NativeChildProcess_Options, isolationMode) == 0);
        assert!(offset_of!(sys::NativeChildProcess_Options, reserved) == 8);
    };

    // Field mutability and integer widths are part of the SDK's C ABI.
    const _: fn(sys::NativeChildProcess_Fd) -> (*mut c_char, i32, *mut sys::NativeChildProcess_Fd) =
        |node| (node.fdName, node.fd, node.next);
    const _: fn(sys::NativeChildProcess_Args) -> (*mut c_char, sys::NativeChildProcess_FdList) =
        |args| (args.entryParams, args.fdList);
    const _: fn(sys::NativeChildProcess_Options) -> (u32, i64) =
        |options| (options.isolationMode, options.reserved);
}

#[cfg(feature = "api-17")]
const _: unsafe extern "C" fn() -> *mut sys::NativeChildProcess_Args =
    sys::OH_Ability_GetCurrentChildProcessArgs;

#[cfg(feature = "api-20")]
mod api20 {
    use super::{c_char, sys};

    const _: unsafe extern "C" fn() -> *mut sys::Ability_ChildProcessConfigs =
        sys::OH_Ability_CreateChildProcessConfigs;
    const _: unsafe extern "C" fn(
        *mut sys::Ability_ChildProcessConfigs,
    ) -> sys::Ability_NativeChildProcess_ErrCode = sys::OH_Ability_DestroyChildProcessConfigs;
    const _: unsafe extern "C" fn(
        *mut sys::Ability_ChildProcessConfigs,
        sys::NativeChildProcess_IsolationMode,
    ) -> sys::Ability_NativeChildProcess_ErrCode =
        sys::OH_Ability_ChildProcessConfigs_SetIsolationMode;
    const _: unsafe extern "C" fn(
        *mut sys::Ability_ChildProcessConfigs,
        *const c_char,
    ) -> sys::Ability_NativeChildProcess_ErrCode =
        sys::OH_Ability_ChildProcessConfigs_SetProcessName;
    const _: unsafe extern "C" fn(
        *const c_char,
        *mut sys::Ability_ChildProcessConfigs,
        sys::OH_Ability_OnNativeChildProcessStarted,
    ) -> sys::Ability_NativeChildProcess_ErrCode =
        sys::OH_Ability_CreateNativeChildProcessWithConfigs;
    const _: unsafe extern "C" fn(
        *const c_char,
        sys::NativeChildProcess_Args,
        *mut sys::Ability_ChildProcessConfigs,
        *mut i32,
    ) -> sys::Ability_NativeChildProcess_ErrCode =
        sys::OH_Ability_StartNativeChildProcessWithConfigs;
    const _: sys::OH_Ability_OnNativeChildProcessExit = None::<unsafe extern "C" fn(i32, i32)>;
    const _: unsafe extern "C" fn(
        sys::OH_Ability_OnNativeChildProcessExit,
    ) -> sys::Ability_NativeChildProcess_ErrCode =
        sys::OH_Ability_RegisterNativeChildProcessExitCallback;
    const _: unsafe extern "C" fn(
        sys::OH_Ability_OnNativeChildProcessExit,
    ) -> sys::Ability_NativeChildProcess_ErrCode =
        sys::OH_Ability_UnregisterNativeChildProcessExitCallback;
    const _: () =
        assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_CALLBACK_NOT_EXIST == 16010009);
}

#[cfg(feature = "api-21")]
const _: unsafe extern "C" fn(
    *mut sys::Ability_ChildProcessConfigs,
    bool,
) -> sys::Ability_NativeChildProcess_ErrCode = sys::OH_Ability_ChildProcessConfigs_SetIsolationUid;

#[cfg(feature = "api-22")]
const _: unsafe extern "C" fn(i32) -> sys::Ability_NativeChildProcess_ErrCode =
    sys::OH_Ability_KillChildProcess;
#[cfg(feature = "api-22")]
const _: () = assert!(sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_INVALID_PID == 16010010);

#[cfg(feature = "api-26")]
const _: unsafe extern "C" fn() -> bool = sys::OH_Ability_IsNativeChildProcessSupported;
