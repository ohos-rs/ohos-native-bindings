# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking

- `accessibility` selects API 23, matching `NodeAccessibility`'s native API
  floor. Enable it only on API-23+ runtimes.
- `ArkUINode::from_raw_handle`, `CustomNodeEventData::{removed_node,host_node}`,
  `GestureEventRef::node`, `TouchRecognizerRef::node` and
  `GestureEventData::node_handle` are unsafe. Keep the native owner live for
  the complete lifetime of each returned borrowed wrapper.
- `LayoutConstraintHandle<'event>` is callback-scoped and no longer Copy or
  Clone. It disposes its native constraint snapshot on Drop; copy dimensions
  rather than retaining the guard outside the callback.
- `KeyEvent::lock_state` requires `api-19`. Enable it only for API-19+ runtimes;
  lock queries link directly through the sys crate instead of runtime loading.
- `Event::string_value` returns `Result<String, StringValueError>` instead of
  `Option<String>`. Native errors, invalid UTF-8 and capacity failures are
  distinct. The fixed 1 MiB cap is removed.

### Fixed

- Copy mixed-event values with indexed output storage, retain accessibility text, and retire NodeContent callbacks safely during reentrant calls.
- Keep API-14 key callbacks independent of the API-20 input discriminator.
- Forward API features weakly to the optional XComponent binding, so API
  selection alone does not enable that capability.
- Borrow key and drag events from their native callbacks; own drag actions, listeners, UDMF data and preview maps through disposal.

- Align text helpers, joystick key sources, and dialog state APIs with their minimum native API levels.
- Gate N-API context consumers and support error formatting when building without default features.

### Added

- Expose `NodeContent`, identity-stable `add_existing_node` and
  `unsafe ArkUINodeRaw::borrowed_node` for externally retained ArkTS nodes.
- Expose `ArkUINode::{into_rc,bind_rc}` with shared-owner disposal checks,
  event-route registration and native receiver add/remove operations for
  host-owned NodeContent moves.
- Add `ArkUINode::{parent,remove_from_parent}` for native parent edges.
- Add callback-scoped custom layout helpers, copied integer attributes, typed view/input policies, and event/gesture registration on identity-stable nodes.
- Add `ArkUINode::request_focus` for the native API-15 focus operation.
- Add safe ArkUI key-event, component-event union, and node animator wrappers so downstream UI frameworks do not need direct `ohos-arkui-sys` access.
- Add `ArkUINode::native_xcomponent` so downstream frameworks do not need the XComponent sys crate to resolve a mounted surface.
- Add API 25/26 feature forwarding and derive the complete API 26 ArkUI key-code, event-type, source, and intention enums through `ohos-enum-derive`.

## [0.3.0](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-arkui-binding-v0.2.5...ohos-arkui-binding-v0.3.0) - 2026-08-26

### Added

- *(arkui)* snapshot gesture input data
- xcomponent arkui gesture ([#141](https://github.com/ohos-rs/ohos-native-bindings/pull/141))

### Fixed

- fix some binding issues ([#133](https://github.com/ohos-rs/ohos-native-bindings/pull/133))

### Other

- migrate examples UI workspace ([#143](https://github.com/ohos-rs/ohos-native-bindings/pull/143))
