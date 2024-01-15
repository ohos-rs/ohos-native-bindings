use std::env;

pub fn main() {
    let _ndk = env::var("OHOS_NDK_HOME").expect("OHOS_NDK_HOME not set");
    // link libpixelmap_ndk.z.so
    println!("cargo:rustc-link-lib=dylib=pixelmap_ndk.z");
}
