# ohos-udmf-binding

This crate is a binding for the UDMF module in OpenHarmony.

## Install

```shell
cargo add ohos-udmf-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_udmf_binding as udmf;

// Use the safe Rust APIs exposed by `ohos-udmf-binding` from your native module.
```

## General entries and type identifiers

`UdmfRecord::add_general_entry(type_id, bytes)` copies a custom entry into the
native record. The SDK validates the supported identifiers and size (the
general-entry API accepts up to 4 KiB). Use typed UDS methods for standard UDS
records such as plain text, HTML and pixel maps.

The upstream `OH_UdmfRecord_AddGeneralEntry` rejects `count == 0`.
Empty input returns `UdmfError::EmptyEntry` before a native call, with no sentinel
encoding. General entries store an `ARRAY_BUFFER`; built-in plain-text and
HTML UDS records are not readable as raw bytes through `general_entry`.
For other apps' standard text clipboards, call `plain_text()` / `html()` on
`UdmfRecord` or `UdmfRecordRef`, then the existing UDS content getters.
These typed methods return independently owned UDS instances (API 12).
Both upstream facts are in
[`framework/ndkimpl/data/udmf.cpp`](https://github.com/openharmony/distributeddatamgr_udmf/blob/master/framework/ndkimpl/data/udmf.cpp)
(`OH_UdmfRecord_AddGeneralEntry` and `GetValueFromUint8Array`).

`UdmfRecord::general_entry` and `UdmfRecordRef::general_entry` return owned
byte vectors copied from record-owned storage. `types()` returns copied
identifiers on owned records, borrowed records and `UdmfData`.
`UdmfData::has_type(&str)` queries the SDK directly. These APIs are available
at the crate's baseline API 12 and reject interior NUL identifiers.

Native type arrays and entry buffers stay owned by the record/data container;
Rust return values remain valid after the native cache changes or is destroyed.
See the [UDMF API reference](https://github.com/openharmony/docs/blob/master/en/application-dev/reference/apis-arkdata/capi-udmf-h.md)
and [native cache implementation](https://github.com/openharmony/distributeddatamgr_udmf/blob/master/framework/ndkimpl/data/udmf.cpp)
for the source ownership contract.

## License

MIT OR Apache-2.0
