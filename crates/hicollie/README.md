# ohos-hicollie-binding

This crate is a binding for the hicollie module in OpenHarmony.

hicollie is a software watchdog that helps an application detect that its own
threads are stuck or janky. A business thread registers a periodic liveness task
(stuck detection) or a pair of begin/end stubs bracketing event processing (jank
detection); the monitor thread can then report a freeze event. This crate wraps
the native `hicollie.h` C API with a safe layer.

## Install

```shell
cargo add ohos-hicollie-binding
```

## Usage

```rust
use ohos_hicollie_binding as hicollie;

// Called every 3 seconds on a HiCollie-owned thread to probe liveness.
unsafe extern "C" fn detect() {
    // signal the business thread and check whether it answered in time
}

hicollie::init_stuck_detection(detect)?;

let six_second = hicollie::report()?;
println!("stuck for six seconds: {six_second}");
```

## Coverage

Available without any feature (API 12):

- Stuck detection: `init_stuck_detection`.
- Jank detection: `init_jank_detection`, returning the begin/end stubs, and
  `deinit_jank_detection` to tear it down again.
- Reporting: `report`.

Behind `api-*` features:

| Feature | Adds |
|---|---|
| `api-18` | `init_stuck_detection_with_timeout` |
| `api-24` | `report_input_block` |

## Notes

- Detection setup (`init_stuck_detection`, `init_stuck_detection_with_timeout`,
  `init_jank_detection`) must run off the main thread, or the runtime returns a
  wrong-thread-context error.
- `report` may only be called from the HiCollie monitor thread on which the
  stuck-detection task runs.
- Jank detection is set up at most once per thread; a repeated
  `init_jank_detection` returns `HiCollieError::JankAlreadyInitialized`. Use
  `deinit_jank_detection` to unregister the thread first.
- Detection callbacks carry no user-data pointer, so they are taken and returned
  as `extern "C" fn` rather than closures.
- The API 18 timer (`OH_HiCollie_SetTimer` / `OH_HiCollie_CancelTimer`) and the
  API 24 freeze callback and associate-process report are reachable through
  `hicollie::sys`.

## License

MIT OR Apache-2.0
