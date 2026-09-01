import {
  accessibilityStatus,
  multiInstancesExercised,
  refreshAccessibilityTrees,
  releaseMultiInstanceA,
} from "libaccessibility_test.so";

interface XComponentAttrs {
  id: string;
  type: number;
  libraryname: string;
}

/** Real-device host for AccessKit's API 15 multi-instance callback contract. */
@Entry
@Component
struct AccessibilityMultiInstanceTestHost {
  @State private readiness: string = "accessibility-multi-starting";
  @State private status: string = "accessibility-status-unavailable";
  @State private canRelease: boolean = false;

  private readonly attrsA: XComponentAttrs = {
    id: "accesskit_multi_a",
    type: XComponentType.SURFACE,
    libraryname: "accessibility_test",
  };
  private readonly attrsB: XComponentAttrs = {
    id: "accesskit_multi_b",
    type: XComponentType.SURFACE,
    libraryname: "accessibility_test",
  };
  private pollTimer: number = -1;

  aboutToAppear(): void {
    this.readiness = "accessibility-multi-ready";
    this.pollTimer = setInterval((): void => {
      try {
        refreshAccessibilityTrees();
        this.status = accessibilityStatus();
        this.canRelease = multiInstancesExercised();
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

  private releaseInstanceA(): void {
    try {
      const released: boolean = releaseMultiInstanceA();
      this.readiness = released
        ? "accessibility-multi-a-released"
        : "accessibility-multi-a-release-missed";
    } catch (error) {
      this.readiness = `accessibility-multi-release-error:${error}`;
    }
  }

  build() {
    Column({ space: 8 }) {
      Text(this.readiness)
        .id("accessibility-test-readiness")
        .fontSize(14)
      Text(this.status)
        .id("accessibility-test-status")
        .fontSize(10)
      Text("AccessKit multi-instance providers").fontSize(14)
      XComponent(this.attrsA)
        .id("accesskit-multi-a-host")
        .width(360)
        .height(128)
        .focusable(true)
        .backgroundColor("#FFF3E0")
      XComponent(this.attrsB)
        .id("accesskit-multi-b-host")
        .width(360)
        .height(128)
        .focusable(true)
        .backgroundColor("#E8EAF6")
      Button("Release instance A")
        .id("accessibility-multi-release-a")
        .enabled(this.canRelease)
        .onClick((): void => this.releaseInstanceA())
    }
    .id("accessibility-test-root")
    .width("100%")
    .height("100%")
    .padding(16)
    .backgroundColor(Color.White);
  }
}
