//! OpenHarmony File URI bindings for Rust
//!
//! This crate provides safe Rust bindings for OpenHarmony's file URI operations,
//! allowing conversion between file paths and URIs, URI validation, and directory operations.

pub mod error;

pub use error::FileUriError;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use error::error_from_code;
use ohos_fileuri_sys::*;

/// Get URI from file path
///
/// Converts a file system path to a file URI.
///
/// # Arguments
///
/// * `path` - The file system path
///
/// # Returns
///
/// Returns the URI string on success, or an error on failure.
///
/// # Example
///
/// ```no_run
/// use ohos_fileuri_binding::get_uri_from_path;
///
/// let uri = get_uri_from_path("/data/storage/el2/base/files/test.txt")?;
/// println!("URI: {}", uri);
/// # Ok::<(), ohos_fileuri_binding::FileUriError>(())
/// ```
pub fn get_uri_from_path(path: &str) -> std::result::Result<String, FileUriError> {
    let c_path = CString::new(path).map_err(|_| FileUriError::NullByteError)?;
    let mut result: *mut c_char = ptr::null_mut();

    let code =
        unsafe { OH_FileUri_GetUriFromPath(c_path.as_ptr(), path.len() as u32, &mut result) };

    if code == FileManagement_ErrCode_ERR_OK {
        if result.is_null() {
            return Err(FileUriError::ConversionError);
        }

        let uri = unsafe {
            let c_str = CStr::from_ptr(result);
            let rust_str = c_str
                .to_str()
                .map_err(|_| FileUriError::ConversionError)?
                .to_string();
            libc::free(result as *mut _);
            rust_str
        };

        Ok(uri)
    } else {
        if !result.is_null() {
            unsafe { libc::free(result as *mut _) };
        }
        Err(error_from_code(code))
    }
}

/// Get file path from URI
///
/// Converts a file URI to a file system path.
///
/// # Arguments
///
/// * `uri` - The file URI
///
/// # Returns
///
/// Returns the file system path on success, or an error on failure.
///
/// # Example
///
/// ```no_run
/// use ohos_fileuri_binding::get_path_from_uri;
///
/// let path = get_path_from_uri("file://com.example.app/data/storage/el2/base/files/test.txt")?;
/// println!("Path: {}", path);
/// # Ok::<(), ohos_fileuri_binding::FileUriError>(())
/// ```
pub fn get_path_from_uri(uri: &str) -> std::result::Result<String, FileUriError> {
    let c_uri = CString::new(uri).map_err(|_| FileUriError::NullByteError)?;
    let mut result: *mut c_char = ptr::null_mut();

    let code = unsafe { OH_FileUri_GetPathFromUri(c_uri.as_ptr(), uri.len() as u32, &mut result) };

    if code == FileManagement_ErrCode_ERR_OK {
        if result.is_null() {
            return Err(FileUriError::ConversionError);
        }

        let path = unsafe {
            let c_str = CStr::from_ptr(result);
            let rust_str = c_str
                .to_str()
                .map_err(|_| FileUriError::ConversionError)?
                .to_string();
            libc::free(result as *mut _);
            rust_str
        };

        Ok(path)
    } else {
        if !result.is_null() {
            unsafe { libc::free(result as *mut _) };
        }
        Err(error_from_code(code))
    }
}

/// Get the full directory URI from a file or directory URI
///
/// Gets the URI of the directory where the given URI is located.
///
/// # Arguments
///
/// * `uri` - The file or directory URI
///
/// # Returns
///
/// Returns the directory URI on success, or an error on failure.
///
/// # Example
///
/// ```no_run
/// use ohos_fileuri_binding::get_full_directory_uri;
///
/// let dir_uri = get_full_directory_uri("file://com.example.app/data/storage/el2/base/files/test.txt")?;
/// println!("Directory URI: {}", dir_uri);
/// # Ok::<(), ohos_fileuri_binding::FileUriError>(())
/// ```
pub fn get_full_directory_uri(uri: &str) -> std::result::Result<String, FileUriError> {
    let c_uri = CString::new(uri).map_err(|_| FileUriError::NullByteError)?;
    let mut result: *mut c_char = ptr::null_mut();

    let code =
        unsafe { OH_FileUri_GetFullDirectoryUri(c_uri.as_ptr(), uri.len() as u32, &mut result) };

    if code == FileManagement_ErrCode_ERR_OK {
        if result.is_null() {
            return Err(FileUriError::ConversionError);
        }

        let dir_uri = unsafe {
            let c_str = CStr::from_ptr(result);
            let rust_str = c_str
                .to_str()
                .map_err(|_| FileUriError::ConversionError)?
                .to_string();
            libc::free(result as *mut _);
            rust_str
        };

        Ok(dir_uri)
    } else {
        if !result.is_null() {
            unsafe { libc::free(result as *mut _) };
        }
        Err(error_from_code(code))
    }
}

/// Check if the given URI is valid
///
/// Validates whether the provided string is a valid file URI.
///
/// # Arguments
///
/// * `uri` - The URI to validate
///
/// # Returns
///
/// Returns `true` if the URI is valid, `false` otherwise.
///
/// # Example
///
/// ```no_run
/// use ohos_fileuri_binding::is_valid_uri;
///
/// if is_valid_uri("file://com.example.app/data/storage/el2/base/files/test.txt") {
///     println!("URI is valid");
/// } else {
///     println!("URI is invalid");
/// }
/// ```
pub fn is_valid_uri(uri: &str) -> bool {
    let Ok(c_uri) = CString::new(uri) else {
        return false;
    };

    unsafe { OH_FileUri_IsValidUri(c_uri.as_ptr(), uri.len() as u32) }
}

/// Get file name from URI (API 13+)
///
/// Extracts the file name (last segment) from a file URI.
/// Note: This function does not support media type URIs.
///
/// # Arguments
///
/// * `uri` - The file URI
///
/// # Returns
///
/// Returns the file name on success, or an error on failure.
///
/// # Example
///
/// ```no_run
/// use ohos_fileuri_binding::get_file_name;
///
/// # #[cfg(feature = "api-13")]
/// let filename = get_file_name("file://com.example.app/data/storage/el2/base/files/test.txt")?;
/// # #[cfg(feature = "api-13")]
/// println!("File name: {}", filename);
/// # Ok::<(), ohos_fileuri_binding::FileUriError>(())
/// ```
#[cfg(feature = "api-13")]
pub fn get_file_name(uri: &str) -> std::result::Result<String, FileUriError> {
    let c_uri = CString::new(uri).map_err(|_| FileUriError::NullByteError)?;
    let mut result: *mut c_char = ptr::null_mut();

    let code = unsafe { OH_FileUri_GetFileName(c_uri.as_ptr(), uri.len() as u32, &mut result) };

    if code == FileManagement_ErrCode_ERR_OK {
        if result.is_null() {
            return Err(FileUriError::ConversionError);
        }

        let filename = unsafe {
            let c_str = CStr::from_ptr(result);
            let rust_str = c_str
                .to_str()
                .map_err(|_| FileUriError::ConversionError)?
                .to_string();
            libc::free(result as *mut _);
            rust_str
        };

        Ok(filename)
    } else {
        if !result.is_null() {
            unsafe { libc::free(result as *mut _) };
        }
        Err(error_from_code(code))
    }
}
