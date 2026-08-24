import { router } from "@kit.ArkUI";
import { common } from "@kit.AbilityKit";
import { fileIo } from "@kit.CoreFileKit";
import { uriFromPath } from "libfileuri_test.so";
import {
  activate,
  checkPersistent,
  deactivate,
  persist,
  revoke,
  smoke,
} from "libfileshare_test.so";

@Entry
@Component
struct FileShare {
  @State result: string = "(not run)";
  @State path: string = "";
  @State uri: string = "";

  aboutToAppear(): void {
    const context = this.getUIContext().getHostContext() as common.UIAbilityContext;
    this.path = `${context.filesDir}/fileshare_demo.txt`;
    const file: fileIo.File = fileIo.openSync(
      this.path,
      fileIo.OpenMode.READ_WRITE | fileIo.OpenMode.CREATE,
    );
    fileIo.closeSync(file);
    try {
      this.uri = uriFromPath(this.path);
      this.result = smoke(this.path);
    } catch (e) {
      this.result = `ERROR: ${e}`;
    }
  }

  private run(name: string, fn: (uri: string) => string): void {
    try {
      this.result = `${name}: ${fn(this.uri)}`;
    } catch (e) {
      this.result = `${name} ERROR: ${e}`;
    }
  }

  build() {
    Column({ space: 10 }) {
      Text("FileShare")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Text("FILE_ACCESS_PERSIST is restricted; errors here are expected on a normal app.")
        .fontSize(11)
        .fontColor("#666666")
        .width("92%")
      Scroll() {
        Text(this.result)
          .fontSize(12)
          .width("100%")
      }
      .width("92%")
      .layoutWeight(1)
      .align(Alignment.Top)
      Row({ space: 6 }) {
        Button("persist")
          .onClick((): void => this.run("persist", persist))
        Button("check")
          .onClick((): void => this.run("check", checkPersistent))
      }
      Row({ space: 6 }) {
        Button("activate")
          .onClick((): void => this.run("activate", activate))
        Button("deactivate")
          .onClick((): void => this.run("deactivate", deactivate))
      }
      Button("revoke")
        .width("90%")
        .onClick((): void => this.run("revoke", revoke))
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
