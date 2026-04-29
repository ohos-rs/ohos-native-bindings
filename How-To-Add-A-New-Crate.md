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

Use `ohrs` to generate sys crate.

```bash
ohrs build --arch aarch
```

After you ran the command, please run `just fmt`.

### Generate binding

Use `cargo` command to generate a new crate in `crates` folder.

```bash
cargo new xx --lib
```

### Develop

Add sys binding as dependency and develop current crate.