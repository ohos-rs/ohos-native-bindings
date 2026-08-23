# ohos-common-event-binding

This crate is a binding for the common event module in OpenHarmony.

Common events are the system-wide notifications an application can listen to —
the screen turning on, a package being installed, the battery level changing —
and, since API 18, publish itself. An application describes what it wants in a
`SubscribeInfo`, creates a `Subscriber` for it and starts it with `subscribe`;
each event arrives as an `RcvData` in the `ReceiveHandler`. This crate wraps the
native `oh_commonevent.h` C API with a safe layer.

## Install

```shell
cargo add ohos-common-event-binding
```

## Usage

```rust
use ohos_common_event_binding as common_event;
use common_event::{event, RcvData, ReceiveHandler, SubscribeInfo, Subscriber};

struct OnPower;

impl ReceiveHandler for OnPower {
    fn on_receive(data: &RcvData<'_>) {
        let name = data.event().unwrap_or_default();
        let level = match data.parameters() {
            Some(parameters) => parameters.int("soc", -1).unwrap_or(-1),
            None => -1,
        };
        println!("{name}: {level}");
    }
}

let mut info = SubscribeInfo::new(&[event::BATTERY_CHANGED, event::POWER_CONNECTED])?;
info.set_publisher_bundle_name("com.example.publisher")?;

let mut subscriber = Subscriber::new::<OnPower>(&info)?;
subscriber.subscribe()?;
// Dropping the subscriber unsubscribes.
```

## Coverage

Available without any feature (API 12):

- Subscription: `SubscribeInfo` with `set_publisher_permission` and
  `set_publisher_bundle_name`, and `Subscriber` with `subscribe` /
  `unsubscribe`.
- Received events: `RcvData` with `event`, `code`, `data`, `bundle_name` and
  `parameters`.
- Reading a payload: `ParametersRef` with `has_key`, the scalar getters `int`,
  `long`, `bool`, `char`, `double` and the array getters `int_array`,
  `long_array`, `bool_array`, `char_array`, `double_array`.
- The names of the predefined system events, in `event`.

Behind `api-*` features:

| Feature | Adds |
|---|---|
| `api-18` | `publish`, `publish_with_info` and `PublishInfo`; the owned `Parameters` with its setters; the ordered-event methods on `Subscriber` (`is_ordered`, `finish`, `is_aborted`, `abort`, `clear_abort`, `code`, `set_code`, `data`, `set_data`) |

## Notes

- Subscribing and unsubscribing are a pair, and dropping a `Subscriber`
  unsubscribes before it releases the handle. The order matters: the common
  event service keeps its own reference to the object that carries the
  callback, so releasing the handle alone would leave the service dispatching
  into it.
- The native receive callback is a bare C function pointer with no user-data
  argument, which leaves a closure nowhere to keep its captures. The handler is
  a type implementing `ReceiveHandler` instead, and `Subscriber::new`
  instantiates one trampoline per handler type. State the handler needs has to
  reach it some other way, such as a channel in a `static`.
- The handler runs on a thread owned by the common event service, and a panic
  crossing back into it aborts the process.
- `RcvData` and `ParametersRef` are borrowed views, not owned values: the
  service allocates them right before it calls the handler and frees them as
  soon as it returns. Their lifetime keeps them inside the callback, and
  anything to be kept has to be copied out — including the buffers the array
  getters return, which belong to the payload and are freed with it.
- The `long` family is a C `long` natively, so it is 32 bits wide on 32-bit
  targets. The crate takes and returns `i64` and reports a value that does not
  fit as `CommonEventError::OutOfRange`.
- The `char` family is a C `char`, which is a byte and not a Rust `char`; it is
  taken and returned as `u8`.
- The `char` *array* is stored natively as a string rather than as an array: the
  setter keeps `min(strlen(value), num)` bytes and the getter hands back
  `length() + 1` of them, terminator included. `set_char_array` copies the bytes
  into a terminated buffer and rejects an interior zero byte, which the native
  side would truncate at, with `CommonEventError::Nul`; `char_array` drops the
  appended terminator. A value therefore reads back exactly as it was written.
- The ordered-event methods on `Subscriber` act on the event currently being
  handled, so they are meant to be called from within the handler. Since the
  handler receives no subscriber, an application that needs them has to reach
  its `Subscriber` some other way.

## License

MIT OR Apache-2.0
