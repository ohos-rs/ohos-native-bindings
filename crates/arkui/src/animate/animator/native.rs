use std::{os::raw::c_void, ptr::NonNull};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_Animator, ArkUI_AnimatorEvent, ArkUI_AnimatorOnFrameEvent, ArkUI_AnimatorOption,
    ArkUI_ContextHandle, ArkUI_CurveHandle, ArkUI_ExpectedFrameRateRange,
    OH_ArkUI_AnimatorEvent_GetUserData, OH_ArkUI_AnimatorOnFrameEvent_GetUserData,
    OH_ArkUI_AnimatorOnFrameEvent_GetValue, OH_ArkUI_AnimatorOption_Create,
    OH_ArkUI_AnimatorOption_Dispose, OH_ArkUI_AnimatorOption_GetBegin,
    OH_ArkUI_AnimatorOption_GetCurve, OH_ArkUI_AnimatorOption_GetDelay,
    OH_ArkUI_AnimatorOption_GetDirection, OH_ArkUI_AnimatorOption_GetDuration,
    OH_ArkUI_AnimatorOption_GetEnd, OH_ArkUI_AnimatorOption_GetExpectedFrameRateRange,
    OH_ArkUI_AnimatorOption_GetFill, OH_ArkUI_AnimatorOption_GetIterations,
    OH_ArkUI_AnimatorOption_GetKeyframeCurve, OH_ArkUI_AnimatorOption_GetKeyframeTime,
    OH_ArkUI_AnimatorOption_GetKeyframeValue, OH_ArkUI_AnimatorOption_RegisterOnCancelCallback,
    OH_ArkUI_AnimatorOption_RegisterOnFinishCallback,
    OH_ArkUI_AnimatorOption_RegisterOnFrameCallback,
    OH_ArkUI_AnimatorOption_RegisterOnRepeatCallback, OH_ArkUI_AnimatorOption_SetBegin,
    OH_ArkUI_AnimatorOption_SetCurve, OH_ArkUI_AnimatorOption_SetDelay,
    OH_ArkUI_AnimatorOption_SetDirection, OH_ArkUI_AnimatorOption_SetDuration,
    OH_ArkUI_AnimatorOption_SetEnd, OH_ArkUI_AnimatorOption_SetExpectedFrameRateRange,
    OH_ArkUI_AnimatorOption_SetFill, OH_ArkUI_AnimatorOption_SetIterations,
    OH_ArkUI_AnimatorOption_SetKeyframe, OH_ArkUI_AnimatorOption_SetKeyframeCurve,
    OH_ArkUI_Animator_Cancel, OH_ArkUI_Animator_Finish, OH_ArkUI_Animator_Pause,
    OH_ArkUI_Animator_Play, OH_ArkUI_Animator_ResetAnimatorOption, OH_ArkUI_Animator_Reverse,
};

use crate::api::ARK_UI_NATIVE_ANIMATE_API_1;
use crate::{check_arkui_status, AnimationDirection, AnimationFillMode, ArkUIError, ArkUIResult};

struct AnimatorFrameCallbackContext {
    callback: Box<dyn Fn(f32)>,
}

struct AnimatorEventCallbackContext {
    callback: Box<dyn Fn()>,
}

pub(crate) struct AnimatorHandle {
    raw: NonNull<ArkUI_Animator>,
}

impl AnimatorHandle {
    pub(crate) fn play(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_Animator_Play(self.raw.as_ptr())) }
    }

    pub(crate) fn finish(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_Animator_Finish(self.raw.as_ptr())) }
    }

    pub(crate) fn pause(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_Animator_Pause(self.raw.as_ptr())) }
    }

    pub(crate) fn cancel(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_Animator_Cancel(self.raw.as_ptr())) }
    }

    pub(crate) fn reverse(&self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_Animator_Reverse(self.raw.as_ptr())) }
    }

    pub(crate) fn reset_option(&self, option: &AnimatorOptions) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_Animator_ResetAnimatorOption(
                self.raw.as_ptr(),
                option.raw.as_ptr()
            ))
        }
    }

    pub(crate) fn dispose(&self) -> ArkUIResult<()> {
        let handle = self.raw.as_ptr();
        ARK_UI_NATIVE_ANIMATE_API_1.with(|api| unsafe {
            if let Some(dispose_animator_func) = (*api.raw()).disposeAnimator {
                dispose_animator_func(handle);
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeAnimateAPI_1::disposeAnimator is None",
                ))
            }
        })
    }
}

pub(crate) struct AnimatorOptions {
    raw: NonNull<ArkUI_AnimatorOption>,
    on_frame_callback: Option<*mut AnimatorFrameCallbackContext>,
    on_finish_callback: Option<*mut AnimatorEventCallbackContext>,
    on_cancel_callback: Option<*mut AnimatorEventCallbackContext>,
    on_repeat_callback: Option<*mut AnimatorEventCallbackContext>,
}

impl AnimatorOptions {
    pub(crate) fn new(keyframe_size: i32) -> ArkUIResult<Self> {
        let raw = unsafe { OH_ArkUI_AnimatorOption_Create(keyframe_size) };
        let raw = NonNull::new(raw).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_AnimatorOption_Create returned null",
            )
        })?;

        Ok(Self {
            raw,
            on_frame_callback: None,
            on_finish_callback: None,
            on_cancel_callback: None,
            on_repeat_callback: None,
        })
    }

    pub(crate) fn dispose(mut self) {
        let _ = self.clear_on_frame_callback();
        let _ = self.clear_on_finish_callback();
        let _ = self.clear_on_cancel_callback();
        let _ = self.clear_on_repeat_callback();
        unsafe { OH_ArkUI_AnimatorOption_Dispose(self.raw.as_ptr()) };
    }

    pub(crate) fn set_duration(&self, value: i32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_AnimatorOption_SetDuration(
                self.raw.as_ptr(),
                value
            ))
        }
    }

    pub(crate) fn set_delay(&self, value: i32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_AnimatorOption_SetDelay(self.raw.as_ptr(), value)) }
    }

    pub(crate) fn set_iterations(&self, value: i32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_AnimatorOption_SetIterations(
                self.raw.as_ptr(),
                value
            ))
        }
    }

    pub(crate) fn set_fill(&self, value: AnimationFillMode) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_AnimatorOption_SetFill(
                self.raw.as_ptr(),
                value.into()
            ))
        }
    }

    pub(crate) fn set_direction(&self, value: AnimationDirection) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_AnimatorOption_SetDirection(
                self.raw.as_ptr(),
                value.into()
            ))
        }
    }

    pub(crate) fn set_curve(&self, value: ArkUI_CurveHandle) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_AnimatorOption_SetCurve(self.raw.as_ptr(), value)) }
    }

    pub(crate) fn set_begin(&self, value: f32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_AnimatorOption_SetBegin(self.raw.as_ptr(), value)) }
    }

    pub(crate) fn set_end(&self, value: f32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_AnimatorOption_SetEnd(self.raw.as_ptr(), value)) }
    }

    pub(crate) fn set_expected_frame_rate_range(
        &self,
        value: &mut ArkUI_ExpectedFrameRateRange,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_AnimatorOption_SetExpectedFrameRateRange(
                self.raw.as_ptr(),
                value
            ))
        }
    }

    pub(crate) fn set_keyframe(&self, time: f32, value: f32, index: i32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_AnimatorOption_SetKeyframe(
                self.raw.as_ptr(),
                time,
                value,
                index
            ))
        }
    }

    pub(crate) fn set_keyframe_curve(
        &self,
        value: ArkUI_CurveHandle,
        index: i32,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_AnimatorOption_SetKeyframeCurve(
                self.raw.as_ptr(),
                value,
                index
            ))
        }
    }

    pub(crate) fn get_duration(&self) -> i32 {
        unsafe { OH_ArkUI_AnimatorOption_GetDuration(self.raw.as_ptr()) }
    }

    pub(crate) fn get_delay(&self) -> i32 {
        unsafe { OH_ArkUI_AnimatorOption_GetDelay(self.raw.as_ptr()) }
    }

    pub(crate) fn get_iterations(&self) -> i32 {
        unsafe { OH_ArkUI_AnimatorOption_GetIterations(self.raw.as_ptr()) }
    }

    pub(crate) fn get_fill(&self) -> AnimationFillMode {
        unsafe { OH_ArkUI_AnimatorOption_GetFill(self.raw.as_ptr()).into() }
    }

    pub(crate) fn get_direction(&self) -> AnimationDirection {
        unsafe { OH_ArkUI_AnimatorOption_GetDirection(self.raw.as_ptr()).into() }
    }

    pub(crate) fn get_curve(&self) -> ArkUI_CurveHandle {
        unsafe { OH_ArkUI_AnimatorOption_GetCurve(self.raw.as_ptr()) }
    }

    pub(crate) fn get_begin(&self) -> f32 {
        unsafe { OH_ArkUI_AnimatorOption_GetBegin(self.raw.as_ptr()) }
    }

    pub(crate) fn get_end(&self) -> f32 {
        unsafe { OH_ArkUI_AnimatorOption_GetEnd(self.raw.as_ptr()) }
    }

    pub(crate) fn get_expected_frame_rate_range(&self) -> Option<ArkUI_ExpectedFrameRateRange> {
        let value = unsafe { OH_ArkUI_AnimatorOption_GetExpectedFrameRateRange(self.raw.as_ptr()) };
        if value.is_null() {
            None
        } else {
            Some(unsafe { *value })
        }
    }

    pub(crate) fn get_keyframe_time(&self, index: i32) -> f32 {
        unsafe { OH_ArkUI_AnimatorOption_GetKeyframeTime(self.raw.as_ptr(), index) }
    }

    pub(crate) fn get_keyframe_value(&self, index: i32) -> f32 {
        unsafe { OH_ArkUI_AnimatorOption_GetKeyframeValue(self.raw.as_ptr(), index) }
    }

    pub(crate) fn get_keyframe_curve(&self, index: i32) -> ArkUI_CurveHandle {
        unsafe { OH_ArkUI_AnimatorOption_GetKeyframeCurve(self.raw.as_ptr(), index) }
    }

    pub(crate) fn register_on_frame_callback<T: Fn(f32) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_frame_callback()?;
        let callback = Box::into_raw(Box::new(AnimatorFrameCallbackContext {
            callback: Box::new(callback),
        }));
        let result = register_on_frame_callback_raw(
            self.raw.as_ptr(),
            callback.cast(),
            Some(animator_frame_callback_trampoline),
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        self.on_frame_callback = Some(callback);
        Ok(())
    }

    pub(crate) fn clear_on_frame_callback(&mut self) -> ArkUIResult<()> {
        register_on_frame_callback_raw(self.raw.as_ptr(), std::ptr::null_mut(), None)?;
        if let Some(callback) = self.on_frame_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
        Ok(())
    }

    pub(crate) fn register_on_finish_callback<T: Fn() + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_finish_callback()?;
        let callback = Box::into_raw(Box::new(AnimatorEventCallbackContext {
            callback: Box::new(callback),
        }));
        let result = register_on_event_callback_raw(
            self.raw.as_ptr(),
            callback.cast(),
            Some(animator_event_callback_trampoline),
            OH_ArkUI_AnimatorOption_RegisterOnFinishCallback,
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        self.on_finish_callback = Some(callback);
        Ok(())
    }

    pub(crate) fn clear_on_finish_callback(&mut self) -> ArkUIResult<()> {
        register_on_event_callback_raw(
            self.raw.as_ptr(),
            std::ptr::null_mut(),
            None,
            OH_ArkUI_AnimatorOption_RegisterOnFinishCallback,
        )?;
        if let Some(callback) = self.on_finish_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
        Ok(())
    }

    pub(crate) fn register_on_cancel_callback<T: Fn() + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_cancel_callback()?;
        let callback = Box::into_raw(Box::new(AnimatorEventCallbackContext {
            callback: Box::new(callback),
        }));
        let result = register_on_event_callback_raw(
            self.raw.as_ptr(),
            callback.cast(),
            Some(animator_event_callback_trampoline),
            OH_ArkUI_AnimatorOption_RegisterOnCancelCallback,
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        self.on_cancel_callback = Some(callback);
        Ok(())
    }

    pub(crate) fn clear_on_cancel_callback(&mut self) -> ArkUIResult<()> {
        register_on_event_callback_raw(
            self.raw.as_ptr(),
            std::ptr::null_mut(),
            None,
            OH_ArkUI_AnimatorOption_RegisterOnCancelCallback,
        )?;
        if let Some(callback) = self.on_cancel_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
        Ok(())
    }

    pub(crate) fn register_on_repeat_callback<T: Fn() + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_repeat_callback()?;
        let callback = Box::into_raw(Box::new(AnimatorEventCallbackContext {
            callback: Box::new(callback),
        }));
        let result = register_on_event_callback_raw(
            self.raw.as_ptr(),
            callback.cast(),
            Some(animator_event_callback_trampoline),
            OH_ArkUI_AnimatorOption_RegisterOnRepeatCallback,
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        self.on_repeat_callback = Some(callback);
        Ok(())
    }

    pub(crate) fn clear_on_repeat_callback(&mut self) -> ArkUIResult<()> {
        register_on_event_callback_raw(
            self.raw.as_ptr(),
            std::ptr::null_mut(),
            None,
            OH_ArkUI_AnimatorOption_RegisterOnRepeatCallback,
        )?;
        if let Some(callback) = self.on_repeat_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
        Ok(())
    }
}

pub(crate) fn create_animator(
    ctx: ArkUI_ContextHandle,
    option: &AnimatorOptions,
) -> ArkUIResult<AnimatorHandle> {
    ARK_UI_NATIVE_ANIMATE_API_1.with(|api| unsafe {
        if let Some(create_animator_func) = (*api.raw()).createAnimator {
            let animator = create_animator_func(ctx, option.raw.as_ptr());
            if let Some(animator) = NonNull::new(animator) {
                Ok(AnimatorHandle { raw: animator })
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "ArkUI_NativeAnimateAPI_1::createAnimator returned null",
                ))
            }
        } else {
            Err(ArkUIError::new(
                ArkUIErrorCode::AttributeOrEventNotSupported,
                "ArkUI_NativeAnimateAPI_1::createAnimator is None",
            ))
        }
    })
}

unsafe extern "C" fn animator_frame_callback_trampoline(event: *mut ArkUI_AnimatorOnFrameEvent) {
    let user_data = unsafe { OH_ArkUI_AnimatorOnFrameEvent_GetUserData(event) };
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut AnimatorFrameCallbackContext) };
    let value = unsafe { OH_ArkUI_AnimatorOnFrameEvent_GetValue(event) };
    (callback.callback)(value);
}

unsafe extern "C" fn animator_event_callback_trampoline(event: *mut ArkUI_AnimatorEvent) {
    let user_data = unsafe { OH_ArkUI_AnimatorEvent_GetUserData(event) };
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut AnimatorEventCallbackContext) };
    (callback.callback)();
}

fn register_on_frame_callback_raw(
    option: *mut ArkUI_AnimatorOption,
    user_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(event: *mut ArkUI_AnimatorOnFrameEvent)>,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_AnimatorOption_RegisterOnFrameCallback(
            option, user_data, callback
        ))
    }
}

type AnimatorEventRegisterCallback = unsafe extern "C" fn(
    option: *mut ArkUI_AnimatorOption,
    user_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(event: *mut ArkUI_AnimatorEvent)>,
) -> i32;

fn register_on_event_callback_raw(
    option: *mut ArkUI_AnimatorOption,
    user_data: *mut c_void,
    callback: Option<unsafe extern "C" fn(event: *mut ArkUI_AnimatorEvent)>,
    register: AnimatorEventRegisterCallback,
) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(register(option, user_data, callback)) }
}
