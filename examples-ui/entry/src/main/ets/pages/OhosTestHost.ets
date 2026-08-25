import { NodeContent, router } from "@kit.ArkUI";
import { webview } from "@kit.ArkWeb";

interface XComponentAttrs {
  id: string;
  type: number;
  libraryname: string;
}

/**
 * Host surfaces for the ohosTest Hypium suite. Lives in the main module
 * because the ohosTest HAP's ArkTS pipeline (arkdown) does not register
 * router pages — loadContent from TestAbility ends in "page not found".
 * Instead the app starts this page directly. Device-side E2E scripts inspect
 * its live surfaces before `aa test` starts a separate Hypium process.
 *
 * XComponent libraries must be loaded via libraryname (not import "lib*.so"),
 * otherwise module_exports calls XComponent::init without
 * __NATIVE_XCOMPONENT_OBJ__ and the process aborts.
 */
@Entry
@Component
struct OhosTestHost {
  private xcomponentAttrs: XComponentAttrs = {
    id: "ohosTest_xcomponent",
    type: XComponentType.SURFACE,
    libraryname: "xcomponent_test",
  };
  private arkuiInputAttrs: XComponentAttrs = {
    id: "ohosTest_arkui_input",
    type: XComponentType.SURFACE,
    libraryname: "arkui_input_test",
  };
  private nativeWindowAttrs: XComponentAttrs = {
    id: "ohosTest_native_window",
    type: XComponentType.SURFACE,
    libraryname: "native_window_test",
  };
  private cameraAttrs: XComponentAttrs = {
    id: "ohosTest_camera",
    type: XComponentType.SURFACE,
    libraryname: "camera_test",
  };
  // Two XComponents bound to the SAME library: the binding's single global
  // callback slot receives surface events from both instances. The E2E suite
  // asserts the interplay via libxcomponent_test.so's surfaceEvents().
  private xcomponentDupAttrs: XComponentAttrs = {
    id: "ohosTest_xcomponent_dup",
    type: XComponentType.SURFACE,
    libraryname: "xcomponent_test",
  };
  // Two XComponents bound to the multi_mode variant (dispatch by id):
  // each instance keeps independent callback state.
  private xcomponentMultiAttrs: XComponentAttrs = {
    id: "ohosTest_xc_multi_a",
    type: XComponentType.SURFACE,
    libraryname: "xcomponent_multi_test",
  };
  private xcomponentMultiAttrsB: XComponentAttrs = {
    id: "ohosTest_xc_multi_b",
    type: XComponentType.SURFACE,
    libraryname: "xcomponent_multi_test",
  };
  private webController: webview.WebviewController = new webview.WebviewController();
  private arkuiSlot: NodeContent = new NodeContent();

  build() {
    Column({ space: 4 }) {
      Text("ohos-native-bindings ohosTest host")
        .fontSize(14)
        .margin({ top: 12 })
      Button("demo menu")
        .fontSize(12)
        .height(28)
        .onClick((): void => {
          router.pushUrl({ url: "pages/Index" });
        })
      Row({ space: 4 }) {
        XComponent(this.xcomponentAttrs)
          .width("50%")
          .height(72)
        XComponent(this.arkuiInputAttrs)
          .width("50%")
          .height(72)
          .focusable(true)
      }
      .width("100%")
      .height(72)
      Row({ space: 4 }) {
        XComponent(this.nativeWindowAttrs)
          .width("50%")
          .height(72)
        XComponent(this.cameraAttrs)
          .width("50%")
          .height(72)
      }
      .width("100%")
      .height(72)
      // Same-library pair (singleton callback slot) + multi_mode pair
      // (per-id dispatch), both observable via their demo exports.
      Row({ space: 4 }) {
        XComponent(this.xcomponentDupAttrs)
          .width("34%")
          .height(72)
        XComponent(this.xcomponentMultiAttrs)
          .width("33%")
          .height(72)
        XComponent(this.xcomponentMultiAttrsB)
          .width("33%")
          .height(72)
      }
      .width("100%")
      .height(72)
      Column() {
        ContentSlot(this.arkuiSlot)
      }
      .width("100%")
      .height(72)
      Web({ controller: this.webController, src: "about:blank" })
        .id("ohosTest_web")
        .width("100%")
        .height(96)
        .javaScriptAccess(true)
        .domStorageAccess(true)
    }
    .width("100%")
    .height("100%");
  }
}
