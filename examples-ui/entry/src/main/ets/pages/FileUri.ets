import { router } from "@kit.ArkUI";
import { common } from "@kit.AbilityKit";
import { fileIo } from "@kit.CoreFileKit";
import {
  directoryUri,
  fileName,
  pathFromUri,
  smoke,
  uriFromPath,
  validUri,
} from "libfileuri_test.so";

@Entry
@Component
struct FileUri {
  @State result: string = "(not run)";
  @State path: string = "";

  aboutToAppear(): void {
    const context = this.getUIContext().getHostContext() as common.UIAbilityContext;
    this.path = `${context.filesDir}/fileuri_demo.txt`;
    const file: fileIo.File = fileIo.openSync(
      this.path,
      fileIo.OpenMode.READ_WRITE | fileIo.OpenMode.CREATE,
    );
    fileIo.closeSync(file);
    try {
      this.result = smoke(this.path);
    } catch (e) {
      this.result = `ERROR: ${e}`;
    }
  }

  build() {
    Column({ space: 10 }) {
      Text("FileUri")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Scroll() {
        Text(this.result)
          .fontSize(12)
          .width("100%")
      }
      .width("92%")
      .layoutWeight(1)
      .align(Alignment.Top)
      Button("uriFromPath")
        .width("90%")
        .onClick((): void => {
          try {
            const uri: string = uriFromPath(this.path);
            this.result = `uri=${uri}\nvalid=${validUri(uri)}\npath=${pathFromUri(uri)}\ndir=${directoryUri(uri)}\nname=${fileName(uri)}`;
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("smoke()")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = smoke(this.path);
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
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
