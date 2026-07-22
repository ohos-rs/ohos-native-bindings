use crate::error::{check, is_param_invalid, AbilityBaseError, Result};
use ohos_ability_base_sys as sys;
use std::ffi::{CStr, CString};
use std::os::fd::{AsRawFd as _, BorrowedFd, RawFd};
use std::os::raw::c_char;
use std::ptr::NonNull;

/// Buffer size of the first attempt when reading a string out of a want, in
/// bytes.
const INITIAL_CAPACITY: usize = 256;

/// Largest buffer the string getters grow to, in bytes.
const MAX_CAPACITY: usize = 64 * 1024;

/// The element a want points at: the bundle, module and ability to reach.
///
/// The native `AbilityBase_Element` is a plain struct of three `char*` fields
/// with no accompanying destructor, and the runtime does not promise to copy
/// what it is handed: up to and including OpenHarmony 6.1
/// `OH_AbilityBase_CreateWant` and `OH_AbilityBase_SetWantElement` store the
/// three pointers verbatim and `OH_AbilityBase_GetWantElement` hands the same
/// pointers back. This type therefore holds owned Rust strings and is converted
/// at the call boundary, while [`Want`] keeps the converted C strings alive for
/// as long as the want that points at them.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Element {
    /// Name of the application bundle.
    pub bundle_name: String,
    /// Name of the module inside the bundle.
    pub module_name: String,
    /// Name of the ability inside the module.
    pub ability_name: String,
}

impl Element {
    /// Build an element from its bundle, module and ability name.
    pub fn new(
        bundle_name: impl Into<String>,
        module_name: impl Into<String>,
        ability_name: impl Into<String>,
    ) -> Self {
        Element {
            bundle_name: bundle_name.into(),
            module_name: module_name.into(),
            ability_name: ability_name.into(),
        }
    }

    /// Encode the three names as C strings, which must outlive the native call
    /// they are passed to.
    fn to_c_strings(&self) -> Result<[CString; 3]> {
        Ok([
            to_c_string(&self.bundle_name)?,
            to_c_string(&self.module_name)?,
            to_c_string(&self.ability_name)?,
        ])
    }

    /// Read back an element written by the runtime.
    ///
    /// The pointers are owned either by the want or by the [`Want`] wrapper
    /// that fed them in, so the strings are copied right away and never freed
    /// here. A field the runtime left unset comes back as an empty string.
    fn from_raw(raw: &sys::AbilityBase_Element) -> Result<Self> {
        Ok(Element {
            bundle_name: borrowed_to_string(raw.bundleName)?,
            module_name: borrowed_to_string(raw.moduleName)?,
            ability_name: borrowed_to_string(raw.abilityName)?,
        })
    }
}

/// A want: the description of the ability to start and the parameters handed
/// to it.
///
/// Created with [`Want::new`] and destroyed on drop, so
/// `OH_AbilityBase_CreateWant` and `OH_AbilityBase_DestroyWant` always pair up.
/// The type owns the native object and is neither `Send` nor `Sync`, matching
/// the native API, which documents no thread safety for it.
///
/// # Example
///
/// ```no_run
/// use ohos_ability_base_binding::{Element, Want};
///
/// let mut want = Want::new(&Element::new("com.example.app", "entry", "EntryAbility"))?;
/// want.set_char_param("greeting", "hello")?;
/// assert_eq!(want.char_param("greeting")?, "hello");
///
/// let element = want.element()?;
/// assert_eq!(element.ability_name, "EntryAbility");
/// # Ok::<(), ohos_ability_base_binding::AbilityBaseError>(())
/// ```
pub struct Want {
    raw: NonNull<sys::AbilityBase_Want>,
    /// The C strings the native want points at.
    ///
    /// `OH_AbilityBase_CreateWant` and `OH_AbilityBase_SetWantElement` take the
    /// element by value, but the three `char*` inside it are, up to and
    /// including OpenHarmony 6.1, stored as they are rather than copied, and
    /// `OH_AbilityBase_GetWantElement` returns those same pointers. Keeping the
    /// strings here ties their lifetime to the want. Releases that do copy the
    /// names only make this redundant, never wrong.
    names: [CString; 3],
}

impl Want {
    /// Create a want aimed at `element`.
    pub fn new(element: &Element) -> Result<Self> {
        let names = element.to_c_strings()?;
        let raw = {
            // SAFETY: the element only borrows the three C strings, which are
            // alive for the whole call and, once the want exists, for as long
            // as it does.
            unsafe { sys::OH_AbilityBase_CreateWant(RawElement(&names).to_raw()) }
        };
        NonNull::new(raw)
            .map(|raw| Want { raw, names })
            .ok_or(AbilityBaseError::CreateWantFailed)
    }

    /// Point this want at another element.
    pub fn set_element(&mut self, element: &Element) -> Result<()> {
        let names = element.to_c_strings()?;
        // SAFETY: the want is alive and the element borrows C strings that
        // outlive the call.
        check(unsafe {
            sys::OH_AbilityBase_SetWantElement(self.raw.as_ptr(), RawElement(&names).to_raw())
        })?;
        // The want may now hold these pointers, so the previous names are only
        // released once the call has taken the new ones.
        self.names = names;
        Ok(())
    }

    /// The element this want points at.
    pub fn element(&self) -> Result<Element> {
        let mut raw = sys::AbilityBase_Element {
            bundleName: std::ptr::null_mut(),
            moduleName: std::ptr::null_mut(),
            abilityName: std::ptr::null_mut(),
        };
        // SAFETY: the want is alive and `raw` is a valid output slot.
        check(unsafe { sys::OH_AbilityBase_GetWantElement(self.raw.as_ptr(), &mut raw) })?;
        Element::from_raw(&raw)
    }

    /// Store a string parameter under `key`.
    pub fn set_char_param(&mut self, key: &str, value: &str) -> Result<()> {
        let key = to_c_string(key)?;
        let value = to_c_string(value)?;
        // SAFETY: the want is alive and both C strings outlive the call.
        check(unsafe {
            sys::OH_AbilityBase_SetWantCharParam(self.raw.as_ptr(), key.as_ptr(), value.as_ptr())
        })
    }

    /// Read the string parameter stored under `key`.
    ///
    /// The output buffer is managed internally: it starts at 256 bytes and is
    /// doubled up to 64 KiB while the runtime rejects the call. Use
    /// [`Want::char_param_with_capacity`] to pick the buffer size explicitly.
    pub fn char_param(&self, key: &str) -> Result<String> {
        let key = to_c_string(key)?;
        grow(|capacity| self.read_char_param(&key, capacity))
    }

    /// Read the string parameter stored under `key` into a buffer of
    /// `capacity` bytes.
    ///
    /// `capacity` covers the terminating character as well, so it must exceed
    /// the length of the value. The native API reports a single error code for
    /// every failure, so a value that does not fit is indistinguishable from a
    /// missing key: both surface as `AbilityBaseError::Native(401)`. A capacity
    /// of zero yields [`AbilityBaseError::BufferTooSmall`] without calling into
    /// the runtime.
    pub fn char_param_with_capacity(&self, key: &str, capacity: usize) -> Result<String> {
        let key = to_c_string(key)?;
        self.read_char_param(&key, capacity)
    }

    fn read_char_param(&self, key: &CStr, capacity: usize) -> Result<String> {
        with_buffer(capacity, |buffer| {
            // SAFETY: the want is alive, the key outlives the call and the
            // buffer holds `capacity` writable bytes.
            unsafe {
                sys::OH_AbilityBase_GetWantCharParam(
                    self.raw.as_ptr(),
                    key.as_ptr(),
                    buffer.as_mut_ptr().cast::<c_char>(),
                    capacity,
                )
            }
        })
    }

    /// Add a file descriptor under `key`.
    ///
    /// The descriptor is only borrowed for the duration of the call; the
    /// runtime duplicates what it needs.
    pub fn add_fd(&mut self, key: &str, fd: BorrowedFd<'_>) -> Result<()> {
        let key = to_c_string(key)?;
        // SAFETY: the want is alive and the key outlives the call.
        check(unsafe {
            sys::OH_AbilityBase_AddWantFd(self.raw.as_ptr(), key.as_ptr(), fd.as_raw_fd())
        })
    }

    /// Read the file descriptor stored under `key`.
    ///
    /// The descriptor belongs to the want, so it must not be closed by the
    /// caller; duplicate it first if it has to outlive the want.
    pub fn fd(&self, key: &str) -> Result<RawFd> {
        let key = to_c_string(key)?;
        let mut fd: i32 = -1;
        // SAFETY: the want is alive, the key outlives the call and `fd` is a
        // valid output slot.
        check(unsafe { sys::OH_AbilityBase_GetWantFd(self.raw.as_ptr(), key.as_ptr(), &mut fd) })?;
        Ok(fd)
    }

    /// Set the URI carried by this want.
    #[cfg(feature = "api-17")]
    pub fn set_uri(&mut self, uri: &str) -> Result<()> {
        let uri = to_c_string(uri)?;
        // SAFETY: the want is alive and the URI outlives the call.
        check(unsafe { sys::OH_AbilityBase_SetWantUri(self.raw.as_ptr(), uri.as_ptr()) })
    }

    /// Read the URI carried by this want.
    ///
    /// The output buffer is managed internally, as for [`Want::char_param`].
    #[cfg(feature = "api-17")]
    pub fn uri(&self) -> Result<String> {
        grow(|capacity| self.read_uri(capacity))
    }

    /// Read the URI carried by this want into a buffer of `capacity` bytes.
    ///
    /// `capacity` covers the terminating character as well; see
    /// [`Want::char_param_with_capacity`] for how a buffer that is too small is
    /// reported.
    #[cfg(feature = "api-17")]
    pub fn uri_with_capacity(&self, capacity: usize) -> Result<String> {
        self.read_uri(capacity)
    }

    #[cfg(feature = "api-17")]
    fn read_uri(&self, capacity: usize) -> Result<String> {
        with_buffer(capacity, |buffer| {
            // SAFETY: the want is alive and the buffer holds `capacity`
            // writable bytes.
            unsafe {
                sys::OH_AbilityBase_GetWantUri(
                    self.raw.as_ptr(),
                    buffer.as_mut_ptr().cast::<c_char>(),
                    capacity,
                )
            }
        })
    }

    /// Store a 32-bit integer parameter under `key`.
    #[cfg(feature = "api-17")]
    pub fn set_int32_param(&mut self, key: &str, value: i32) -> Result<()> {
        let key = to_c_string(key)?;
        // SAFETY: the want is alive and the key outlives the call.
        check(unsafe {
            sys::OH_AbilityBase_SetWantInt32Param(self.raw.as_ptr(), key.as_ptr(), value)
        })
    }

    /// Read the 32-bit integer parameter stored under `key`.
    #[cfg(feature = "api-17")]
    pub fn int32_param(&self, key: &str) -> Result<i32> {
        let key = to_c_string(key)?;
        let mut value: i32 = 0;
        // SAFETY: the want is alive, the key outlives the call and `value` is a
        // valid output slot.
        check(unsafe {
            sys::OH_AbilityBase_GetWantInt32Param(self.raw.as_ptr(), key.as_ptr(), &mut value)
        })?;
        Ok(value)
    }

    /// Store a boolean parameter under `key`.
    #[cfg(feature = "api-17")]
    pub fn set_bool_param(&mut self, key: &str, value: bool) -> Result<()> {
        let key = to_c_string(key)?;
        // SAFETY: the want is alive and the key outlives the call.
        check(unsafe {
            sys::OH_AbilityBase_SetWantBoolParam(self.raw.as_ptr(), key.as_ptr(), value)
        })
    }

    /// Read the boolean parameter stored under `key`.
    #[cfg(feature = "api-17")]
    pub fn bool_param(&self, key: &str) -> Result<bool> {
        let key = to_c_string(key)?;
        let mut value = false;
        // SAFETY: the want is alive, the key outlives the call and `value` is a
        // valid output slot.
        check(unsafe {
            sys::OH_AbilityBase_GetWantBoolParam(self.raw.as_ptr(), key.as_ptr(), &mut value)
        })?;
        Ok(value)
    }

    /// Store a double parameter under `key`.
    #[cfg(feature = "api-17")]
    pub fn set_double_param(&mut self, key: &str, value: f64) -> Result<()> {
        let key = to_c_string(key)?;
        // SAFETY: the want is alive and the key outlives the call.
        check(unsafe {
            sys::OH_AbilityBase_SetWantDoubleParam(self.raw.as_ptr(), key.as_ptr(), value)
        })
    }

    /// Read the double parameter stored under `key`.
    #[cfg(feature = "api-17")]
    pub fn double_param(&self, key: &str) -> Result<f64> {
        let key = to_c_string(key)?;
        let mut value = 0.0;
        // SAFETY: the want is alive, the key outlives the call and `value` is a
        // valid output slot.
        check(unsafe {
            sys::OH_AbilityBase_GetWantDoubleParam(self.raw.as_ptr(), key.as_ptr(), &mut value)
        })?;
        Ok(value)
    }
}

impl Drop for Want {
    fn drop(&mut self) {
        // SAFETY: the want was created by `OH_AbilityBase_CreateWant`, is
        // destroyed exactly once and is not used afterwards. The names it may
        // still point at are dropped after this body, so they outlive the call.
        unsafe {
            sys::OH_AbilityBase_DestroyWant(self.raw.as_ptr());
        }
    }
}

/// The three C strings of an element, borrowed for the duration of one call.
///
/// `AbilityBase_Element` is three bare `char*`, so the lifetime is what keeps
/// the raw struct from outliving the strings that back it.
#[derive(Debug, Clone, Copy)]
struct RawElement<'a>(&'a [CString; 3]);

impl RawElement<'_> {
    /// The native struct takes `char*` even where it only reads, hence the cast.
    /// The result borrows `self` and must not outlive the call it is passed to.
    fn to_raw(self) -> sys::AbilityBase_Element {
        sys::AbilityBase_Element {
            bundleName: self.0[0].as_ptr().cast_mut(),
            moduleName: self.0[1].as_ptr().cast_mut(),
            abilityName: self.0[2].as_ptr().cast_mut(),
        }
    }
}

fn to_c_string(value: &str) -> Result<CString> {
    Ok(CString::new(value)?)
}

/// Copy a string the runtime handed out, without taking ownership of it.
fn borrowed_to_string(pointer: *const c_char) -> Result<String> {
    if pointer.is_null() {
        return Ok(String::new());
    }
    // SAFETY: when non-null, the runtime hands out a NUL-terminated string that
    // lives as long as the want it came from; it is only read here.
    let bytes = unsafe { CStr::from_ptr(pointer) }.to_bytes();
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| AbilityBaseError::InvalidUtf8)
}

/// Run a getter against an owned buffer of `capacity` bytes and decode what it
/// wrote.
///
/// `want.h` documents the size parameter as the "size of the value" and says
/// nothing about what happens when the buffer is too small, so the buffer is
/// treated as a capacity that has to cover the terminating character, and a
/// result without a terminator inside the buffer is rejected as
/// [`AbilityBaseError::BufferTooSmall`] rather than truncated.
fn with_buffer(
    capacity: usize,
    call: impl FnOnce(&mut [u8]) -> sys::AbilityBase_ErrorCode,
) -> Result<String> {
    if capacity == 0 {
        return Err(AbilityBaseError::BufferTooSmall(0));
    }
    let mut buffer = vec![0u8; capacity];
    check(call(&mut buffer))?;
    let end = buffer
        .iter()
        .position(|byte| *byte == 0)
        .ok_or(AbilityBaseError::BufferTooSmall(capacity))?;
    std::str::from_utf8(&buffer[..end])
        .map(str::to_owned)
        .map_err(|_| AbilityBaseError::InvalidUtf8)
}

/// Retry a string getter with an ever larger buffer while the runtime keeps
/// rejecting the call.
fn grow(mut read: impl FnMut(usize) -> Result<String>) -> Result<String> {
    let mut capacity = INITIAL_CAPACITY;
    loop {
        match read(capacity) {
            Err(error) if is_param_invalid(&error) && capacity < MAX_CAPACITY => {
                capacity = (capacity * 2).min(MAX_CAPACITY);
            }
            result => return result,
        }
    }
}
