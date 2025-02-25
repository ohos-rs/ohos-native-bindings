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
