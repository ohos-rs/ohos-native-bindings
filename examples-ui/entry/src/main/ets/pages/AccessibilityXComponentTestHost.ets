import {
  accessibilityStatus,
  refreshAccessibilityTrees,
} from "libaccessibility_test.so";

interface XComponentAttrs {
  id: string;
  type: number;
  libraryname: string;
}

/** Real-device host for AccessKit over an XComponent provider. */
@Entry
@Component
struct AccessibilityXComponentTestHost {
  @State private status: string = "accessibility-status-unavailable";

  private readonly attrs: XComponentAttrs = {
    id: "accesskit_xcomponent",
    type: XComponentType.SURFACE,
    libraryname: "accessibility_test",
  };
  private pollTimer: number = -1;

  aboutToAppear(): void {
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
  }

  build() {
    Column({ space: 12 }) {
      Text("accessibility-xcomponent-ready")
        .id("accessibility-test-readiness")
        .fontSize(14)
      Text(this.status)
        .id("accessibility-test-status")
        .fontSize(12)
      Text("XComponent AccessKit provider")
        .fontSize(16)
      XComponent(this.attrs)
        .id("accesskit-xcomponent-host")
        .width(360)
        .height(128)
        .focusable(true)
        .backgroundColor("#E3F2FD")
    }
    .id("accessibility-test-root")
    .width("100%")
    .height("100%")
    .padding(24)
    .backgroundColor(Color.White);
  }
}
