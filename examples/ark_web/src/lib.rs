use napi_derive_ohos::napi;
use ohos_hilog_binding::hilog_info;
use ohos_web_binding::WebProxyBuilder;

#[napi]
pub fn init(web_tag: String) {
    let _proxy = WebProxyBuilder::new(web_tag, "ipc".to_string())
        .add_method("postMessage", |_web_tag, args: Vec<String>| {
            hilog_info!("postMessage called with args: {:?}", args);
        })
        .build()
        .unwrap();
}
