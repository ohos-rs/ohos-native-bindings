use crate::code::XComponentResultCode;
use ohos_xcomponent_sys::{
    OH_NativeXComponent, OH_NativeXComponent_GetXComponentId, OH_XCOMPONENT_ID_LEN_MAX,
};

/// get xcomponent id
pub fn resolve_id(component: *mut OH_NativeXComponent) -> Option<String> {
    let mut id_len: u64 = (OH_XCOMPONENT_ID_LEN_MAX + 1).into();
    let mut origin_id = vec![0; id_len as usize];

    let ret: XComponentResultCode = unsafe {
        OH_NativeXComponent_GetXComponentId(component, origin_id.as_mut_ptr(), &mut id_len).into()
    };

    if ret != XComponentResultCode::Success {
        return None;
    }

    // id_len will change to real length if OH_NativeXComponent_GetXComponentId call successfully.
    let id_str: Vec<u8> = origin_id
        .into_iter()
        .take(id_len as usize)
        .map(|x| x as u8)
        .collect();
    let id = String::from_utf8_lossy(&id_str).into_owned();
    Some(id)
}
