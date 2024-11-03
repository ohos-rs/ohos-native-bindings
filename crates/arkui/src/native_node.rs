use ohos_arkui_sys::ArkUI_NodeContentHandle;

pub trait NativeNode {
    fn remove_child(&mut self) {}
    fn add_child(&mut self) {}
    fn insert_child(&mut self) {}
}

pub struct RootNode {
    child: Vec<Box<dyn NativeNode>>,
    handle: ArkUI_NodeContentHandle
}

impl RootNode {}
