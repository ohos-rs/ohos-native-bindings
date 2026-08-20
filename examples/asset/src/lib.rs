use std::mem::ManuallyDrop;
use std::panic::{catch_unwind, AssertUnwindSafe};

use napi_derive_ohos::napi;
use ohos_asset_binding::{
    asset_add, asset_query, asset_remove, asset_update, AssetAccessibility, AssetAttr,
    AssetResultCode, AssetTag, AssetValue,
};

fn catch_str(f: impl FnOnce() -> String) -> String {
    catch_unwind(AssertUnwindSafe(f)).unwrap_or_else(|payload| {
        let msg = payload
            .downcast_ref::<String>()
            .map(|s| s.as_str())
            .or_else(|| payload.downcast_ref::<&str>().copied())
            .unwrap_or("non-string panic");
        format!("panic: {msg}")
    })
}

const ALIAS: &[u8] = b"ohos-rs-asset-demo";

fn blob(tag: AssetTag, data: &[u8]) -> AssetAttr {
    AssetAttr {
        tag,
        value: AssetValue::Blob(ManuallyDrop::new(data.to_vec())),
    }
}

fn number(tag: AssetTag, value: u32) -> AssetAttr {
    AssetAttr {
        tag,
        value: AssetValue::U32IntT(value),
    }
}

fn alias_query() -> Vec<AssetAttr> {
    vec![blob(AssetTag::AssetTagAlias, ALIAS)]
}

fn add_attrs(secret: &[u8]) -> Vec<AssetAttr> {
    vec![
        blob(AssetTag::AssetTagAlias, ALIAS),
        blob(AssetTag::AssetTagSecret, secret),
        number(
            AssetTag::AssetTagAccessibility,
            u32::from(AssetAccessibility::AssetAccessibilityDevicePoweredOn),
        ),
    ]
}

#[napi]
pub fn add_secret(secret: String) -> String {
    catch_str(|| format!("{:?}", asset_add(add_attrs(secret.as_bytes()))))
}

#[napi]
pub fn query_secret() -> String {
    catch_str(|| match asset_query(alias_query()) {
        Ok(set) => format!("count={} results={}", set.count, set.result.len()),
        Err(code) => format!("ERR {code}"),
    })
}

#[napi]
pub fn update_secret(secret: String) -> String {
    catch_str(|| {
        let update = vec![blob(AssetTag::AssetTagSecret, secret.as_bytes())];
        format!("{:?}", asset_update(alias_query(), update))
    })
}

#[napi]
pub fn remove_secret() -> String {
    catch_str(|| format!("{:?}", asset_remove(alias_query())))
}

#[napi]
pub fn smoke() -> String {
    catch_str(|| smoke_inner())
}

fn smoke_inner() -> String {
    let _ = asset_remove(alias_query());
    let add = asset_add(add_attrs(b"first"));
    let query = asset_query(alias_query());
    let update = asset_update(
        alias_query(),
        vec![blob(AssetTag::AssetTagSecret, b"second")],
    );
    let query2 = asset_query(alias_query());
    let remove = asset_remove(alias_query());
    let query3 = asset_query(alias_query());
    format!(
        "add={add:?}\nquery={}\nupdate={update:?}\nquery2={}\nremove={remove:?}\nquery3={}",
        match query {
            Ok(set) => format!("ok count={}", set.count),
            Err(code) => format!("ERR {code}"),
        },
        match query2 {
            Ok(set) => format!("ok count={}", set.count),
            Err(code) => format!("ERR {code}"),
        },
        match query3 {
            Ok(set) => format!("ok count={}", set.count),
            Err(code) => format!("ERR {code}"),
        },
    )
}

// Keep AssetResultCode in the demo binary so Success is named in output.
#[allow(dead_code)]
fn _success() -> AssetResultCode {
    AssetResultCode::Success
}
