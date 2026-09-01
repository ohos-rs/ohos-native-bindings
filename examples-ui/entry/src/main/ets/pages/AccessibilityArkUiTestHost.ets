import { NodeContent } from "@kit.ArkUI";
import {
  accessibilityStatus,
  ArkUiAccessibilityApp,
  refreshAccessibilityTrees,
} from "libaccessibility_test.so";

/** Real-device host for AccessKit over an ArkUI custom node. */
@Entry
@Component
struct AccessibilityArkUiTestHost {
  @State private readiness: string = "accessibility-arkui-starting";
  @State private status: string = "accessibility-status-unavailable";

  private readonly slot: NodeContent = new NodeContent();
  private app: ArkUiAccessibilityApp | undefined = undefined;
  private pollTimer: number = -1;

  aboutToAppear(): void {
    try {
      this.app = new ArkUiAccessibilityApp(this.slot);
      this.app.mount();
      this.readiness = "accessibility-arkui-ready";
    } catch (error) {
      this.readiness = `accessibility-arkui-error:${error}`;
    }
    this.pollTimer = setInterval((): void => {
      try {
        refreshAccessibilityTrees();
        this.status = accessibilityStatus();
      } catch (error) {
        this.status = `accessibility-status-error:${error}`;
      }
    }, 100);
  }

  aboutToDisappear(): void {
    if (this.pollTimer >= 0) {
      clearInterval(this.pollTimer);
      this.pollTimer = -1;
    }
    try {
      this.app?.unmount();
    } catch {}
  }

  build() {
    Column({ space: 12 }) {
      Text(this.readiness)
        .id("accessibility-test-readiness")
        .fontSize(14)
      Text(this.status)
        .id("accessibility-test-status")
        .fontSize(12)
      Text("ArkUI custom-node AccessKit provider").fontSize(16)
      Column() {
        ContentSlot(this.slot)
      }
      .id("accesskit-arkui-host")
      .width(360)
      .height(128)
      .backgroundColor("#E8F5E9")
    }
    .id("accessibility-test-root")
    .width("100%")
    .height("100%")
    .padding(24)
    .backgroundColor(Color.White);
  }
}
