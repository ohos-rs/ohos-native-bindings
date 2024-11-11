use napi_derive_ohos::napi;
use napi_ohos::Result;
use ohos_arkui_binding::{
    ArkUICommonAttribute, ArkUICommonFontAttribute, ArkUIErrorCode, ArkUIEvent, ArkUIHandle,
    Dialog, List, ListItem, RootNode, Text, TextAlignment,
};
use ohos_hilog_binding::hilog_info;

#[napi]
struct MyApp {
    root: RootNode,
    dialog: Option<Dialog>,
}

#[napi]
impl MyApp {
    #[napi(constructor)]
    pub fn new(#[napi(ts_arg_type = "NodeContent")] slot: ArkUIHandle) -> Self {
        Self {
            root: RootNode::new(slot),
            dialog: None,
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

            list_item.on_click(move || hilog_info!(format!("ohos-rs: click event {i}")));

            list.add_child(list_item)?;
        }

        self.root.mount(list)?;
        Ok(())
    }

    #[napi]
    pub fn show_dialog(&mut self) -> Result<(), ArkUIErrorCode> {
        let dialog = Dialog::new()?;

        let text = Text::new()?;
        text.set_content("rs dialog")?;

        dialog.set_content(text)?;
        dialog.set_auto_cancel(true)?;

        dialog.on_will_dismiss(|_| hilog_info!("ohos-rs: dialog will dismiss"))?;

        dialog.show()?;

        self.dialog = Some(dialog);

        Ok(())
    }

    #[napi]
    pub fn destroy_native_node(&mut self) -> Result<(), ArkUIErrorCode> {
        self.root.unmount()?;
        Ok(())
    }
}
