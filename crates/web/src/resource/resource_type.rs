use ohos_web_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkWeb_ResourceType, "ArkWeb_ResourceType_")]
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
}
