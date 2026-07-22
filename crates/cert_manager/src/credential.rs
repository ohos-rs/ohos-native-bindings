use crate::error::{check, CertManagerError, Result};
use crate::r#type::{CertUri, CertificatePurpose};
use ohos_cert_manager_sys as sys;
use std::borrow::Cow;
use std::fmt;
use std::os::raw::c_char;

/// Maximum length of the certificate chain a credential can carry.
pub const MAX_LEN_CERTIFICATE_CHAIN: usize = sys::OH_CM_MAX_LEN_CERTIFICATE_CHAIN as usize;

/// A borrowed view of one credential held by the certificate manager.
///
/// The data behind the view is owned by the [`Credential`] or [`CredentialList`]
/// it came from and is released when that owner is dropped.
#[derive(Clone, Copy)]
pub struct CredentialView<'a>(&'a sys::OH_CM_Credential);

impl<'a> CredentialView<'a> {
    /// Whether the credential holds certificate data.
    pub fn exists(&self) -> bool {
        self.0.isExist != 0
    }

    /// The credential type name.
    pub fn type_name(&self) -> Cow<'a, str> {
        field_str(&self.0.type_)
    }

    /// The credential alias.
    pub fn alias(&self) -> Cow<'a, str> {
        field_str(&self.0.alias)
    }

    /// The uri the credential is addressed by.
    pub fn key_uri(&self) -> Cow<'a, str> {
        field_str(&self.0.keyUri)
    }

    /// Number of certificates the credential contains.
    pub fn certificate_count(&self) -> u32 {
        self.0.certNum
    }

    /// Number of keys the credential contains.
    pub fn key_count(&self) -> u32 {
        self.0.keyNum
    }

    /// The purpose the certificate is stored for, or `None` when the runtime
    /// reports a purpose this binding does not know.
    pub fn purpose(&self) -> Option<CertificatePurpose> {
        CertificatePurpose::from_raw(self.0.certPurpose)
    }

    /// The certificate chain in its binary form, empty when the credential
    /// carries no data.
    pub fn data(&self) -> &'a [u8] {
        let blob = &self.0.credData;
        if blob.data.is_null() || blob.size == 0 {
            return &[];
        }
        // SAFETY: on success the certificate manager reports `size` readable
        // bytes at `data`; the buffer is owned by the credential this view
        // borrows and outlives the view.
        unsafe { std::slice::from_raw_parts(blob.data, blob.size as usize) }
    }
}

impl fmt::Debug for CredentialView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CredentialView")
            .field("exists", &self.exists())
            .field("type_name", &self.type_name())
            .field("alias", &self.alias())
            .field("key_uri", &self.key_uri())
            .field("certificate_count", &self.certificate_count())
            .field("key_count", &self.key_count())
            .field("purpose", &self.purpose())
            .field("data_len", &self.data().len())
            .finish()
    }
}

/// Read one of the fixed-size, NUL-terminated character fields of a credential.
fn field_str<const N: usize>(field: &[c_char; N]) -> Cow<'_, str> {
    // SAFETY: `c_char` and `u8` have the same size and alignment, and the array
    // is fully initialized for its whole length.
    let bytes = unsafe { std::slice::from_raw_parts(field.as_ptr().cast::<u8>(), N) };
    let end = bytes.iter().position(|b| *b == 0).unwrap_or(N);
    String::from_utf8_lossy(&bytes[..end])
}

/// A single credential returned by the certificate manager.
///
/// Owns the buffers the service allocated inside it and releases them with
/// `OH_CertManager_FreeCredential` when dropped.
pub struct Credential {
    inner: Box<sys::OH_CM_Credential>,
}

impl Credential {
    /// Borrow the credential to read its fields.
    pub fn view(&self) -> CredentialView<'_> {
        CredentialView(&self.inner)
    }

    /// Whether the credential holds certificate data.
    pub fn exists(&self) -> bool {
        self.view().exists()
    }

    /// The credential type name.
    pub fn type_name(&self) -> Cow<'_, str> {
        self.view().type_name()
    }

    /// The credential alias.
    pub fn alias(&self) -> Cow<'_, str> {
        self.view().alias()
    }

    /// The uri the credential is addressed by.
    pub fn key_uri(&self) -> Cow<'_, str> {
        self.view().key_uri()
    }

    /// Number of certificates the credential contains.
    pub fn certificate_count(&self) -> u32 {
        self.view().certificate_count()
    }

    /// Number of keys the credential contains.
    pub fn key_count(&self) -> u32 {
        self.view().key_count()
    }

    /// The purpose the certificate is stored for, or `None` when the runtime
    /// reports a purpose this binding does not know.
    pub fn purpose(&self) -> Option<CertificatePurpose> {
        self.view().purpose()
    }

    /// The certificate chain in its binary form, empty when the credential
    /// carries no data.
    pub fn data(&self) -> &[u8] {
        self.view().data()
    }
}

impl fmt::Debug for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.view(), f)
    }
}

impl Drop for Credential {
    fn drop(&mut self) {
        // SAFETY: `inner` was filled in by a successful certificate manager call
        // and is freed exactly once, here; the struct itself stays owned by the
        // box and is only read from before this point.
        unsafe { sys::OH_CertManager_FreeCredential(&mut *self.inner) };
    }
}

// SAFETY: a credential owns its buffers exclusively, has no interior mutability
// and no thread affinity: reading its fields and releasing it are valid from any
// thread as long as Rust's borrow rules are respected.
unsafe impl Send for Credential {}
// SAFETY: see above; all accessors take `&self` and only read.
unsafe impl Sync for Credential {}

/// A list of credentials returned by [`get_ukey_certificate`].
///
/// Owns the array the service allocated and releases it with
/// `OH_CertManager_FreeUkeyCertificate` when dropped. Individual entries are
/// borrowed as [`CredentialView`] and are never released on their own.
pub struct CredentialList {
    inner: Box<sys::OH_CM_CredentialDetailList>,
}

impl CredentialList {
    /// Number of credentials in the list.
    pub fn len(&self) -> usize {
        if self.inner.credential.is_null() {
            0
        } else {
            self.inner.credentialCount as usize
        }
    }

    /// Whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The credential at `index`, or `None` when out of range.
    pub fn get(&self, index: usize) -> Option<CredentialView<'_>> {
        if index >= self.len() {
            return None;
        }
        // SAFETY: `index` is below `credentialCount` and the service reports
        // that many initialized entries at `credential`, owned by this list.
        let entry = unsafe { &*self.inner.credential.add(index) };
        Some(CredentialView(entry))
    }

    /// Iterate over the credentials in the list.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = CredentialView<'_>> + '_ {
        (0..self.len()).map(|index| self.get(index).expect("index below len is always in range"))
    }
}

impl fmt::Debug for CredentialList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a> IntoIterator for &'a CredentialList {
    type Item = CredentialView<'a>;
    type IntoIter = Box<dyn ExactSizeIterator<Item = CredentialView<'a>> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

impl Drop for CredentialList {
    fn drop(&mut self) {
        // SAFETY: `inner` was filled in by a successful
        // OH_CertManager_GetUkeyCertificate call and is freed exactly once, here.
        unsafe { sys::OH_CertManager_FreeUkeyCertificate(&mut *self.inner) };
    }
}

// SAFETY: see the corresponding impls for `Credential`.
unsafe impl Send for CredentialList {}
// SAFETY: see the corresponding impls for `Credential`.
unsafe impl Sync for CredentialList {}

/// Borrowed bytes passed to the certificate manager as an `OH_CM_Blob`.
///
/// The lifetime keeps the raw blob tied to the buffer that backs it: an
/// `OH_CM_Blob` is a bare pointer and length, so it must never outlive the
/// call it is built for.
#[derive(Debug, Clone, Copy)]
struct CertBlob<'a>(&'a [u8]);

impl CertBlob<'_> {
    /// The blob borrows `self`, so it must not outlive the call it is passed to.
    fn to_raw(self) -> Result<sys::OH_CM_Blob> {
        let size = u32::try_from(self.0.len()).map_err(|_| {
            CertManagerError::invalid_argument("uri length does not fit in a blob size")
        })?;
        Ok(sys::OH_CM_Blob {
            size,
            data: self.0.as_ptr().cast_mut(),
        })
    }
}

fn empty_credential() -> Box<sys::OH_CM_Credential> {
    // SAFETY: `OH_CM_Credential` is a plain C struct of integers, character
    // arrays and one pointer, for all of which an all-zero value is valid.
    Box::new(unsafe { std::mem::zeroed() })
}

/// Get the detail of an application private certificate.
///
/// Requires the `ohos.permission.ACCESS_CERT_MANAGER` permission.
pub fn get_private_certificate(uri: &CertUri) -> Result<Credential> {
    let buffer = uri.to_bytes_with_nul();
    let key_uri = CertBlob(&buffer).to_raw()?;
    let mut credential = empty_credential();
    // SAFETY: `key_uri` points at a valid blob backed by `buffer` for the whole
    // call, and `credential` is a zeroed, exclusively owned output struct.
    let code = unsafe { sys::OH_CertManager_GetPrivateCertificate(&key_uri, &mut *credential) };
    check(code)?;
    Ok(Credential { inner: credential })
}

/// Get the detail of a user public certificate.
///
/// Requires the `ohos.permission.ACCESS_CERT_MANAGER` permission.
pub fn get_public_certificate(uri: &CertUri) -> Result<Credential> {
    let buffer = uri.to_bytes_with_nul();
    let key_uri = CertBlob(&buffer).to_raw()?;
    let mut credential = empty_credential();
    // SAFETY: `key_uri` points at a valid blob backed by `buffer` for the whole
    // call, and `credential` is a zeroed, exclusively owned output struct.
    let code = unsafe { sys::OH_CertManager_GetPublicCertificate(&key_uri, &mut *credential) };
    check(code)?;
    Ok(Credential { inner: credential })
}

/// Get the detail of a USB key certificate, selected by `purpose`.
///
/// Requires the `ohos.permission.ACCESS_CERT_MANAGER` permission.
pub fn get_ukey_certificate(uri: &CertUri, purpose: CertificatePurpose) -> Result<CredentialList> {
    let buffer = uri.to_bytes_with_nul();
    let key_uri = CertBlob(&buffer).to_raw()?;
    let ukey_info = sys::OH_CM_UkeyInfo {
        certPurpose: purpose.to_raw(),
    };
    // SAFETY: `OH_CM_CredentialDetailList` is a count plus a pointer, for both of
    // which an all-zero value is valid.
    let mut list: Box<sys::OH_CM_CredentialDetailList> = Box::new(unsafe { std::mem::zeroed() });
    // SAFETY: both inputs are valid for the whole call and `list` is a zeroed,
    // exclusively owned output struct.
    let code = unsafe { sys::OH_CertManager_GetUkeyCertificate(&key_uri, &ukey_info, &mut *list) };
    check(code)?;
    Ok(CredentialList { inner: list })
}
