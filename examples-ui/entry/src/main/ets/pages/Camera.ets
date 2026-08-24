import { router } from "@kit.ArkUI";
import { abilityAccessCtrl, common, Permissions } from "@kit.AbilityKit";

interface XComponentAttrs {
  id: string;
  type: number;
  libraryname: string;
}

@Entry
@Component
struct CameraDemo {
  private xComponentAttrs: XComponentAttrs = {
    id: "cameraSurface",
    type: XComponentType.SURFACE,
    libraryname: "camera_test",
  };
  private context: Record<string, Function> | undefined = undefined;
  @State result: string = "requesting camera permission";

  aboutToAppear(): void {
    const ctx = this.getUIContext().getHostContext() as common.UIAbilityContext;
    const permissions: Array<Permissions> = ["ohos.permission.CAMERA"];
    abilityAccessCtrl
      .createAtManager()
      .requestPermissionsFromUser(ctx, permissions)
      .then((): void => {
        this.result = "permission flow finished — wait for preview surface";
      })
      .catch((e: object) => {
        this.result = `permission ERROR: ${e}`;
      });
  }

  build() {
    Column({ space: 10 }) {
      Text("Camera")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      XComponent(this.xComponentAttrs)
        .width("100%")
        .height("40%")
        .onLoad((ctx?: object) => {
          this.context = ctx as Record<string, Function>;
          this.result = this.context?.lastResult?.() ?? "loaded";
        })
      Text(this.result)
        .fontSize(12)
        .width("92%")
      Row({ space: 6 }) {
        Button("info")
          .onClick((): void => {
            try {
              this.result = this.context?.sessionInfo?.() ?? "no context";
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
        Button("capture")
          .onClick((): void => {
            try {
              this.result = this.context?.capture?.() ?? "no context";
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
      }
      Row({ space: 6 }) {
        Button("flash on")
          .onClick((): void => {
            try {
              this.result =
                (this.context?.switchFlash?.(true) as string | undefined) ?? "no context";
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
        Button("flash off")
          .onClick((): void => {
            try {
              this.result =
                (this.context?.switchFlash?.(false) as string | undefined) ?? "no context";
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
      }
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
