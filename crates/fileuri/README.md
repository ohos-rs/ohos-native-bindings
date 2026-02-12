# ohos-fileuri-binding

OpenHarmony File URI bindings for Rust.

This crate provides safe Rust bindings for OpenHarmony's file URI operations, allowing conversion between file paths and URIs, URI validation, and directory operations.

## Features

- Convert file paths to URIs
- Convert URIs to file paths
- Get directory URI from a file URI
- Validate URI strings
- Extract file names from URIs (API 13+)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ohos-fileuri-binding = { version = "0.1.0" }
```

For API 13+ features:

```toml
[dependencies]
ohos-fileuri-binding = { version = "0.1.0", features = ["api-13"] }
```

## Examples

### Convert path to URI

```rust
use ohos_fileuri_binding::get_uri_from_path;

let uri = get_uri_from_path("/data/storage/el2/base/files/test.txt")?;
println!("URI: {}", uri);
```

### Convert URI to path

```rust
use ohos_fileuri_binding::get_path_from_uri;

let path = get_path_from_uri("file://com.example.app/data/storage/el2/base/files/test.txt")?;
println!("Path: {}", path);
```

### Get directory URI

```rust
use ohos_fileuri_binding::get_full_directory_uri;

let dir_uri = get_full_directory_uri("file://com.example.app/data/storage/el2/base/files/test.txt")?;
println!("Directory URI: {}", dir_uri);
```

### Validate URI

```rust
use ohos_fileuri_binding::is_valid_uri;

if is_valid_uri("file://com.example.app/data/storage/el2/base/files/test.txt") {
    println!("URI is valid");
}
```

### Get file name (API 13+)

```rust
use ohos_fileuri_binding::get_file_name;

let filename = get_file_name("file://com.example.app/data/storage/el2/base/files/test.txt")?;
println!("File name: {}", filename);
```

## Error Handling

All functions return a `Result<T, FileUriError>` (except `is_valid_uri` which returns `bool`).

Possible errors:
- `PermissionError` - Permission verification failed
- `InvalidParameter` - Invalid input parameter
- `DeviceNotSupported` - Device not supported
- `OperationNotPermitted` - Operation not permitted
- `NoSuchFileOrDirectory` - No such file or directory
- `OutOfMemory` - Out of memory
- `Unknown` - Unknown error
- `NullByteError` - String contains null byte
- `ConversionError` - Failed to convert result from C

## API Levels

This crate supports multiple OpenHarmony API levels:
- Base features: API 12+
- `get_file_name`: API 13+

Use feature flags to enable specific API levels:
- `api-13` through `api-20`

## System Capability

SystemCapability.FileManagement.AppFileService

## License

MIT OR Apache-2.0
