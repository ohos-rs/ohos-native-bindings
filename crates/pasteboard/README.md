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

The optional `napi` feature provides `Clipboard::{has_data, has_text, text,
set_text}` for API-12 system module calls. Each call uses the current UI
callback's `&Env`; no module, value or environment survives it. Presence queries
read MIME metadata, while content access follows system permission policy.

## License

MIT OR Apache-2.0
