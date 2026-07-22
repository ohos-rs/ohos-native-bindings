# ohos-transient-task-binding

This crate is a binding for the transient task module in OpenHarmony.

A transient task is the short background grace period an application asks for
when it is about to be suspended: it requests a delayed transition to the
suspended state, the system grants a duration of its own choosing and calls back
shortly before it runs out, and the request must be cancelled once the work is
done. This crate wraps `transient_task_api.h` with a safe layer — an RAII
request guard, `Result`-based error handling, and typed durations.

## Requirements

- **Feature `api-13`.** The whole native API is `@since 13`, so with default
  features this crate exposes nothing but the raw `sys` re-export. The
  application info query is `@since 20` and sits behind `api-20`.

## Install

```shell
cargo add ohos-transient-task-binding --features api-13
```

## Usage

```rust
use ohos_transient_task_binding::SuspendDelay;

// Runs on a system thread shortly before the delay expires; must not unwind.
unsafe extern "C" fn on_expired() {
    // signal the worker to wind down
}

let task = SuspendDelay::request("uploading pending records", on_expired)?;
println!("granted {:?} for request {}", task.actual_delay(), task.request_id());

// ... do the short piece of background work ...
let left = task.remaining_delay()?;

// Cancel explicitly to observe failures; dropping cancels silently.
task.cancel()?;
```

## Coverage

Behind `api-13`:

- Request lifetime: `SuspendDelay::request` → `cancel`, with `Drop` cancelling
  the request so it never outlives the guard.
- Granted values: `DelaySuspendInfo` (`request_id`, `actual_delay_time_ms`,
  `actual_delay`), reachable on the guard directly or through `info()`.
- Remaining time: `remaining_delay_time_ms` / `remaining_delay`.

Behind `api-20`:

- `transient_task_info()` → `TransientTaskInfo` (`remaining_quota`, `tasks`,
  `granted`), plus `MAX_TRANSIENT_TASKS`.

`describe` maps a raw `TransientTask_ErrorCode` to a short description.

## Notes

- The expiry callback is `void (*)(void)` — it carries no user-data pointer, so
  it is taken as an `extern "C" fn` rather than a closure, and any state it
  needs must be reached through a `static`. It runs on a system-owned thread and
  must not unwind.
- The system decides the granted duration; the C API has no way to ask for a
  particular one.
- `TransientTask_TransientTaskInfo` reports no task count, so slots the system
  did not fill keep the zeroed value passed in; `granted()` iterates only over
  slots that carry a request id.

The raw bindings are re-exported as `ohos_transient_task_binding::sys` for
anything not covered by the safe layer.

## Status

Compile-checked for all three OpenHarmony targets, but not verified on a device.

## License

MIT OR Apache-2.0
