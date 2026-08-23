use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_udmf_binding::{UdmfData, UdmfMeta, UdmfRecord, Uds, UdsHtml, UdsPlainText, Utd};

fn to_err(e: ohos_udmf_binding::UdmfError) -> Error {
    Error::from_reason(e.to_string())
}

#[napi]
pub fn plain_text_roundtrip(text: String) -> Result<String> {
    let plain = UdsPlainText::new();
    plain.set_content(&text).map_err(to_err)?;
    plain.get_content().map_err(to_err)
}

#[napi]
pub fn html_roundtrip(html: String, plain: String) -> Result<String> {
    let item = UdsHtml::new();
    item.set_html(&html).map_err(to_err)?;
    item.set_primary_content(&plain).map_err(to_err)?;
    Ok(format!(
        "html={} plain={}",
        item.get_html().map_err(to_err)?,
        item.get_primary_content().map_err(to_err)?
    ))
}

#[napi]
pub fn record_and_data(text: String) -> Result<String> {
    let plain = UdsPlainText::new();
    plain.set_content(&text).map_err(to_err)?;
    let html = UdsHtml::new();
    html.set_html("<p>hi</p>").map_err(to_err)?;
    html.set_primary_content("hi").map_err(to_err)?;

    let record = UdmfRecord::new();
    record.add(Uds::PlainText(plain)).map_err(to_err)?;
    record.add(Uds::Html(html)).map_err(to_err)?;

    let data = UdmfData::new();
    data.add_record(&record).map_err(to_err)?;
    let count = data.count();
    let records = data.records().map_err(to_err)?;
    Ok(format!("count={count} records_len={}", records.len()))
}

#[napi]
pub fn utd_equals() -> Result<bool> {
    let a = Utd::new(UdmfMeta::PlainText).map_err(to_err)?;
    let b = Utd::new(UdmfMeta::PlainText).map_err(to_err)?;
    let c = Utd::new(UdmfMeta::Html).map_err(to_err)?;
    Ok(a == b && a != c)
}

#[napi]
pub fn smoke() -> Result<String> {
    Ok(format!(
        "plain={}\n{}\n{}\nutd_equals={}",
        plain_text_roundtrip("hello udmf".to_string())?,
        html_roundtrip("<b>x</b>".to_string(), "x".to_string())?,
        record_and_data("record".to_string())?,
        utd_equals()?
    ))
}
