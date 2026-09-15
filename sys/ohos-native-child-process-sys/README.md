# ohos-native-child-process-sys

Raw Rust bindings for `AbilityKit/native_child_process.h`, generated with
bindgen 0.65.1 from DevEco Studio 26.0.0.821's OpenHarmony Native SDK
26.0.0.105. Links `libchild_process.so`.

## Install and availability

```sh
cargo add ohos-native-child-process-sys --features api-22
```

Default features are empty (API12 baseline). Features `api-13` through `api-26`
are chained; select the minimum runtime API, not merely the compilation SDK.
`api-22` does not expose `OH_Ability_IsNativeChildProcessSupported` (API26).

| API | Raw surface |
| --- | --- |
| 12 | Error codes, `OHIPCRemoteProxy`, startup callback and Binder create family |
| 13 | Isolation enum, named FD list, by-value args/options, extended Native start |
| 17 | Current child arguments |
| 20 | Opaque configs, create/destroy, mode/name setters, configured create/start, exit callbacks, callback-not-exist error |
| 21 | Config UID-isolation flag |
| 22 | Explicit child termination, invalid-PID error |
| 26 | Native child-process support query returning C `bool` |

Enum constants follow bindgen's names, such as
`NativeChildProcess_IsolationMode_NCP_ISOLATION_MODE_NORMAL`. Both SDK isolation
values are exposed faithfully; this is not an independent-process configuration
API. No application policy is encoded here.

## Safety boundary

All platform calls are unsafe. Callers own string/list lifetimes, descriptor
ownership, callback synchronization, panic containment and config destruction.
Rust pointers or allocator-owned values must never be transferred as process
payloads. The entry ABI for the extended start family is
`unsafe extern "C" fn(NativeChildProcess_Args)` with args passed **by value**;
start loads a shared library and exported symbol, not an executable.

API12 Binder creation remains raw/unsafe. The recursively generated proxy type
is opaque; this crate does not implement an IPCKit facade, proxy release or a
safe IPC contract. SDK comments describe IPC ownership and thread requirements.
Successful compile checks are not device/lifecycle acceptance.

## Regeneration and ABI checks

Use the [single-config generator](../../tools/generate/README.md). The generated
`src/lib.rs` is not handwritten; assertions live separately in `tests/abi.rs`.

```sh
cargo check -p ohos-native-child-process-sys \
  --target aarch64-unknown-linux-ohos --all-features --tests
"$OHOS_NDK_HOME/native/llvm/bin/clang++" --target=aarch64-linux-ohos \
  --sysroot="$OHOS_NDK_HOME/native/sysroot" -std=c++17 -Werror -fsyntax-only \
  sys/ohos-native-child-process-sys/tests/sdk_abi.cpp
```

Run the C++ command from the workspace root. It only compiles SDK layout and
signature assertions; it produces no process shim or runtime library. Rust
`cargo check --tests` verifies signatures/layouts without a device or native
linker. On-device execution, callback ordering and FD transfer remain pending.

## License

MIT OR Apache-2.0. SDK declaration comments retain their upstream provenance.
