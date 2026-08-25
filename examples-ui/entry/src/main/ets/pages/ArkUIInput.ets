import { router } from "@kit.ArkUI";

interface XComponentAttrs {
  id: string;
  type: number;
  libraryname: string;
}

@Entry
@Component
struct ArkUIInput {
  private xComponentAttrs: XComponentAttrs = {
    id: "arkuiInputSurface",
    type: XComponentType.SURFACE,
    libraryname: "arkui_input_test",
  };
  private context: Record<string, () => string> | undefined = undefined;
  @State result: string = "touch / axis on the surface";

  build() {
    Column({ space: 10 }) {
      Text("ArkUI Input")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      XComponent(this.xComponentAttrs)
        .width("100%")
        .height("50%")
        .focusable(true)
        .onLoad((ctx?: object) => {
          this.context = ctx as Record<string, () => string>;
          this.result = this.context?.lastEvent?.() ?? "loaded";
        })
      Text(this.result)
        .fontSize(12)
        .width("92%")
      Button("lastEvent()")
        .width("90%")
        .onClick((): void => {
          this.result = this.context?.lastEvent?.() ?? "no context";
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
