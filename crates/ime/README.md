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

The host owns the process-wide IME's window routing and serialized lifecycle.
Detach the previous editor before transferring ownership. `try_attach` performs
the native attachment synchronously; `show_attached` shows its keyboard without
implicitly attaching or recovering. The host chooses any worker/event queue and
cancellation policy.

`IME::new_for_window(options, window_id)` selects the actual native Window ID;
zero selects the main window. `set_window_id` changes the next attachment's
configuration. With `api-26`, `IME::set_consume_key_events(bool)` configures
hardware-key consumption. It is compiled only with that feature and requires
an API-26 runtime. `TextConfig::set_consume_key_events` exposes the fallible
native setter directly.

Native callbacks remain allocated across stale-session recovery. Failed detach
retains the table for retry. If Drop cannot establish a native release boundary,
it logs and leaves the native table allocated until process exit, after
unregistering Rust callbacks. Use fallible detach before releasing the owner.
Use the `try_*` methods to inspect native errors:

```rust
ime.try_show_keyboard()?;
ime.try_hide_keyboard()?;
ime.try_detach()?;
# Ok::<(), ohos_ime_binding::ImeError>(())
```

Configure the native editor with `try_update_configuration(EnterKey, InputType)`
and `try_update_cursor(Rect)`. Attributes and the absolute physical screen
cursor rectangle are retained for the next attach. The host sends updates only
to its current editor. Registering a preview callback enables preview text in
the attachment configuration. Call these APIs on the editor's thread.

`try_update_text_state(TextState)` retains a snapshot for IPC-thread requests
for text before/after the selection and the absolute cursor index. `offset`
is the snapshot's starting UTF-16 document offset; selection positions are
absolute UTF-16 offsets. Native configuration queries also receive that selection.
The active editor receives a selection notification for whole text of at most
8192 UTF-16 units. Offset or larger snapshots only answer text queries; they are
not sent as whole-document notifications. Unchanged snapshots are deduplicated,
and reattachment sends the next update again.

Register `on_delete_forward`, `on_move_cursor`, `on_set_selection`, and
`on_extend_action` for system editing requests. These callbacks run on the same
thread as other editor callbacks. Text decoding preserves surrogate pairs and
replaces invalid UTF-16 units instead of silently dropping them.

## Host tests

`cargo test --manifest-path crates/ime/tests/host/Cargo.toml --features api-26`
runs production IME code with native substitutes. Native callback timing and
window/keyboard behavior also need device acceptance.

## License

MIT OR Apache-2.0
