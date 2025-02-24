use std::env;

fn main() {
    let _ndk = env::var("OHOS_NDK_HOME").expect("OHOS_NDK_HOME not set");
    println!("cargo:rustc-link-lib=dylib=ohimage");
    println!("cargo:rustc-link-lib=dylib=image_packer");
    println!("cargo:rustc-link-lib=dylib=picture");
    println!("cargo:rustc-link-lib=dylib=image_receiver");
    println!("cargo:rustc-link-lib=dylib=image_source");
    println!("cargo:rustc-link-lib=dylib=pixelmap");
}
