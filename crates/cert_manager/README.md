# ohos-cert-manager-binding

This crate is a binding for the certificate manager module in OpenHarmony.

The certificate manager stores the certificates and credentials an application
is allowed to use: private (application) certificates, public (user)
certificates, and certificates held on a USB key. This crate wraps the native
`cm_native_api.h` C API with a safe layer.

## Install

```shell
cargo add ohos-cert-manager-binding --features api-22
```

## Usage

```rust
use ohos_cert_manager_binding as cert_manager;
use cert_manager::{CertUri, CertificatePurpose};

let uri = CertUri::new("oh:t=ak;o=alias;u=0;a=0")?;

// A single credential; its buffers are released when it is dropped.
let credential = cert_manager::get_public_certificate(&uri)?;
println!("alias: {}", credential.alias());
println!("chain: {} bytes", credential.data().len());

// A USB key may hold several credentials.
let list = cert_manager::get_ukey_certificate(&uri, CertificatePurpose::Sign)?;
for entry in list.iter() {
    println!("{} -> {} bytes", entry.alias(), entry.data().len());
}
```

## Coverage

The whole certificate manager NDK surface arrived in API 22, so everything in
this crate sits behind the `api-22` feature. Without it the crate exposes only
the raw `sys` re-export.

| Feature | Adds |
|---|---|
| `api-22` | `get_private_certificate`, `get_public_certificate`, `get_ukey_certificate`, `Credential`, `CredentialList`, `CredentialView`, `CertUri`, `CertificatePurpose`, `CertManagerError`, `describe` |

## Notes

- Every call needs the `ohos.permission.ACCESS_CERT_MANAGER` permission.
- `CertUri` is validated on construction: an empty uri, one longer than
  `OH_CM_MAX_LEN_URI - 1`, or one containing an interior NUL is rejected before
  the call rather than coming back as an opaque parameter-validation error. The
  uri is handed to the native API as a sized blob whose length includes the
  terminator.
- Ownership follows the free functions the header ships:
  `OH_CertManager_FreeCredential` "destroys a credential detail" and
  `OH_CertManager_FreeUkeyCertificate` "destroys a credential detail list", so
  the service owns what it wrote into the output structs. `Credential` and
  `CredentialList` are therefore RAII wrappers that call the matching free
  function on drop; the output struct is zeroed before the call and no
  caller-allocated buffer is attached to it. Entries of a `CredentialList` are
  borrowed as `CredentialView` and are never freed on their own.
- The fixed-size `type`, `alias` and `keyUri` fields are NUL-terminated
  character arrays; they are read back as `Cow<str>` with lossy UTF-8 decoding
  so a malformed field cannot fail a getter.
- `certPurpose` is reported as `Option<CertificatePurpose>`: `None` means the
  runtime returned a purpose newer than this binding.
- The header documents `OH_CM_MAX_LEN_CERTIFICATE_CHAIN` as the upper bound of
  the certificate data a credential carries; it is re-exported as
  `MAX_LEN_CERTIFICATE_CHAIN` for callers that want to size their own storage.

## License

MIT OR Apache-2.0
