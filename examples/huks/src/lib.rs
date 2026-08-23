use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_huks_binding::{
    HuksAlias, HuksKeyAlg, HuksKeyDigest, HuksKeyPadding, HuksKeyPurpose, ParamSet,
};

fn to_err(e: ohos_huks_binding::HuksError) -> Error {
    Error::from_reason(e.to_string())
}

fn hmac_params() -> Result<ParamSet> {
    ParamSet::builder()
        .algorithm(HuksKeyAlg::Hmac)
        .purposes(&[HuksKeyPurpose::Mac])
        .key_size(256)
        .digest(HuksKeyDigest::Sha256)
        .build()
        .map_err(to_err)
}

fn rsa_params() -> Result<ParamSet> {
    ParamSet::builder()
        .algorithm(HuksKeyAlg::Rsa)
        .purposes(&[HuksKeyPurpose::Sign, HuksKeyPurpose::Verify])
        .key_size(2048)
        .digest(HuksKeyDigest::Sha256)
        .padding(HuksKeyPadding::Pss)
        .build()
        .map_err(to_err)
}

fn delete_if_exists(alias: &str) -> Result<()> {
    let alias = HuksAlias::new(alias.as_bytes()).map_err(to_err)?;
    if alias.exists().map_err(to_err)? {
        alias.delete().map_err(to_err)?;
    }
    Ok(())
}

#[napi]
pub fn hmac_mac(data: String) -> Result<String> {
    const ALIAS: &str = "ohos-rs-demo-hmac";
    delete_if_exists(ALIAS)?;
    let params = hmac_params()?;
    let alias = HuksAlias::new(ALIAS.as_bytes()).map_err(to_err)?;
    alias.generate(&params).map_err(to_err)?;
    let exists = alias.exists().map_err(to_err)?;
    let session = alias.init_session(&params).map_err(to_err)?;
    let mac = session
        .finish(&ParamSet::empty().map_err(to_err)?, data.as_bytes())
        .map_err(to_err)?;
    alias.delete().map_err(to_err)?;
    Ok(format!("exists={exists} mac_len={}", mac.len()))
}

#[napi]
pub fn rsa_generate_export() -> Result<String> {
    const ALIAS: &str = "ohos-rs-demo-rsa";
    delete_if_exists(ALIAS)?;
    let params = rsa_params()?;
    let alias = HuksAlias::new(ALIAS.as_bytes()).map_err(to_err)?;
    alias.generate(&params).map_err(to_err)?;
    let exists = alias.exists().map_err(to_err)?;
    let public_key = alias.export_public_key(&params).map_err(to_err)?;
    alias.delete().map_err(to_err)?;
    Ok(format!(
        "exists={exists} public_key_len={}",
        public_key.len()
    ))
}

#[napi]
pub fn abort_session() -> Result<String> {
    const ALIAS: &str = "ohos-rs-demo-hmac-abort";
    delete_if_exists(ALIAS)?;
    let params = hmac_params()?;
    let alias = HuksAlias::new(ALIAS.as_bytes()).map_err(to_err)?;
    alias.generate(&params).map_err(to_err)?;
    let session = alias.init_session(&params).map_err(to_err)?;
    session
        .abort(&ParamSet::empty().map_err(to_err)?)
        .map_err(to_err)?;
    alias.delete().map_err(to_err)?;
    Ok("aborted".to_string())
}

#[napi]
pub fn smoke() -> Result<String> {
    Ok(format!(
        "{}\n{}\n{}",
        hmac_mac("hello-huks".to_string())?,
        rsa_generate_export()?,
        abort_session()?
    ))
}
