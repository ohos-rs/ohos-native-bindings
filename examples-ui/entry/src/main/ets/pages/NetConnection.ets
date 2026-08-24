import { router } from "@kit.ArkUI";
import {
  allNetIds,
  currentProxyInfo,
  defaultNetInfo,
  defaultNetMetered,
  googleVpnProbe,
  hasDefaultNet,
  resolveHost,
} from "libnet_connection_test.so";

@Entry
@Component
struct NetConnection {
  @State result: string = "(not run)";
  @State running: boolean = false;
  @State host: string = "www.huawei.com";

  build() {
    Column({ space: 10 }) {
      Text("NetConnection")
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
      Row({ space: 6 }) {
        Button("hasDefaultNet")
          .onClick((): void => {
            try {
              this.result = `has default net: ${hasDefaultNet()}`;
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
        Button("metered")
          .onClick((): void => {
            try {
              this.result = `default net metered: ${defaultNetMetered()}`;
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
      }
      Row({ space: 6 }) {
        Button("allNetIds")
          .onClick((): void => {
            try {
              this.result = allNetIds();
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
        Button("proxyInfo")
          .onClick((): void => {
            try {
              this.result = currentProxyInfo();
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
      }
      Button("defaultNetInfo() (caps + props)")
        .width("90%")
        .fontSize(13)
        .onClick((): void => {
          try {
            this.result = defaultNetInfo();
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      TextInput({ text: this.host, placeholder: "host to resolve" })
        .width("90%")
        .onChange((value: string): void => {
          this.host = value;
        })
      Button("resolveHost (DNS)")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = resolveHost(this.host);
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("googleVpnProbe() (needs network)")
        .width("90%")
        .fontSize(13)
        .onClick((): void => {
          if (this.running) {
            return;
          }
          this.running = true;
          this.result = "requesting...";
          try {
            this.result = googleVpnProbe();
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
          this.running = false;
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
