fn main() {
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("ohos") {
        napi_build_ohos::setup();
    }
}
