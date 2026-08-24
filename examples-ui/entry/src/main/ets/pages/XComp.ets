import { router } from "@kit.ArkUI";

interface XComponentAttrs {
  id: string;
  type: number;
  libraryname: string;
}

@Entry
@Component
struct XComp {
  private xComponentAttrs: XComponentAttrs = {
    id: "xcomponentId",
    type: XComponentType.SURFACE,
    libraryname: "xcomponent_test",
  };
  private xComponentContext: Record<string, Function> | undefined = undefined;
  @State note: string = "";

  build() {
    Column({ space: 12 }) {
      XComponent(this.xComponentAttrs)
        .width("100%")
        .height("55%")
        .focusable(true)
        .onLoad((context?: object) => {
          // The framework loads libxcomponent_test.so and injects
          // __NATIVE_XCOMPONENT_OBJ__ before handing the module exports here.
          // Do NOT statically import the .so at the top of this file: the
          // module would load before the property is injected and abort.
          this.xComponentContext = context as Record<string, Function>;
        })
        .onDestroy(() => {
          console.log("XComp onDestroy");
        })
      Text(this.note)
        .fontSize(11)
        .fontColor("#666666")
      Button("draw (blue)")
        .width("90%")
        .onClick((): void => {
          this.xComponentContext?.drawXcomponent();
        })
      Row({ space: 8 }) {
        Button("rate 30")
          .onClick((): void => {
            this.xComponentContext?.setFrameRate(30, 30, 30);
            this.note = "frame rate range set to 30";
          })
        Button("rate 90")
          .onClick((): void => {
            this.xComponentContext?.setFrameRate(60, 120, 90);
            this.note = "frame rate range set to 60-120@90";
          })
      }
      Text("surface/touch/mouse/hover/frame/key events in hilog (tag hilog_rs)")
        .fontSize(10)
        .fontColor("#999999")
      Button("back")
        .width("90%")
        .onClick((): void => {
          router.back();
        })
    }
    .width("100%")
    .height("100%")
    .padding(12);
  }
}
