## Sys binding generator

The build script in `build/main.rs` generates registered SDK bindings. `ohrs`
supplies the OHOS target, sysroot and libclang environment. Set `OHOS_NDK_HOME`
to the pinned SDK's `openharmony` directory (the parent of `native`).

### Bounded generation

Add a `SysConfig` in `build/config`, export it in `build/config/mod.rs`, and add
it to `CONFIG` in `build/main.rs`. Run from this directory:

```sh
OHOS_BINDINGS_GENERATE_CONFIG=ohos-native-child-process-sys ohrs build --arch aarch
```

`OHOS_BINDINGS_GENERATE_CONFIG` accepts exactly one registered crate name.
Empty, unknown, non-UTF-8, partial, wildcard, whitespace-padded and multi-name
values fail before output writes. Duplicate registry matches also fail. In this
mode both generated Rust and Cargo API-feature synchronization are restricted
to the selected crate. Generation errors fail the build instead of silently
claiming success.

If the variable is unset, the legacy best-effort full-registry mode remains
available, with the registry's declared order. It may rewrite unrelated sys
crates and synchronize all their API features; do not use it for an isolated
module change. Inspect tracked and untracked changed paths against the task's
explicit allowlist before handoff.

API12 is the baseline. Later declarations receive `api-N` gates, and manifests
chain every intermediate feature. Opaque forward declarations whose comments
bindgen omits inherit their earliest documented use. New child-process bindings
cover chained `api-13` through `api-26`; building against SDK26 does not enable
API26 imports when only `api-22` is selected.

### Verification

The selector can be tested without an SDK or executing the build script:

```sh
rustc --edition 2021 --test build/selection.rs -o /tmp/ohos-config-selector-tests
/tmp/ohos-config-selector-tests
```

The registry and availability regressions use the owning build-script modules.
From the workspace root, with `OHOS_NDK_HOME` configured:

```sh
OHOS_BINDINGS_GENERATE_CONFIG=ohos-native-child-process-sys \
LIBCLANG_PATH="$OHOS_NDK_HOME/native/llvm/lib" \
BINDGEN_EXTRA_CLANG_ARGS="--target=aarch64-linux-ohos --sysroot=$OHOS_NDK_HOME/native/sysroot" \
cargo test -p generate --test generator_contract
```

This Cargo test still runs the build script first, so keep the selector set.
Run formatting from the workspace root after generation. To force a second
generation with identical inputs, use `cargo clean -p generate` and repeat the
selected command; compare output digests and confirm unrelated sys files remain
unchanged. Lockfiles and `dist` outputs remain ignored per repository policy.
