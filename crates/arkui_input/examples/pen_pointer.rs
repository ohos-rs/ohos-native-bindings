//! Native API 24 allocation and value round-trip smoke test.
//! Run on an OHOS API 24+ runtime. This does not inject system input or validate
//! hardware pen delivery, hover, eraser recognition, or node dispatch.
use ohos_arkui_input_binding::{
    ArkUIInputEvent, ArkUIOwnedPointerEvent, InteractionHand, UIInputAction, UIInputSourceType,
    UIInputToolType, UIMouseEventAction, UIMouseEventButton, UIPointerEventAction,
    UIPointerEventType,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", run_probe()?);
    Ok(())
}

/// Can also be called from an application's UI thread on restricted runtimes.
pub fn run_probe() -> Result<String, Box<dyn std::error::Error>> {
    let mut event = ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch)?;
    // Select the last allocated contact: current ArkUI pressure/tilt getters
    // read the final contact even when passed another pointer index.
    let pointer_index = ArkUIInputEvent::from_raw(event.raw())
        .pointer_count()
        .checked_sub(1)
        .ok_or("new touch event has no contacts")? as usize;
    event.set_source_type(UIInputSourceType::TouchScreen)?;
    event.set_tool_type(UIInputToolType::Pen)?;
    event.set_action(UIPointerEventAction::Touch(UIInputAction::Move))?;
    event.set_event_time(9_000_000_000)?;
    event.set_device_id(42)?;
    event.set_target_display_id(0)?;
    event.set_changed_finger_id(42)?;
    event.set_finger_id_by_index(42, pointer_index)?;
    event.set_changed_window_position(120.0, 180.0)?;
    event.set_window_position_by_index(120.0, 180.0, pointer_index)?;
    event.set_changed_screen_position(220.0, 280.0)?;
    event.set_screen_position_by_index(220.0, 280.0, pointer_index)?;
    event.set_changed_global_display_position(220.0, 280.0)?;
    event.set_global_display_position_by_index(220.0, 280.0, pointer_index)?;
    event.set_handle_id(1)?;
    event.set_pressure(1.5)?;
    event.set_pressure_by_index(1.5, pointer_index)?;
    event.set_tilt_angle(30.0, -45.0)?;
    event.set_roll_angle(275.0)?;
    event.set_changed_touch_area(4.0, 6.0)?;
    event.set_touch_area_by_index(4.0, 6.0, pointer_index)?;
    event.set_changed_interaction_hand(InteractionHand::Right)?;
    event.set_interaction_hand_by_index(InteractionHand::Right, pointer_index)?;
    event.set_pressed_time_by_index(8_000_000_000, pointer_index)?;
    event.set_pressed_keys(&[2017])?;
    let (tilt_readback_matches, rotation_readback_matches) = {
        // The borrowed facade is only used while its native owner is alive.
        let view = ArkUIInputEvent::from_raw(event.raw());
        assert_eq!(view.tool_type, UIInputToolType::Pen);
        assert_eq!(view.event_time(), 9_000_000_000);
        assert_eq!(view.device_id(), 42);
        assert_eq!(view.pointer_id(pointer_index as u32), 42);
        assert_eq!(view.pointer_pressure(pointer_index as u32), 1.5);
        // New native events do not necessarily expose the same fields through
        // these getters that the API 24 setters write. Report, never fabricate,
        // round-trip coverage for tilt and rotation.
        (
            view.pointer_tilt_x(pointer_index as u32) == 30.0
                && view.pointer_tilt_y(pointer_index as u32) == -45.0,
            view.pointer_roll_angle()? == 275.0,
        )
    };
    let mut clone = event.try_clone()?;
    drop(event);
    clone.set_pressure_by_index(0.75, pointer_index)?;
    assert_eq!(
        ArkUIInputEvent::from_raw(clone.raw()).pointer_pressure(pointer_index as u32),
        0.75
    );
    drop(clone);

    let mut mouse = ArkUIOwnedPointerEvent::new(UIPointerEventType::Mouse)?;
    mouse.set_action(UIPointerEventAction::Mouse(UIMouseEventAction::Move))?;
    mouse.set_mouse_button(UIMouseEventButton::Right)?;
    mouse.set_pressed_buttons(&[UIMouseEventButton::Left, UIMouseEventButton::Right])?;
    mouse.set_raw_delta_x(12.5)?;
    mouse.set_raw_delta_y(-8.25)?;
    drop(mouse);

    let mut axis = ArkUIOwnedPointerEvent::new(UIPointerEventType::Axis)?;
    axis.set_pinch_axis_scale_value(1.25)?;
    axis.set_horizontal_axis_scale_value(2.5)?;
    axis.set_vertical_axis_scale_value(-3.5)?;
    axis.set_scroll_step(2)?;
    drop(axis);
    Ok(format!(
        "PASS: 33 setters, pressure readback, independent clone/drop; tilt_readback_matches={tilt_readback_matches}; rotation_readback_matches={rotation_readback_matches}"
    ))
}
