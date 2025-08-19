# ohos-hilog-binding

## Install

```shell
cargo add ohos-hilog-binding
```

## Usage

```rust
use hilog_binding::hilog_debug;
use napi_derive_ohos::napi;

#[napi]
pub fn add(left: u32, right: u32) -> u32 {
    hilog_debug!("hello world");
    hilog_debug!(
        "test",
        LogOptions {
          tag: Some("testTag"),
          domain: None
      }
    );
    left + right
}
```

## Feature

### redirect

Allow us redirect stdout/stderr to hilog.

```toml
# Cargo.toml

[dependencies]
ohos-hilog-binding = {version = "*", features = ["redirect"]}
```

```rust
use napi_derive_ohos::napi;

#[napi]
pub fn add(left: u32, right: u32) -> u32 {
  // setup at first
  let _handle = ohos_hilog_binding::forward_stdio_to_hilog();
  // can be redirected to hilog with info level
  println!("hello");

  left + right
}


```

### log

Allow us use `log` as log library.

**For `log` trace level, we will convert to debug level which is not supported in OHOS.**

```toml
# Cargo.toml

[dependencies]
ohos-hilog-binding = {version = "*", features = ["log"]}
log                = { version = "*" }
```

```rust
use napi_derive_ohos::napi;

use log::{debug, error, LevelFilter};
use ohos_hilog_binding::log::Config;

#[napi]
pub fn info() {
    ohos_hilog_binding::log::init_once(Config::default().with_max_level(LevelFilter::Info));

    debug!("this is a debug {}", "message");
    error!("this is printed by default");
}
```
