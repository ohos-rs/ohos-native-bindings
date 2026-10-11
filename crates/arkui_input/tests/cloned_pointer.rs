//! Host-only ABI test double exercising the actual public binding crate.
//! Run with tests/run-cloned-pointer.sh. This is not device validation.
#![cfg(all(feature = "api-24", not(target_env = "ohos")))]
#![allow(non_snake_case)]

use ohos_arkui_input_binding::{sys::*, *};
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq)]
enum Arg {
    Int(i64),
    Float(f64),
    List(Vec<i32>),
}
impl From<i32> for Arg {
    fn from(value: i32) -> Self {
        Self::Int(value.into())
    }
}
impl From<i64> for Arg {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
impl From<f32> for Arg {
    fn from(value: f32) -> Self {
        Self::Float(value.into())
    }
}
impl From<f64> for Arg {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

#[derive(Default)]
struct State {
    runtime_unavailable: bool,
    status: u32,
    null_success: bool,
    live: Vec<usize>,
    destroyed: Vec<usize>,
    calls: Vec<(&'static str, Vec<Arg>)>,
}
thread_local! { static STATE: RefCell<State> = RefCell::new(State::default()); }

struct NativeEvent {
    kind: u32,
}

#[no_mangle]
unsafe extern "C" fn OH_ArkUI_QueryModuleInterfaceByName(
    kind: u32,
    name: *const std::ffi::c_char,
) -> *mut std::ffi::c_void {
    assert_eq!(kind, 0);
    assert_eq!(
        unsafe { std::ffi::CStr::from_ptr(name) },
        c"ArkUI_NativeNodeAPI_1"
    );
    if STATE.with_borrow(|state| state.runtime_unavailable) {
        std::ptr::null_mut()
    } else {
        std::ptr::NonNull::<u8>::dangling().as_ptr().cast()
    }
}

fn reset() {
    STATE.with_borrow_mut(|state| {
        assert!(state.live.is_empty(), "native event leaked");
        *state = State::default();
    });
}

fn result() -> u32 {
    STATE.with_borrow_mut(|state| std::mem::take(&mut state.status))
}

unsafe fn allocate(out: *mut *mut ArkUI_UIInputEvent, kind: u32) -> u32 {
    let status = result();
    if status != 0 {
        return status;
    }
    if STATE.with_borrow_mut(|state| std::mem::take(&mut state.null_success)) {
        return 0;
    }
    let raw = Box::into_raw(Box::new(NativeEvent { kind }));
    STATE.with_borrow_mut(|state| state.live.push(raw as usize));
    unsafe {
        *out = raw.cast();
    }
    0
}

fn record(event: *const ArkUI_UIInputEvent, name: &'static str, args: Vec<Arg>) -> u32 {
    STATE.with_borrow_mut(|state| {
        assert!(state.live.contains(&(event as usize)), "use after free");
        state.calls.push((name, args));
    });
    result()
}

#[no_mangle]
unsafe extern "C" fn OH_ArkUI_PointerEvent_CreatePointerEvent(
    out: *mut *mut ArkUI_UIInputEvent,
    kind: u32,
) -> u32 {
    unsafe { allocate(out, kind) }
}

#[no_mangle]
unsafe extern "C" fn OH_ArkUI_UIInputEvent_GetType(event: *const ArkUI_UIInputEvent) -> i32 {
    unsafe { (*event.cast::<NativeEvent>()).kind as i32 }
}

#[no_mangle]
unsafe extern "C" fn OH_ArkUI_PointerEvent_CreateClonedPointerEvent(
    event: *const ArkUI_UIInputEvent,
    out: *mut *mut ArkUI_UIInputEvent,
) -> u32 {
    STATE.with_borrow_mut(|state| {
        assert!(state.live.contains(&(event as usize)));
        state.calls.push(("clone", vec![]));
    });
    unsafe { allocate(out, (*event.cast::<NativeEvent>()).kind) }
}

#[no_mangle]
unsafe extern "C" fn OH_ArkUI_PointerEvent_DestroyClonedPointerEvent(
    event: *const ArkUI_UIInputEvent,
) -> u32 {
    STATE.with_borrow_mut(|state| {
        let index = state
            .live
            .iter()
            .position(|raw| *raw == event as usize)
            .expect("double free");
        state.live.swap_remove(index);
        state.destroyed.push(event as usize);
    });
    unsafe {
        drop(Box::from_raw(event.cast_mut().cast::<NativeEvent>()));
    }
    0
}

#[no_mangle]
unsafe extern "C" fn OH_ArkUI_PointerEvent_PostClonedEventWithStrategy(
    node: ArkUI_NodeHandle,
    event: *const ArkUI_UIInputEvent,
    strategy: u32,
) -> u32 {
    assert!(!node.is_null());
    record(event, "post", vec![(strategy as i32).into()])
}

macro_rules! setter {
    ($name:ident, $($arg:ident: $ty:ty),+) => {
        #[no_mangle]
        unsafe extern "C" fn $name(event: *const ArkUI_UIInputEvent, $($arg: $ty),+) -> u32 {
            record(event, stringify!($name), vec![$($arg.into()),+])
        }
    };
}

setter!(OH_ArkUI_ClonedEvent_SetActionType, type_: i32);
setter!(OH_ArkUI_ClonedEvent_SetSourceType, sourceType: i32);
setter!(OH_ArkUI_ClonedEvent_SetToolType, toolType: i32);
setter!(OH_ArkUI_ClonedEvent_SetPressure, pressure: f32);
setter!(OH_ArkUI_ClonedEvent_SetPressureByIndex, pressure: f32, pointerIndex: i32);
setter!(OH_ArkUI_ClonedEvent_SetEventTime, timestamp: i64);
setter!(OH_ArkUI_ClonedEvent_SetDeviceId, deviceId: i32);
setter!(OH_ArkUI_ClonedEvent_SetTargetDisplayId, targetDisplayId: i32);
setter!(OH_ArkUI_ClonedEvent_SetChangedFingerId, fingerId: i32);
setter!(OH_ArkUI_ClonedEvent_SetFingerIdByIndex, fingerId: i32, pointerIndex: i32);
setter!(OH_ArkUI_ClonedEvent_SetChangedWindowPosition, x: f32, y: f32);
setter!(OH_ArkUI_ClonedEvent_SetWindowPositionByIndex, x: f32, y: f32, pointerIndex: i32);
setter!(OH_ArkUI_ClonedEvent_SetChangedScreenPosition, x: f32, y: f32);
setter!(OH_ArkUI_ClonedEvent_SetScreenPositionByIndex, x: f32, y: f32, pointerIndex: i32);
setter!(OH_ArkUI_ClonedEvent_SetChangedGlobalDisplayPosition, x: f32, y: f32);
setter!(OH_ArkUI_ClonedEvent_SetGlobalDisplayPositionByIndex, x: f32, y: f32, pointerIndex: i32);
setter!(OH_ArkUI_ClonedEvent_SetHandleId, eventHandleId: i32);
setter!(OH_ArkUI_ClonedEvent_SetTiltAngle, tiltX: f32, tiltY: f32);
setter!(OH_ArkUI_ClonedEvent_SetRollAngle, rollAngle: f32);
setter!(OH_ArkUI_ClonedEvent_SetChangedTouchArea, width: f32, height: f32);
setter!(OH_ArkUI_ClonedEvent_SetTouchAreaByIndex, width: f32, height: f32, pointerIndex: i32);
setter!(OH_ArkUI_ClonedEvent_SetChangedInteractionHand, hand: i32);
setter!(OH_ArkUI_ClonedEvent_SetInteractionHandByIndex, hand: i32, pointerIndex: i32);
setter!(OH_ArkUI_ClonedEvent_SetPressedTimeByIndex, pressedTime: i64, pointerIndex: i32);
setter!(OH_ArkUI_ClonedEvent_SetPinchAxisScaleValue, pinchAxisScaleValue: f64);
setter!(OH_ArkUI_ClonedEvent_SetHorizontalAxisScaleValue, horizontalAxisScaleValue: f64);
setter!(OH_ArkUI_ClonedEvent_SetVerticalAxisScaleValue, verticalAxisScaleValue: f64);
setter!(OH_ArkUI_ClonedEvent_SetScrollStep, scrollStep: i32);
setter!(OH_ArkUI_ClonedEvent_SetMouseButton, button: i32);
setter!(OH_ArkUI_ClonedEvent_SetRawDeltaX, rawDeltaX: f32);
setter!(OH_ArkUI_ClonedEvent_SetRawDeltaY, rawDeltaY: f32);

#[no_mangle]
unsafe extern "C" fn OH_ArkUI_ClonedEvent_SetPressedKeys(
    event: *const ArkUI_UIInputEvent,
    keys: *mut i32,
    length: i32,
) -> u32 {
    assert!(length > 0);
    let keys = unsafe { std::slice::from_raw_parts_mut(keys, length as usize) };
    let status = record(event, "keys", vec![Arg::List(keys.to_vec())]);
    // Simulate a native implementation that writes through the mutable ABI.
    keys[0] = -999;
    status
}

#[no_mangle]
unsafe extern "C" fn OH_ArkUI_ClonedEvent_SetPressedButtons(
    event: *const ArkUI_UIInputEvent,
    buttons: *const i32,
    length: i32,
) -> u32 {
    assert!(length > 0);
    let buttons = unsafe { std::slice::from_raw_parts(buttons, length as usize) };
    record(event, "buttons", vec![Arg::List(buttons.to_vec())])
}

fn assert_error<T: std::fmt::Debug>(value: Result<T, ArkUIInputError>, expected: u32) {
    match value {
        Err(ArkUIInputError::InternalError(code)) => assert_eq!(code, expected as i32),
        other => panic!("expected native error {expected}, got {other:?}"),
    }
}

#[test]
fn construction_failure_and_null_success_do_not_create_an_owner() {
    reset();
    STATE.with_borrow_mut(|state| state.status = 777);
    assert_error(ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch), 777);
    STATE.with_borrow_mut(|state| state.null_success = true);
    assert_error(ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch), 401);
    assert_error(
        unsafe { ArkUIOwnedPointerEvent::clone_from_raw(std::ptr::null()) },
        401,
    );
    STATE.with_borrow(|state| {
        assert!(state.live.is_empty());
        assert!(state.destroyed.is_empty());
    });
}

#[test]
fn unavailable_native_runtime_does_not_allocate_an_unreleasable_event() {
    reset();
    STATE.with_borrow_mut(|state| state.runtime_unavailable = true);
    assert_error(ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch), 401);
    STATE.with_borrow(|state| {
        assert!(state.live.is_empty());
        assert!(state.destroyed.is_empty());
    });
}

#[test]
fn clones_own_independent_allocations_and_dispatch_does_not_transfer_ownership() {
    reset();
    let event = ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch).unwrap();
    let clone = event.try_clone().unwrap();
    let callback_copy = unsafe { ArkUIOwnedPointerEvent::clone_from_raw(event.raw()) }.unwrap();
    assert_ne!(event.raw(), clone.raw());
    assert_ne!(event.raw(), callback_copy.raw());
    assert_eq!(callback_copy.event_type(), UIPointerEventType::Touch);
    drop(event);
    // A non-null sentinel stands for the live node in this host-only double.
    let node = std::ptr::NonNull::<u8>::dangling().as_ptr().cast();
    unsafe { clone.post_to_node(node, UIEventCompetitionStrategy::Competition) }.unwrap();
    STATE.with_borrow_mut(|state| state.status = 180001);
    assert_error(
        unsafe { clone.post_to_node(node, UIEventCompetitionStrategy::Default) },
        180001,
    );
    STATE.with_borrow(|state| assert_eq!(state.live.len(), 2));
    drop(clone);
    drop(callback_copy);
    STATE.with_borrow(|state| {
        assert!(state.live.is_empty());
        assert_eq!(state.destroyed.len(), 3);
    });
}

#[test]
fn failed_clone_leaves_the_source_alive() {
    reset();
    let mut event = ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch).unwrap();
    STATE.with_borrow_mut(|state| state.status = 777);
    assert_error(event.try_clone(), 777);
    STATE.with_borrow_mut(|state| state.null_success = true);
    assert_error(event.try_clone(), 401);
    event.set_pressure(0.5).unwrap();
    STATE.with_borrow(|state| {
        assert_eq!(state.live.len(), 1);
        assert!(state.destroyed.is_empty());
    });
    drop(event);
    STATE.with_borrow(|state| assert_eq!(state.destroyed.len(), 1));
}

#[test]
fn pen_values_are_lossless_and_indices_are_not_pointer_ids() {
    reset();
    let mut event = ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch).unwrap();
    event.set_tool_type(UIInputToolType::Pen).unwrap();
    event.set_pressure(1.5).unwrap();
    event.set_tilt_angle(30.0, -45.0).unwrap();
    event.set_roll_angle(275.0).unwrap();
    event.set_finger_id_by_index(42, 1).unwrap();
    event.set_pressure_by_index(0.75, 1).unwrap();
    event.set_event_time(9_000_000_000).unwrap();
    STATE.with_borrow(|state| {
        assert_eq!(
            state.calls,
            vec![
                ("OH_ArkUI_ClonedEvent_SetToolType", vec![Arg::Int(2)]),
                ("OH_ArkUI_ClonedEvent_SetPressure", vec![Arg::Float(1.5)]),
                (
                    "OH_ArkUI_ClonedEvent_SetTiltAngle",
                    vec![Arg::Float(30.0), Arg::Float(-45.0)]
                ),
                ("OH_ArkUI_ClonedEvent_SetRollAngle", vec![Arg::Float(275.0)]),
                (
                    "OH_ArkUI_ClonedEvent_SetFingerIdByIndex",
                    vec![Arg::Int(42), Arg::Int(1)]
                ),
                (
                    "OH_ArkUI_ClonedEvent_SetPressureByIndex",
                    vec![Arg::Float(0.75), Arg::Int(1)]
                ),
                (
                    "OH_ArkUI_ClonedEvent_SetEventTime",
                    vec![Arg::Int(9_000_000_000)]
                ),
            ]
        )
    });
}

#[test]
fn invalid_inputs_do_not_cross_ffi_and_native_errors_are_preserved() {
    reset();
    let mut event = ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch).unwrap();
    assert_error(event.set_pressure(f32::NAN), 401);
    assert_error(event.set_pressure(-0.1), 401);
    assert_error(event.set_tilt_angle(90.1, 0.0), 401);
    assert_error(event.set_roll_angle(f32::INFINITY), 401);
    assert_error(event.set_pressure_by_index(0.5, usize::MAX), 401);
    assert_error(
        event.set_action(UIPointerEventAction::Mouse(UIMouseEventAction::Move)),
        401,
    );
    assert_error(event.set_pressed_keys(&[]), 401);
    assert_error(event.set_pressed_buttons(&[]), 401);
    STATE.with_borrow(|state| assert!(state.calls.is_empty()));
    STATE.with_borrow_mut(|state| state.status = 180006);
    assert_error(event.set_mouse_button(UIMouseEventButton::Left), 180006);
}

#[test]
fn slice_setters_preserve_callers_storage_and_convert_button_codes() {
    reset();
    let mut event = ArkUIOwnedPointerEvent::new(UIPointerEventType::Mouse).unwrap();
    let keys = [2017, 2072];
    event.set_pressed_keys(&keys).unwrap();
    assert_eq!(keys, [2017, 2072]);
    event
        .set_pressed_buttons(&[UIMouseEventButton::Left, UIMouseEventButton::Forward])
        .unwrap();
    STATE.with_borrow(|state| {
        assert_eq!(
            state.calls,
            vec![
                ("keys", vec![Arg::List(vec![2017, 2072])]),
                ("buttons", vec![Arg::List(vec![1, 5])]),
            ]
        )
    });
}

#[test]
fn action_codes_follow_the_event_category() {
    reset();
    let mut touch = ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch).unwrap();
    let mut mouse = ArkUIOwnedPointerEvent::new(UIPointerEventType::Mouse).unwrap();
    let mut axis = ArkUIOwnedPointerEvent::new(UIPointerEventType::Axis).unwrap();
    touch
        .set_action(UIPointerEventAction::Touch(UIInputAction::Move))
        .unwrap();
    mouse
        .set_action(UIPointerEventAction::Mouse(UIMouseEventAction::Move))
        .unwrap();
    axis.set_action(UIPointerEventAction::Axis(UIAxisEventAction::Cancel))
        .unwrap();
    STATE.with_borrow(|state| {
        let args: Vec<_> = state.calls.iter().map(|(_, args)| args.clone()).collect();
        assert_eq!(
            args,
            vec![vec![Arg::Int(2)], vec![Arg::Int(3)], vec![Arg::Int(4)]]
        );
    });
}
