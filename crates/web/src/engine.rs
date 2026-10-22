/// Kernel generation reported by the active ArkWeb engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WebEngineVersion {
    SystemDefault,
    M114,
    M132,
    M144,
    Evergreen,
    Unknown(u32),
}

impl From<u32> for WebEngineVersion {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::SystemDefault,
            1 => Self::M114,
            2 => Self::M132,
            3 => Self::M144,
            99999 => Self::Evergreen,
            other => Self::Unknown(other),
        }
    }
}

/// Queries the active native engine without an ArkTS bridge round trip.
pub struct WebEngine;

impl WebEngine {
    /// Requires the `api-20` feature and API 20+ at runtime.
    pub fn active_version() -> WebEngineVersion {
        unsafe { ohos_web_sys::OH_NativeArkWeb_GetActiveWebEngineVersion() }.into()
    }

    /// Requires the `api-23` feature and API 23+ at runtime.
    #[cfg(feature = "api-23")]
    pub fn is_evergreen() -> bool {
        unsafe { ohos_web_sys::OH_NativeArkWeb_IsActiveWebEngineEvergreen() }
    }
}

#[cfg(test)]
mod tests {
    use super::WebEngineVersion;

    #[test]
    fn preserves_unknown_future_engine_generations() {
        assert_eq!(WebEngineVersion::from(4), WebEngineVersion::Unknown(4));
        assert_eq!(WebEngineVersion::from(99999), WebEngineVersion::Evergreen);
    }
}
