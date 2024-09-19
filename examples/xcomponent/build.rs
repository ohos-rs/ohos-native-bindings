use std::error::Error;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, Profile, Registry};

fn main() -> Result<(), Box<dyn Error>> {
    let out = std::env::var("OUT_DIR")?;
    let out = Path::new(&out);

    let mut file = File::create(out.join("egl_bindings.rs")).unwrap();
    Registry::new(Api::Egl, (1, 5), Profile::Core, Fallbacks::All, [])
        .write_bindings(gl_generator::StaticStructGenerator, &mut file)
        .unwrap();
    println!("cargo:rustc-link-lib=EGL");

    napi_build_ohos::setup();
    Ok(())
}
