use std::{ffi::CString, ptr::NonNull, sync::LazyLock};

use ohos_web_sys::{
    ArkWeb_ControllerAPI, ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_CONTROLLER, ArkWeb_ProxyObject,
    OH_ArkWeb_GetNativeAPI,
};

use crate::{ark_web_member_missing, ArkWebError};

pub struct Controller {
    raw: NonNull<ArkWeb_ControllerAPI>,
}

impl Controller {
    pub fn new() -> Self {
        let ret = unsafe {
            OH_ArkWeb_GetNativeAPI(ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_CONTROLLER)
                as *mut ArkWeb_ControllerAPI
        };
        #[cfg(debug_assertions)]
        assert!(
            !ret.is_null(),
            "Failed to get Controller API by OH_ArkWeb_GetNativeAPI"
        );

        Self {
            raw: unsafe { NonNull::new_unchecked(ret) },
        }
    }

    pub fn check_member_missing(&self, member: &str) -> Result<(), ArkWebError> {
        match member {
            "runJavaScript" => {
                if unsafe { !ark_web_member_missing!(self.raw.as_ptr(), runJavaScript) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "registerJavaScriptProxy" => {
                if unsafe { !ark_web_member_missing!(self.raw.as_ptr(), registerJavaScriptProxy) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "deleteJavaScriptRegister" => {
                if unsafe { !ark_web_member_missing!(self.raw.as_ptr(), deleteJavaScriptRegister) }
                {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "refresh" => {
                if unsafe { !ark_web_member_missing!(self.raw.as_ptr(), refresh) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "registerAsyncJavaScriptProxy" => {
                if unsafe {
                    !ark_web_member_missing!(self.raw.as_ptr(), registerAsyncJavaScriptProxy)
                } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "createWebMessagePorts" => {
                if unsafe { !ark_web_member_missing!(self.raw.as_ptr(), createWebMessagePorts) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "destroyWebMessagePorts" => {
                if unsafe { !ark_web_member_missing!(self.raw.as_ptr(), destroyWebMessagePorts) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "postWebMessage" => {
                if unsafe { !ark_web_member_missing!(self.raw.as_ptr(), postWebMessage) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "getLastJavascriptProxyCallingFrameUrl" => {
                if unsafe {
                    !ark_web_member_missing!(
                        self.raw.as_ptr(),
                        getLastJavascriptProxyCallingFrameUrl
                    )
                } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            _ => Ok(()),
        }
    }

    pub fn register_javascript_proxy(
        &self,
        web_tag: String,
        proxy_object: *const ArkWeb_ProxyObject,
    ) -> Result<(), ArkWebError> {
        self.check_member_missing("registerJavaScriptProxy")?;

        let tag = CString::new(web_tag).map_err(|e| {
            ArkWebError::JsApiRegisterFailed(format!(
                "Failed to create CString when call registerJavaScriptProxy: {}",
                e
            ))
        })?;

        unsafe {
            if let Some(cb) = (*self.raw.as_ptr()).registerJavaScriptProxy {
                cb(tag.as_ptr().cast(), proxy_object);
            }
        }
        Ok(())
    }

    pub fn refresh(&self, web_tag: String) -> Result<(), ArkWebError> {
        let tag = CString::new(web_tag).map_err(|e| {
            ArkWebError::JsApiRegisterFailed(format!(
                "Failed to create CString when call refresh: {}",
                e
            ))
        })?;

        unsafe {
            if let Some(cb) = (*self.raw.as_ptr()).refresh {
                cb(tag.as_ptr().cast());
            }
        }
        Ok(())
    }
}

unsafe impl Send for Controller {}
unsafe impl Sync for Controller {}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

pub static ARK_WEB_CONTROLLER_API: LazyLock<Controller> = LazyLock::new(Controller::new);
