use ohos_udmf_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(Udmf_Intention, "Udmf_Intention_UDMF_INTENTION_")]
pub enum UdmfIntention {
    Drag = 0,
    Pasteboard,
}
