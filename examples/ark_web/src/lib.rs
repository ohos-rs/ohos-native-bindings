use napi_derive_ohos::napi;
use napi_ohos::threadsafe_function::ThreadsafeFunction;
use ohos_hilog_binding::hilog_info;
use ohos_web_binding::Web;

#[napi]
pub fn init(create_web: ThreadsafeFunction<(), String>) {
    create_web.call_with_return_value(
        Ok(()),
        napi_ohos::threadsafe_function::ThreadsafeFunctionCallMode::NonBlocking,
        |ret, _| {
            if let Ok(web_tag) = ret {
                let web = Web::new(web_tag).unwrap();
                let _ = web.on_controller_attach(|| {
                    hilog_info!("ark_web controller_attach");
                });

                let _ = web.on_page_begin(|| {
                    hilog_info!("ark_web on_page_begin");
                });
            }
            Ok(())
        },
    );
}
