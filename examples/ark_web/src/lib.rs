use std::sync::LazyLock;

use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_hilog_binding::hilog_info;
use ohos_web_binding::{Web, WebProxyBuilder};

static WEB_PROXY: LazyLock<std::sync::Mutex<Vec<ohos_web_binding::WebProxy>>> =
    LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

#[napi]
pub fn init(web_tag: String) {
    let _proxy = WebProxyBuilder::new(web_tag, "ipc".to_string())
        .add_method("postMessage", |_web_tag, args: Vec<String>| {
            hilog_info!("postMessage called with args: {:?}", args);
        })
        .build()
        .unwrap();
}

/// Register the JS proxy and keep it alive so the injected `ipc` object
/// survives page refreshes.
#[napi]
pub fn init_and_keep(web_tag: String) -> Result<()> {
    let proxy = WebProxyBuilder::new(web_tag, "ipc".to_string())
        .add_method("postMessage", |_web_tag, args: Vec<String>| {
            hilog_info!("postMessage called with args: {:?}", args);
        })
        .add_method("ping", |_web_tag, _args: Vec<String>| {
            hilog_info!("ping called");
        })
        .build()
        .map_err(|e| Error::from_reason(e.to_string()))?;
    WEB_PROXY.lock().unwrap().push(proxy);
    Ok(())
}

/// Re-register the proxy (if needed) and refresh the page so the injected
/// object becomes visible to the freshly loaded document.
#[napi]
pub fn refresh_page(web_tag: String) -> Result<()> {
    let guard = WEB_PROXY.lock().unwrap();
    if let Some(proxy) = guard.last() {
        proxy
            .refresh()
            .map_err(|e| Error::from_reason(e.to_string()))?;
        hilog_info!("web page refreshed for tag {web_tag}");
    } else {
        return Err(Error::from_reason(
            "no proxy registered — call initAndKeep first",
        ));
    }
    Ok(())
}

/// Register web lifecycle callbacks (controller attach, page begin/end,
/// destroy) for the given tag.
#[napi]
pub fn watch_lifecycle(web_tag: String) -> Result<()> {
    let web = Web::new(web_tag.clone());
    let attach_tag = web_tag.clone();
    web.on_controller_attach(move || {
        hilog_info!("web on_controller_attach: {}", attach_tag);
    })
    .map_err(|e| Error::from_reason(e.to_string()))?;

    let web2 = Web::new(web_tag.clone());
    web2
        .on_page_begin(move || {
            hilog_info!("web on_page_begin");
        })
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let web3 = Web::new(web_tag.clone());
    web3
        .on_page_end(move || {
            hilog_info!("web on_page_end");
        })
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let web4 = Web::new(web_tag);
    web4
        .on_destroy(move || {
            hilog_info!("web on_destroy");
        })
        .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(())
}

/// Register a custom `custom://` scheme handler that serves a small HTML
/// document without any network access.
#[napi]
pub fn register_custom_protocol(web_tag: String) -> Result<bool> {
    let web = Web::new(web_tag);
    let handler = ohos_web_binding::CustomProtocolHandler::new();
    handler.on_request_start(|request, handle| {
        hilog_info!("custom protocol request: {}", request.url());
        handle.receive_data("<html><body><h1>Hello from custom protocol (rust)</h1></body></html>");
        true
    });
    handler.on_request_stop(|request| {
        hilog_info!("custom protocol request stopped: {}", request.url());
    });
    web.custom_protocol("custom", handler)
        .map_err(|e| Error::from_reason(e.to_string()))
}
