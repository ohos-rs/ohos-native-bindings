import { router } from "@kit.ArkUI";

interface XComponentAttrs {
  id: string;
  type: number;
  libraryname: string;
}

@Entry
@Component
struct NativeWindow {
  private xComponentAttrs: XComponentAttrs = {
    id: "nativeWindowSurface",
    type: XComponentType.SURFACE,
    libraryname: "native_window_test",
  };
  private context: Record<string, () => string> | undefined = undefined;
  @State result: string = "waiting for surface";

  build() {
    Column({ space: 10 }) {
      Text("NativeWindow")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      XComponent(this.xComponentAttrs)
        .width("100%")
        .height("45%")
        .onLoad((ctx?: object) => {
          this.context = ctx as Record<string, () => string>;
          this.result = this.context?.lastResult?.() ?? "loaded";
        })
      Text(this.result)
        .fontSize(12)
        .width("92%")
      Button("lastResult()")
        .width("90%")
        .onClick((): void => {
          this.result = this.context?.lastResult?.() ?? "no context";
        })
      Button("back")
        .width("90%")
        .onClick((): void => {
          router.back();
        })
    }
    .width("100%")
    .height("100%");
  }
}
