import { router } from "@kit.ArkUI";
import { common } from "@kit.AbilityKit";
import { fileIo } from "@kit.CoreFileKit";
import { cancel, customStart, start } from "libvibrator_test.so";

@Entry
@Component
struct Vibrator {
  @State result: string = "(not run)";
  @State effectPath: string = "";

  aboutToAppear(): void {
    // Copy the bundled haptic effect to the sandbox so the native side can
    // open it by an absolute path.
    const context = this.getUIContext().getHostContext() as common.UIAbilityContext;
    const target: string = `${context.filesDir}/coin_drop.json`;
    context.resourceManager
      .getRawFileContent("coin_drop.json")
      .then((content: Uint8Array) => {
        const file: fileIo.File = fileIo.openSync(
          target,
          fileIo.OpenMode.READ_WRITE | fileIo.OpenMode.CREATE | fileIo.OpenMode.TRUNC,
        );
        fileIo.writeSync(file.fd, content.buffer);
        fileIo.closeSync(file);
        this.effectPath = target;
      })
      .catch(() => {
        this.effectPath = "";
      });
  }

  build() {
    Column({ space: 12 }) {
      Text("Vibrator")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 48 })
      Text(this.result).fontSize(14)
      Text(this.effectPath === "" ? "effect file not prepared" : this.effectPath)
        .fontSize(11)
        .fontColor("#666666")
      Button("start()")
        .width("90%")
        .onClick(() => {
          try {
            start();
            this.result = "start ok";
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("cancel()")
        .width("90%")
        .onClick(() => {
          try {
            cancel();
            this.result = "cancel ok";
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("customStart(coin_drop.json)")
        .width("90%")
        .fontSize(14)
        .onClick(() => {
          if (this.effectPath === "") {
            this.result = "effect file not prepared";
            return;
          }
          try {
            customStart(this.effectPath);
            this.result = "customStart ok";
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("back")
        .width("90%")
        .onClick(() => {
          router.back();
        })
    }
    .width("100%")
    .height("100%");
  }
}
