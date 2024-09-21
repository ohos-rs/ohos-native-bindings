use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-link-lib=EGL");
    println!("cargo:rustc-link-lib=GLESv3");

    napi_build_ohos::setup();
    Ok(())
}
