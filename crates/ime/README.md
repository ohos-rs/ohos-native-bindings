# ohos-ime-binding

This crate is a binding for the input method module in OpenHarmony.

## Install

```shell
cargo add ohos-ime-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_ime_binding as ime;

// Use the safe Rust APIs exposed by `ohos-ime-binding` from your native module.
```

`IME::hide_keyboard` only hides the keyboard and keeps the native session
attached so it can be shown again. Call `IME::detach` explicitly when the text
editor session is finished.

On API 22 and newer, applications with main-thread-only callback state can opt
in to main-thread delivery:

```rust
use ohos_ime_binding::{AttachOptions, IME};

let ime = IME::new_with_main_thread_callbacks(AttachOptions::new(false));
```

`IME::new` preserves the platform default callback thread.

Each `IME` owns an independent callback set. Activating one session safely
replaces the previously active native editor, and a later activation repairs a
proxy invalidated by another ArkUI input or an application lifecycle change.
Use the `try_*` methods when the caller needs the HarmonyOS error code:

```rust
ime.try_show_keyboard()?;
ime.try_hide_keyboard()?;
ime.try_detach()?;
# Ok::<(), ohos_ime_binding::ImeError>(())
```

## License

MIT OR Apache-2.0
