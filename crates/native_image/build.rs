fn main() {
    if std::env::var_os("CARGO_FEATURE_TEXTURE_READBACK").is_some()
        && std::env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|target| target == "ohos")
    {
        // khronos-egl's no-pkg-config mode delegates link metadata to consumers.
        println!("cargo:rustc-link-lib=EGL");
    }
}
