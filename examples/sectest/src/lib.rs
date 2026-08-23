//! On-device probes for the certificate manager and TEE client safe layers.
//!
//! Both services are expected to refuse the calls made here: the certificate
//! uris point at credentials this application does not own, and no trusted
//! application is reachable. What is verified is therefore the behaviour on the
//! failure path — the safe layer must return `Err` with a readable code instead
//! of panicking or corrupting the native state, and the RAII types must unwind
//! cleanly. Every string below is produced from a real call.

use napi_derive_ohos::napi;
use ohos_cert_manager_binding as cert_manager;
use ohos_tee_client_binding as tee;

const TAG: &str = "SECTEST";

/// A certificate uri in the documented `oh:t=<type>;o=<alias>;u=<user>;a=<app>`
/// form that no credential is stored under.
const ABSENT_APP_URI: &str = "oh:t=ak;o=rust-sectest-absent;u=0;a=0";
/// The same, for the user certificate store.
const ABSENT_USER_URI: &str = "oh:t=uc;o=rust-sectest-absent;u=0;a=0";

fn report(message: String) -> String {
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, message);
    message
}

fn cert_error(error: &cert_manager::CertManagerError) -> String {
    format!(
        "err({} {}, is_not_found={}, message={}, display=\"{error}\")",
        error.code(),
        cert_manager::describe(error.code()),
        error.is_not_found(),
        error.message().unwrap_or("none"),
    )
}

fn tee_error(error: &tee::TeeError) -> String {
    format!(
        "err(0x{:08X} {}, origin={}, message={}, display=\"{error}\")",
        error.code(),
        error.kind().map_or("unclassified", tee::ErrorKind::as_str),
        error.origin().map_or("none", tee::ReturnOrigin::as_str),
        error.message().unwrap_or("none"),
    )
}

// CertUri must reject an empty, over-long or NUL-containing uri before any FFI
// call happens; a well-formed uri must be accepted.
#[napi]
pub fn test_cert_uri_validation() -> String {
    let over_long = "a".repeat(cert_manager::CertUri::MAX_LEN + 1);
    let cases = [
        ("empty", String::new()),
        ("over_max_len", over_long),
        ("interior_nul", "oh:t=ak;o=ru\0st;u=0;a=0".to_string()),
        ("at_max_len", "b".repeat(cert_manager::CertUri::MAX_LEN)),
        ("well_formed", ABSENT_APP_URI.to_string()),
    ];
    let mut parts = vec![format!("max_len={}", cert_manager::CertUri::MAX_LEN)];
    for (name, value) in cases {
        parts.push(match cert_manager::CertUri::new(value) {
            Ok(uri) => format!("{name} ok(len={})", uri.as_str().len()),
            Err(error) => format!("{name} {}", cert_error(&error)),
        });
    }
    report(parts.join("; "))
}

// The description table must map every documented code and fall back for the
// rest. Inputs come from the raw constants, not from copied numbers.
#[napi]
pub fn test_cert_describe_table() -> String {
    use cert_manager::sys as raw;

    let codes = [
        raw::OH_CM_ErrorCode_OH_CM_SUCCESS as i32,
        raw::OH_CM_ErrorCode_OH_CM_HAS_NO_PERMISSION as i32,
        raw::OH_CM_ErrorCode_OH_CM_CAPABILITY_NOT_SUPPORTED as i32,
        raw::OH_CM_ErrorCode_OH_CM_INNER_FAILURE as i32,
        raw::OH_CM_ErrorCode_OH_CM_NOT_FOUND as i32,
        raw::OH_CM_ErrorCode_OH_CM_NO_AUTHORIZATION as i32,
        raw::OH_CM_ErrorCode_OH_CM_ACCESS_UKEY_SERVICE_FAILED as i32,
        raw::OH_CM_ErrorCode_OH_CM_PARAMETER_VALIDATION_FAILED as i32,
        -1, // outside the enum, must fall back
    ];
    let parts: Vec<String> = codes
        .iter()
        .map(|code| format!("{code}=>{}", cert_manager::describe(*code)))
        .collect();
    report(parts.join("; "))
}

// Look up a private certificate that does not exist. Expected: Err, most likely
// 17500002 (certificate does not exist) or 201 (permission denied) when the
// application lacks ohos.permission.ACCESS_CERT_MANAGER.
#[napi]
pub fn test_get_private_certificate_absent() -> String {
    let uri = match cert_manager::CertUri::new(ABSENT_APP_URI) {
        Ok(uri) => uri,
        Err(error) => return report(format!("private uri rejected {}", cert_error(&error))),
    };
    let message = match cert_manager::get_private_certificate(&uri) {
        Ok(credential) => format!(
            "private ok(exists={}, alias={}, key_uri={}, certs={}, keys={}, purpose={:?}, data_len={})",
            credential.exists(),
            credential.alias(),
            credential.key_uri(),
            credential.certificate_count(),
            credential.key_count(),
            credential.purpose(),
            credential.data().len()
        ),
        Err(error) => format!("private {}", cert_error(&error)),
    };
    // The credential, if any, is dropped here: its buffers go back to the
    // certificate manager before this function returns.
    report(message)
}

// Same for a user certificate that does not exist.
#[napi]
pub fn test_get_public_certificate_absent() -> String {
    let uri = match cert_manager::CertUri::new(ABSENT_USER_URI) {
        Ok(uri) => uri,
        Err(error) => return report(format!("public uri rejected {}", cert_error(&error))),
    };
    let message = match cert_manager::get_public_certificate(&uri) {
        Ok(credential) => format!(
            "public ok(exists={}, alias={}, type={}, certs={}, data_len={})",
            credential.exists(),
            credential.alias(),
            credential.type_name(),
            credential.certificate_count(),
            credential.data().len()
        ),
        Err(error) => format!("public {}", cert_error(&error)),
    };
    report(message)
}

// The USB key query is the only path returning a CredentialList. No USB key is
// attached, so the expected outcome is Err (17500010 access to usb key service
// failed, 17500002, or 201); on success the list is iterated and released.
#[napi]
pub fn test_get_ukey_certificate_absent() -> String {
    let uri = match cert_manager::CertUri::new(ABSENT_APP_URI) {
        Ok(uri) => uri,
        Err(error) => return report(format!("ukey uri rejected {}", cert_error(&error))),
    };
    let message =
        match cert_manager::get_ukey_certificate(&uri, cert_manager::CertificatePurpose::All) {
            Ok(list) => {
                let aliases: Vec<String> = list
                    .iter()
                    .map(|entry| format!("{}:{}", entry.alias(), entry.data().len()))
                    .collect();
                format!(
                    "ukey ok(len={}, is_empty={}, entries=[{}])",
                    list.len(),
                    list.is_empty(),
                    aliases.join(",")
                )
            }
            Err(error) => format!("ukey {}", cert_error(&error)),
        };
    report(message)
}

// Connect to the default TEE. Without a reachable secure world this is expected
// to fail; the point is that the failed context is not finalised on drop and the
// error keeps its code.
#[napi]
pub fn test_tee_context_init() -> String {
    let message = match tee::Context::new() {
        Ok(context) => {
            drop(context);
            "context ok(initialised and finalised)".to_string()
        }
        Err(error) => format!("context {}", tee_error(&error)),
    };
    report(message)
}

// A path with an interior NUL must be refused by the safe layer, before the
// native call, with the bad-parameters code and a message.
#[napi]
pub fn test_tee_context_bad_path() -> String {
    let message = match tee::Context::with_path("/dev/tee\0bad") {
        Ok(context) => {
            drop(context);
            "context bad path unexpectedly accepted".to_string()
        }
        Err(error) => format!("context bad path {}", tee_error(&error)),
    };
    report(message)
}

// Shared memory needs a live context; when there is none the context error is
// reported as is. Both the allocated and the registered form are exercised.
#[napi]
pub fn test_tee_shared_memory() -> String {
    let context = match tee::Context::new() {
        Ok(context) => context,
        Err(error) => {
            return report(format!(
                "shared memory skipped, context {}",
                tee_error(&error)
            ))
        }
    };
    let mut parts = Vec::new();
    match tee::SharedMemory::allocate(&context, 64, tee::Direction::InOut) {
        Ok(memory) => parts.push(format!(
            "allocate ok(len={}, direction={:?}, is_allocated={})",
            memory.len(),
            memory.direction(),
            memory.is_allocated()
        )),
        Err(error) => parts.push(format!("allocate {}", tee_error(&error))),
    }
    let mut buffer = [0u8; 64];
    match tee::SharedMemory::register(&context, &mut buffer, tee::Direction::Input) {
        Ok(memory) => parts.push(format!(
            "register ok(len={}, is_allocated={})",
            memory.len(),
            memory.is_allocated()
        )),
        Err(error) => parts.push(format!("register {}", tee_error(&error))),
    }
    report(parts.join("; "))
}

// The packed paramTypes field is derived from the slots, so it can be checked
// without a TEE: slot 0 VALUE_INPUT (1), slot 1 MEMREF_TEMP_INPUT (5), slot 2
// MEMREF_TEMP_OUTPUT (6), slot 3 NONE (0) pack to 0x0651.
#[napi]
pub fn test_tee_operation_param_types() -> String {
    let request = [1u8, 2, 3, 4];
    let mut response = [0u8; 16];
    let mut operation = tee::Operation::with_params([
        tee::Parameter::Value {
            direction: tee::Direction::Input,
            a: 7,
            b: 9,
        },
        tee::Parameter::TempMemoryInput(&request),
        tee::Parameter::TempMemoryOutput(&mut response),
        tee::Parameter::None,
    ]);
    let packed = operation.param_types();
    let mut parts = vec![
        format!("param_count={}", tee::PARAM_COUNT),
        format!("param_types=0x{packed:04X}"),
        format!("value0={:?}", operation.value(0)),
        format!("output_size1={:?}", operation.output_size(1)),
        format!("output_size3={:?}", operation.output_size(3)),
    ];
    parts.push(
        match operation.set_param(tee::PARAM_COUNT, tee::Parameter::None) {
            Ok(()) => "set_param out of range unexpectedly accepted".to_string(),
            Err(error) => format!("set_param out of range {}", tee_error(&error)),
        },
    );
    parts.push(
        match operation.set_param(
            3,
            tee::Parameter::Value {
                direction: tee::Direction::Output,
                a: 0,
                b: 0,
            },
        ) {
            Ok(()) => format!(
                "set_param 3 ok, param_types=0x{:04X}",
                operation.param_types()
            ),
            Err(error) => format!("set_param 3 {}", tee_error(&error)),
        },
    );
    // Cancellation on an operation that was never submitted must be a no-op
    // rather than a crash.
    operation.request_cancellation();
    parts.push("request_cancellation returned".to_string());
    report(parts.join("; "))
}

// UUID parsing is pure and local: a canonical string must round-trip, and the
// malformed forms must be rejected with the bad-parameters code.
#[napi]
pub fn test_tee_uuid_parsing() -> String {
    let canonical = "79b77788-9789-4a7a-a2be-b60155eef5f3";
    let mut parts = Vec::new();
    match canonical.parse::<tee::Uuid>() {
        Ok(uuid) => parts.push(format!(
            "canonical ok(round_trip={}, matches={})",
            uuid,
            uuid.to_string() == canonical
        )),
        Err(error) => parts.push(format!("canonical {}", tee_error(&error))),
    }
    for (name, text) in [
        ("short", "79b77788-9789-4a7a-a2be"),
        ("no_hyphens", "79b7778897894a7aa2beb60155eef5f3aaaa"),
        ("non_hex", "79b77788-9789-4a7a-a2be-b60155eef5fz"),
    ] {
        parts.push(match text.parse::<tee::Uuid>() {
            Ok(uuid) => format!("{name} unexpectedly parsed as {uuid}"),
            Err(error) => format!("{name} {}", tee_error(&error)),
        });
    }
    let from_bytes = tee::Uuid::from_bytes([
        0x79, 0xb7, 0x77, 0x88, 0x97, 0x89, 0x4a, 0x7a, 0xa2, 0xbe, 0xb6, 0x01, 0x55, 0xee, 0xf5,
        0xf3,
    ]);
    parts.push(format!(
        "from_bytes={from_bytes}, to_bytes_round_trip={}",
        tee::Uuid::from_bytes(from_bytes.to_bytes()) == from_bytes
    ));
    report(parts.join("; "))
}
