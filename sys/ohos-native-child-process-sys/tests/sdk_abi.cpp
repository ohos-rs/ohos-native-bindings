// Compile-only SDK assertions; this is not a production process shim.
#include <AbilityKit/native_child_process.h>
#include <stddef.h>

#define ABI_FUNCTION(name, ...) static_assert(__is_same(decltype(&name), __VA_ARGS__), #name)

static_assert(sizeof(Ability_NativeChildProcess_ErrCode) == 4);
static_assert(__is_same(__underlying_type(Ability_NativeChildProcess_ErrCode), unsigned int));
static_assert(sizeof(NativeChildProcess_IsolationMode) == 4);
static_assert(__is_same(__underlying_type(NativeChildProcess_IsolationMode), unsigned int));
static_assert(sizeof(NativeChildProcess_Fd) == 3 * sizeof(void*));
static_assert(alignof(NativeChildProcess_Fd) == sizeof(void*));
static_assert(offsetof(NativeChildProcess_Fd, fdName) == 0);
static_assert(offsetof(NativeChildProcess_Fd, fd) == sizeof(void*));
static_assert(offsetof(NativeChildProcess_Fd, next) == 2 * sizeof(void*));
static_assert(sizeof(NativeChildProcess_FdList) == sizeof(void*));
static_assert(offsetof(NativeChildProcess_FdList, head) == 0);
static_assert(sizeof(NativeChildProcess_Args) == 2 * sizeof(void*));
static_assert(alignof(NativeChildProcess_Args) == sizeof(void*));
static_assert(offsetof(NativeChildProcess_Args, entryParams) == 0);
static_assert(offsetof(NativeChildProcess_Args, fdList) == sizeof(void*));
static_assert(sizeof(NativeChildProcess_Options) == 16);
static_assert(alignof(NativeChildProcess_Options) == 8);
static_assert(offsetof(NativeChildProcess_Options, isolationMode) == 0);
static_assert(offsetof(NativeChildProcess_Options, reserved) == 8);
static_assert(__is_same(decltype(NativeChildProcess_Fd::fdName), char*));
static_assert(__is_same(decltype(NativeChildProcess_Fd::fd), int32_t));
static_assert(__is_same(decltype(NativeChildProcess_Options::reserved), int64_t));

ABI_FUNCTION(OH_Ability_CreateNativeChildProcess, int (*)(const char*, OH_Ability_OnNativeChildProcessStarted));
static_assert(__is_same(OH_Ability_OnNativeChildProcessStarted, void (*)(int, OHIPCRemoteProxy*)));
ABI_FUNCTION(OH_Ability_StartNativeChildProcess, Ability_NativeChildProcess_ErrCode (*)(const char*, NativeChildProcess_Args, NativeChildProcess_Options, int32_t*));
ABI_FUNCTION(OH_Ability_GetCurrentChildProcessArgs, NativeChildProcess_Args* (*)());
ABI_FUNCTION(OH_Ability_CreateChildProcessConfigs, Ability_ChildProcessConfigs* (*)());
ABI_FUNCTION(OH_Ability_DestroyChildProcessConfigs, Ability_NativeChildProcess_ErrCode (*)(Ability_ChildProcessConfigs*));
ABI_FUNCTION(OH_Ability_ChildProcessConfigs_SetIsolationMode, Ability_NativeChildProcess_ErrCode (*)(Ability_ChildProcessConfigs*, NativeChildProcess_IsolationMode));
ABI_FUNCTION(OH_Ability_ChildProcessConfigs_SetIsolationUid, Ability_NativeChildProcess_ErrCode (*)(Ability_ChildProcessConfigs*, bool));
ABI_FUNCTION(OH_Ability_ChildProcessConfigs_SetProcessName, Ability_NativeChildProcess_ErrCode (*)(Ability_ChildProcessConfigs*, const char*));
ABI_FUNCTION(OH_Ability_CreateNativeChildProcessWithConfigs, Ability_NativeChildProcess_ErrCode (*)(const char*, Ability_ChildProcessConfigs*, OH_Ability_OnNativeChildProcessStarted));
ABI_FUNCTION(OH_Ability_StartNativeChildProcessWithConfigs, Ability_NativeChildProcess_ErrCode (*)(const char*, NativeChildProcess_Args, Ability_ChildProcessConfigs*, int32_t*));
static_assert(__is_same(OH_Ability_OnNativeChildProcessExit, void (*)(int32_t, int32_t)));
ABI_FUNCTION(OH_Ability_RegisterNativeChildProcessExitCallback, Ability_NativeChildProcess_ErrCode (*)(OH_Ability_OnNativeChildProcessExit));
ABI_FUNCTION(OH_Ability_UnregisterNativeChildProcessExitCallback, Ability_NativeChildProcess_ErrCode (*)(OH_Ability_OnNativeChildProcessExit));
ABI_FUNCTION(OH_Ability_KillChildProcess, Ability_NativeChildProcess_ErrCode (*)(int32_t));
ABI_FUNCTION(OH_Ability_IsNativeChildProcessSupported, bool (*)());
