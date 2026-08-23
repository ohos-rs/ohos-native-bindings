use napi_derive_ohos::napi;
use ohos_common_event_binding as common_event;
use ohos_common_event_binding::{
    describe, event, CommonEventError, Parameters, PublishInfo, RcvData, ReceiveHandler,
    SubscribeInfo, Subscriber,
};
use std::sync::Mutex;
use std::time::Duration;

const TAG: &str = "COMMONEVENT_TEST";

/// The custom event this example publishes to itself.
const EVENT: &str = "com.uiskeleton.app.BINDTEST_EVENT";
/// A second custom name, used where nothing should be received.
const EVENT_PLAIN: &str = "com.uiskeleton.app.BINDTEST_PLAIN";

const KEY_INT: &str = "bindtest_int";
const KEY_LONG: &str = "bindtest_long";
const KEY_BOOL: &str = "bindtest_bool";
const KEY_CHAR: &str = "bindtest_char";
const KEY_DOUBLE: &str = "bindtest_double";
const KEY_INT_ARRAY: &str = "bindtest_int_array";

const SENT_INT: i32 = 42;
const SENT_LONG: i64 = 1_234_567;
const SENT_DOUBLE: f64 = 2.5;
const SENT_CHAR: u8 = b'K';
const SENT_INT_ARRAY: [i32; 3] = [7, 8, 9];
const SENT_CODE: i32 = 77;
const SENT_DATA: &str = "bindtest-payload";

/// Everything the handler copies out of one received event.
#[derive(Debug, Default)]
struct Record {
    event: String,
    code: i32,
    data: String,
    bundle_name: String,
    has_parameters: bool,
    param_i32: i32,
    param_i64: i64,
    param_bool: bool,
    param_char: u8,
    param_double: f64,
    param_int_array: Vec<i32>,
    has_key_int: bool,
}

/// Filled by the receive callback, drained by the napi functions.
static RECEIVED: Mutex<Vec<Record>> = Mutex::new(Vec::new());

/// Records every event it receives. Never panics: the callback runs on a
/// thread owned by the common event service.
struct Recorder;

impl ReceiveHandler for Recorder {
    fn on_receive(data: &RcvData<'_>) {
        let mut record = Record {
            event: data.event().unwrap_or_default().to_owned(),
            code: data.code(),
            data: data.data().unwrap_or_default().to_owned(),
            bundle_name: data.bundle_name().unwrap_or_default().to_owned(),
            ..Record::default()
        };
        if let Some(parameters) = data.parameters() {
            record.has_parameters = true;
            record.param_i32 = parameters.int(KEY_INT, -1).unwrap_or(-1);
            record.param_i64 = parameters.long(KEY_LONG, -1).unwrap_or(-1);
            record.param_bool = parameters.bool(KEY_BOOL, false).unwrap_or(false);
            record.param_char = parameters.char(KEY_CHAR, 0).unwrap_or(0);
            record.param_double = parameters.double(KEY_DOUBLE, -1.0).unwrap_or(-1.0);
            record.param_int_array = parameters.int_array(KEY_INT_ARRAY).unwrap_or_default();
            record.has_key_int = parameters.has_key(KEY_INT).unwrap_or(false);
        }
        if let Ok(mut received) = RECEIVED.lock() {
            received.push(record);
        }
    }
}

fn clear_received() {
    if let Ok(mut received) = RECEIVED.lock() {
        received.clear();
    }
}

/// Wait for an event named `name`, polling in short slices so the UI thread is
/// never blocked for long. Returns `None` when nothing arrived in time.
fn wait_for(name: &str, slices: u32, slice: Duration) -> Option<Record> {
    for _ in 0..slices {
        if let Ok(mut received) = RECEIVED.lock() {
            if let Some(index) = received.iter().position(|record| record.event == name) {
                return Some(received.remove(index));
            }
        }
        std::thread::sleep(slice);
    }
    None
}

/// Build the payload published with the event.
fn sent_parameters() -> common_event::Result<Parameters> {
    let mut parameters = Parameters::new()?;
    parameters.set_int(KEY_INT, SENT_INT)?;
    parameters.set_long(KEY_LONG, SENT_LONG)?;
    parameters.set_bool(KEY_BOOL, true)?;
    parameters.set_char(KEY_CHAR, SENT_CHAR)?;
    parameters.set_double(KEY_DOUBLE, SENT_DOUBLE)?;
    parameters.set_int_array(KEY_INT_ARRAY, &SENT_INT_ARRAY)?;
    Ok(parameters)
}

/// Note the outcome of one step and report whether it succeeded.
fn record(name: &str, result: common_event::Result<()>, steps: &mut Vec<String>) -> bool {
    match result {
        Ok(()) => {
            steps.push(format!("{name}=ok"));
            true
        }
        Err(e) => {
            steps.push(format!("{name}=err({e})"));
            false
        }
    }
}

fn log(message: &str) {
    ohos_hilog_binding::hilog_info!("{TAG}: {message}");
}

/// Subscribe to the custom event, publish it with a full payload and report
/// what the handler received. `received=false` is a real outcome: the system
/// may refuse to dispatch a custom event to the publishing application.
#[napi]
pub fn test_self_publish_receive() -> String {
    clear_received();

    let info = match SubscribeInfo::new(&[EVENT]) {
        Ok(info) => info,
        Err(e) => return finish(format!("subscribe_info err({e}) match=false")),
    };
    let mut subscriber = match Subscriber::new::<Recorder>(&info) {
        Ok(subscriber) => subscriber,
        Err(e) => return finish(format!("subscriber_new err({e}) match=false")),
    };
    if let Err(e) = subscriber.subscribe() {
        return finish(format!("subscribe err({e}) match=false"));
    }

    let parameters = match sent_parameters() {
        Ok(parameters) => parameters,
        Err(e) => return finish(format!("parameters err({e}) match=false")),
    };
    let mut publish_info = match PublishInfo::new(false) {
        Ok(publish_info) => publish_info,
        Err(e) => return finish(format!("publish_info err({e}) match=false")),
    };
    if let Err(e) = publish_info.set_code(SENT_CODE) {
        return finish(format!("set_code err({e}) match=false"));
    }
    if let Err(e) = publish_info.set_data(SENT_DATA) {
        return finish(format!("set_data err({e}) match=false"));
    }
    if let Err(e) = publish_info.set_parameters(parameters) {
        return finish(format!("set_parameters err({e}) match=false"));
    }
    if let Err(e) = common_event::publish_with_info(EVENT, &publish_info) {
        return finish(format!("publish_with_info err({e}) match=false"));
    }

    // Ten 30 ms slices: 300 ms at worst, and it returns as soon as the event
    // shows up.
    let waited = wait_for(EVENT, 10, Duration::from_millis(30));
    let message = match waited {
        Some(record) => {
            let name_ok = record.event == EVENT;
            let payload_ok = record.has_parameters
                && record.param_i32 == SENT_INT
                && record.param_i64 == SENT_LONG
                && record.param_bool
                && record.param_char == SENT_CHAR
                && (record.param_double - SENT_DOUBLE).abs() < f64::EPSILON
                && record.param_int_array == SENT_INT_ARRAY
                && record.has_key_int;
            let matched = name_ok && record.code == SENT_CODE && record.data == SENT_DATA;
            format!(
                "received=true event={} code={} data={} bundle_name={} has_parameters={} \
                 param_i32={} param_i64={} param_bool={} param_char={} param_double={} \
                 param_int_array={:?} has_key_int={} payload_match={payload_ok} match={matched}",
                record.event,
                record.code,
                record.data,
                record.bundle_name,
                record.has_parameters,
                record.param_i32,
                record.param_i64,
                record.param_bool,
                record.param_char,
                record.param_double,
                record.param_int_array,
                record.has_key_int,
            )
        }
        None => format!("received=false event={EVENT} waited_ms=300 match=false"),
    };

    // Explicit unsubscribe, then the drop below runs the destroy path.
    let unsubscribed = match subscriber.unsubscribe() {
        Ok(()) => "ok".to_string(),
        Err(e) => format!("err({e})"),
    };
    drop(subscriber);
    drop(info);
    finish(format!("{message} unsubscribe={unsubscribed}"))
}

/// Publish the plain, payload-free form of the API.
#[napi]
pub fn test_publish_plain() -> String {
    clear_received();
    let message = match common_event::publish(EVENT_PLAIN) {
        Ok(()) => "publish=ok match=true".to_string(),
        Err(e) => format!("publish=err({e}) match=false"),
    };
    finish(message)
}

/// Exercise `PublishInfo` end to end: ordered flag, bundle name, permissions,
/// code, data and payload, then publish it.
#[napi]
pub fn test_publish_info_fields() -> String {
    let mut publish_info = match PublishInfo::new(true) {
        Ok(publish_info) => publish_info,
        Err(e) => return finish(format!("publish_info=err({e}) match=false")),
    };
    let mut steps: Vec<String> = Vec::new();
    let mut ok = true;

    ok &= record("set_code", publish_info.set_code(SENT_CODE), &mut steps);
    ok &= record("set_data", publish_info.set_data(SENT_DATA), &mut steps);
    ok &= record(
        "set_bundle_name",
        publish_info.set_bundle_name("com.uiskeleton.app"),
        &mut steps,
    );
    ok &= record(
        "set_permissions",
        publish_info.set_permissions(&["ohos.permission.INTERNET"]),
        &mut steps,
    );
    // Setting the payload twice has to release the first one without leaving
    // the native handle pointing at freed memory.
    for name in ["set_parameters", "replace_parameters"] {
        match sent_parameters() {
            Ok(parameters) => {
                ok &= record(name, publish_info.set_parameters(parameters), &mut steps);
            }
            Err(e) => {
                steps.push(format!("{name}=err({e})"));
                ok = false;
            }
        }
    }

    let published = match common_event::publish_with_info(EVENT_PLAIN, &publish_info) {
        Ok(()) => "ok".to_string(),
        Err(e) => {
            ok = false;
            format!("err({e})")
        }
    };
    drop(publish_info);
    finish(format!(
        "{} publish_with_info={published} match={ok}",
        steps.join(" ")
    ))
}

/// Write every scalar and array type into an owned payload and read it back
/// through the borrowed view.
#[napi]
pub fn test_parameters_roundtrip() -> String {
    let mut parameters = match Parameters::new() {
        Ok(parameters) => parameters,
        Err(e) => return finish(format!("parameters=err({e}) match=false")),
    };
    let writes = [
        parameters.set_int(KEY_INT, SENT_INT),
        parameters.set_long(KEY_LONG, SENT_LONG),
        parameters.set_bool(KEY_BOOL, true),
        parameters.set_char(KEY_CHAR, SENT_CHAR),
        parameters.set_double(KEY_DOUBLE, SENT_DOUBLE),
        parameters.set_int_array(KEY_INT_ARRAY, &SENT_INT_ARRAY),
        parameters.set_long_array("bindtest_long_array", &[1, 2]),
        parameters.set_bool_array("bindtest_bool_array", &[true, false]),
        parameters.set_char_array("bindtest_char_array", b"ab"),
        parameters.set_double_array("bindtest_double_array", &[0.5, 1.5]),
    ];
    let write_errors: Vec<String> = writes
        .iter()
        .filter_map(|result| result.as_ref().err().map(ToString::to_string))
        .collect();

    // The native layer stores a char array as a string and cuts it at the first
    // zero byte, so the safe layer rejects an interior one instead of storing a
    // truncated value.
    let interior_nul_rejected = matches!(
        parameters.set_char_array("bindtest_char_array_nul", b"a\0b"),
        Err(CommonEventError::Nul)
    );
    let empty_written = parameters
        .set_char_array("bindtest_char_array_empty", b"")
        .is_ok();

    let view = parameters.view();
    let int = view.int(KEY_INT, -1).unwrap_or(-1);
    let long = view.long(KEY_LONG, -1).unwrap_or(-1);
    let boolean = view.bool(KEY_BOOL, false).unwrap_or(false);
    let character = view.char(KEY_CHAR, 0).unwrap_or(0);
    let double = view.double(KEY_DOUBLE, -1.0).unwrap_or(-1.0);
    let int_array = view.int_array(KEY_INT_ARRAY).unwrap_or_default();
    let long_array = view.long_array("bindtest_long_array").unwrap_or_default();
    let bool_array = view.bool_array("bindtest_bool_array").unwrap_or_default();
    let char_array = view.char_array("bindtest_char_array").unwrap_or_default();
    let char_array_empty = view
        .char_array("bindtest_char_array_empty")
        .unwrap_or_else(|_| vec![0xff]);
    let double_array = view
        .double_array("bindtest_double_array")
        .unwrap_or_default();
    let has_key = view.has_key(KEY_INT).unwrap_or(false);
    let missing_key = view.has_key("bindtest_absent").unwrap_or(true);
    let missing_default = view.int("bindtest_absent", -7).unwrap_or(0);

    // Named per field, so a failure says which one drifted instead of only that
    // something did.
    let mut mismatched: Vec<&str> = Vec::new();
    if !write_errors.is_empty() {
        mismatched.push("write_errors");
    }
    if int != SENT_INT {
        mismatched.push("int");
    }
    if long != SENT_LONG {
        mismatched.push("long");
    }
    if !boolean {
        mismatched.push("bool");
    }
    if character != SENT_CHAR {
        mismatched.push("char");
    }
    if (double - SENT_DOUBLE).abs() >= f64::EPSILON {
        mismatched.push("double");
    }
    if int_array != SENT_INT_ARRAY {
        mismatched.push("int_array");
    }
    if long_array != [1, 2] {
        mismatched.push("long_array");
    }
    if bool_array != [true, false] {
        mismatched.push("bool_array");
    }
    if char_array != b"ab" {
        mismatched.push("char_array");
    }
    if !interior_nul_rejected {
        mismatched.push("char_array_nul");
    }
    if !empty_written || !char_array_empty.is_empty() {
        mismatched.push("char_array_empty");
    }
    if double_array != [0.5, 1.5] {
        mismatched.push("double_array");
    }
    if !has_key {
        mismatched.push("has_key");
    }
    if missing_key {
        mismatched.push("missing_key");
    }
    if missing_default != -7 {
        mismatched.push("missing_default");
    }
    let matched = mismatched.is_empty();

    let message = format!(
        "write_errors={write_errors:?} int={int} long={long} bool={boolean} char={character} \
         double={double} int_array={int_array:?} long_array={long_array:?} \
         bool_array={bool_array:?} char_array={char_array:?} \
         char_array_nul_rejected={interior_nul_rejected} \
         char_array_empty={char_array_empty:?} double_array={double_array:?} \
         has_key={has_key} missing_key={missing_key} missing_default={missing_default} \
         mismatched={mismatched:?} match={matched}"
    );
    drop(parameters);
    finish(message)
}

/// Create subscribe information the way a real subscriber does, including the
/// publisher filters, and check that an empty event list is rejected.
#[napi]
pub fn test_subscribe_info() -> String {
    let mut info = match SubscribeInfo::new(&[EVENT, event::SCREEN_ON]) {
        Ok(info) => info,
        Err(e) => return finish(format!("new=err({e}) match=false")),
    };
    let bundle = match info.set_publisher_bundle_name("com.uiskeleton.app") {
        Ok(()) => "ok".to_string(),
        Err(e) => format!("err({e})"),
    };
    let permission = match info.set_publisher_permission("ohos.permission.INTERNET") {
        Ok(()) => "ok".to_string(),
        Err(e) => format!("err({e})"),
    };
    let empty = SubscribeInfo::new(&[]);
    let empty_rejected = matches!(empty, Err(CommonEventError::InvalidParameter));
    let nul = SubscribeInfo::new(&["bad\0name"]);
    let nul_rejected = matches!(nul, Err(CommonEventError::Nul));
    drop(info);

    let matched = bundle == "ok" && permission == "ok" && empty_rejected && nul_rejected;
    finish(format!(
        "new=ok set_publisher_bundle_name={bundle} set_publisher_permission={permission} \
         empty_rejected={empty_rejected} nul_rejected={nul_rejected} match={matched}"
    ))
}

/// Create and destroy subscribers, with and without an active subscription, to
/// check that both the explicit and the RAII teardown paths hold.
#[napi]
pub fn test_subscriber_lifecycle() -> String {
    let info = match SubscribeInfo::new(&[EVENT_PLAIN]) {
        Ok(info) => info,
        Err(e) => return finish(format!("subscribe_info=err({e}) match=false")),
    };

    let mut created = 0_u32;
    for _ in 0..3 {
        match Subscriber::new::<Recorder>(&info) {
            Ok(subscriber) => {
                created += 1;
                drop(subscriber);
            }
            Err(e) => return finish(format!("create=err({e}) match=false")),
        }
    }

    // Subscribed and dropped without unsubscribing: the drop has to unsubscribe
    // before it destroys the handle.
    let dropped_while_subscribed = match Subscriber::new::<Recorder>(&info) {
        Ok(mut subscriber) => match subscriber.subscribe() {
            Ok(()) => {
                drop(subscriber);
                "ok".to_string()
            }
            Err(e) => format!("subscribe_err({e})"),
        },
        Err(e) => format!("create_err({e})"),
    };

    // Subscribed, unsubscribed explicitly, then dropped.
    let (unsubscribed, second_unsubscribe) = match Subscriber::new::<Recorder>(&info) {
        Ok(mut subscriber) => {
            let first = match subscriber
                .subscribe()
                .and_then(|()| subscriber.unsubscribe())
            {
                Ok(()) => "ok".to_string(),
                Err(e) => format!("err({e})"),
            };
            let second = match subscriber.unsubscribe() {
                Ok(()) => "ok".to_string(),
                Err(e) => format!("err({e})"),
            };
            drop(subscriber);
            (first, second)
        }
        Err(e) => (format!("create_err({e})"), "skipped".to_string()),
    };
    drop(info);

    let matched = created == 3 && dropped_while_subscribed == "ok" && unsubscribed == "ok";
    finish(format!(
        "created={created} dropped_while_subscribed={dropped_while_subscribed} \
         unsubscribe={unsubscribed} unsubscribe_again={second_unsubscribe} match={matched}"
    ))
}

/// Read the predefined system event names back out of the crate. They come from
/// the SDK headers, so an empty or malformed one means the constants are wrong.
#[napi]
pub fn test_event_names() -> String {
    let names = [
        ("SCREEN_ON", event::SCREEN_ON),
        ("SCREEN_OFF", event::SCREEN_OFF),
        ("BATTERY_CHANGED", event::BATTERY_CHANGED),
        ("POWER_CONNECTED", event::POWER_CONNECTED),
        ("SHUTDOWN", event::SHUTDOWN),
    ];
    let matched = names
        .iter()
        .all(|(_, value)| !value.is_empty() && !value.contains('\0'));
    let rendered: Vec<String> = names
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect();
    finish(format!("{} match={matched}", rendered.join(" ")))
}

/// Check the error mapping against the codes the native layer reports.
#[napi]
pub fn test_error_mapping() -> String {
    let cases = [
        CommonEventError::PermissionDenied,
        CommonEventError::InvalidParameter,
        CommonEventError::SendingLimitExceeded,
        CommonEventError::NotSystemService,
        CommonEventError::SendingRequestFailed,
        CommonEventError::InitUndone,
        CommonEventError::ObtainSystemParams,
        CommonEventError::SubscriberNumExceeded,
        CommonEventError::AllocMemoryFailed,
    ];
    let mut rendered = Vec::new();
    let mut matched = true;
    for case in &cases {
        match case.code() {
            Some(code) => {
                let text = describe(code);
                if text == "unknown error" {
                    matched = false;
                }
                rendered.push(format!("{code}={text}"));
            }
            None => {
                matched = false;
                rendered.push(format!("{case}=no_code"));
            }
        }
    }
    let no_code = CommonEventError::Nul.code().is_none();
    let success = describe(0);
    matched &= no_code && success == "success";
    finish(format!(
        "{} nul_has_no_code={no_code} describe_0={success} match={matched}",
        rendered.join(" ")
    ))
}

fn finish(message: String) -> String {
    log(&message);
    message
}
