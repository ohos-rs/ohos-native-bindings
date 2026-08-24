import { router } from "@kit.ArkUI";

interface DemoEntry {
  name: string;
  page: string;
  note: string;
}

@Entry
@Component
struct Index {
  private demos: DemoEntry[] = [
    { name: "AbilityAccess", page: "pages/AbilityAccess", note: "checkSelfPermission" },
    { name: "ArkUI", page: "pages/ArkUI", note: "node tree / dialog / input / animation" },
    { name: "ArkUIInput", page: "pages/ArkUIInput", note: "ArkUI_UIInputEvent dump" },
    { name: "ArkWeb", page: "pages/ArkWeb", note: "web proxy postMessage" },
    { name: "Ashmem", page: "pages/Ashmem", note: "anonymous shared memory" },
    { name: "Asset", page: "pages/Asset", note: "keystore asset add/query/update/remove" },
    { name: "Bundle", page: "pages/Bundle", note: "app id / identifier / main element" },
    { name: "Camera", page: "pages/Camera", note: "preview + capture via XComponent" },
    { name: "Display", page: "pages/Display", note: "default display metrics" },
    { name: "DisplaySoloist", page: "pages/DisplaySoloist", note: "soloist frame callback" },
    { name: "Drawing", page: "pages/Drawing", note: "offscreen canvas / path / pen / brush" },
    { name: "FileShare", page: "pages/FileShare", note: "persist/activate URI policy" },
    { name: "FileUri", page: "pages/FileUri", note: "path <-> file URI" },
    { name: "Hilog", page: "pages/Hilog", note: "log / info redirect" },
    { name: "HUKS", page: "pages/Huks", note: "HMAC / RSA keystore" },
    { name: "IME", page: "pages/Ime", note: "input method attach/show/hide" },
    { name: "Image", page: "pages/Image", note: "NAPI PixelMap + packer" },
    { name: "ImageNative", page: "pages/ImageNative", note: "OH_PixelmapNative create/transform" },
    { name: "Init", page: "pages/Init", note: "canIUse SystemCapability" },
    { name: "JSVM", page: "pages/Jsvm", note: "napi value roundtrips in JSVM" },
    { name: "NativeBuffer", page: "pages/NativeBuffer", note: "alloc / map / write" },
    { name: "NativeWindow", page: "pages/NativeWindow", note: "request buffer + fill" },
    { name: "NetConnection", page: "pages/NetConnection", note: "default http proxy" },
    { name: "NetStack", page: "pages/NetStack", note: "http headers / websocket / ssl" },
    { name: "Pasteboard", page: "pages/Pasteboard", note: "clipboard read" },
    { name: "QoS", page: "pages/Qos", note: "thread qos set/get/reset" },
    { name: "Raw", page: "pages/Raw", note: "rawfile dir listing" },
    { name: "Sensor", page: "pages/Sensor", note: "accelerometer subscribe" },
    { name: "UDMF", page: "pages/Udmf", note: "plain text / html records" },
    { name: "Vibrator", page: "pages/Vibrator", note: "vibrate / custom effect" },
    { name: "Vsync", page: "pages/Vsync", note: "frame callbacks" },
    { name: "XComponent", page: "pages/XComp", note: "surface + GL + touch" },
  ];

  build() {
    Column() {
      Text("ohos-native-bindings demos")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 48, bottom: 16 })
      List({ space: 4 }) {
        ForEach(
          this.demos,
          (demo: DemoEntry) => {
            ListItem() {
              Column() {
                Text(demo.name)
                  .fontSize(18)
                  .fontWeight(FontWeight.Medium)
                Text(demo.note)
                  .fontSize(12)
                  .fontColor("#666666")
              }
              .width("100%")
              .alignItems(HorizontalAlign.Start)
              .padding(10)
              .onClick(() => {
                router.pushUrl({ url: demo.page });
              })
            };
          },
          (demo: DemoEntry) => demo.page,
        )
      }
      .width("92%")
      .layoutWeight(1)
    }
    .width("100%")
    .height("100%")
    .alignItems(HorizontalAlign.Center);
  }
}
