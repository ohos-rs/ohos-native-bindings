use std::{
    ffi::{c_char, c_void, CString},
    ptr::NonNull,
    sync::LazyLock,
};

use ohos_web_sys::{
    ArkWeb_ControllerAPI, ArkWeb_JavaScriptBridgeData, ArkWeb_JavaScriptObject,
    ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_CONTROLLER, ArkWeb_ProxyObject,
    OH_ArkWeb_GetNativeAPI,
};

use crate::{ark_web_member_missing, ArkWebError};

unsafe extern "C" fn on_javascript_callback<F>(
    _web_tag: *const c_char,
    data: *const ArkWeb_JavaScriptBridgeData,
    user_data: *mut c_void,
) where
    F: FnOnce(Option<&[u8]>) + 'static,
{
    if user_data.is_null() {
        return;
    }
    // SAFETY: `user_data` was allocated by `Box::into_raw(Box::new(callback))` for
    // a single ArkWeb completion and is reclaimed once here.
    let callback = unsafe { Box::from_raw(user_data.cast::<F>()) };
    let bytes = unsafe { data.as_ref() }.map(|bridge| {
        if bridge.buffer.is_null() || bridge.size == 0 {
            &[][..]
        } else {
            // SAFETY: ArkWeb lends `bridge.buffer[..bridge.size]` for this callback.
            unsafe { std::slice::from_raw_parts(bridge.buffer, bridge.size) }
        }
    });
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        callback(bytes);
    }));
}

pub struct Controller {
    raw: Option<NonNull<ArkWeb_ControllerAPI>>,
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
            raw: NonNull::new(ret),
        }
    }

    pub fn check_member_missing(&self, member: &str) -> Result<(), ArkWebError> {
        let Some(raw) = self.raw else {
            return Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()));
        };
        let ptr = raw.as_ptr();
        match member {
            "runJavaScript" => {
                if unsafe { !ark_web_member_missing!(ptr, runJavaScript) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "registerJavaScriptProxy" => {
                if unsafe { !ark_web_member_missing!(ptr, registerJavaScriptProxy) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "deleteJavaScriptRegister" => {
                if unsafe { !ark_web_member_missing!(ptr, deleteJavaScriptRegister) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "refresh" => {
                if unsafe { !ark_web_member_missing!(ptr, refresh) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "registerAsyncJavaScriptProxy" => {
                if unsafe { !ark_web_member_missing!(ptr, registerAsyncJavaScriptProxy) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "createWebMessagePorts" => {
                if unsafe { !ark_web_member_missing!(ptr, createWebMessagePorts) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "destroyWebMessagePorts" => {
                if unsafe { !ark_web_member_missing!(ptr, destroyWebMessagePorts) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "postWebMessage" => {
                if unsafe { !ark_web_member_missing!(ptr, postWebMessage) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            #[cfg(feature = "api-14")]
            "getLastJavascriptProxyCallingFrameUrl" => {
                if unsafe { !ark_web_member_missing!(ptr, getLastJavascriptProxyCallingFrameUrl) } {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            _ => Ok(()),
        }
    }

    /// Evaluates `js_code` in the page currently displayed by `web_tag` and invokes
    /// `callback` once on the owning UI thread with the result bytes (`None` if
    /// ArkWeb supplies no bridge payload).
    pub fn run_javascript<F>(
        &self,
        web_tag: &str,
        js_code: &str,
        callback: F,
    ) -> Result<(), ArkWebError>
    where
        F: FnOnce(Option<&[u8]>) + 'static,
    {
        self.check_member_missing("runJavaScript")?;
        let Some(raw) = self.raw else {
            return Err(ArkWebError::ArkWebApiMemberMissing(
                "runJavaScript".to_string(),
            ));
        };
        let tag = CString::new(web_tag).map_err(|e| {
            ArkWebError::WebviewCreateFailed(format!(
                "Failed to create CString when call runJavaScript: {}",
                e
            ))
        })?;
        // SAFETY: `check_member_missing("runJavaScript")` verified the slot is in-bounds and set.
        let Some(run_js) = (unsafe { (*raw.as_ptr()).runJavaScript }) else {
            return Err(ArkWebError::ArkWebApiMemberMissing(
                "runJavaScript".to_string(),
            ));
        };

        let mut script = Vec::with_capacity(js_code.len() + 1);
        script.extend_from_slice(js_code.as_bytes());
        script.push(0);

        let user_data = Box::into_raw(Box::new(callback)).cast::<c_void>();
        let js_object = ArkWeb_JavaScriptObject {
            buffer: script.as_ptr(),
            size: js_code.len(),
            callback: Some(on_javascript_callback::<F>),
            userData: user_data,
        };
        // SAFETY: Called on the UI thread; `tag`, `script`, and `js_object` outlive the call,
        // and `user_data` is reclaimed once in `on_javascript_callback`.
        unsafe {
            run_js(tag.as_ptr().cast(), &js_object);
        }
        Ok(())
    }

    pub fn register_javascript_proxy(
        &self,
        web_tag: String,
        proxy_object: *const ArkWeb_ProxyObject,
    ) -> Result<(), ArkWebError> {
        self.check_member_missing("registerJavaScriptProxy")?;
        let Some(raw) = self.raw else {
            return Err(ArkWebError::ArkWebApiMemberMissing(
                "registerJavaScriptProxy".to_string(),
            ));
        };

        let tag = CString::new(web_tag).map_err(|e| {
            ArkWebError::JsApiRegisterFailed(format!(
                "Failed to create CString when call registerJavaScriptProxy: {}",
                e
            ))
        })?;

        unsafe {
            if let Some(cb) = (*raw.as_ptr()).registerJavaScriptProxy {
                cb(tag.as_ptr().cast(), proxy_object);
            }
        }
        Ok(())
    }

    pub fn refresh(&self, web_tag: String) -> Result<(), ArkWebError> {
        self.check_member_missing("refresh")?;
        let Some(raw) = self.raw else {
            return Err(ArkWebError::ArkWebApiMemberMissing("refresh".to_string()));
        };
        let tag = CString::new(web_tag).map_err(|e| {
            ArkWebError::JsApiRegisterFailed(format!(
                "Failed to create CString when call refresh: {}",
                e
            ))
        })?;

        unsafe {
            if let Some(cb) = (*raw.as_ptr()).refresh {
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
