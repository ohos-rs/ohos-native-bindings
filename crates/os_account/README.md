# ohos-os-account-binding

This crate is a binding for the os account module in OpenHarmony.

The native `os_account.h` C API exposes the OS account the calling process
belongs to. As of API 24 that is a single call, `OH_OsAccount_GetName`, which
writes the account name into a caller-owned character array. This crate wraps
it with a safe layer, so the name comes back as a `String` (or as a `&str`
borrowed from a caller-provided byte slice), NUL-terminated and UTF-8
validated.

## Install

```shell
cargo add ohos-os-account-binding
```

## Usage

```rust
use ohos_os_account_binding as os_account;

// Grows the buffer as needed until the name fits.
let name = os_account::name()?;
println!("account name: {name}");

// Or read into a buffer owned by the caller, without allocating.
let mut buffer = [0u8; 128];
let name = os_account::read_name_into(&mut buffer)?;
println!("account name: {name}");
```

## Coverage

Available without any feature (API 12):

- Account name: `name`, `name_with_capacity`, `read_name_into`.

No `api-*` feature adds further calls yet; the whole native surface is covered.

## Notes

- The platform allocates nothing for this API: the buffer belongs to the
  caller, and there is no release call to pair with the read.
- The native call does not report the required buffer size. `name` therefore
  starts at `INITIAL_NAME_BUFFER_SIZE` bytes and doubles while the platform
  rejects the buffer, up to `MAX_NAME_BUFFER_SIZE`, after which it returns
  `OsAccountError::NameTooLong`. Use `name_with_capacity` to make exactly one
  native call with a size of your own choosing.
- A buffer that is too small is reported by the platform as the same
  invalid-parameter code as a null buffer; since this crate never passes a null
  buffer, that code means the name did not fit.

## License

MIT OR Apache-2.0
