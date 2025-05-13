## How to add a new crate binding?

This document will help you develop a new binding for OpenHarmony.

### Add config

Add a new config to `tools/generate/build/config` and mark it as public. Then add it to `CONFIG`

```rust
// tools/generate/build/main.rs
static CONFIG: Lazy<Vec<Lazy<SysConfig>>> = Lazy::new(|| {
    vec![
        // ... old config
        // add here
    ]
});
```

### Generate sys binding

Use `ohrs` to generate sys crate.

```bash
ohrs build --arch aarch
```

**You may get some build errors. You need to fix the origin c header. Make sure all source file's syntax is valid c syntax.**


Add `build.rs` file to generated crate folder. And link dynamic library if need.

```rust
// Here is an example.
use std::env;

fn main() {
    let _ndk = env::var("OHOS_NDK_HOME").expect("OHOS_NDK_HOME not set");
    println!("cargo:rustc-link-lib=dylib=udmf");
}
```

### Generate binding

Use `cargo` command to generate a new crate in `crates` folder.

```bash
cargo new xx --lib
```

### Develop

Add sys binding as dependency and develop current crate.