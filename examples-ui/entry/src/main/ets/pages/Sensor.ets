import { router } from "@kit.ArkUI";
import { sensorList, sensorTest, subscribeSensor, unsubscribeSensor } from "libsensor_test.so";

@Entry
@Component
struct Sensor {
  @State started: boolean = false;
  @State listResult: string = "(not loaded)";
  @State typeCode: number = 1;

  build() {
    Column({ space: 10 }) {
      Text("Sensor")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Text(this.started ? "subscribed — events in hilog (tag hilog_rs)" : "not subscribed")
      Row({ space: 6 }) {
        Button("subscribe")
          .onClick((): void => {
            subscribeSensor(this.typeCode, 200000000);
            this.started = true;
          })
        Button("unsubscribe")
          .onClick((): void => {
            unsubscribeSensor();
            this.started = false;
          })
      }
      Row({ space: 6 }) {
        Button("accel")
          .onClick((): void => {
            this.typeCode = 1;
          })
        Button("gyro")
          .onClick((): void => {
            this.typeCode = 2;
          })
        Button("light")
          .onClick((): void => {
            this.typeCode = 5;
          })
        Button("gravity")
          .onClick((): void => {
            this.typeCode = 257;
          })
      }
      Text(`sensor type code: ${this.typeCode}`)
        .fontSize(12)
        .fontColor("#666666")
      Button("sensorList()")
        .width("90%")
        .onClick((): void => {
          try {
            this.listResult = sensorList();
          } catch (e) {
            this.listResult = `ERROR: ${e}`;
          }
        })
      Scroll() {
        Text(this.listResult)
          .fontSize(11)
          .width("100%")
      }
      .width("92%")
      .layoutWeight(1)
      .align(Alignment.Top)
      Button("legacy sensorTest()")
        .width("90%")
        .fontSize(13)
        .onClick((): void => {
          sensorTest();
          this.started = true;
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
