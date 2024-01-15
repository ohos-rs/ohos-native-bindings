# image-binding

## Install

```shell
cargo add image_binding
```

## Usage

Rust source code:

```rust
use hilog_binding::hilog_info;
use image_binding::get_pixel_map_info;
use napi_derive_ohos::{js_function, module_exports};
use napi_ohos::bindgen_prelude::pre_init;
use napi_ohos::{module_init, CallContext, JsObject, JsUndefined, NapiRaw, Result};

#[js_function(1)]
pub fn get_pixel_info(ctx: CallContext) -> Result<JsUndefined> {
    let pixel_map = ctx.get::<JsObject>(0)?;
    let env = ctx.env.raw();
    unsafe {
        let value = pixel_map.raw();
        let res = get_pixel_map_info(env, value).unwrap();
        hilog_info!(format!("{:?}", res.width));
        Ok(ctx.env.get_undefined().unwrap())
    }
}

#[module_exports]
pub fn register_js(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("getPixelInfo", get_pixel_info)?;
    Ok(())
}

#[module_init]
fn init() {
    pre_init();
}
```

ArkTS source code:

```ts
import tsfn from 'libtest.so';
import image from '@ohos.multimedia.image';
import { BusinessError } from '@ohos.base';
import hilog from '@ohos.hilog';

@Entry
@Component
struct Tsfn {
  @State message: string = '';
  private ctx = getContext(this);
  test = async () => {
    try {
      // 获取resourceManager资源管理
      const resourceMgr = this.ctx.resourceManager;
      const fileData = await resourceMgr.getRawFileContent('test.png');
      // 获取图片的ArrayBuffer
      const buffer = fileData.buffer;

      const imageSource = image.createImageSource(buffer);

      let decodingOptions: image.DecodingOptions = {
        editable: true,
        desiredPixelFormat: 3,
      }

      const pixelMap = await imageSource.createPixelMap(decodingOptions);
      tsfn.getPixelInfo(pixelMap);
    } catch (e) {
      hilog.info(0x0000, 'testTag', `${(e as BusinessError).message}`);
    }
  }

  build() {
    Row() {
      Column({ space: 20 }) {
        Button('Run TSFN').onClick(() => {
          this.test();
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}
```