use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_udmf_binding::{
    UdmfData, UdmfMeta, UdmfRecord, Uds, UdsFileUri, UdsHtml, UdsPlainText, Utd,
};

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

    let mut data = UdmfData::new();
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

/// Exercise real UDMF ownership: source records may be released after adding,
/// queried records are borrowed, and repeated queries/container drops stay valid.
#[napi]
pub fn file_uri_ownership_roundtrip() -> Result<bool> {
    let expected =
        "file://com.richerfu.ohos_example/data/storage/el2/base/files/space%20%E4%B8%AD.txt";
    for _ in 0..100 {
        let mut data = UdmfData::try_new().map_err(to_err)?;
        for file_type in ["general.file", "general.folder"] {
            let uri = UdsFileUri::new().map_err(to_err)?;
            uri.set_file_uri(expected).map_err(to_err)?;
            uri.set_file_type(file_type).map_err(to_err)?;
            let record = UdmfRecord::try_new().map_err(to_err)?;
            record.add_file_uri(&uri).map_err(to_err)?;
            data.add_record(&record).map_err(to_err)?;
            // The native container retains the added contents.
        }
        if data.count() != 2 {
            return Ok(false);
        }
        for _ in 0..3 {
            let records = data.records().map_err(to_err)?;
            if records.len() != 2 {
                return Ok(false);
            }
            for (record, expected_type) in records.iter().zip(["general.file", "general.folder"]) {
                let uri = record.file_uri().map_err(to_err)?;
                if uri.file_uri().map_err(to_err)? != expected
                    || uri.file_type().map_err(to_err)? != expected_type
                {
                    return Ok(false);
                }
            }
        }
        let uri = data.record(0).map_err(to_err)?.file_uri().map_err(to_err)?;
        drop(data);
        if uri.file_uri().map_err(to_err)? != expected {
            return Ok(false);
        }
    }
    let uri = UdsFileUri::new().map_err(to_err)?;
    Ok(uri.set_file_uri("file://invalid\0uri").is_err()
        && uri.set_file_type("general.file\0").is_err())
}
