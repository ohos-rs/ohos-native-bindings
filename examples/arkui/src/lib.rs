use std::sync::{LazyLock, Mutex};

use napi_derive_ohos::napi;
use napi_ohos::Result;
use ohos_arkui_binding::{ArkUIHandle, RootNode, Text};

static ROOT: LazyLock<Mutex<Option<RootNode>>> = LazyLock::new(|| Mutex::new(None));

#[napi]
pub fn create_native_node(#[napi(ts_arg_type = "NodeContent")] slot: ArkUIHandle) -> Result<()> {
    let mut root = RootNode::new(slot);
    let text = Text::new()?;

    text.set_content("hello")?;
    text.set_font_size(20.0)?;
    text.set_percent_width(1.0)?;
    text.set_percent_height(1.0)?;
    text.set_background_color(0xFFfffacd)?;

    root.set_node(text.node());

    root.mount()?;

    let mut root_guard = ROOT.lock().unwrap();
    *root_guard = Some(root);
    Ok(())
}
