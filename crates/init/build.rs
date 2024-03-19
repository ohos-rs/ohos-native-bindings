use std::env;

pub fn main() {
    let _ndk = env::var("OHOS_NDK_HOME").expect("OHOS_NDK_HOME not set");
    // link libdeviceinfo_ndk.z.so
    println!("cargo:rustc-link-lib=dylib=deviceinfo_ndk.z");
}
