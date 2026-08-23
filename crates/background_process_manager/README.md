# ohos-background-process-manager-binding

This crate is a binding for the background process manager module in
OpenHarmony.

The background process manager lets an application hint the resource schedule
service about how much work one of its processes still has to do once that
process is no longer in the foreground. Lowering the priority of an idle process
frees resources for the rest of the system; the hint is dropped again by
resetting the priority. This crate wraps the native
`background_process_manager.h` C API with a safe layer.

## Install

```shell
cargo add ohos-background-process-manager-binding --features api-17
```

## Usage

```rust
use ohos_background_process_manager_binding as bpm;
use ohos_background_process_manager_binding::ProcessPriority;

// A worker process that has finished its job may be deprioritized.
bpm::set_process_priority(worker_pid, ProcessPriority::Background)?;

// Restore the default scheduling once it has work again.
bpm::reset_process_priority(worker_pid)?;
```

## Coverage

The native API was introduced in API 17, so the whole safe layer sits behind the
`api-17` feature.

| Item | Native counterpart |
|---|---|
| `set_process_priority` | `OH_BackgroundProcessManager_SetProcessPriority` |
| `reset_process_priority` | `OH_BackgroundProcessManager_ResetProcessPriority` |
| `set_current_process_priority` | same, for `std::process::id()` |
| `reset_current_process_priority` | same, for `std::process::id()` |
| `ProcessPriority` | `BackgroundProcessManager_ProcessPriority` |
| `BackgroundProcessManagerError`, `describe` | `BackgroundProcessManager_ErrorCode` |

## Notes

- `pid` is the calling process itself or a child process it owns; the priority
  hint applies while that process runs in the background.
- `ProcessPriority::Background` means the process has stopped working,
  `ProcessPriority::Inactive` means it is still working in the background.
- The native calls are synchronous and carry no handle, so there is nothing to
  release; a priority hint is undone with `reset_process_priority`.
- An out-of-range priority is reported as
  `BackgroundProcessManagerError::InvalidParam`; a failure to reach the resource
  schedule service as `BackgroundProcessManagerError::RemoteError`.

## License

MIT OR Apache-2.0
