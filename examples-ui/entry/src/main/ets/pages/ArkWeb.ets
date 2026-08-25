import { router } from "@kit.ArkUI";
import { webview } from "@kit.ArkWeb";
import {
  init,
  initAndKeep,
  refreshPage,
  registerCustomProtocol,
  watchLifecycle,
} from "libark_web_test.so";

const WEB_TAG: string = "demo_web";

@Entry
@Component
struct ArkWeb {
  private controller: webview.WebviewController = new webview.WebviewController();
  @State customUrl: string = "custom://demo/index.html";
  @State note: string = "";

  build() {
    Column({ space: 10 }) {
      Web({ controller: this.controller, src: this.customUrl })
        .width("100%")
        .layoutWeight(1)
        .javaScriptAccess(true)
        .domStorageAccess(true)
      Text(this.note)
        .fontSize(12)
        .fontColor("#666666")
      Text('after register+refresh, run in page console:\n  ipc.postMessage("hi") / ipc.ping()')
        .fontSize(11)
        .fontColor("#666666")
      Row({ space: 6 }) {
        Button("init")
          .onClick((): void => {
            init(WEB_TAG);
            this.note = "proxy registered (page may need refresh)";
          })
        Button("initAndKeep")
          .onClick((): void => {
            initAndKeep(WEB_TAG);
            this.note = "proxy registered & kept alive";
          })
      }
      Row({ space: 6 }) {
        Button("refreshPage")
          .onClick((): void => {
            try {
              refreshPage(WEB_TAG);
              this.note = "page refreshed — proxy re-injected";
            } catch (e) {
              this.note = `ERROR: ${e}`;
            }
          })
        Button("watchLifecycle")
          .onClick((): void => {
            watchLifecycle(WEB_TAG);
            this.note = "lifecycle callbacks in hilog (tag hilog_rs)";
          })
      }
      Button("registerCustomProtocol (custom://)")
        .width("90%")
        .fontSize(13)
        .onClick((): void => {
          const ok: boolean = registerCustomProtocol(WEB_TAG);
          this.note = `custom protocol registered: ${ok} — reload the page`;
        })
      Button("back")
        .width("90%")
        .onClick((): void => {
          router.back();
        })
    }
    .width("100%")
    .height("100%")
    .padding(12);
  }
}
