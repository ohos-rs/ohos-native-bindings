use napi_derive_ohos::napi;
use ohos_ipc_binding as ipc;

const TAG: &str = "IPC_TEST";
const DESCRIPTOR: &str = "org.example.IIpcSelfTest";

fn log(msg: &str) {
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
}

// Round-trip every typed accessor through a single parcel and compare each
// value read back with the value written. This is the hardest check of the FFI
// core: a wrong signature or a missing cursor advance shows up as a mismatch.
#[napi]
pub fn test_parcel_roundtrip() -> String {
    let msg = match roundtrip() {
        Ok(msg) => msg,
        Err(e) => format!("parcel roundtrip err({e})"),
    };
    log(&msg);
    msg
}

fn roundtrip() -> ipc::Result<String> {
    const I8: i8 = -12;
    const I16: i16 = 4096;
    const I32: i32 = 42;
    const I64: i64 = -7;
    const F32: f32 = 1.5;
    const F64: f64 = 3.5;
    const STR: &str = "hello";
    const BUF: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

    let mut parcel = ipc::Parcel::new()?;
    parcel.write_interface_token(DESCRIPTOR)?;
    parcel.write_i8(I8)?;
    parcel.write_i16(I16)?;
    parcel.write_i32(I32)?;
    parcel.write_i64(I64)?;
    parcel.write_f32(F32)?;
    parcel.write_f64(F64)?;
    parcel.write_string(STR)?;
    parcel.write_buffer(&BUF)?;

    let token = parcel.read_interface_token()?;
    let i8_read = parcel.read_i8()?;
    let i16_read = parcel.read_i16()?;
    let i32_read = parcel.read_i32()?;
    let i64_read = parcel.read_i64()?;
    let f32_read = parcel.read_f32()?;
    let f64_read = parcel.read_f64()?;
    let str_read = parcel.read_string()?;
    let buf_read = parcel.read_buffer(BUF.len())?;

    let matched = token == DESCRIPTOR
        && i8_read == I8
        && i16_read == I16
        && i32_read == I32
        && i64_read == I64
        && f32_read == F32
        && f64_read == F64
        && str_read == STR
        && buf_read == BUF;

    Ok(format!(
        "parcel roundtrip token=\"{token}\" i8={i8_read} i16={i16_read} i32={i32_read} \
         i64={i64_read} f32={f32_read} f64={f64_read} str=\"{str_read}\" \
         buf={buf_read:02x?} buf_len={} match={matched}",
        buf_read.len()
    ))
}

// Check that the size, cursor and remaining-space getters agree with each
// other, and that rewinding the read cursor lets the same value be read twice.
#[napi]
pub fn test_parcel_cursor() -> String {
    let msg = match cursor() {
        Ok(msg) => msg,
        Err(e) => format!("parcel cursor err({e})"),
    };
    log(&msg);
    msg
}

fn cursor() -> ipc::Result<String> {
    const VALUE: i32 = 0x5a5a;

    let mut parcel = ipc::Parcel::new()?;
    let empty_size = parcel.data_size()?;
    let writable_empty = parcel.writable_bytes()?;

    parcel.write_i32(VALUE)?;
    let size = parcel.data_size()?;
    let write_pos = parcel.write_position()?;
    let readable = parcel.readable_bytes()?;
    let writable = parcel.writable_bytes()?;

    let first = parcel.read_i32()?;
    let read_pos = parcel.read_position()?;
    let readable_after = parcel.readable_bytes()?;

    parcel.rewind_read_position(0)?;
    let rewound_pos = parcel.read_position()?;
    let second = parcel.read_i32()?;

    // `writable_bytes` reports the unused part of the buffer the parcel has
    // allocated so far, not a fixed budget: an untouched parcel owns no buffer
    // yet, so `writable_empty` is 0, and the first write allocates a whole
    // chunk of which `writable` is the remainder. The invariant is therefore on
    // the capacity, which is the write cursor plus what is left after it.
    let capacity = write_pos + writable;

    // A single i32 occupies at least four bytes; the transport is free to pad.
    let checks = [
        ("empty_size", empty_size == 0),
        ("size", size >= 4),
        ("write_pos", write_pos == size),
        ("readable", readable == size),
        ("read_pos", read_pos == size),
        ("readable_after", readable_after == 0),
        ("rewound_pos", rewound_pos == 0),
        ("first", first == VALUE),
        ("second", second == VALUE),
        ("capacity", capacity >= size && capacity >= writable_empty),
    ];
    let failed = checks
        .iter()
        .filter(|(_, ok)| !ok)
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let matched = failed.is_empty();

    Ok(format!(
        "parcel cursor empty_size={empty_size} size={size} write_pos={write_pos} \
         readable={readable} read_pos_after_read={read_pos} readable_after={readable_after} \
         rewound_pos={rewound_pos} writable_empty={writable_empty} writable_after={writable} \
         capacity={capacity} first={first} second_after_rewind={second} match={matched} \
         failed=[{}]",
        failed.join(",")
    ))
}

// Append one parcel to another and read the concatenated payload back.
#[napi]
pub fn test_parcel_append() -> String {
    let msg = match append() {
        Ok(msg) => msg,
        Err(e) => format!("parcel append err({e})"),
    };
    log(&msg);
    msg
}

fn append() -> ipc::Result<String> {
    const HEAD: i32 = 11;
    const TAIL: i64 = 22;

    let mut head = ipc::Parcel::new()?;
    head.write_i32(HEAD)?;
    let head_size = head.data_size()?;

    let mut tail = ipc::Parcel::new()?;
    tail.write_i64(TAIL)?;
    let tail_size = tail.data_size()?;

    head.append(&tail)?;
    let total = head.data_size()?;
    let first = head.read_i32()?;
    let second = head.read_i64()?;

    let matched = total == head_size + tail_size && first == HEAD && second == TAIL;
    Ok(format!(
        "parcel append head_size={head_size} tail_size={tail_size} total={total} \
         first={first} second={second} match={matched}"
    ))
}

// Reading past the end of the payload must fail rather than hand back garbage.
#[napi]
pub fn test_parcel_underflow() -> String {
    let msg = match underflow() {
        Ok(msg) => msg,
        Err(e) => format!("parcel underflow err({e})"),
    };
    log(&msg);
    msg
}

fn underflow() -> ipc::Result<String> {
    let mut parcel = ipc::Parcel::new()?;
    parcel.write_i32(1)?;
    parcel.read_i32()?;

    let scalar = parcel.read_i64();
    let buffer = parcel.read_buffer(64);
    let empty = parcel.read_buffer(0);
    let readable = parcel.readable_bytes()?;

    let matched = scalar.is_err() && buffer.is_err() && empty.as_deref() == Ok(&[][..]);
    Ok(format!(
        "parcel underflow readable={readable} read_i64={} read_buffer_64={} \
         read_buffer_0={} match={matched}",
        outcome(&scalar),
        outcome(&buffer),
        outcome(&empty)
    ))
}

fn outcome<T>(result: &ipc::Result<T>) -> String {
    match result {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("err({e})"),
    }
}

// Report the process identity the skeleton exposes. Outside an IPC context the
// caller identity is this process's own, so the values are compared with the
// self token ID and with the running process ID.
#[napi]
pub fn test_skeleton_identity() -> String {
    let calling_pid = ipc::calling_pid();
    let calling_uid = ipc::calling_uid();
    let calling_token = ipc::calling_token_id();
    let first_token = ipc::first_token_id();
    let self_token = ipc::self_token_id();
    let local = ipc::is_local_calling();
    let handling = ipc::is_handling_transaction();
    let own_pid = u64::from(std::process::id());

    let matched = calling_pid == own_pid
        && calling_token == self_token
        && local
        && !handling
        && calling_uid > 0;
    let msg = format!(
        "skeleton identity calling_pid={calling_pid} own_pid={own_pid} calling_uid={calling_uid} \
         calling_token={calling_token} self_token={self_token} first_token={first_token} \
         is_local={local} is_handling_transaction={handling} match={matched}"
    );
    log(&msg);
    msg
}

// Reset the calling credentials of the current context and restore them, then
// check that the identity values are unchanged afterwards.
#[napi]
pub fn test_calling_identity() -> String {
    let before = ipc::calling_token_id();
    let msg = match ipc::reset_calling_identity() {
        Ok(identity) => {
            let saved = identity.to_string_lossy().into_owned();
            let during = ipc::calling_token_id();
            let restored = ipc::set_calling_identity(&identity);
            let after = ipc::calling_token_id();
            let matched = restored.is_ok() && after == before;
            format!(
                "calling identity saved_len={} saved=\"{saved}\" token_before={before} \
                 token_during={during} token_after={after} restore={} match={matched}",
                saved.len(),
                outcome(&restored)
            )
        }
        Err(e) => format!("calling identity reset err({e})"),
    };
    log(&msg);
    msg
}

// The worker thread count must be accepted inside 1..=32 and rejected outside
// it, which also tells us the native argument checking is reached.
#[napi]
pub fn test_max_work_thread_num() -> String {
    let default_num = ipc::set_max_work_thread_num(16);
    let one = ipc::set_max_work_thread_num(1);
    let zero = ipc::set_max_work_thread_num(0);
    let too_many = ipc::set_max_work_thread_num(64);
    let restored = ipc::set_max_work_thread_num(16);

    let matched = default_num.is_ok()
        && one.is_ok()
        && zero.is_err()
        && too_many.is_err()
        && restored.is_ok();
    let msg = format!(
        "max work thread num set_16={} set_1={} set_0={} set_64={} restore_16={} match={matched}",
        outcome(&default_num),
        outcome(&one),
        outcome(&zero),
        outcome(&too_many),
        outcome(&restored)
    );
    log(&msg);
    msg
}

// Create a stub in this process, hand it to a parcel and take a proxy for it
// back out, then read the interface descriptor the proxy reports. No request is
// sent: a synchronous request would have to be answered by an IPC worker thread
// while this thread waits for it, which cannot be done from a single napi call.
#[napi]
pub fn test_remote_stub_local() -> String {
    let msg = match remote_stub_local() {
        Ok(msg) => msg,
        Err(e) => format!("remote stub err({e})"),
    };
    log(&msg);
    msg
}

fn remote_stub_local() -> ipc::Result<String> {
    let handled = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
    let seen = handled.clone();
    let stub = ipc::RemoteStub::new(DESCRIPTOR, move |code, data, reply| {
        seen.store(code, std::sync::atomic::Ordering::SeqCst);
        let message = data.read_string()?;
        reply.write_string(&message)
    })?;

    let mut parcel = ipc::Parcel::new()?;
    parcel.write_remote_stub(&stub)?;
    let written = parcel.data_size()?;

    let proxy = parcel.read_remote_proxy();
    let detail = match proxy {
        Ok(mut proxy) => {
            let descriptor = proxy.interface_descriptor();
            let dead = proxy.is_remote_dead();
            let matched = descriptor.as_deref() == Ok(DESCRIPTOR) && !dead;
            format!(
                "descriptor={} is_remote_dead={dead} match={matched}",
                match &descriptor {
                    Ok(value) => format!("\"{value}\""),
                    Err(e) => format!("err({e})"),
                }
            )
        }
        Err(e) => format!("read_remote_proxy=err({e})"),
    };

    let requests = handled.load(std::sync::atomic::Ordering::SeqCst);
    Ok(format!(
        "remote stub created=true written_bytes={written} {detail} handler_invocations={requests}"
    ))
}

// Exercise the error mapping: a code inside the user range is kept, one outside
// it is replaced, and every mapped code has a description.
#[napi]
pub fn test_error_codes() -> String {
    let user = ipc::IpcError::user(1909001);
    let out_of_range = ipc::IpcError::user(1);
    let dead = ipc::IpcError::Native(ipc::sys::OH_IPC_ErrorCode_OH_IPC_DEAD_REMOTE_OBJECT as i32);

    let matched = user.code() == Some(1909001)
        && out_of_range.code()
            == Some(ipc::sys::OH_IPC_ErrorCode_OH_IPC_INVALID_USER_ERROR_CODE as i32)
        && dead.is_dead_remote_object()
        && !user.is_dead_remote_object();
    let msg = format!(
        "error codes user=\"{user}\" out_of_range=\"{out_of_range}\" dead=\"{dead}\" \
         success=\"{}\" match={matched}",
        ipc::describe(ipc::sys::OH_IPC_ErrorCode_OH_IPC_SUCCESS as i32)
    );
    log(&msg);
    msg
}
