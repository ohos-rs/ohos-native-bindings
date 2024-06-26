mod data;
mod r#error;
mod r#type;

use std::ptr;

pub use data::*;
pub use r#error::*;
pub use r#type::*;

/// add
pub fn asset_add(attrs: Vec<AssetAttr>) -> AssetResultCode {
    let count = attrs.len() as u32;
    let real_attrs = attrs.iter().map(|a| a.into()).collect::<Vec<_>>();
    let ret_status = unsafe { ohos_asset_sys::OH_Asset_Add(real_attrs.as_ptr(), count) };
    AssetResultCode::from(ret_status as u32)
}

/// query
/// note: If you're trying to access key assets that require user authentication, make sure to run the function asset_pre_query first.
pub fn asset_query(attrs: Vec<AssetAttr>) -> Result<AssetResultSet, AssetResultCode> {
    let count = attrs.len() as u32;
    let real_attrs = attrs.iter().map(|a| a.into()).collect::<Vec<_>>();
    let mut ret_set = ohos_asset_sys::Asset_ResultSet {
        count: 0,
        results: ptr::null_mut(),
    };
    let query_status =
        unsafe { ohos_asset_sys::OH_Asset_Query(real_attrs.as_ptr(), count, &mut ret_set) };
    let query_code = AssetResultCode::from(query_status as u32);
    if query_code != AssetResultCode::Success {
        return Err(query_code);
    }
    let ret = AssetResultSet::from((&mut ret_set) as *mut ohos_asset_sys::Asset_ResultSet);

    // free origin data
    unsafe { ohos_asset_sys::OH_Asset_FreeResultSet(&mut ret_set) };
    Ok(ret)
}

/// remove
pub fn asset_remove(attrs: Vec<AssetAttr>) -> AssetResultCode {
    let count = attrs.len() as u32;
    let real_attrs = attrs.iter().map(|a| a.into()).collect::<Vec<_>>();
    let ret_status = unsafe { ohos_asset_sys::OH_Asset_Remove(real_attrs.as_ptr(), count) };
    AssetResultCode::from(ret_status as u32)
}

/// update
pub fn asset_update(query_attrs: Vec<AssetAttr>, update_attrs: Vec<AssetAttr>) -> AssetResultCode {
    let query_count = query_attrs.len() as u32;
    let query_real_attrs = query_attrs.iter().map(|a| a.into()).collect::<Vec<_>>();
    let update_count = update_attrs.len() as u32;
    let update_real_attrs = update_attrs.iter().map(|a| a.into()).collect::<Vec<_>>();
    let ret_status = unsafe {
        ohos_asset_sys::OH_Asset_Update(
            query_real_attrs.as_ptr(),
            query_count,
            update_real_attrs.as_ptr(),
            update_count,
        )
    };
    AssetResultCode::from(ret_status as u32)
}
