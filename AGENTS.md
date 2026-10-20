# Native binding constraints

- Native capabilities must be selected by Cargo features and linked through the matching `*-sys` package. Runtime capability loading through `libloading`, `dlopen`, `dlsym`, or equivalent symbol probes is forbidden.
- Keep FFI declarations in `sys/*` and ownership, lifetime validation, null checks and native error handling in the owning binding crate. Applications and framework facades consume binding APIs, not `*-sys` dependencies or locally copied FFI declarations.
- Features introducing newer APIs must state their minimum runtime API. Runtime version checks do not make a linked symbol compatible with older loaders. With a feature disabled, its native symbols must not be referenced.
- Host regressions must use statically linked substitutes or Rust mocks. They must not reintroduce dynamic loaders.
