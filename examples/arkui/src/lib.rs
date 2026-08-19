#![allow(clippy::all)]
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use napi_derive_ohos::napi;
use napi_ohos::Result;
use ohos_arkui_binding::{
    animate::options::Animation,
    common::ui_context::ArkUIContext,
    component::{
        attribute::{ArkUICommonAttribute, ArkUICommonFontAttribute, ArkUIEvent, ArkUIGesture},
        built_in_component::{
            Button, Checkbox, List, ListItem, Progress, Radio, Slider, Stack, Swiper, Text,
            TextInput, Toggle,
        },
    },
    dialog::Dialog,
    gesture::inner_gesture::Gesture,
    types::{
        animation_mode::AnimationMode, curve::Curve, gesture_direction::GestureDirection,
        gesture_event::GestureEventAction, text_alignment::TextAlignment,
    },
    ArkUIHandle, RootNode,
};
use ohos_arkui_input_binding::ArkUIErrorCode;
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

            list_item.add_child(text.into_node())?;

            list_item.on_click(move || hilog_info!(format!("ohos-rs: click event {i}")));

            list.add_child(list_item.into_node())?;
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
        animation_test.update(move || {
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

    /// A gallery page exercising more built-in components, gestures and
    /// events: Button, Toggle, Slider, Progress, Checkbox, Radio, Swiper,
    /// Stack, plus tap/pan/pinch/swipe/rotation gestures and hover/focus/
    /// appear events.
    #[napi]
    pub fn create_gallery(&mut self) -> Result<(), ArkUIErrorCode> {
        let mut list = List::new()?;
        list.percent_width(1.0)?;
        list.percent_height(1.0)?;

        // --- section: buttons with click/hover/focus/appear events ------
        for label in ["tap me", "hover me", "focus me"] {
            let mut item = ListItem::new()?;
            let mut button = Button::new()?;
            button.set_button_label(label)?;
            button.width(200.0)?;
            button.height(48.0)?;

            let tag = label.to_string();
            button.on_click(move || hilog_info!("ohos-rs: button click: {tag}"));
            let hover_tag = label.to_string();
            button.on_hover(move |hover| {
                hilog_info!("ohos-rs: button hover: {hover_tag} = {hover}");
            });
            let focus_tag = label.to_string();
            button.on_focus(move || hilog_info!("ohos-rs: button focus: {focus_tag}"));
            let appear_tag = label.to_string();
            button.on_appear(move || hilog_info!("ohos-rs: button appear: {appear_tag}"));

            item.add_child(button.into_node())?;
            list.add_child(item.into_node())?;
        }

        // --- section: slider with change gesture ------------------------
        {
            let mut item = ListItem::new()?;
            let slider = Slider::new()?;
            slider.width(250.0)?;
            let pan = Gesture::create_pan_gesture(1, GestureDirection::All, 5.0)?;
            pan.on_gesture(
                GestureEventAction::Accept | GestureEventAction::Update | GestureEventAction::End,
                |_| {
                hilog_info!("ohos-rs: pan gesture on slider");
            })?;
            slider.add_gesture(pan, None, None)?;
            item.add_child(slider.into_node())?;
            list.add_child(item.into_node())?;
        }

        // --- section: toggle / progress / checkbox / radio ---------------
        {
            let mut item = ListItem::new()?;
            let mut toggle = Toggle::new()?;
            toggle.set_toggle_value(true)?;
            toggle.width(60.0)?;
            toggle.on_click(|| hilog_info!("ohos-rs: toggle clicked"));
            item.add_child(toggle.into_node())?;
            list.add_child(item.into_node())?;
        }
        {
            let mut item = ListItem::new()?;
            let progress = Progress::new()?;
            progress.width(250.0)?;
            progress.set_progress_value(40.0)?;
            item.add_child(progress.into_node())?;
            list.add_child(item.into_node())?;
        }
        {
            let mut item = ListItem::new()?;
            let mut checkbox = Checkbox::new()?;
            checkbox.set_checkbox_select(true)?;
            checkbox.set_checkbox_shape(1)?;
            checkbox.width(40.0)?;
            checkbox.on_click(|| hilog_info!("ohos-rs: checkbox clicked"));
            item.add_child(checkbox.into_node())?;
            list.add_child(item.into_node())?;
        }
        {
            let mut item = ListItem::new()?;
            let radio = Radio::new()?;
            radio.set_radio_checked(true)?;
            radio.width(40.0)?;
            item.add_child(radio.into_node())?;
            list.add_child(item.into_node())?;
        }

        // --- section: swiper with 3 pages --------------------------------
        {
            let mut item = ListItem::new()?;
            let mut swiper = Swiper::new()?;
            swiper.height(120.0)?;
            swiper.percent_width(1.0)?;
            for page in ["page 1", "page 2", "page 3"] {
                let text = Text::new()?;
                text.content(page)?;
                text.font_size(20.0)?;
                text.height(100.0)?;
                text.percent_width(1.0)?;
                text.background_color(0xFFe8eaf6)?;
                text.alignment(TextAlignment::Center)?;
                swiper.add_child(text.into_node())?;
            }
            let swipe = Gesture::create_swipe_gesture(1, GestureDirection::Horizontal, 100.0)?;
            swipe.on_gesture(
                GestureEventAction::Accept | GestureEventAction::Update | GestureEventAction::End,
                |_| {
                hilog_info!("ohos-rs: swipe gesture on swiper");
            })?;
            swiper.add_gesture(swipe, None, None)?;
            item.add_child(swiper.into_node())?;
            list.add_child(item.into_node())?;
        }

        // --- section: pinch + rotation on a stack ------------------------
        {
            let mut item = ListItem::new()?;
            let mut stack = Stack::new()?;
            stack.height(120.0)?;
            stack.percent_width(1.0)?;
            let pin = Gesture::create_pinch_gesture(2, 5.0)?;
            pin.on_gesture(
                GestureEventAction::Accept | GestureEventAction::Update | GestureEventAction::End,
                |_| {
                hilog_info!("ohos-rs: pinch gesture on stack");
            })?;
            stack.add_gesture(pin, None, None)?;
            let rot = Gesture::create_rotation_gesture(2, 1.0)?;
            rot.on_gesture(
                GestureEventAction::Accept | GestureEventAction::Update | GestureEventAction::End,
                |_| {
                hilog_info!("ohos-rs: rotation gesture on stack");
            })?;
            stack.add_gesture(rot, None, None)?;

            let text = Text::new()?;
            text.content("pinch / rotate here")?;
            text.font_size(16.0)?;
            text.height(100.0)?;
            text.percent_width(1.0)?;
            text.background_color(0xFFfff0f0)?;
            text.alignment(TextAlignment::Center)?;
            stack.add_child(text.into_node())?;

            item.add_child(stack.into_node())?;
            list.add_child(item.into_node())?;
        }

        self.root.mount(list)?;
        Ok(())
    }
}
