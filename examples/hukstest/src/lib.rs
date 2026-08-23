use napi_derive_ohos::napi;
use ohos_huks_binding as huks;

use huks::{
    describe, HuksAlias, HuksCipherMode, HuksKeyAlg, HuksKeyDigest, HuksKeyPadding, HuksKeyPurpose,
    HuksTag, HuksTagType, ParamSet, Result,
};

const TAG: &str = "HUKS_TEST";

// Fixed aliases plus a delete-before-generate step: every entry point leaves the
// keystore in the state it found it, so pressing a button twice yields the same
// result. Unique per-run aliases would instead accumulate keys that nothing ever
// cleans up.
const AES_ALIAS: &[u8] = b"rust_hukstest_aes";
const ECC_ALIAS: &[u8] = b"rust_hukstest_ecc";
const NUL_ALIAS: &[u8] = b"rust_hukstest\0nul";
const ABSENT_ALIAS: &[u8] = b"rust_hukstest_absent";
const ENCODING_ALIAS: &[u8] = b"rust_hukstest_encoding";

// Constants transcribed from native_huks_type.h, so what a parameter is supposed
// to carry is stated by the header rather than by the crate under test.
const HDR_ALG_ECC: u32 = 2;
const HDR_ALG_AES: u32 = 20;
const HDR_DIGEST_SHA256: u32 = 12;
const HDR_MODE_CBC: u32 = 2;
const HDR_PADDING_NONE: u32 = 0;

const IV: [u8; 16] = *b"hukstest_iv_0123";
// 32 bytes: a multiple of the AES block size, so CBC works with padding NONE.
const PLAINTEXT: &[u8] = b"ohos huks binding roundtrip 0123";

// Well above the per-process concurrent session limit, so a leaked slot shows up.
const SESSION_ROUNDS: usize = 20;

fn emit(msg: String) -> String {
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// Every parameter set below is spelled with the typed setters
// (`algorithm` / `digest` / `padding` / `block_mode`), so each device run
// exercises the enum encoding the crate produces rather than working around it.
// The header constants above are kept as the independent yardstick that
// `test_tag_type_validation` measures that encoding against.
fn aes_gen_params() -> Result<ParamSet> {
    ParamSet::builder()
        .algorithm(HuksKeyAlg::Aes)
        .purposes(&[HuksKeyPurpose::Encrypt, HuksKeyPurpose::Decrypt])
        .key_size(256)
        .padding(HuksKeyPadding::None)
        .block_mode(HuksCipherMode::Cbc)
        .build()
}

fn aes_op_params(purpose: HuksKeyPurpose) -> Result<ParamSet> {
    ParamSet::builder()
        .algorithm(HuksKeyAlg::Aes)
        .purposes(&[purpose])
        .key_size(256)
        .padding(HuksKeyPadding::None)
        .block_mode(HuksCipherMode::Cbc)
        .add(HuksTag::Iv, &IV[..])
        .build()
}

fn ecc_gen_params() -> Result<ParamSet> {
    ParamSet::builder()
        .algorithm(HuksKeyAlg::Ecc)
        .purposes(&[HuksKeyPurpose::Sign, HuksKeyPurpose::Verify])
        .key_size(256)
        .digest(HuksKeyDigest::Sha256)
        .build()
}

fn ecc_op_params(purpose: HuksKeyPurpose) -> Result<ParamSet> {
    ParamSet::builder()
        .algorithm(HuksKeyAlg::Ecc)
        .purposes(&[purpose])
        .key_size(256)
        .digest(HuksKeyDigest::Sha256)
        .build()
}

/// Delete any key left over from an earlier run, then generate a fresh one.
fn ensure_key(alias: HuksAlias<'_>, params: &ParamSet) -> Result<()> {
    if alias.exists()? {
        alias.delete()?;
    }
    alias.generate(params)
}

fn aes_encrypt(alias: HuksAlias<'_>, plain: &[u8]) -> Result<Vec<u8>> {
    let params = aes_op_params(HuksKeyPurpose::Encrypt)?;
    let mut session = alias.init_session(&params)?;
    let split = plain.len() / 2;
    let mut out = session.update(&params, &plain[..split])?;
    out.extend_from_slice(&session.finish(&params, &plain[split..])?);
    Ok(out)
}

fn aes_decrypt(alias: HuksAlias<'_>, cipher: &[u8]) -> Result<Vec<u8>> {
    let params = aes_op_params(HuksKeyPurpose::Decrypt)?;
    let mut session = alias.init_session(&params)?;
    let split = cipher.len() / 2;
    let mut out = session.update(&params, &cipher[..split])?;
    out.extend_from_slice(&session.finish(&params, &cipher[split..])?);
    Ok(out)
}

// --- 1. key life cycle -------------------------------------------------------

fn run_key_lifecycle() -> Result<String> {
    let alias = HuksAlias::new(ECC_ALIAS)?;
    if alias.exists()? {
        alias.delete()?;
    }
    let before = alias.exists()?;
    alias.generate(&ecc_gen_params()?)?;
    let after_generate = alias.exists()?;
    let public_key = alias.export_public_key(&ParamSet::empty()?)?;
    alias.delete()?;
    let after_delete = alias.exists()?;
    let ok = !before && after_generate && !public_key.is_empty() && !after_delete;
    Ok(format!(
        "lifecycle ECC-256: exists_before={before} exists_after_generate={after_generate} \
         public_key_len={} exists_after_delete={after_delete} ok={ok}",
        public_key.len()
    ))
}

#[napi]
pub fn test_key_lifecycle() -> String {
    emit(match run_key_lifecycle() {
        Ok(s) => s,
        Err(e) => format!("lifecycle err({e}) ok=false"),
    })
}

// --- 2. three-stage session round trip ---------------------------------------

fn run_aes_roundtrip() -> Result<String> {
    let alias = HuksAlias::new(AES_ALIAS)?;
    ensure_key(alias, &aes_gen_params()?)?;
    let cipher = aes_encrypt(alias, PLAINTEXT);
    let result = cipher.and_then(|c| {
        let len = c.len();
        aes_decrypt(alias, &c).map(|p| (len, p))
    });
    let _ = alias.delete();
    let (cipher_len, plain) = result?;
    let matches = plain == PLAINTEXT;
    Ok(format!(
        "AES-256-CBC init/update/finish: plain_len={} cipher_len={cipher_len} decrypted_len={} \
         match={matches}",
        PLAINTEXT.len(),
        plain.len()
    ))
}

#[napi]
pub fn test_aes_roundtrip() -> String {
    emit(match run_aes_roundtrip() {
        Ok(s) => s,
        Err(e) => format!("aes roundtrip err({e}) match=false"),
    })
}

/// Sign in three stages and report, alongside the signature, how many bytes the
/// update stage claimed to produce.
///
/// A sign session produces nothing before finish: the update stages only absorb
/// the message, and the signature is the finish output on its own. So the second
/// element must be 0. Anything else means `Session::update` is reporting the
/// output capacity it allocated as if it were produced data, which would prefix
/// the signature with a block of zero bytes for any caller that appends the
/// update output — the natural reading of an API returning `Vec<u8>`.
fn ecc_sign(alias: HuksAlias<'_>) -> Result<(Vec<u8>, usize)> {
    let params = ecc_op_params(HuksKeyPurpose::Sign)?;
    let mut session = alias.init_session(&params)?;
    let split = PLAINTEXT.len() / 2;
    let update_out = session.update(&params, &PLAINTEXT[..split])?;
    let signature = session.finish(&params, &PLAINTEXT[split..])?;
    Ok((signature, update_out.len()))
}

/// Verify `signature` over the first `message_len` bytes of the plaintext. The
/// message goes in through update; finish carries the signature, not more data.
fn ecc_verify(alias: HuksAlias<'_>, signature: &[u8], message_len: usize) -> Result<()> {
    let params = ecc_op_params(HuksKeyPurpose::Verify)?;
    let mut session = alias.init_session(&params)?;
    session.update(&params, &PLAINTEXT[..message_len])?;
    session.finish(&params, signature)?;
    Ok(())
}

fn state_of(result: Result<()>) -> String {
    match result {
        Ok(()) => "ok".to_string(),
        Err(e) => format!("err({e})"),
    }
}

fn run_ecc_sign_verify() -> Result<String> {
    let alias = HuksAlias::new(ECC_ALIAS)?;
    ensure_key(alias, &ecc_gen_params()?)?;

    let signed = ecc_sign(alias);
    // Whether the finish input is folded into the signed message is what
    // distinguishes the two candidates, so both are tried in one device run
    // rather than one per run.
    let verified = signed.as_ref().ok().map(|(sig, _)| {
        let full = state_of(ecc_verify(alias, sig, PLAINTEXT.len()));
        let partial = if full == "ok" {
            "not_tried".to_string()
        } else {
            state_of(ecc_verify(alias, sig, PLAINTEXT.len() / 2))
        };
        (full, partial)
    });

    let _ = alias.delete();
    let (signature, update_out_len) = signed?;
    let (verify_full, verify_first_half) =
        verified.unwrap_or_else(|| ("skipped".to_string(), "skipped".to_string()));

    let sign_verify_ok = verify_full == "ok" && !signature.is_empty();
    // A stage that produces no output must report none, so the signature is
    // exactly what finish returned.
    let update_len_ok = update_out_len == 0;
    let ok = sign_verify_ok && update_len_ok;
    Ok(format!(
        "ECC-256/SHA256 sign+verify: signature_len={} verify_full_message={verify_full} \
         verify_first_half_only={verify_first_half} sign_verify_ok={sign_verify_ok} \
         sign_update_out_len={update_out_len} expected=0{} ok={ok}",
        signature.len(),
        if update_len_ok {
            ""
        } else {
            " BINDING BUG [update reported output for a stage that produces none]"
        }
    ))
}

#[napi]
pub fn test_ecc_sign_verify() -> String {
    emit(match run_ecc_sign_verify() {
        Ok(s) => s,
        Err(e) => format!("ecc sign/verify err({e}) ok=false"),
    })
}

// --- 3. session slots are released by finish ---------------------------------

fn run_session_slot_reuse() -> Result<String> {
    let alias = HuksAlias::new(AES_ALIAS)?;
    ensure_key(alias, &aes_gen_params()?)?;

    let mut completed = 0usize;
    let mut identical = true;
    let mut reference: Option<Vec<u8>> = None;
    let mut failure = None;
    for round in 0..SESSION_ROUNDS {
        match aes_encrypt(alias, PLAINTEXT) {
            Ok(cipher) => {
                match &reference {
                    Some(first) => identical &= *first == cipher,
                    None => reference = Some(cipher),
                }
                completed += 1;
            }
            Err(e) => {
                failure = Some(format!("round {round} failed: {e}"));
                break;
            }
        }
    }
    let _ = alias.delete();

    let ok = completed == SESSION_ROUNDS && identical;
    let detail = failure.unwrap_or_else(|| "no failure".to_string());
    Ok(format!(
        "session slot reuse: completed={completed}/{SESSION_ROUNDS} ciphertext_identical={identical} \
         {detail} ok={ok}"
    ))
}

#[napi]
pub fn test_session_slot_reuse() -> String {
    emit(match run_session_slot_reuse() {
        Ok(s) => s,
        Err(e) => format!("session slot reuse err({e}) ok=false"),
    })
}

// --- 4. Drop aborts a session that was never finished -------------------------

fn run_session_drop_abort() -> Result<String> {
    let alias = HuksAlias::new(AES_ALIAS)?;
    ensure_key(alias, &aes_gen_params()?)?;
    let params = aes_op_params(HuksKeyPurpose::Encrypt)?;

    let mut opened = 0usize;
    let mut failure = None;
    for round in 0..SESSION_ROUNDS {
        match alias.init_session(&params) {
            Ok(session) => {
                drop(session);
                opened += 1;
            }
            Err(e) => {
                failure = Some(format!("init {round} failed: {e}"));
                break;
            }
        }
    }

    // If the dropped sessions had leaked their slots, this one would not start.
    let after = aes_encrypt(alias, PLAINTEXT).and_then(|c| aes_decrypt(alias, &c));
    let _ = alias.delete();

    let (after_state, matches) = match after {
        Ok(plain) => ("ok".to_string(), plain == PLAINTEXT),
        Err(e) => (format!("err({e})"), false),
    };
    let ok = opened == SESSION_ROUNDS && matches;
    let detail = failure.unwrap_or_else(|| "no failure".to_string());
    Ok(format!(
        "dropped sessions: opened_and_dropped={opened}/{SESSION_ROUNDS} \
         roundtrip_after={after_state} match={matches} {detail} ok={ok}"
    ))
}

#[napi]
pub fn test_session_drop_abort() -> String {
    emit(match run_session_drop_abort() {
        Ok(s) => s,
        Err(e) => format!("session drop err({e}) ok=false"),
    })
}

// --- 5. a value must match the type its tag declares, and must carry the number
//        the header defines -----------------------------------------------------

/// Every enum the builder accepts, as the two encodings that exist for it: the
/// `From` impl the derive builds out of the header constants, which is what the
/// builder must use, and the `as` cast, which is the declaration index and is
/// reported for the record only — the two coincide just where the header numbers
/// from zero without gaps, which is why nothing may depend on the cast.
fn enum_encoding() -> Vec<(String, bool)> {
    let cases: [(&str, u32, u32, u32); 5] = [
        (
            "ALG_AES",
            HuksKeyAlg::Aes as u32,
            u32::from(HuksKeyAlg::Aes),
            HDR_ALG_AES,
        ),
        (
            "ALG_ECC",
            HuksKeyAlg::Ecc as u32,
            u32::from(HuksKeyAlg::Ecc),
            HDR_ALG_ECC,
        ),
        (
            "DIGEST_SHA256",
            HuksKeyDigest::Sha256 as u32,
            u32::from(HuksKeyDigest::Sha256),
            HDR_DIGEST_SHA256,
        ),
        (
            "MODE_CBC",
            HuksCipherMode::Cbc as u32,
            u32::from(HuksCipherMode::Cbc),
            HDR_MODE_CBC,
        ),
        (
            "PADDING_NONE",
            HuksKeyPadding::None as u32,
            u32::from(HuksKeyPadding::None),
            HDR_PADDING_NONE,
        ),
    ];
    cases
        .iter()
        .map(|(name, cast, from_impl, want)| {
            (
                format!("{name}: from={from_impl} header={want} (as={cast})"),
                from_impl == want,
            )
        })
        .collect()
}

/// The same AES key, generated twice: once through the typed setters and once
/// from the header constants written out by hand. Both must be accepted — if
/// only the second one is, the numbers the typed setters emit are not the ones
/// the header defines and the device is rejecting the set they build.
fn typed_vs_raw_generate() -> (String, bool) {
    let alias = match HuksAlias::new(ENCODING_ALIAS) {
        Ok(a) => a,
        Err(e) => return (format!("alias err({e})"), false),
    };
    let attempt = |params: Result<ParamSet>| -> String {
        let outcome = params.and_then(|p| {
            if alias.exists()? {
                alias.delete()?;
            }
            alias.generate(&p)
        });
        let text = match outcome {
            Ok(()) => "Ok".to_string(),
            Err(e) => format!("Err(code={})", e.code()),
        };
        let _ = alias.delete();
        text
    };

    let typed = attempt(
        ParamSet::builder()
            .algorithm(HuksKeyAlg::Aes)
            .purposes(&[HuksKeyPurpose::Encrypt, HuksKeyPurpose::Decrypt])
            .key_size(256)
            .padding(HuksKeyPadding::None)
            .block_mode(HuksCipherMode::Cbc)
            .build(),
    );
    let raw = attempt(
        ParamSet::builder()
            .add(HuksTag::Algorithm, HDR_ALG_AES)
            .purposes(&[HuksKeyPurpose::Encrypt, HuksKeyPurpose::Decrypt])
            .key_size(256)
            .add(HuksTag::Padding, HDR_PADDING_NONE)
            .add(HuksTag::BlockMode, HDR_MODE_CBC)
            .build(),
    );
    let ok = typed == "Ok" && raw == "Ok";
    (format!("typed_setters={typed} header_values={raw}"), ok)
}

#[napi]
pub fn test_tag_type_validation() -> String {
    let type_checks = [
        (
            "Algorithm",
            HuksTag::Algorithm.value_type(),
            HuksTagType::Uint,
        ),
        ("KeySize", HuksTag::KeySize.value_type(), HuksTagType::Uint),
        ("Iv", HuksTag::Iv.value_type(), HuksTagType::Bytes),
        (
            "IsKeyAlias",
            HuksTag::IsKeyAlias.value_type(),
            HuksTagType::Bool,
        ),
    ];
    let types_ok = type_checks.iter().all(|(_, got, want)| got == want);
    let types: Vec<String> = type_checks
        .iter()
        .map(|(name, got, _)| format!("{name}={got:?}"))
        .collect();

    // Every mismatch must be rejected; the matching control case must build.
    let rejected = [
        (
            "KeySize<-bytes",
            ParamSet::builder()
                .add(HuksTag::KeySize, vec![0u8, 1])
                .build()
                .is_err(),
        ),
        (
            "Iv<-u32",
            ParamSet::builder()
                .add(HuksTag::Iv, 256u32)
                .build()
                .is_err(),
        ),
        (
            "IsKeyAlias<-u32",
            ParamSet::builder()
                .add(HuksTag::IsKeyAlias, 1u32)
                .build()
                .is_err(),
        ),
        (
            "Algorithm<-bool",
            ParamSet::builder()
                .add(HuksTag::Algorithm, true)
                .build()
                .is_err(),
        ),
        (
            "Iteration<-u64",
            ParamSet::builder()
                .add(HuksTag::Iteration, 1000u64)
                .build()
                .is_err(),
        ),
    ];
    let control_ok = ParamSet::builder()
        .add(HuksTag::KeySize, 256u32)
        .build()
        .is_ok();

    let all_rejected = rejected.iter().all(|(_, r)| *r);
    let details: Vec<String> = rejected
        .iter()
        .map(|(name, r)| format!("{name}=>{}", if *r { "Err" } else { "Ok" }))
        .collect();

    let encoding = enum_encoding();
    let encoding_ok = encoding.iter().all(|(_, ok)| *ok);
    let wrong: Vec<&str> = encoding
        .iter()
        .filter(|(_, ok)| !*ok)
        .map(|(text, _)| text.as_str())
        .collect();
    let (ab, generate_ok) = typed_vs_raw_generate();

    let ok = types_ok && all_rejected && control_ok && encoding_ok && generate_ok;
    emit(format!(
        "tag types [{}] mismatches [{}] control_KeySize<-u32=Ok:{control_ok} \
         enum_encoding_ok={encoding_ok}{} generate[{ab}] ok={ok}",
        types.join(" "),
        details.join(" "),
        if wrong.is_empty() {
            String::new()
        } else {
            format!(" BINDING BUG [{}]", wrong.join("; "))
        }
    ))
}

// --- 6. alias validation ------------------------------------------------------

fn describe_alias(bytes: &[u8]) -> String {
    match HuksAlias::new(bytes) {
        Ok(a) => format!("Ok(len={})", a.as_bytes().len()),
        Err(e) => format!("Err({})", e.code()),
    }
}

#[napi]
pub fn test_alias_validation() -> String {
    let empty = HuksAlias::new(b"").is_err();
    let max = vec![b'a'; 64];
    let too_long = vec![b'a'; 65];
    let max_ok = HuksAlias::new(&max).is_ok();
    let too_long_err = HuksAlias::new(&too_long).is_err();

    // An alias is a length-carrying blob, not a C string, so an embedded NUL is
    // legal. Prove it is not truncated: a key stored under it must be findable
    // and deletable under the exact same bytes.
    let nul_state = match run_nul_alias() {
        Ok(s) => s,
        Err(e) => format!("err({e})"),
    };

    let ok = empty && max_ok && too_long_err && nul_state == "roundtrip ok";
    emit(format!(
        "alias empty=>{} 64B=>{} 65B=>{} embedded_NUL={nul_state} ok={ok}",
        describe_alias(b""),
        describe_alias(&max),
        describe_alias(&too_long),
    ))
}

fn run_nul_alias() -> Result<String> {
    let alias = HuksAlias::new(NUL_ALIAS)?;
    ensure_key(alias, &aes_gen_params()?)?;
    let exists = alias.exists()?;
    alias.delete()?;
    let gone = !alias.exists()?;
    Ok(if exists && gone {
        "roundtrip ok".to_string()
    } else {
        format!("exists={exists} deleted={gone}")
    })
}

// --- 7. error code table ------------------------------------------------------

#[napi]
pub fn test_error_codes() -> String {
    // Expected strings come from the native header, not from the crate, so a
    // pattern that decayed into a catch-all binding would show up here.
    let cases: [(i32, &str); 9] = [
        (0, "success"),
        (201, "permission denied"),
        (401, "illegal argument"),
        (12000010, "session limit reached"),
        (12000011, "item does not exist"),
        (12000016, "device password unset"),
        (12000017, "key already exists"),
        (12000025, "limit exceeded"),
        (12009999, "unknown error"),
    ];
    let mismatched: Vec<String> = cases
        .iter()
        .filter(|(code, want)| describe(*code) != *want)
        .map(|(code, want)| format!("{code}: want '{want}' got '{}'", describe(*code)))
        .collect();

    // A real error from the device, to confirm codes survive the FFI boundary.
    let live = match HuksAlias::new(ABSENT_ALIAS).and_then(|a| a.delete()) {
        Ok(()) => "delete of absent key unexpectedly succeeded".to_string(),
        Err(e) => format!("code={} describe='{}'", e.code(), describe(e.code())),
    };
    let live_ok = live == format!("code=12000011 describe='{}'", describe(12000011));

    let ok = mismatched.is_empty() && live_ok;
    emit(format!(
        "describe(): {}/{} correct{} | delete absent key: {live} ok={ok}",
        cases.len() - mismatched.len(),
        cases.len(),
        if mismatched.is_empty() {
            String::new()
        } else {
            format!(" [{}]", mismatched.join("; "))
        }
    ))
}

// --- 8. regenerating over an existing alias -----------------------------------

fn run_generate_twice() -> Result<String> {
    let alias = HuksAlias::new(AES_ALIAS)?;
    let params = aes_gen_params()?;
    ensure_key(alias, &params)?;
    let second = match alias.generate(&params) {
        Ok(()) => "Ok".to_string(),
        Err(e) => format!("Err(code={})", e.code()),
    };
    let usable = aes_encrypt(alias, PLAINTEXT).and_then(|c| aes_decrypt(alias, &c));
    let _ = alias.delete();
    let matches = matches!(&usable, Ok(p) if p == PLAINTEXT);
    let state = match usable {
        Ok(_) => "ok".to_string(),
        Err(e) => format!("err({e})"),
    };
    Ok(format!(
        "generate over existing alias: second_generate={second} key_still_usable={state} \
         match={matches} ok={matches}"
    ))
}

#[napi]
pub fn test_generate_twice() -> String {
    emit(match run_generate_twice() {
        Ok(s) => s,
        Err(e) => format!("generate twice err({e}) ok=false"),
    })
}
