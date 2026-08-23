# ohos-notification-binding

This crate is a binding for the notification module in OpenHarmony.

The native `notification.h` C API of NotificationKit exposes a single entry
point: a query telling the application whether the user currently allows it to
publish notifications. This crate wraps that entry point with a safe layer.

## Install

```shell
cargo add ohos-notification-binding --features api-13
```

## Usage

```rust
use ohos_notification_binding as notification;

if notification::is_notification_enabled() {
    // publish through the ArkTS notification manager
} else {
    // fall back to an in-app hint, or guide the user to system settings
}
```

## Coverage

Behind `api-*` features:

| Feature | Adds |
|---|---|
| `api-13` | `is_notification_enabled` |

With the default feature set the crate only re-exports the raw bindings as
`notification::sys`.

## Notes

- The native call takes no argument, allocates nothing and returns a plain
  `bool`, so there is no resource to release and no error code to map; a denied
  permission and a failed query are alike reported as `false`.
- The result reflects the notification switch the user controls in system
  settings and may change at any time, so it is best queried right before
  publishing rather than cached.
- Publishing notifications, notification slots and subscriptions are not part
  of the C API — they are only reachable from ArkTS through
  `@ohos.notificationManager`.

## License

MIT OR Apache-2.0
