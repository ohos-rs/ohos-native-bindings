# ohos-hitrace-binding

This crate is a binding for the HiTrace module in OpenHarmony.

The native `hitrace/trace.h` API covers two unrelated things, and this crate
keeps them apart: **HiTraceMeter** (trace slices and counters read by the system
trace tooling) and **HiTraceChain** (a chain id propagated across threads,
processes and devices).

## Install

```shell
cargo add ohos-hitrace-binding
```

## Usage

```rust
use ohos_hitrace_binding as hitrace;

let _chain = hitrace::begin_chain("sync_photos", hitrace::TraceFlags::INCLUDE_ASYNC)?;

{
    let _slice = hitrace::start_trace("read_index")?;
    // ... traced work ...
}

let upload = hitrace::start_async_trace("upload", 1)?;
hitrace::count_trace("pending_uploads", 1)?;
drop(upload);
```

The raw bindings are re-exported as `hitrace::sys` for anything not yet covered
by the safe layer.

## Coverage

- Trace slices: `start_trace` / `start_async_trace` return guards that close the
  slice on drop; `count_trace` for integer counters.
- Output levels (`api-19`): `start_trace_ex`, `start_async_trace_ex`,
  `count_trace_ex`, `is_trace_enabled`.
- Trace chain: `begin_chain`, `current_id`, `set_current_id`, `clear_current_id`,
  `create_span`, and the `TraceId` value type with its chain / span / parent span
  ids, `TraceFlags`, and `TraceId::tracepoint`.
- Trace switch listeners (`api-22`): `register_trace_listener` returns a
  `TraceListener` guard that unregisters on drop, or on `TraceListener::unregister`
  when the failure has to be reported.

The synchronous slice guard and the chain guard are `!Send`: both are matched
against per-thread state, so they must be dropped on the thread that created
them. Asynchronous slice guards are matched by name and task id and may cross
threads.

The NDK trace listener callback has no user-data parameter, so only
`extern "C" fn` listeners can be registered; closures are not supported.

## License

MIT OR Apache-2.0
