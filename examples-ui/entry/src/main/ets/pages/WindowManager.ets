import { window } from "@kit.ArkUI";
import { smoke } from "libwindow_manager_test.so";

@Entry
@Component
struct WindowManager {
  @State result: string = "(not run)";

  aboutToAppear(): void {
    this.runSmoke();
  }

  private async runSmoke(): Promise<void> {
    try {
      const currentWindow = await window.getLastWindow(getContext(this));
      this.result = smoke(currentWindow.getWindowProperties().id);
    } catch (error) {
      this.result = `ERROR: ${error}`;
    }
  }

  build() {
    Column({ space: 12 }) {
      Text("Window Manager")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 32 })
      Text(this.result)
        .fontSize(14)
        .width("92%")
      Button("smoke()")
        .width("90%")
        .onClick((): void => {
          this.runSmoke();
        })
    }
    .width("100%")
    .height("100%");
  }
}
