# ohos-native-child-process-binding

Rust bindings for `AbilityKit/native_child_process.h`, linked to `libchild_process.so`.
Enable the API feature matching the application's minimum supported OHOS runtime.

## APIs

| Rust API | Native API | Since |
| --- | --- | --- |
| `NativeChildProcess::create` | `OH_Ability_CreateNativeChildProcess` | 12 |
| `NativeChildProcess::start` | `OH_Ability_StartNativeChildProcess` | 13 |
| `NativeChildProcess::current_args` | `OH_Ability_GetCurrentChildProcessArgs` | 17 |
| `ChildProcessConfigs::new` / Drop | Create / DestroyChildProcessConfigs | 20 |
| `ChildProcessConfigs::set_isolation_mode` | ChildProcessConfigs_SetIsolationMode | 20 |
| `ChildProcessConfigs::set_process_name` | ChildProcessConfigs_SetProcessName | 20 |
| `NativeChildProcess::start_with_configs` | StartNativeChildProcessWithConfigs | 20 |
| `NativeChildProcess::create_with_configs` | CreateNativeChildProcessWithConfigs | 20 |
| `NativeChildProcess::register_exit_callback` | RegisterNativeChildProcessExitCallback | 20 |
| `NativeChildProcess::unregister_exit_callback` | UnregisterNativeChildProcessExitCallback | 20 |
| `ChildProcessConfigs::set_isolation_uid` | ChildProcessConfigs_SetIsolationUid | 21 |
| `NativeChildProcess::kill` | OH_Ability_KillChildProcess | 22 |
| `NativeChildProcess::is_supported` | OH_Ability_IsNativeChildProcessSupported | 26 |

The complete generated API is available through `sys`.

## Process scenarios

This crate implements extended Native child processes (扩展子进程). `start` and
`start_with_configs` load a shared-library entry and transfer arguments/FDs;
`create` and `create_with_configs` establish an IPC channel. A child exits when
its entry returns and follows its parent's lifetime.

Independent processes (独立进程) host application components. They are selected
by UIAbility `process`, `isolationProcess` with `AbilityStage.onNewProcessRequest`,
or HAP-level `isolationMode`. Their tests must launch components and observe
component lifecycle, process allocation and reuse through the component APIs.

`IsolationMode::Normal` / `Isolated` here control a Native child's data sandbox
and network sharing. `set_isolation_uid` controls UID isolation in isolated mode.
Neither selects the independent component-process scenario. See the official
[Native child guide](https://github.com/openharmony/docs/blob/master/zh-cn/application-dev/application-models/capi-nativechildprocess-development-guideline.md)
and [independent-process guide](https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/isolation-process-development-guideline).

## Start a child entry

```rust,no_run
use std::{os::fd::AsFd, os::unix::net::UnixStream};
use ohos_native_child_process_binding::{
    ChildProcessArgs, ChildProcessOptions, NativeChildProcess,
};

let (parent, child) = UnixStream::pair()?;
let mut args = ChildProcessArgs::new();
args.set_entry_params("protocol-v1")?.add_fd("control", child.as_fd())?;
let pid = NativeChildProcess::start("libchild.so:Main", &args, ChildProcessOptions::default())?;
drop(args);
drop(child);
# Ok::<(), Box<dyn std::error::Error>>(())
```

`entry` is the official `library:function` string, not a filesystem path.
Native validates the library and symbol. Rust rejects interior NUL bytes before
FFI; it adds no library-prefix, symbol-identifier or entry-length restrictions.
Launch arguments own strings and borrow parent descriptors. The C list is built
for the synchronous call, which must stay off the application's UI thread.
The documented FD list limit is 16; other parameter validation remains native.

## Child entry

```rust,no_run
use ohos_native_child_process_binding::{ChildProcessArgsRef, native_child_entry};

fn child_main(args: ChildProcessArgsRef<'_>) {
    for descriptor in args.fds() {
        // descriptor.name is &CStr; descriptor.fd is BorrowedFd.
        // Clone it to create an independent Rust FD owner when needed.
    }
}
native_child_entry!(Main, child_main);
```

Argument views borrow native storage and descriptors; they never free or close
native resources. The macro contains handler failures and panics and returns to
AbilityKit; returning ends the child. Keep work alive inside the entry.
`current_args` returns an optional system-owned pointer. Creating an argument
view from it is unsafe: uphold the documented storage and FD lifetime contract.

## Configs, callbacks and IPC

`ChildProcessConfigs` owns one opaque native object and destroys it on Drop.
Its setters return native validation errors. UID isolation takes effect only
with isolated mode.

Exit callbacks use the official `extern "C" fn(pid, signal)` signature. Register
and unregister the same function explicitly. Registration is process-wide;
launching does not change it. Callbacks execute on a native thread, must keep work
short and must not unwind. `kill(pid)` performs the official explicit termination
request; success does not mean the exit callback has already arrived.

IPC creation is asynchronous. `create` success acknowledges the request, while
the startup callback reports the actual result. Both creation methods are unsafe:
the callback must synchronize its state, contain panics and release received
remote proxies through IPCKit. The library must implement the official
`NativeChildProcess_OnConnect` and `NativeChildProcess_MainProc` exports.

Native failures return `NativeChildProcessError::InternalError(u32)` containing
the original code. String conversion, FD-list capacity and null configs allocation
have separate Rust errors.

[Official API reference](https://github.com/openharmony/docs/blob/master/zh-cn/application-dev/reference/apis-ability-kit/capi-native-child-process-h.md)

## Device E2E

The [example suite](../../examples/native_child_process/README.md#e2e-scenarios)
exercises every public binding operation on the full 2in1 QEMU image, including
separate FD and IPC fixtures, sandbox/UID checks, callback lifetimes, argument
boundaries, native entry errors/panics and ArkTS termination interoperability.
The default E2E build enables `api-26`. Known image failures are listed in the
example documentation.
