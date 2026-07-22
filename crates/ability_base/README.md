# ohos-ability-base-binding

This crate is a binding for the ability base module in OpenHarmony.

A want describes which ability to reach — the bundle, module and ability name —
plus the parameters handed to it. The native `want.h` C API creates the want
with one call and destroys it with another, stores parameters through key/value
setters and reads strings back into caller-provided character arrays. This
crate wraps that C API with a safe layer.

## Install

```shell
cargo add ohos-ability-base-binding --features api-17
```

## Usage

```rust
use ohos_ability_base_binding::{Element, Want};

let mut want = Want::new(&Element::new("com.example.app", "entry", "EntryAbility"))?;

want.set_char_param("greeting", "hello")?;
assert_eq!(want.char_param("greeting")?, "hello");

let element = want.element()?;
assert_eq!(element.ability_name, "EntryAbility");
```

## Coverage

The want API starts at API 15, so nothing is available without a feature.

| Feature | Adds |
|---|---|
| `api-15` | `Want::new` and its `Drop`, `set_element`, `element`, `set_char_param`, `char_param`, `char_param_with_capacity`, `add_fd`, `fd` |
| `api-17` | `set_uri`, `uri`, `uri_with_capacity`, `set_int32_param`, `int32_param`, `set_bool_param`, `bool_param`, `set_double_param`, `double_param` |

## Notes

- `Want` owns the native object: it is created by `OH_AbilityBase_CreateWant`
  and destroyed by `OH_AbilityBase_DestroyWant` on drop. It is neither `Send`
  nor `Sync`, as the native API documents no thread safety for it.
- `Element` holds owned Rust strings. The native `AbilityBase_Element` is a
  plain struct of three `char*` fields with no destructor: on the way in the
  runtime copies the strings, on the way out it hands back pointers owned by
  the want, which `element()` copies right away and never frees.
- The string getters take a buffer size that `want.h` documents only as "size
  of the value", without saying what happens when the buffer is too small. It
  is treated as a capacity that has to cover the terminating character; a
  result without a terminator inside the buffer is reported as
  `AbilityBaseError::BufferTooSmall` instead of being truncated, and the bytes
  are UTF-8 checked before becoming a `String`.
- `char_param` and `uri` manage that buffer: they start at 256 bytes and double
  up to 64 KiB while the runtime rejects the call. The native API has a single
  failure code, so a value that does not fit cannot be told apart from a
  missing key; both end up as `AbilityBaseError::Native(401)`. Use the
  `*_with_capacity` variants to size the buffer once.
- `add_fd` only borrows the descriptor for the duration of the call, and the
  descriptor returned by `fd` belongs to the want and must not be closed by the
  caller.

## License

MIT OR Apache-2.0
