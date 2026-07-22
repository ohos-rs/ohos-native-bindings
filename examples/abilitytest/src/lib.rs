use napi_derive_ohos::napi;
use ohos_ability_base_binding::{AbilityBaseError, Element, Want};
use ohos_child_process_binding as child_process;
use ohos_child_process_binding::{
    ChildProcessArgs, ChildProcessConfigs, ChildProcessError, IsolationMode,
};

const TAG: &str = "ABILITYTEST";

fn log(message: &str) {
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, message);
}

const BUNDLE: &str = "com.example.abilitytest";

// Write every kind of parameter the want exposes and read it back, so that the
// safe layer is checked against the value that went in rather than against the
// error code alone.
#[napi]
pub fn test_want_roundtrip() -> String {
    let message = want_roundtrip();
    log(&message);
    message
}

fn want_roundtrip() -> String {
    let first = Element::new(BUNDLE, "entry", "EntryAbility");
    let second = Element::new(BUNDLE, "entry", "SecondAbility");

    let mut want = match Want::new(&first) {
        Ok(want) => want,
        Err(error) => return format!("want roundtrip create=err({error}) match=false"),
    };

    let mut report = Vec::new();
    let mut matched = true;

    matched &= step(&mut report, "element_created", want.element(), |value| {
        (format!("\"{}\"", value.ability_name), *value == first)
    });
    matched &= step(&mut report, "set_element", want.set_element(&second), done);
    matched &= step(&mut report, "element_updated", want.element(), |value| {
        (format!("\"{}\"", value.ability_name), *value == second)
    });

    matched &= step(
        &mut report,
        "set_str",
        want.set_char_param("str", "x"),
        done,
    );
    matched &= step(&mut report, "str", want.char_param("str"), |value| {
        (format!("\"{value}\""), value == "x")
    });

    matched &= step(
        &mut report,
        "set_i32",
        want.set_int32_param("i32", 42),
        done,
    );
    matched &= step(&mut report, "i32", want.int32_param("i32"), |value| {
        (value.to_string(), *value == 42)
    });

    matched &= step(
        &mut report,
        "set_bool",
        want.set_bool_param("bool", true),
        done,
    );
    matched &= step(&mut report, "bool", want.bool_param("bool"), |value| {
        (value.to_string(), *value)
    });

    matched &= step(
        &mut report,
        "set_f64",
        want.set_double_param("f64", 1.5),
        done,
    );
    matched &= step(&mut report, "f64", want.double_param("f64"), |value| {
        (value.to_string(), (*value - 1.5).abs() < f64::EPSILON)
    });

    const URI: &str = "https://example.com/a";
    matched &= step(&mut report, "set_uri", want.set_uri(URI), done);
    matched &= step(&mut report, "uri", want.uri(), |value| {
        (format!("\"{value}\""), value == URI)
    });

    format!("want roundtrip {} match={matched}", report.join(" "))
}

// Record one step: what it returned and whether that is the value that went in.
// Every step is reported and none of them ends the case, so a failure names the
// call it came from instead of hiding the ones behind it.
fn step<T>(
    report: &mut Vec<String>,
    name: &str,
    result: Result<T, AbilityBaseError>,
    judge: impl FnOnce(&T) -> (String, bool),
) -> bool {
    let (text, ok) = match &result {
        Ok(value) => judge(value),
        Err(error) => (format!("err({error})"), false),
    };
    report.push(format!("{name}={text}{}", if ok { "" } else { "[BAD]" }));
    ok
}

// Judgement for a step that only has to succeed, such as a setter.
fn done(_: &()) -> (String, bool) {
    ("ok".to_string(), true)
}

// The buffer handling is the part of the safe layer that has no counterpart in
// the C API, so it gets its own case: a value of five bytes needs six bytes of
// capacity, and every capacity below that must fail instead of returning a
// truncated string.
#[napi]
pub fn test_want_small_buffer() -> String {
    let message = match want_small_buffer() {
        Ok(message) => message,
        Err(error) => format!("want small buffer err({error}) match=false"),
    };
    log(&message);
    message
}

fn want_small_buffer() -> Result<String, AbilityBaseError> {
    let mut want = Want::new(&Element::new(BUNDLE, "entry", "EntryAbility"))?;
    want.set_char_param("greeting", "hello")?;

    let zero = want.char_param_with_capacity("greeting", 0);
    let one = want.char_param_with_capacity("greeting", 1);
    let short = want.char_param_with_capacity("greeting", 5);
    let exact = want.char_param_with_capacity("greeting", 6);

    // A capacity of zero is rejected by the wrapper itself, before the runtime
    // is called at all, so it is the one outcome that is fully specified.
    let zero_is_buffer_too_small = matches!(&zero, Err(AbilityBaseError::BufferTooSmall(0)));
    let no_truncation = one.is_err() && short.is_err();
    let exact_ok = matches!(exact.as_deref(), Ok("hello"));
    let matched = zero_is_buffer_too_small && no_truncation && exact_ok;

    Ok(format!(
        "want small buffer value=\"hello\" cap0={} cap1={} cap5={} cap6={} no_truncation={no_truncation} match={matched}",
        outcome(&zero),
        outcome(&one),
        outcome(&short),
        outcome(&exact)
    ))
}

fn outcome(result: &Result<String, AbilityBaseError>) -> String {
    match result {
        Ok(value) => format!("ok(\"{value}\")"),
        Err(error) => format!("err({error:?})"),
    }
}

// Create, configure and drop a configs object. Nothing is started, so this
// only exercises the owned wrapper and its RAII destruction.
#[napi]
pub fn test_child_process_configs() -> String {
    let message = match child_process_configs() {
        Ok(message) => message,
        Err(error) => format!("child configs err({error}) match=false"),
    };
    log(&message);
    message
}

fn child_process_configs() -> Result<String, ChildProcessError> {
    let mut configs = ChildProcessConfigs::new()?;
    configs.set_isolation_mode(IsolationMode::Normal)?;
    configs.set_process_name("worker")?;
    configs.set_isolation_mode(IsolationMode::Isolated)?;

    // A NUL byte is caught by the wrapper, never handed to the C API.
    let nul_guard = matches!(
        configs.set_process_name("bad\0name"),
        Err(ChildProcessError::InteriorNul)
    );

    // A name the runtime documents as invalid: reported as it comes back.
    let invalid_name = match configs.set_process_name("bad name!") {
        Ok(()) => "accepted".to_string(),
        Err(error) => format!("{error}"),
    };

    drop(configs);

    let builder = ChildProcessConfigs::new()?
        .with_isolation_mode(IsolationMode::Isolated)?
        .with_process_name("worker2")?;
    drop(builder);

    Ok(format!(
        "child configs create=ok isolation=ok name=ok nul_guard={nul_guard} invalid_name=\"{invalid_name}\" builder=ok drop=ok match={nul_guard}"
    ))
}

// Build the argument set locally and read it back. No native call is involved,
// only the owned representation the start functions would consume.
#[napi]
pub fn test_child_process_args() -> String {
    let message = match child_process_args() {
        Ok(message) => message,
        Err(error) => format!("child args err({error}) match=false"),
    };
    log(&message);
    message
}

fn child_process_args() -> Result<String, ChildProcessError> {
    let args = ChildProcessArgs::new()
        .with_entry_params("--mode=worker")?
        .with_fd("log", 3)?
        .with_fd("data", 4)?;

    let entry_params = args.entry_params().unwrap_or_default().into_owned();
    let fds: Vec<String> = args
        .fds()
        .map(|(name, fd)| format!("{name}={fd}"))
        .collect();
    let nul_guard = matches!(
        ChildProcessArgs::new().with_entry_params("bad\0params"),
        Err(ChildProcessError::InteriorNul)
    );

    let matched = entry_params == "--mode=worker"
        && args.fd_count() == 2
        && fds == ["log=3", "data=4"]
        && nul_guard;

    Ok(format!(
        "child args entry_params=\"{entry_params}\" fd_count={} fds=[{}] nul_guard={nul_guard} match={matched}",
        args.fd_count(),
        fds.join(",")
    ))
}

// Called from the main process, so there are no child process arguments to
// read. Whatever the runtime answers is reported as it is.
#[napi]
pub fn test_current_child_process_args() -> String {
    let message = match child_process::current_child_process_args() {
        Some(args) => format!(
            "current child args Some(entry_params=\"{}\" fd_count={}) match=true",
            args.entry_params().unwrap_or_default(),
            args.fd_count()
        ),
        None => "current child args None (main process, no child arguments) match=true".to_string(),
    };
    log(&message);
    message
}
