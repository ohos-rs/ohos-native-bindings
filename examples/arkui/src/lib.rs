use napi_derive_ohos::napi;
use napi_ohos::Result;
use ohos_arkui_binding::{
    ArkUICommonAttribute, ArkUICommonFontAttribute, ArkUIErrorCode, ArkUIHandle, List, ListItem,
    RootNode, Text, TextAlignment,
};

#[napi]
struct MyApp {
    root: RootNode,
}

#[napi]
impl MyApp {
    #[napi(constructor)]
    pub fn new(#[napi(ts_arg_type = "NodeContent")] slot: ArkUIHandle) -> Self {
        Self {
            root: RootNode::new(slot),
        }
    }

    #[napi]
    pub fn create_native_node(&mut self) -> Result<(), ArkUIErrorCode> {
        let mut list = List::new()?;

        list.set_percent_width(1.0)?;
        list.set_percent_height(1.0)?;

        for i in 0..30 {
            let mut list_item = ListItem::new()?;
            let text = Text::new()?;

            text.set_content(i.to_string())?;
            text.set_font_size(20.0)?;
            text.set_percent_width(1.0)?;
            text.set_height(100.0)?;
            text.set_background_color(0xFFfffacd)?;
            text.set_alignment(TextAlignment::Center)?;

            list_item.add_child(text)?;

            list.add_child(list_item)?;
        }

        self.root.mount(list)?;
        Ok(())
    }

    #[napi]
    pub fn destroy_native_node(&mut self) -> Result<(), ArkUIErrorCode> {
        self.root.unmount()?;
        Ok(())
    }
}
