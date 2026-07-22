# ohos-hidebug-binding

This crate is a binding for the hidebug module in OpenHarmony.

hidebug lets an application inspect itself at runtime: CPU and memory usage,
in-process trace capture, backtraces with symbolization, crash-context
attachments and the resource profiler. This crate wraps the native `hidebug.h`
C API with a safe layer.

## Install

```shell
cargo add ohos-hidebug-binding
```

## Usage

```rust
use ohos_hidebug_binding as hidebug;

let mem = hidebug::app_native_mem_info();
println!("pss={} rss={}", mem.pss, mem.rss);

let capture = hidebug::start_app_trace_capture(
    hidebug::TraceFlag::MainThread,
    hidebug::trace_tag::ARKUI | hidebug::trace_tag::GRAPHICS,
    10 * 1024 * 1024,
)?;
println!("tracing into {}", capture.file_name());
capture.stop()?;
```

## Coverage

Available without any feature (API 12):

- CPU usage: `system_cpu_usage`, `app_cpu_usage`, `app_thread_cpu_usage`.
- Memory: `system_mem_info`, `app_native_mem_info`, `app_memory_limit`.
- Trace capture: `start_app_trace_capture` returns an `AppTraceCapture` guard
  (stops on drop, or explicitly via `stop`), with tag sets in `trace_tag`.

Behind `api-*` features:

| Feature | Adds |
|---|---|
| `api-14` | `graphics_memory` |
| `api-20` | `BacktraceObject` (`backtrace_from_fp`, `symbolic_address`), `app_native_mem_info_cached` |
| `api-21` | `graphics_memory_summary` |
| `api-22` | `request_thread_lite_sampling` |
| `api-23` | `set_crash_obj` / `set_crash_obj_leaked` / `reset_crash_obj` |
| `api-24` | `request_trace`, `start_profiler` (returns a `ProfilerSession` guard) |

## Notes

- `set_crash_obj` only accepts `'static` data: hidebug keeps the pointer and
  reads it while handling a crash, so a borrowed buffer would dangle.
- The MallocDispatch table API replaces the process-wide allocator and is not
  wrapped; use `hidebug::sys` for it.
- Callbacks other than symbolization carry no user-data pointer, so they are
  taken as `extern "C" fn` instead of closures.
- Trace capture and the resource profiler need the corresponding application
  permissions and are not exercised by CI.

## License

MIT OR Apache-2.0
