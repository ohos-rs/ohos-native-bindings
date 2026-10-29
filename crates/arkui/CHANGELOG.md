# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.3](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-arkui-binding-v0.5.2...ohos-arkui-binding-v0.5.3) - 2026-10-29

### Added

- *(arkui)* read and write node attributes without heap allocation  ([#204](https://github.com/ohos-rs/ohos-native-bindings/pull/204))

### Added

- `ArkUINodeAttributeItem` keeps up to 16 numbers inline and converts from
  arrays and slices of `ArkUINodeAttributeNumber`, `f32`, `i32` and `u32`;
  `from_numbers` builds one from a borrowed slice.
- `ArkUICommonAttribute::set_number_slice_attribute` and `set_str_attribute`
  set numbers and strings without heap allocation; `f32_attribute`,
  `u32_attribute` and `f32_pair_attribute` copy values out of a native
  attribute without allocating. `set_i32/u32/f32/bool_attribute` no longer
  allocate.
- `ArkUIContext::post_frame_callback` (api-18) and `post_idle_callback`
  (api-20) post frame and idle callbacks on a UI context, without a node;
  `ArkUIContext::raw_handle` returns its native handle.
- `ArkUIContext::enable_event_passthrough` and `RawInputEventType` (api-26).

### Changed

- Frame and idle callbacks (`post_frame_callback`, `post_idle_callback` on
  nodes and contexts) take `FnOnce`, as ArkUI calls them once; `Fn`
  callbacks still compile.

### Breaking

- `ArkUICommonAttribute::set_string_attribute` and `id` take
  `impl AsRef<str>` instead of `impl Into<String>`, so a value reaches ArkUI
  without a copy. Pass a `&str` or `String` where a `char` or another
  `Into<String>` type was passed.

## [0.5.2](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-arkui-binding-v0.5.1...ohos-arkui-binding-v0.5.2) - 2026-10-29

### Added

- *(drawing)* add owned native command recordings ([#202](https://github.com/ohos-rs/ohos-native-bindings/pull/202))

## [0.5.1](https://github.com/ohos-rs/ohos-native-bindings/compare/ohos-arkui-binding-v0.5.0...ohos-arkui-binding-v0.5.1) - 2026-10-27

### Added

- expand ArkUI, Drawing, ResourceManager, and DeviceInfo bindings ([#199](https://github.com/ohos-rs/ohos-native-bindings/pull/199))

### Breaking

- `Dialog::content` owns transferred content until successful native removal
  and disposal. Use fallible `remove_content`, `close` and `dispose` to retry
  cleanup; failed Drop logs and retains native-reachable owners for safety.
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

- Own dialog content and dismiss callback data through native cleanup, with custom content style and animation control.
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
