use ohos_web_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkWeb_ResourceType, "ArkWeb_ResourceType_")]
#[repr(u32)]
pub enum ResourceType {
    MainFrame = 0,
    SubFrame = 1,
    StyleSheet = 2,
    Script = 3,
    Image = 4,
    FontResource = 5,
    SubResource = 6,
    Object = 7,
    Media = 8,
    Worker = 9,
    SharedWorker = 10,
    Prefetch = 11,
    Favicon = 12,
    Xhr = 13,
    Ping = 14,
    ServiceWorker = 15,
    CspReport = 16,
    PluginResource = 17,
    NavigationPreloadMainFrame = 19,
    NavigationPreloadSubFrame = 20,
    #[fallback]
    Unknown(u32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NetError;

    #[test]
    fn preserves_unknown_resource_types_and_net_errors() {
        assert_eq!(
            ResourceType::from(-1i32 as u32),
            ResourceType::Unknown(u32::MAX)
        );
        assert_eq!(ResourceType::try_from_raw(u32::MAX), None);
        assert_eq!(NetError::from(-9999), NetError::Unknown(-9999));
        assert_eq!(ArkWeb_NetError::from(NetError::Unknown(-9999)), -9999);
    }
}
