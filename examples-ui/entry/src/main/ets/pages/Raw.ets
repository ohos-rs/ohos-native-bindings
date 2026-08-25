import { router } from "@kit.ArkUI";
import { common } from "@kit.AbilityKit";
import { isRawDir, openRawDir, readRawFile, seekRawFile } from "libraw_test.so";

@Entry
@Component
struct Raw {
  @State result: string = "(not run)";

  private context(): common.UIAbilityContext {
    return this.getUIContext().getHostContext() as common.UIAbilityContext;
  }

  build() {
    Column({ space: 10 }) {
      Text("Raw")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Text(this.result).fontSize(13)
      Row({ space: 6 }) {
        Button("openDir(rawdir)")
          .onClick((): void => {
            try {
              openRawDir(this.context().resourceManager, "rawdir");
              this.result = "opened — file list in hilog (tag hilog_rs)";
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
        Button("isRawDir")
          .onClick((): void => {
            this.result = `rawdir is dir: ${isRawDir(this.context().resourceManager, "rawdir")}`;
          })
      }
      Row({ space: 6 }) {
        Button("readRawFile")
          .onClick((): void => {
            try {
              this.result = readRawFile(this.context().resourceManager, "rawdir/hello.txt");
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
        Button("seekRawFile")
          .onClick((): void => {
            try {
              this.result = seekRawFile(this.context().resourceManager, "rawdir/readme_sample.txt");
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
