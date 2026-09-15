## Generate tool

This tool generates sys crates from the OpenHarmony SDK using `ohrs` and bindgen.

### Run

1. Add the header paths, symbol allowlist and library name in `build/config`.
2. Export the config in `build/config/mod.rs` and register it in `build/main.rs`.
3. Run `ohrs build --arch aarch` from this directory, then format from the workspace root.

The generator processes all registered sys crates. Check the resulting diff before
committing. `native_child_process` uses `AbilityKit/native_child_process.h` and links
`child_process`, following the same registration path as the other components.

API12 is the baseline. Later declarations are gated by their `@since` annotations;
undocumented opaque declarations inherit their earliest documented use. The
native child-process configs type therefore requires `api-20`.
