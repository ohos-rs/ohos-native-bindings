# ohos-hiappevent-binding

This crate is a binding for the HiAppEvent module in OpenHarmony.

HiAppEvent records application events into a local event file: custom events
written by the application, and OS events such as crashes, freezes and jank. An
application can subscribe to them with a `Watcher`, tune how the OS collects them
with an `EventConfig`, and upload them through a `Processor`.

## Install

```shell
cargo add ohos-hiappevent-binding
```

## Usage

```rust
use ohos_hiappevent_binding as hiappevent;
use hiappevent::{event, param, AppEventType, EventParams};

let params = EventParams::builder()
    .add(param::USER_ID, 1234_i64)
    .add("page", "home")
    .build()?;

hiappevent::write("my_domain", event::USER_LOGIN, AppEventType::Behavior, &params)?;
```

The raw bindings are re-exported as `hiappevent::sys` for anything not yet
covered by the safe layer.

## Coverage

- Writing events: `write`, `EventParams` / `EventParamsBuilder` with a single
  typed `add` for every scalar, array and string parameter.
- Logging configuration: `configure`, `set_logging_disabled`, `set_max_storage`,
  `clear_data`, and `EventConfig` for per-event OS settings (`api-15`).
- Subscriptions: `Watcher` with trigger conditions, event filters, the
  `OnReceive` / `OnTrigger` / `OnTake` callbacks, and `app_event_groups` /
  `taken_events` to read their raw arguments.
- Reporting: `Processor` and `remove_processor` (`api-18`).
- The predefined domain, event, parameter and config item names, as `&str`
  constants under `domain`, `event`, `param` and `config_item`.

The watcher callbacks are plain `extern "C" fn`s: the NDK passes them no user
data, so a closure has nowhere to keep its captures.

## License

MIT OR Apache-2.0
