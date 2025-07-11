use std::{
    ffi::{c_void, CString},
    ptr::NonNull,
    sync::LazyLock,
};

use ohos_web_sys::{
    ArkWeb_ComponentAPI, ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_COMPONENT,
    OH_ArkWeb_GetNativeAPI,
};

use crate::{
    ark_web_member_missing, on_controller_attach, on_destroy, on_page_begin, on_page_end,
    ArkWebError,
};

pub struct Component {
    pub(crate) raw: NonNull<ArkWeb_ComponentAPI>,
}

impl Component {
    pub fn new() -> Self {
        let ret = unsafe {
            OH_ArkWeb_GetNativeAPI(ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_COMPONENT)
                as *mut ArkWeb_ComponentAPI
        };
        #[cfg(debug_assertions)]
        assert!(
            !ret.is_null(),
            "Failed to get Component API by OH_ArkWeb_GetNativeAPI"
        );

        Self {
            raw: unsafe { NonNull::new_unchecked(ret) },
        }
    }

    pub fn check_member_missing(&self, member: &str) -> Result<(), ArkWebError> {
        match member {
            "onControllerAttached" => {
                if !ark_web_member_missing!(self.raw.as_ptr(), onControllerAttached) {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "onPageBegin" => {
                if !ark_web_member_missing!(self.raw.as_ptr(), onPageBegin) {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "onPageEnd" => {
                if !ark_web_member_missing!(self.raw.as_ptr(), onPageEnd) {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            "onDestroy" => {
                if !ark_web_member_missing!(self.raw.as_ptr(), onDestroy) {
                    Ok(())
                } else {
                    Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
                }
            }
            _ => Err(ArkWebError::ArkWebApiMemberMissing(member.to_string())),
        }
    }

    pub fn on_controller_attached(
        &self,
        web_tag: String,
        user_data: *mut c_void,
    ) -> Result<(), ArkWebError> {
        self.check_member_missing("onControllerAttached")?;

        unsafe {
            if let Some(cb) = (*self.raw.as_ptr()).onControllerAttached {
                cb(web_tag.as_ptr(), Some(on_controller_attach), user_data);
            }
        }

        Ok(())
    }

    pub fn on_page_begin(
        &self,
        web_tag: String,
        user_data: *mut c_void,
    ) -> Result<(), ArkWebError> {
        self.check_member_missing("onPageBegin")?;

        unsafe {
            if let Some(cb) = (*self.raw.as_ptr()).onPageBegin {
                cb(web_tag.as_ptr(), Some(on_page_begin), user_data);
            }
        }

        Ok(())
    }

    pub fn on_page_end(&self, web_tag: String, user_data: *mut c_void) -> Result<(), ArkWebError> {
        self.check_member_missing("onPageEnd")?;

        let tag = CString::new(web_tag).expect("Failed to create CString");

        unsafe {
            if let Some(cb) = (*self.raw.as_ptr()).onPageEnd {
                cb(tag.as_ptr().cast(), Some(on_page_end), user_data);
            }
        }

        Ok(())
    }

    pub fn on_destroy(&self, web_tag: String, user_data: *mut c_void) -> Result<(), ArkWebError> {
        self.check_member_missing("onDestroy")?;

        let tag = CString::new(web_tag).expect("Failed to create CString");

        unsafe {
            if let Some(cb) = (*self.raw.as_ptr()).onDestroy {
                cb(tag.as_ptr().cast(), Some(on_destroy), user_data);
            }
        }

        Ok(())
    }
}

unsafe impl Send for Component {}
unsafe impl Sync for Component {}

pub static ARK_WEB_COMPONENT_API: LazyLock<Component> = LazyLock::new(Component::new);
