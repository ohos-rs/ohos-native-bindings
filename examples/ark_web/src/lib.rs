use napi_derive_ohos::napi;
use ohos_hilog_binding::hilog_info;
use ohos_web_binding::Web;

#[napi]
pub fn init(web_tag: String) {
    let web = Web::new(web_tag).unwrap();
    let _ = web
        .register_js_api("test", "test", |web_tag, data| {
            hilog_info!("ark_web register_js_api: {}", web_tag);
            hilog_info!("ark_web register_js_api: {:?}", data);
        })
        .map_err(|e| {
            hilog_info!("ark_web register_js_api error: {}", e);
        });
}
