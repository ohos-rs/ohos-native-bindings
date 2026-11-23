use std::{
    ffi::{c_char, CStr, CString},
    ptr::NonNull,
    sync::LazyLock,
};

use ohos_web_sys::{
    ArkWeb_CookieManagerAPI, ArkWeb_ErrorCode_ARKWEB_SUCCESS,
    ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_COOKIE_MANAGER, OH_ArkWeb_GetNativeAPI,
    OH_ArkWeb_ReleaseString,
};

use crate::{ark_web_member_missing, ArkWebError};

struct OwnedArkWebString(*mut c_char);

impl Drop for OwnedArkWebString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: string was allocated by ArkWeb and must be freed via `OH_ArkWeb_ReleaseString`.
            unsafe { OH_ArkWeb_ReleaseString(self.0) };
        }
    }
}

/// Safe wrapper around `ArkWeb_CookieManagerAPI` (API 12+).
pub struct CookieManager {
    raw: Option<NonNull<ArkWeb_CookieManagerAPI>>,
}

impl CookieManager {
    /// Retrieves the native ArkWeb CookieManager API table. Must be initialized on the UI thread.
    pub fn new() -> Self {
        let ret = unsafe {
            OH_ArkWeb_GetNativeAPI(ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_COOKIE_MANAGER)
                as *mut ArkWeb_CookieManagerAPI
        };
        #[cfg(debug_assertions)]
        assert!(
            !ret.is_null(),
            "Failed to get CookieManager API by OH_ArkWeb_GetNativeAPI"
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
        let present = match member {
            "fetchCookieSync" => unsafe { !ark_web_member_missing!(ptr, fetchCookieSync) },
            "configCookieSync" => unsafe { !ark_web_member_missing!(ptr, configCookieSync) },
            "existCookies" => unsafe { !ark_web_member_missing!(ptr, existCookies) },
            "clearAllCookiesSync" => unsafe { !ark_web_member_missing!(ptr, clearAllCookiesSync) },
            "clearSessionCookiesSync" => unsafe {
                !ark_web_member_missing!(ptr, clearSessionCookiesSync)
            },
            _ => return Err(ArkWebError::ArkWebApiMemberMissing(member.to_string())),
        };
        if present {
            Ok(())
        } else {
            Err(ArkWebError::ArkWebApiMemberMissing(member.to_string()))
        }
    }

    /// Synchronously fetches cookies for `url`.
    pub fn fetch_cookie_sync(
        &self,
        url: &str,
        incognito: bool,
        include_http_only: bool,
    ) -> Result<String, ArkWebError> {
        self.check_member_missing("fetchCookieSync")?;
        let raw = self
            .raw
            .ok_or_else(|| ArkWebError::ArkWebApiMemberMissing("fetchCookieSync".to_string()))?;
        let fetch = unsafe { (*raw.as_ptr()).fetchCookieSync }
            .ok_or_else(|| ArkWebError::ArkWebApiMemberMissing("fetchCookieSync".to_string()))?;
        let url = CString::new(url).map_err(|_| ArkWebError::NativeError {
            operation: "fetchCookieSync",
            code: ohos_web_sys::ArkWeb_ErrorCode_ARKWEB_INVALID_URL,
        })?;
        let mut out: *mut c_char = std::ptr::null_mut();
        // SAFETY: `url` is NUL-terminated and `out` is writable; any returned buffer
        // is freed by `OwnedArkWebString`.
        let code = unsafe { fetch(url.as_ptr(), incognito, include_http_only, &mut out) };
        let guard = OwnedArkWebString(out);
        if code != ArkWeb_ErrorCode_ARKWEB_SUCCESS {
            return Err(ArkWebError::NativeError {
                operation: "fetchCookieSync",
                code,
            });
        }
        if guard.0.is_null() {
            return Ok(String::new());
        }
        let cookie = unsafe { CStr::from_ptr(guard.0) }
            .to_string_lossy()
            .into_owned();
        Ok(cookie)
    }

    /// Synchronously sets a cookie value for `url`.
    pub fn config_cookie_sync(
        &self,
        url: &str,
        cookie_value: &str,
        incognito: bool,
        include_http_only: bool,
    ) -> Result<(), ArkWebError> {
        self.check_member_missing("configCookieSync")?;
        let raw = self
            .raw
            .ok_or_else(|| ArkWebError::ArkWebApiMemberMissing("configCookieSync".to_string()))?;
        let config = unsafe { (*raw.as_ptr()).configCookieSync }
            .ok_or_else(|| ArkWebError::ArkWebApiMemberMissing("configCookieSync".to_string()))?;
        let url = CString::new(url).map_err(|_| ArkWebError::NativeError {
            operation: "configCookieSync",
            code: ohos_web_sys::ArkWeb_ErrorCode_ARKWEB_INVALID_URL,
        })?;
        let value = CString::new(cookie_value).map_err(|_| ArkWebError::NativeError {
            operation: "configCookieSync",
            code: ohos_web_sys::ArkWeb_ErrorCode_ARKWEB_INVALID_COOKIE_VALUE,
        })?;
        let code = unsafe { config(url.as_ptr(), value.as_ptr(), incognito, include_http_only) };
        if code == ArkWeb_ErrorCode_ARKWEB_SUCCESS {
            Ok(())
        } else {
            Err(ArkWebError::NativeError {
                operation: "configCookieSync",
                code,
            })
        }
    }

    /// Returns whether any cookies exist in the selected cookie jar.
    pub fn exist_cookies(&self, incognito: bool) -> Result<bool, ArkWebError> {
        self.check_member_missing("existCookies")?;
        let raw = self
            .raw
            .ok_or_else(|| ArkWebError::ArkWebApiMemberMissing("existCookies".to_string()))?;
        let exist = unsafe { (*raw.as_ptr()).existCookies }
            .ok_or_else(|| ArkWebError::ArkWebApiMemberMissing("existCookies".to_string()))?;
        Ok(unsafe { exist(incognito) })
    }

    /// Synchronously clears all cookies in the selected cookie jar.
    pub fn clear_all_cookies_sync(&self, incognito: bool) -> Result<(), ArkWebError> {
        self.check_member_missing("clearAllCookiesSync")?;
        let raw = self.raw.ok_or_else(|| {
            ArkWebError::ArkWebApiMemberMissing("clearAllCookiesSync".to_string())
        })?;
        let clear = unsafe { (*raw.as_ptr()).clearAllCookiesSync }.ok_or_else(|| {
            ArkWebError::ArkWebApiMemberMissing("clearAllCookiesSync".to_string())
        })?;
        unsafe { clear(incognito) };
        Ok(())
    }

    /// Synchronously clears all session cookies.
    pub fn clear_session_cookies_sync(&self) -> Result<(), ArkWebError> {
        self.check_member_missing("clearSessionCookiesSync")?;
        let raw = self.raw.ok_or_else(|| {
            ArkWebError::ArkWebApiMemberMissing("clearSessionCookiesSync".to_string())
        })?;
        let clear = unsafe { (*raw.as_ptr()).clearSessionCookiesSync }.ok_or_else(|| {
            ArkWebError::ArkWebApiMemberMissing("clearSessionCookiesSync".to_string())
        })?;
        unsafe { clear() };
        Ok(())
    }

    /// Persists all cookies to disk synchronously (API 20).
    #[cfg(feature = "api-20")]
    pub fn save_cookie_sync() -> Result<(), ArkWebError> {
        let code = unsafe { ohos_web_sys::OH_ArkWebCookieManager_SaveCookieSync() };
        if code == ArkWeb_ErrorCode_ARKWEB_SUCCESS {
            Ok(())
        } else {
            Err(ArkWebError::NativeError {
                operation: "saveCookieSync",
                code,
            })
        }
    }
}

// SAFETY: `ArkWeb_CookieManagerAPI` is a static native vtable pointer.
unsafe impl Send for CookieManager {}
unsafe impl Sync for CookieManager {}

impl Default for CookieManager {
    fn default() -> Self {
        Self::new()
    }
}

pub static ARK_WEB_COOKIE_MANAGER_API: LazyLock<CookieManager> = LazyLock::new(CookieManager::new);
