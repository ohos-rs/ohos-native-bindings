import { NodeContent } from "@kit.ArkUI";
import { MyApp, resetXcomponentGestureEvents, xcomponentGestureEvents } from "libarkui_test.so";

/**
 * Startup-driven host for one ArkUI-native XComponent node.
 *
 * Rust obtains the node's NativeXComponent handle for raw touch events and
 * attaches ArkUI system recognizers to the same node. No button is involved.
 */
@Entry
@Component
struct XComponentGestureHost {
  @State private readiness: string = "gesture-test-starting";
  @State private inputResult: string = "xcomponent:waiting";

  private readonly arkuiSlot: NodeContent = new NodeContent();
  private arkuiApp: MyApp | undefined = undefined;
  private pollTimer: number = -1;

  aboutToAppear(): void {
    try {
      this.arkuiApp = new MyApp(this.arkuiSlot);
      this.arkuiApp.createXcomponentGestureDemo();
      resetXcomponentGestureEvents();
    } catch (error) {
      this.readiness = `gesture-test-error:${error}`;
    }

    this.pollTimer = setInterval((): void => {
      this.refreshResults();
    }, 100);
  }

  aboutToDisappear(): void {
    if (this.pollTimer >= 0) {
      clearInterval(this.pollTimer);
      this.pollTimer = -1;
    }
    try {
      this.arkuiApp?.destroyNativeNode();
    } catch {}
  }

  private refreshResults(): void {
    try {
      this.inputResult = `xcomponent:${xcomponentGestureEvents()}`;
      if (this.arkuiApp !== undefined) {
        this.readiness = "gesture-test-ready";
      }
    } catch (error) {
      this.readiness = `gesture-test-error:${error}`;
    }
  }

  build() {
    Column({ space: 4 }) {
      Text(this.readiness)
        .id("gesture-test-readiness")
        .fontSize(12)
        .height(24)
      Text(this.inputResult)
        .id("xcomponent-input-result")
        .fontSize(10)
        .height(32)
      Column() {
        ContentSlot(this.arkuiSlot)
      }
      .id("xcomponent-input-surface")
      .width("100%")
      .height(180)
      .backgroundColor("#E3F2FD")
    }
    .id("gesture-test-root")
    .width("100%")
    .height("100%")
    .backgroundColor(Color.White);
  }
}
