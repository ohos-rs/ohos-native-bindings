# hilog_binding

## Install

```shell
cargo add hilog_binding
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