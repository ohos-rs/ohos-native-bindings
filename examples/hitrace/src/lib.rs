use napi_derive_ohos::napi;
use ohos_hitrace_binding::TraceId;
const TAG: &str = "BINDTEST_HITRACE";

#[napi]
pub fn test_hitrace() -> String {
    // Build a valid id, round-trip through bytes, confirm equality (covers the
    // to_bytes/from_bytes fix from the audit).
    let mut id = TraceId::invalid();
    id.set_chain_id(0x1122334455667788);
    id.set_span_id(0xABCD);
    let bytes = id.to_bytes();
    let back = TraceId::from_bytes(&bytes);
    let msg = match back {
        Ok(b) => format!(
            "roundtrip chain={:#x} span={:#x} bytes_len={} match={}",
            b.chain_id(),
            b.span_id(),
            bytes.len(),
            b.chain_id() == id.chain_id() && b.span_id() == id.span_id()
        ),
        Err(e) => format!("from_bytes Err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}
