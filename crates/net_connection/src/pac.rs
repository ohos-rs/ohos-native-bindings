use std::ffi::CString;

use ohos_net_connection_sys::*;

use crate::{NetConnectionError, Result};

pub fn set_pac_url(pac_url: &str) -> Result<()> {
    let c_pac_url = CString::new(pac_url).map_err(|_| NetConnectionError::NullByte)?;
    let code = unsafe { OH_NetConn_SetPacUrl(c_pac_url.as_ptr()) } as i32;
    if code != 0 {
        return Err(NetConnectionError::Code(code));
    }
    Ok(())
}

pub fn get_pac_url() -> Result<String> {
    let mut pac_url = [0; NETCONN_MAX_STR_LEN as usize];
    let code = unsafe { OH_NetConn_GetPacUrl(pac_url.as_mut_ptr()) } as i32;
    if code != 0 {
        return Err(NetConnectionError::Code(code));
    }
    let len = pac_url
        .iter()
        .position(|item| *item == 0)
        .unwrap_or(pac_url.len());
    String::from_utf8(pac_url[..len].iter().map(|item| *item as u8).collect())
        .map_err(|_| NetConnectionError::Conversion)
}
