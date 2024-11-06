use napi_derive_ohos::napi;
use napi_ohos::Result;
use ohos_arkui_binding::{ArkUICommonAttribute, ArkUIHandle, RootNode, Text};

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
    pub fn create_native_node(&mut self) -> Result<()> {
        let text = Text::new()?;

        text.set_content("hello")?;
        text.set_font_size(20.0)?;
        text.set_percent_width(1.0)?;
        text.set_height(200.0)?;
        text.set_background_color(0xFFfffacd)?;
        self.root.mount(text)?;
        Ok(())
    }

    #[napi]
    pub fn destroy_native_node(&mut self) -> Result<()> {
        self.root.unmount()?;
        Ok(())
    }
}
