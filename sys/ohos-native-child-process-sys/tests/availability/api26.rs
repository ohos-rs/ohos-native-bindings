// Compile-fail under api-22; compile-pass with api-26. No runtime call/link.
use ohos_native_child_process_sys as sys;

const _: unsafe extern "C" fn() -> bool = sys::OH_Ability_IsNativeChildProcessSupported;
