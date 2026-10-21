# ohos-accessibility-binding

Safe Rust wrappers for the ArkUI native accessibility provider API.

The crate follows the platform integration flow: obtain a provider from an
XComponent or an `ARKUI_NODE_CUSTOM`, register provider callbacks, populate
the element objects supplied by ArkUI, and proactively send accessibility
events.

```shell
cargo add ohos-accessibility-binding --features api-24
```

Platform adapters such as AccessKit should implement `ProviderCallbacks` and
retain their own accessibility tree. This crate intentionally does not own a
UI toolkit's tree or action model.

## Action description lifetime

`ElementInfo::set_operation_actions` accepts `&[(ActionType, &'static CStr)]`.
Use static descriptions such as `c"Activate"`. ArkUI shallow-copies these
pointers into element snapshots and exposes no retirement notification. Dynamic
text therefore requires caller-owned process-lifetime storage; the caller
controls its allocation policy.

## License

MIT OR Apache-2.0
