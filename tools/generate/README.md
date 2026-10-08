## Generate tool

This tool generates sys crates from the OpenHarmony SDK using `ohrs` and bindgen.

### Run

1. Add the header paths, symbol allowlist and library name in `build/config`.
2. Export the config in `build/config/mod.rs` and register it in `build/main.rs`.
3. Run `ohrs build --arch aarch` from this directory, then format from the workspace root.

The generator processes all registered sys crates. Check the resulting diff before
committing. `native_child_process` uses `AbilityKit/native_child_process.h` and links
`child_process`, following the same registration path as the other components.

API12 is the baseline. Later declarations are gated by their `@since` annotations.
For opaque records whose comments bindgen drops, the generator recovers annotations
from forward declarations in the included headers and combines them with the
earliest documented uses of shared types. Nested callback fields are parsed as
complete Rust declarations so parameter commas cannot truncate their dependencies.

### Regression tests

With libclang available (set `LIBCLANG_PATH` if needed), run from the workspace root:

```sh
OHOS_BINDINGS_SKIP_GENERATION=1 cargo test -p generate
```

This runs fixture-based generation tests without regenerating workspace bindings
or requiring an OpenHarmony SDK. Leave `OHOS_BINDINGS_SKIP_GENERATION` unset when
generating the sys crates.
