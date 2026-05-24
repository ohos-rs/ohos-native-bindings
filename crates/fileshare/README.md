# ohos-fileshare-binding

Rust wrapper for `ohos-fileshare-sys`.

Provides:

- policy model (`PolicyInfo`)
- permission APIs (`persist`, `revoke`, `activate`, `deactivate`)
- persistent permission check API
- Rust error mapping for fileshare/file-management error codes

## Install

```shell
cargo add ohos-fileshare-binding
```

## Usage

```rust
use ohos_fileshare_binding as fileshare;

// Use the policy model and permission APIs exposed by `fileshare`
// from your native module.
```

## License

MIT OR Apache-2.0
