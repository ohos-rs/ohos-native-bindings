use std::env;

fn main() {
    let _ndk = env::var("OHOS_NDK_HOME").expect("OHOS_NDK_HOME not set");
    println!("cargo:rustc-link-lib=dylib=image_ndk.z");
    println!("cargo:rustc-link-lib=dylib=image_packer_ndk.z");
    println!("cargo:rustc-link-lib=dylib=pixelmap_ndk.z");
    println!("cargo:rustc-link-lib=dylib=image_receiver_ndk.z");
    println!("cargo:rustc-link-lib=dylib=image_source_ndk.z");
}
