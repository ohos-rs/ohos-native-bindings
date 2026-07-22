# ohos-time-service-binding

This crate is a binding for the time service module in OpenHarmony.

The native `time_service.h` C API exposes a single entry point,
`OH_TimeService_GetTimeZone`, which writes the current system time zone ID into
a caller-provided character array. This crate wraps it so that callers never
deal with the output buffer, its size, the terminating character or the numeric
error codes.

## Install

```shell
cargo add ohos-time-service-binding
```

## Usage

```rust
use ohos_time_service_binding as time_service;

let zone = time_service::get_time_zone()?;
println!("current time zone: {zone}");
```

## Coverage

The whole API is available since API 12, so nothing sits behind an `api-*`
feature.

- `get_time_zone`: returns the current system time zone ID, managing the output
  buffer internally.
- `get_time_zone_with_capacity`: the same, with an explicit buffer size.

## Notes

- `get_time_zone` starts with a 64-byte buffer and doubles it up to 4096 bytes
  while the service reports that the ID does not fit; the native documentation
  recommends at least 31 bytes and gives no upper bound.
- A buffer that is too small surfaces as `TimeServiceError::BufferTooSmall`
  rather than as the native invalid-parameter code, since the pointer and the
  length passed to the service are always valid.
- The returned ID is validated as UTF-8; a malformed one yields
  `TimeServiceError::InvalidUtf8`.

## License

MIT OR Apache-2.0
