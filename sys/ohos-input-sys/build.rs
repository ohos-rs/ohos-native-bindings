use std::env;

fn main() {
    let _ndk = env::var("OHOS_NDK_HOME").expect("OHOS_NDK_HOME not set");
    println!("cargo:rustc-link-lib=dylib=ohinput");
}
