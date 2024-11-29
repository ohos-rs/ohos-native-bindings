use std::{cell::RefCell, rc::Rc};

use napi_derive_ohos::napi;
use napi_ohos::Result;
use ohos_arkui_binding::{
    Animation, AnimationMode, ArkUICommonAttribute, ArkUICommonFontAttribute, ArkUIContext,
    ArkUIErrorCode, ArkUIEvent, ArkUIGesture, ArkUIHandle, Curve, Dialog, Gesture,
    GestureEventAction, List, ListItem, RootNode, Text, TextAlignment, TextInput,
};
use ohos_hilog_binding::hilog_info;

#[napi]
struct MyApp {
    root: RootNode,
    dialog: Option<Dialog>,
    input: Rc<RefCell<Option<TextInput>>>,
}

#[napi]
impl MyApp {
    #[napi(constructor)]
    pub fn new(#[napi(ts_arg_type = "NodeContent")] slot: ArkUIHandle) -> Self {
        Self {
            root: RootNode::new(slot),
            dialog: None,
            input: Rc::new(RefCell::new(None)),
        }
    }

    #[napi]
    pub fn create_native_node(&mut self) -> Result<(), ArkUIErrorCode> {
        let mut list = List::new()?;

        list.percent_width(1.0)?;
        list.percent_height(1.0)?;

        for i in 0..30 {
            let mut list_item = ListItem::new()?;
            let text = Text::new()?;

            let long_gesture = Gesture::create_long_gesture(1, true, 1000)?;

            let a =
                GestureEventAction::Accept | GestureEventAction::Update | GestureEventAction::End;

            long_gesture.on_gesture(a, |_| {
                hilog_info!("ohos-rs: long gesture");
            })?;

            list_item.add_gesture(long_gesture, None, None)?;

            text.content(i.to_string())?;
            text.font_size(20.0)?;
            text.percent_width(1.0)?;
            text.height(100.0)?;
            text.background_color(0xFFfffacd)?;
            text.alignment(TextAlignment::Center)?;

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
        text.content("rs dialog")?;

        dialog.content(text)?;
        dialog.auto_cancel(true)?;

        dialog.on_will_dismiss(|_| {
            hilog_info!("ohos-rs: dialog will dismiss");
            Some(true)
        })?;

        dialog.show()?;

        self.dialog = Some(dialog);

        Ok(())
    }

    #[napi]
    pub fn create_text_input(&mut self) -> Result<(), ArkUIErrorCode> {
        let input = TextInput::new()?;
        self.input.replace(Some(input));

        let i = self.input.borrow_mut();
        if let Some(i) = i.as_ref() {
            self.root.mount(i.clone())?;
        }

        Ok(())
    }

    #[napi]
    pub fn animation(&mut self, ctx: ArkUIContext) -> Result<(), ArkUIErrorCode> {
        let animation_test = Animation::new();
        animation_test.duration(2000);
        animation_test.delay(20);
        animation_test.tempo(1.1);
        animation_test.iterations(1);
        animation_test.curve(Curve::Ease);
        animation_test.mode(AnimationMode::Normal);

        let input = self.input.borrow_mut().clone();
        animation_test.update(std::ptr::null_mut(), move |_| {
            if let Some(input) = input.as_ref() {
                input.width(200.0).unwrap();
                input.height(200.0).unwrap();
            }
        });

        animation_test.animate_to(ctx)?;

        Ok(())
    }

    #[napi]
    pub fn destroy_native_node(&mut self) -> Result<(), ArkUIErrorCode> {
        self.root.unmount()?;
        Ok(())
    }
}
