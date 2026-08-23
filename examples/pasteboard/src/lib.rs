use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_pasteboard_binding::Pasteboard;
use ohos_udmf_binding::{UdmfData, UdmfRecord, Uds, UdsPlainText};

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

/// Write plain text to the system pasteboard.
#[napi]
pub fn set_pasteboard_data(content: String) -> Result<()> {
    let board = Pasteboard::new();

    let plain = UdsPlainText::new();
    plain
        .set_content(content)
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let record = UdmfRecord::new();
    record
        .add(Uds::PlainText(plain))
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let data = UdmfData::new();
    data.add_record(&record)
        .map_err(|e| Error::from_reason(e.to_string()))?;

    board
        .set_data(&data)
        .map_err(|e| Error::from_reason(e.to_string()))
}

/// Whether the pasteboard currently holds any data.
#[napi]
pub fn has_pasteboard_data() -> bool {
    Pasteboard::new().has_data()
}

/// Whether the pasteboard currently holds a plain-text record.
#[napi]
pub fn has_pasteboard_plain_text() -> Result<bool> {
    let board = Pasteboard::new();
    board
        .has_type(UdsPlainText::new())
        .map_err(|e| Error::from_reason(e.to_string()))
}

/// Whether the pasteboard content comes from a remote device
/// (distributed copy).
#[napi]
pub fn is_pasteboard_remote_data() -> bool {
    Pasteboard::new().is_remote_data()
}
