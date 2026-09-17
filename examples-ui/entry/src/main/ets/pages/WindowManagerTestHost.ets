import { window } from "@kit.ArkUI";
import { callbackReport, uiContentReport } from "libwindow_manager_test.so";

@Entry
@Component
struct WindowManagerTestHost {
  @State private report: string = "window-manager-host-starting";
  @State private callback: string = "window-manager-callback-pending";

  aboutToAppear(): void {
    setTimeout((): void => {
      this.runProbe();
    }, 1000);
  }

  private async runProbe(): Promise<void> {
    try {
      const currentWindow = await window.getLastWindow(getContext(this));
      const windowId: number = currentWindow.getWindowProperties().id;
      this.report = uiContentReport(windowId);
      console.info(`WINDOW_MANAGER_UI_API_REPORT ${this.report}`);
      setTimeout((): void => {
        this.callback = callbackReport(windowId);
        console.info(`WINDOW_MANAGER_UI_CALLBACK_REPORT ${this.callback}`);
      }, 800);
    } catch (error) {
      this.report = `error:${error}`;
      console.error(`WINDOW_MANAGER_UI_API_REPORT ${this.report}`);
    }
  }

  build() {
    Column({ space: 8 }) {
      Text("WindowManager QEMU host")
        .fontSize(18)
      Text(this.report)
        .fontSize(10)
      Text(this.callback)
        .fontSize(10)
    }
    .width("100%")
    .height("100%")
  }
}
