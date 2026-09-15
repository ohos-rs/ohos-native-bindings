## How to add a new crate binding?

This document will help you develop a new binding for OpenHarmony.

### Add config

Add a new config to `tools/generate/build/config` and mark it as public. Then add it to `CONFIG`

```rust
// tools/generate/build/main.rs
static CONFIG: Lazy<Vec<Lazy<SysConfig>>> = Lazy::new(|| {
    vec![
        // ... old config
        // add here
    ]
});
```

### Generate sys binding

Use `ohrs` from `tools/generate` to generate only the new sys crate. The selector
is one exact registered crate name; it is not a module alias, pattern or list.

```bash
cd tools/generate
OHOS_BINDINGS_GENERATE_CONFIG=ohos-native-child-process-sys ohrs build --arch aarch
```

After you ran the command, please run `pnpm run format`.

Compare `git status --short` and `git diff --name-only` with the task's explicit
path allowlist. The selected mode generates and synchronizes API features only
for that crate. An unset selector retains the legacy full-registry generation
mode and can rewrite unrelated modules; do not use it for a bounded new-crate
change. See [generator documentation](./tools/generate/README.md) for testing and
SDK setup.

### Generate binding

Use `cargo` command to generate a new crate in `crates` folder.

```bash
cargo new xx --lib
```

### Develop

Add sys binding as dependency and develop current crate.
