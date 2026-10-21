# ohos-pasteboard-binding

This crate is a binding for the pasteboard module in OpenHarmony.

**Before use it, you need to make sure the following notes:**

- Api version is 13 or higher.
- Request the pasteboard permission: `ohos.permission.PASTEBOARD_READ`

## Usage

```rust
use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_pasteboard_binding::Pasteboard;

#[napi]
pub fn get_pasteboard_data() -> Result<String> {
    let board = Pasteboard::new();

    let data = board
        .data()
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let text = data
        .primary_plain_text()
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let s = text
        .get_content()
        .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(s)
}
```

## Native multi-MIME clipboard

Use `ohos-udmf-binding` records and `UdmfRecord::add_general_entry` for custom
byte entries. Standard UDS values use their typed record methods.
`Pasteboard::set_data` publishes the resulting container, and `data()` returns
an owned container whose records can enumerate types and copy general entries.

- `clear()` clears the native pasteboard with `api-13`.
- `mime_types()` copies every returned MIME type with `api-14`; the SDK owns
  and releases the source array and strings with its pasteboard cache.
- `change_count()` returns the native revision counter with `api-18`.
  Zero is also the SDK's failure sentinel, and a service/system restart resets
  the counter. Clearing data leaves the counter's history intact.

All methods call native SDK APIs directly. Entry bytes, type identifiers and
MIME strings preserve the SDK's representation. Consumers choose their own
revision comparison and MIME selection rules.

The [native API reference](https://github.com/openharmony/docs/blob/master/en/application-dev/reference/apis-basic-services-kit/capi-oh-pasteboard-h.md)
defines the feature levels. Native ownership follows
[`OH_Pasteboard_GetMimeTypes` and `OH_Pasteboard_Destroy`](https://github.com/openharmony/distributeddatamgr_pasteboard/blob/master/interfaces/ndk/src/oh_pasteboard.cpp):
the pasteboard owns its MIME string vector and pointer array.

## License

MIT OR Apache-2.0
