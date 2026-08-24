#![allow(clippy::all)]
#![allow(dead_code)]

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{LazyLock, Mutex},
};

use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_arkui_binding::{
    animate::options::Animation,
    common::ui_context::ArkUIContext,
    component::{
        attribute::{ArkUICommonAttribute, ArkUICommonFontAttribute, ArkUIEvent, ArkUIGesture},
        built_in_component::{
            Button, Checkbox, List, ListItem, Progress, Radio, Slider, Stack, Swiper, Text,
            TextInput, Toggle, XComponent,
        },
    },
    dialog::Dialog,
    gesture::{gesture_data::GestureData, inner_gesture::Gesture},
    types::{
        animation_mode::AnimationMode, curve::Curve, gesture_direction::GestureDirection,
        gesture_event::GestureEventAction, text_alignment::TextAlignment,
    },
    ArkUIHandle, RootNode,
};
use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_hilog_binding::hilog_info;
use ohos_xcomponent_binding::TouchEvent;

#[napi]
struct MyApp {
    root: RootNode,
    dialog: Option<Dialog>,
    input: Rc<RefCell<Option<TextInput>>>,
    xcomponent_gesture_node: Option<XComponent>,
    xcomponent_gesture_handles: Vec<Gesture>,
}

#[derive(Default)]
struct XComponentGestureEvents {
    raw_downs: u32,
    raw_moves: u32,
    raw_ups: u32,
    raw_cancels: u32,
    taps: u32,
    pan_accepts: u32,
    pan_updates: u32,
    pan_ends: u32,
    pan_cancels: u32,
    swipes: u32,
    last: String,
}

static XCOMPONENT_GESTURE_EVENTS: LazyLock<Mutex<XComponentGestureEvents>> =
    LazyLock::new(|| Mutex::new(XComponentGestureEvents::default()));

#[napi]
impl MyApp {
    #[napi(constructor)]
    pub fn new(#[napi(ts_arg_type = "NodeContent")] slot: ArkUIHandle) -> Self {
        Self {
            root: RootNode::new(slot),
            dialog: None,
            input: Rc::new(RefCell::new(None)),
            xcomponent_gesture_node: None,
            xcomponent_gesture_handles: Vec::new(),
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
        self.release_xcomponent_gestures();
        self.root.unmount()?;
        Ok(())
    }

    /// Mount one ArkUI-native XComponent input surface.
    ///
    /// The same node exposes the original NativeXComponent touch stream and
    /// uses ArkUI's system gesture recognizers.
    #[napi]
    pub fn create_xcomponent_gesture_demo(&mut self) -> Result<(), ArkUIErrorCode> {
        let xcomponent = XComponent::new()?;
        self.configure_xcomponent_gestures(&xcomponent)?;
        self.root.mount(xcomponent)?;
        Ok(())
    }

    /// A gallery page exercising more built-in components, gestures and
    /// events: Button, Toggle, Slider, Progress, Checkbox, Radio, Swiper,
    /// Stack, plus tap/pan/pinch/swipe/rotation gestures and hover/focus/
    /// appear events.
    #[napi]
    pub fn create_gallery(&mut self) -> Result<(), ArkUIErrorCode> {
        reset_xcomponent_gesture_events();

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
                },
            )?;
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
                },
            )?;
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
                },
            )?;
            stack.add_gesture(pin, None, None)?;
            let rot = Gesture::create_rotation_gesture(2, 1.0)?;
            rot.on_gesture(
                GestureEventAction::Accept | GestureEventAction::Update | GestureEventAction::End,
                |_| {
                    hilog_info!("ohos-rs: rotation gesture on stack");
                },
            )?;
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

        // --- section: native input + system gestures on one XComponent ----
        {
            let mut item = ListItem::new()?;
            let xcomponent = XComponent::new()?;
            self.configure_xcomponent_gestures(&xcomponent)?;
            item.add_child(xcomponent.into_node())?;
            list.add_child(item.into_node())?;
        }

        self.root.mount(list)?;
        Ok(())
    }

    /// Read the raw touch and ArkUI system gesture state from ArkTS/E2E.
    #[napi]
    pub fn xcomponent_gesture_events(&self) -> String {
        xcomponent_gesture_events()
    }

    /// Clear the raw touch and ArkUI system gesture counters.
    #[napi]
    pub fn reset_xcomponent_gesture_events(&self) {
        reset_xcomponent_gesture_events();
    }
}

impl MyApp {
    fn configure_xcomponent_gestures(
        &mut self,
        xcomponent: &XComponent,
    ) -> Result<(), ArkUIErrorCode> {
        reset_xcomponent_gesture_events();

        xcomponent.set_x_component_id("arkui-xcomponent-gesture-demo")?;
        xcomponent.percent_width(1.0)?;
        xcomponent.height(180.0)?;
        xcomponent.background_color(0xFFd9edf7)?;

        // This is the same construction used by openharmony-ability: build an
        // ArkUI XComponent node, obtain its native handle, then keep both the
        // raw touch callback and ArkUI recognizers attached to that one node.
        let native = xcomponent.native_xcomponent();
        native.on_touch_event(move |_xcomponent, _window, event| {
            let mut state = XCOMPONENT_GESTURE_EVENTS.lock().unwrap();
            match event.event_type {
                TouchEvent::Down => state.raw_downs += 1,
                TouchEvent::Move => state.raw_moves += 1,
                TouchEvent::Up => state.raw_ups += 1,
                TouchEvent::Cancel => state.raw_cancels += 1,
                TouchEvent::Unknown => {}
            }
            hilog_info!(format!(
                "ohos-rs: native XComponent touch {:?}",
                event.event_type
            ));
            Ok(())
        });
        native.register_callback().map_err(|error| {
            Error::new(
                ArkUIErrorCode::AttributeOrEventNotSupported,
                error.to_string(),
            )
        })?;

        let tap = xcomponent.on_tap_gesture(1, 1, move |event| {
            let mut state = XCOMPONENT_GESTURE_EVENTS.lock().unwrap();
            state.taps += 1;
            state.last = if matches!(event.event_action_data, GestureData::Tap) {
                "tap".to_string()
            } else {
                "tap:unexpected-payload".to_string()
            };
            hilog_info!("ohos-rs: ArkUI XComponent tap");
        })?;

        let pan = xcomponent.on_pan_gesture(1, GestureDirection::All, 8.0, move |event| {
            let mut state = XCOMPONENT_GESTURE_EVENTS.lock().unwrap();
            if event.event_action_type.contains(GestureEventAction::Accept) {
                state.pan_accepts += 1;
            } else if event.event_action_type.contains(GestureEventAction::Update) {
                state.pan_updates += 1;
            } else if event.event_action_type.contains(GestureEventAction::End) {
                state.pan_ends += 1;
            } else if event.event_action_type.contains(GestureEventAction::Cancel) {
                state.pan_cancels += 1;
            }
            if let GestureData::Pan(pan) = event.event_action_data {
                state.last = format!("pan@{:.1},{:.1}", pan.offset_x, pan.offset_y);
            }
            hilog_info!("ohos-rs: ArkUI XComponent pan");
        })?;

        let swipe = xcomponent.on_swipe_gesture(1, GestureDirection::All, 800.0, move |event| {
            let mut state = XCOMPONENT_GESTURE_EVENTS.lock().unwrap();
            state.swipes += 1;
            if let GestureData::Swipe(swipe) = event.event_action_data {
                state.last = format!("swipe@{:.1}/{:.1}", swipe.angle, swipe.velocity);
            }
            hilog_info!("ohos-rs: ArkUI XComponent swipe");
        })?;

        self.xcomponent_gesture_node = Some(xcomponent.clone());
        self.xcomponent_gesture_handles = vec![tap, pan, swipe];
        Ok(())
    }

    fn release_xcomponent_gestures(&mut self) {
        // Detach recognizers while the XComponent node is still alive, then
        // release their callback contexts before disposing the node tree.
        if let Some(xcomponent) = self.xcomponent_gesture_node.take() {
            xcomponent.native_xcomponent().unregister_callbacks();
            for gesture in self.xcomponent_gesture_handles.drain(..) {
                if let Err(error) = xcomponent.remove_gesture(&gesture) {
                    hilog_info!("ohos-rs: remove XComponent gesture failed: {error}");
                }
                if let Err(error) = gesture.dispose() {
                    hilog_info!("ohos-rs: dispose XComponent gesture failed: {error}");
                }
            }
        }
    }
}

/// Raw native touch and ArkUI system gesture counters for UI/E2E runners.
#[napi]
pub fn xcomponent_gesture_events() -> String {
    let events = XCOMPONENT_GESTURE_EVENTS.lock().unwrap();
    format!(
        "rawDown={} rawMove={} rawUp={} rawCancel={} tap={} panAccept={} panUpdate={} panEnd={} panCancel={} swipe={} last={}",
        events.raw_downs,
        events.raw_moves,
        events.raw_ups,
        events.raw_cancels,
        events.taps,
        events.pan_accepts,
        events.pan_updates,
        events.pan_ends,
        events.pan_cancels,
        events.swipes,
        events.last
    )
}

/// Reset the raw native touch and ArkUI system gesture counters.
#[napi]
pub fn reset_xcomponent_gesture_events() {
    *XCOMPONENT_GESTURE_EVENTS.lock().unwrap() = XComponentGestureEvents::default();
}
