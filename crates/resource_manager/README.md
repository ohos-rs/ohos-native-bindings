# ohos-resource-manager-binding

Using rust to implement the ResourceManager.

## Install

```shell
cargo add ohos-resource-manager-binding
```

## Usage

```rust
use ohos_resource_manager_binding::ResourceManager;

// Create a `ResourceManager` from the native resource manager handle,
// then use it to read raw resources, media, or drawable descriptors.
```

`open_raw_file(path)` rejects embedded NULs and absent resources.
`RawFile::read_bounded(limit)` checks the remaining length before allocation,
then validates every partial read. The caller selects its byte limit.
`configuration()` copies API-12 locale, device type and appearance, releasing
the native locale allocation before returning. Serialize shared-manager access.

## License

MIT OR Apache-2.0
