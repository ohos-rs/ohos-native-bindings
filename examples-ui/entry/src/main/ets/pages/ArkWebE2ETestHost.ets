import { webview } from "@kit.ArkWeb";
import {
  init,
  initAndKeep,
  prepareCustomProtocol,
  refreshPage,
  registerCustomProtocol,
  watchLifecycle,
} from "libark_web_test.so";

const WEB_TAG: string = "ark-web-e2e-surface";

/**
 * Runs ArkWeb native APIs in the same application process as the Web
 * component. The device-side runner reads the status node and treats only the
 * explicit pass marker as success.
 */
@Entry
@Component
struct ArkWebE2ETestHost {
  private readonly controller: webview.WebviewController = new webview.WebviewController(WEB_TAG);
  @State private status: string = "ark-web-e2e-starting";
  private setupComplete: boolean = false;
  private verificationStarted: boolean = false;
  private refreshRequested: boolean = false;

  aboutToAppear(): void {
    try {
      prepareCustomProtocol();
      watchLifecycle(WEB_TAG);
    } catch (error) {
      this.status = `ark-web-e2e-fail:${error}`;
    }
  }

  private setupBindings(): void {
    try {
      const protocolRegistered: boolean = registerCustomProtocol(WEB_TAG);
      if (!protocolRegistered) {
        throw new Error("custom protocol registration returned false");
      }
      init(WEB_TAG);
      initAndKeep(WEB_TAG);
      this.setupComplete = true;
      this.status = "ark-web-e2e-verifying";
      this.controller.loadUrl("custom://demo/index.html");
    } catch (error) {
      this.status = `ark-web-e2e-fail:${error}`;
    }
  }

  private async verifyProxy(): Promise<void> {
    if (!this.setupComplete || this.verificationStarted) {
      return;
    }
    this.verificationStarted = true;
    try {
      const proxyAvailable: string = await this.controller.runJavaScript(
        "typeof ipc === 'object' && typeof ipc.postMessage === 'function' && typeof ipc.ping === 'function'",
      );
      if (proxyAvailable !== "true") {
        throw new Error(`JavaScript proxy is unavailable: ${proxyAvailable}`);
      }
      const customPage: string = await this.controller.runJavaScript(
        "document.body.textContent.includes('Hello from custom protocol (rust)')",
      );
      if (customPage !== "true") {
        throw new Error(`custom protocol response is unavailable: ${customPage}`);
      }
      const invocationResult: string = await this.controller.runJavaScript(
        "ipc.postMessage('ark-web-e2e'); ipc.ping(); true",
      );
      if (invocationResult !== "true") {
        throw new Error(`JavaScript proxy invocation failed: ${invocationResult}`);
      }
      if (!this.refreshRequested) {
        this.refreshRequested = true;
        this.verificationStarted = false;
        this.status = "ark-web-e2e-refreshing";
        refreshPage(WEB_TAG);
        return;
      }
      this.status = "ark-web-e2e-reading-bodies";
      await this.controller.runJavaScript("runBodyStreamTests(); 'started'");
      let bodyStatus: string = "running";
      for (let attempt: number = 0; attempt < 200; attempt++) {
        bodyStatus = JSON.parse(
          await this.controller.runJavaScript("window.bodyE2EStatus"),
        ) as string;
        if (bodyStatus.startsWith("pass:")) {
          console.info(`ArkWeb body stream E2E ${bodyStatus}`);
          break;
        }
        if (bodyStatus.startsWith("fail:")) {
          throw new Error(bodyStatus);
        }
        await new Promise<void>((resolve) => setTimeout(resolve, 100));
      }
      if (!bodyStatus.startsWith("pass:")) {
        throw new Error(`body stream tests timed out: ${bodyStatus}`);
      }
      this.status = "ark-web-e2e-pass";
    } catch (error) {
      this.status = `ark-web-e2e-fail:${error}`;
    }
  }

  build() {
    Column({ space: 8 }) {
      Text(this.status)
        .id("ark-web-e2e-status")
        .fontSize(14)
        .height(32)
      Web({ controller: this.controller, src: "" })
        .id(WEB_TAG)
        .width("100%")
        .layoutWeight(1)
        .javaScriptAccess(true)
        .domStorageAccess(true)
        .onControllerAttached((): void => {
          this.setupBindings();
        })
        .onPageEnd((): void => {
          this.verifyProxy();
        })
    }
    .id("ark-web-e2e-root")
    .width("100%")
    .height("100%")
    .padding(12)
    .backgroundColor(Color.White);
  }
}
