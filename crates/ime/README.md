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

Stale-session errors invalidate the input proxy's usable state, not the native
editor's lifetime. Recovery reuses the editor callback table, so a delayed
`OnInputStop` can still report keyboard status safely. Explicit detach disables
Rust callbacks. If attach/detach fails without establishing a release boundary,
the native table is retained until the next successful attach/detach; application
callback closures are not retained after the `IME` is dropped. If there is no
later successful transition, the small retired native allocation remains until
process exit.

Native proxy operations are serialized across sessions. Reentrant or competing
native operations return `IME_ERR_IMCLIENT` through the `try_*` APIs instead of
waiting inside a native callback. Applications can retry after the current call
has returned.

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

Configure the native editor with `try_update_configuration(EnterKey, InputType)`
and `try_update_cursor(Rect)`. Attributes and the absolute physical screen
cursor rectangle are retained for the next attach. Updates are sent only when
this editor owns the active native session, so an inactive window cannot change
another window's editor. Registering a preview callback enables preview text in
the attachment configuration. These APIs must be called on the editor's thread.

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

## Host lifecycle regression tests

Run `bash crates/ime/tests/run-host.sh --offline` from the repository root.
The script builds a host-only NDK double and tests the real binding's FFI calls,
including delayed keyboard callbacks, failed transitions, replacement sessions,
and callback reentry. It does not change or emulate a device's system libraries.

## License

MIT OR Apache-2.0
