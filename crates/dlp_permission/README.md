# ohos-dlp-permission-binding

This crate is a binding for the dlp permission module in OpenHarmony.

Data loss prevention (DLP) files are opened by the system in a dedicated sandbox
application. A sandbox application can read the permission granted on the file
it opened, and a regular application can manage the configuration handed to the
sandbox. This crate wraps the native `dlp_permission_api.h` C API with a safe
layer.

## Install

```shell
cargo add ohos-dlp-permission-binding --features api-14
```

## Usage

```rust
use ohos_dlp_permission_binding as dlp;

if dlp::is_in_sandbox()? {
    let info = dlp::permission_info()?;
    if info.actions.contains(dlp::Actions::PRINT) {
        println!("printing allowed, access: {:?}", info.access);
    }
} else {
    dlp::set_sandbox_app_config("{\"key\":\"value\"}")?;
}

let original = dlp::original_file_name("report.docx.dlp")?;
println!("original file name: {original}");
```

## Coverage

The whole native API is `@since 14`, so every item below requires the `api-14`
feature. Without it the crate exposes only the raw bindings through
`ohos_dlp_permission_binding::sys`.

| Feature | Adds |
|---|---|
| `api-14` | `permission_info`, `original_file_name`, `is_in_sandbox`, `set_sandbox_app_config`, `sandbox_app_config`, `clean_sandbox_app_config` |

Types:

- `PermissionInfo` bundles the two out-parameters of
  `OH_DLP_GetDlpPermissionInfo`: a `FileAccess` enum and an `Actions` bit set.
- `Actions` exposes the documented action bits as associated constants
  (`VIEW`, `SAVE`, `SAVE_AS`, `EDIT`, `SCREEN_CAPTURE`, `SCREEN_SHARE`,
  `SCREEN_RECORD`, `COPY`, `PRINT`, `EXPORT`, `PERMISSION_CHANGE`) and supports
  `|`, `&`, `contains` and `intersects`. Unknown bits from newer system
  versions are preserved.
- `DlpError` carries the raw `DLP_ErrCode` plus the conversion failures of the
  safe layer; `describe` maps a code to a short description.

## Notes

- `permission_info` is only callable from a DLP sandbox application; a
  non-sandbox caller gets `DLP_ErrCode` 19100006.
- `set_sandbox_app_config` and `clean_sandbox_app_config` are only callable
  from a non-sandbox application; a sandbox caller gets `DLP_ErrCode` 19100007.
- `OH_DLP_GetOriginalFileName` and `OH_DLP_GetSandboxAppConfig` return
  `malloc`-allocated buffers, but the DLP library exports no matching
  deallocator. The safe layer copies the content into a `String` and releases
  the native buffer with the C library `free`, as the official NDK usage guide
  does, so no buffer is leaked and callers never see a raw pointer.
- Strings returned by the native API are validated as UTF-8; invalid content
  yields `DlpError::InvalidUtf8` rather than a silently lossy conversion.

## License

MIT OR Apache-2.0
