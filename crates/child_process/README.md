# ohos-child-process-binding

This crate is a binding for the native child process module in OpenHarmony.

An application can start a child process that loads one of its own dynamic
libraries and runs an exported entry function there. The parent hands over a
parameter string and a set of named file descriptors, chooses how far the child
is isolated from it, and is notified when the child exits. This crate wraps the
native `native_child_process.h` C API with a safe layer.

## Install

```shell
cargo add ohos-child-process-binding --features api-20
```

## Usage

```rust
use ohos_child_process_binding as child_process;
use child_process::{ChildProcessArgs, ChildProcessConfigs, IsolationMode};

// Report every child process that exits. The registration lasts as long as the
// guard is held.
unsafe extern "C" fn on_exit(pid: i32, signal: i32) {
    println!("child {pid} exited with signal {signal}");
}
let _exit_callback = child_process::register_exit_callback(on_exit)?;

let args = ChildProcessArgs::new()
    .with_entry_params("--mode=worker")?
    .with_fd("log", 3)?;

let configs = ChildProcessConfigs::new()?
    .with_isolation_mode(IsolationMode::Isolated)?
    .with_process_name("worker")?;

// "libEntry.so:Main" is the library and the entry function to run.
let pid = child_process::start_child_process_with_configs("libEntry.so:Main", &args, &configs)?;
println!("started child process {pid}");
```

## Coverage

The whole safe layer sits behind `api-*` features; without any feature the crate
exposes only the raw bindings, re-exported as `sys`.

| Feature | Adds |
|---|---|
| `api-13` | `start_child_process`, `ChildProcessArgs`, `ChildProcessOptions`, `IsolationMode` |
| `api-17` | `current_child_process_args` |
| `api-20` | `ChildProcessConfigs` (isolation mode, process name), `start_child_process_with_configs`, `register_exit_callback` (returns an `ExitCallbackRegistration` guard), `unregister_exit_callback` |
| `api-21` | `ChildProcessConfigs::set_isolation_uid` |
| `api-22` | `kill_child_process` |

## Notes

- `ChildProcessConfigs` owns the native configs object: it is created by
  `ChildProcessConfigs::new` and destroyed on drop. Every setter has a
  builder-style `with_*` counterpart.
- The process name must be non-empty, at most 64 characters and made of
  letters, digits or underscores; the process is finally named
  `{bundleName}:{name}`. The UID isolation flag only takes effect together with
  `IsolationMode::Isolated`.
- `ChildProcessArgs` owns its strings and validates them on insertion, so a
  string with an interior NUL byte returns `ChildProcessError::InteriorNul`
  instead of panicking. File descriptors stay owned by the caller and must be
  kept open until the child process has been started.
- `current_child_process_args` copies the runtime-owned arguments out, so the
  returned value is independent of the native buffers.
- The exit callback carries no user-data pointer, so it is taken as an
  `extern "C" fn` rather than a closure.
- The API 12 callback flow (`OH_Ability_CreateNativeChildProcess` and its API 20
  configs variant) hands an `OHIPCRemoteProxy` to a C callback. That object
  belongs to the IPC module and cannot be owned safely from this crate, so the
  callback flow is reachable through `child_process::sys` only.

## License

MIT OR Apache-2.0
