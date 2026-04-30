use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};

use ohos_jsvm_binding::{
    Buffer, Callback, CallbackInfo, External, FromJsValue, Object, Runtime, ToJsValue, TypeTag,
    TypedArrayValue,
};

fn to_napi_error(error: ohos_jsvm_binding::JsvmError) -> Error {
    Error::from_reason(error.to_string())
}

fn with_jsvm<T>(
    f: impl FnOnce(&ohos_jsvm_binding::Env) -> ohos_jsvm_binding::Result<T>,
) -> Result<T> {
    let runtime = Runtime::new().map_err(to_napi_error)?;
    let _vm_scope = runtime.open_scope().map_err(to_napi_error)?;
    let env = runtime.create_env().map_err(to_napi_error)?;
    let _env_scope = env.open_scope().map_err(to_napi_error)?;
    let _handle_scope = env.open_handle_scope().map_err(to_napi_error)?;
    f(&env).map_err(to_napi_error)
}

#[napi]
pub fn eval_in_jsvm(script: String) -> Result<String> {
    with_jsvm(|env| {
        let value = env.evaluate(&script)?;
        String::from_js_value(env, value)
    })
}

#[napi]
pub fn object_roundtrip() -> Result<String> {
    with_jsvm(|env| {
        let object = env.create_object()?;
        object.set_named_property(env, "name", "jsvm")?;
        object.set_named_property(env, "version", &8i32)?;
        object.set_named_property(env, "enabled", &true)?;

        let name: String = object.get_named_property(env, "name")?;
        let version: i32 = object.get_named_property(env, "version")?;
        let enabled: bool = object.get_named_property(env, "enabled")?;
        let keys = object.keys(env)?;

        Ok(format!(
            "name={name};version={version};enabled={enabled};keys={}",
            keys.join(",")
        ))
    })
}

#[napi]
pub fn promise_roundtrip() -> Result<bool> {
    with_jsvm(|env| {
        let (deferred, promise) = env.create_promise()?;
        deferred.resolve(env, "resolved from example")?;
        promise.as_value().is_promise(env)
    })
}

#[napi]
pub fn object_keys() -> Result<Vec<String>> {
    with_jsvm(|env| {
        let object = Object::new(env)?;
        object.set_named_property(env, "alpha", &1i32)?;
        object.set_named_property(env, "beta", &2i32)?;
        object.keys(env)
    })
}

#[napi]
pub fn buffer_roundtrip() -> Result<Vec<u8>> {
    with_jsvm(|env| {
        let value = Buffer::from(vec![1, 2, 3, 4]).to_js_value(env)?;
        let buffer = Buffer::from_js_value(env, value)?;
        Ok(buffer.into_vec())
    })
}

#[napi]
pub fn typed_array_roundtrip() -> Result<Vec<i32>> {
    with_jsvm(|env| {
        let value = TypedArrayValue::from(vec![1i32, 2, 3]).to_js_value(env)?;
        let typed = TypedArrayValue::<i32>::from_js_value(env, value)?;
        Ok(typed.into_vec())
    })
}

#[napi]
pub fn bigint_roundtrip(value: String) -> Result<String> {
    let value = value
        .parse::<u128>()
        .map_err(|err| Error::from_reason(err.to_string()))?;
    with_jsvm(|env| {
        let js_value = value.to_js_value(env)?;
        let value = u128::from_js_value(env, js_value)?;
        Ok(value.to_string())
    })
}

#[napi]
pub fn external_roundtrip() -> Result<String> {
    with_jsvm(|env| {
        let value = env.create_external(String::from("external-data"))?;
        let external = External::<String>::from_js_value(env, value)?;
        let value = unsafe { external.as_ref() };
        Ok(value.clone())
    })
}

#[napi]
pub fn wrap_roundtrip() -> Result<bool> {
    with_jsvm(|env| {
        let object = Object::new(env)?;
        let reference = object.wrap(env, 42i32)?;
        let native = unsafe { object.unwrap::<i32>(env)? };
        let matched = *native == 42;
        reference.delete(env)?;
        Ok(matched)
    })
}

#[napi]
pub fn type_tag_roundtrip() -> Result<bool> {
    with_jsvm(|env| {
        let object = Object::new(env)?;
        let tag = TypeTag::new(0x1234, 0x5678);
        object.type_tag(env, &tag)?;
        object.check_type_tag(env, &tag)
    })
}

#[napi]
pub fn json_roundtrip() -> Result<String> {
    with_jsvm(|env| {
        let value = env.json_parse(r#"{"name":"jsvm","count":2}"#)?;
        env.json_stringify(value)
    })
}

#[napi]
pub fn exception_message() -> Result<String> {
    with_jsvm(|env| {
        let error = env.evaluate("throw new Error('boom')").unwrap_err();
        let message = env
            .take_pending_exception_string()?
            .unwrap_or_else(|| error.to_string());
        Ok(message)
    })
}

#[napi]
pub fn native_method_with_data() -> Result<String> {
    static CALLBACK: Callback = Callback::new(native_echo, std::ptr::null_mut());

    unsafe extern "C" fn native_echo(
        raw_env: ohos_jsvm_binding::ohos_jsvm_sys::JSVM_Env,
        raw_info: ohos_jsvm_binding::ohos_jsvm_sys::JSVM_CallbackInfo,
    ) -> ohos_jsvm_binding::ohos_jsvm_sys::JSVM_Value {
        let env = ohos_jsvm_binding::Env::from_borrowed_raw(raw_env).expect("valid env");
        let info = CallbackInfo::from_raw(raw_info).expect("valid callback info");
        let value = info.arg::<String>(&env, 0).unwrap_or_default();
        env.create_string(&format!("native:{value}"))
            .map(|value| value.as_raw())
            .unwrap_or(std::ptr::null_mut())
    }

    with_jsvm(|env| {
        let object = env.create_object()?;
        object.define_method(env, "echo", &CALLBACK)?;
        let global = env.global()?;
        global.set_named_property(env, "host", &object.as_value())?;
        let value = env.evaluate("host.echo('ok')")?;
        String::from_js_value(env, value)
    })
}
