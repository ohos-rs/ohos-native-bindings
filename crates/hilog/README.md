# hilog_binding

## Install

```shell
cargo add hilog_binding
```

## Usage

```rust
use hilog_binding::hilog_debug;
use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::pre_init;
use napi_ohos::module_init;

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

#[module_init]
fn init() {
    pre_init();
}

```