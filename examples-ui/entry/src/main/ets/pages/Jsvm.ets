import { router } from "@kit.ArkUI";
import {
  bigintRoundtrip,
  bufferRoundtrip,
  evalInJsvm,
  exceptionMessage,
  externalRoundtrip,
  jsonRoundtrip,
  nativeMethodWithData,
  objectKeys,
  objectRoundtrip,
  promiseRoundtrip,
  typeTagRoundtrip,
  typedArrayRoundtrip,
  wrapRoundtrip,
} from "libjsvm_test.so";

@Entry
@Component
struct Jsvm {
  @State lines: string[] = [];
  @State script: string = "1 + 1";
  @State bigValue: string = "9007199254740993";

  private append(name: string, result: string): void {
    this.lines = [...this.lines, `${name} => ${result}`];
  }

  private run(name: string, fn: () => string | boolean | Array<number> | Array<string>): void {
    try {
      const result: string | boolean | Array<number> | Array<string> = fn();
      this.append(name, `${result}`);
    } catch (e) {
      this.append(name, `ERROR: ${e}`);
    }
  }

  build() {
    Column({ space: 8 }) {
      Text("JSVM")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Scroll() {
        Column({ space: 2 }) {
          ForEach(
            this.lines,
            (line: string) => {
              Text(line).fontSize(11).fontColor("#333333").width("100%");
            },
            (line: string, index: number) => `${index}-${line}`,
          )
        }.width("100%")
      }
      .width("92%")
      .layoutWeight(1)
      .align(Alignment.Top)
      TextInput({ text: this.script, placeholder: "script for evalInJsvm" })
        .width("90%")
        .onChange((value: string): void => {
          this.script = value;
        })
      TextInput({ text: this.bigValue, placeholder: "bigint value" })
        .width("90%")
        .onChange((value: string): void => {
          this.bigValue = value;
        })
      Row({ space: 6 }) {
        Button("evalInJsvm")
          .onClick((): void => {
            this.run("evalInJsvm", (): string | boolean | Array<number> | Array<string> =>
              evalInJsvm(this.script),
            );
          })
        Button("bigint")
          .onClick((): void => {
            this.run("bigintRoundtrip", (): string | boolean | Array<number> | Array<string> =>
              bigintRoundtrip(this.bigValue),
            );
          })
      }
      Row({ space: 6 }) {
        Button("buffer")
          .onClick((): void => {
            this.run("bufferRoundtrip", (): string | boolean | Array<number> | Array<string> =>
              bufferRoundtrip(),
            );
          })
        Button("typedArray")
          .onClick((): void => {
            this.run("typedArrayRoundtrip", (): string | boolean | Array<number> | Array<string> =>
              typedArrayRoundtrip(),
            );
          })
      }
      Row({ space: 6 }) {
        Button("object")
          .onClick((): void => {
            this.run("objectRoundtrip", (): string | boolean | Array<number> | Array<string> =>
              objectRoundtrip(),
            );
          })
        Button("objectKeys")
          .onClick((): void => {
            this.run("objectKeys", (): string | boolean | Array<number> | Array<string> =>
              objectKeys(),
            );
          })
      }
      Row({ space: 6 }) {
        Button("json")
          .onClick((): void => {
            this.run("jsonRoundtrip", (): string | boolean | Array<number> | Array<string> =>
              jsonRoundtrip(),
            );
          })
        Button("external")
          .onClick((): void => {
            this.run("externalRoundtrip", (): string | boolean | Array<number> | Array<string> =>
              externalRoundtrip(),
            );
          })
      }
      Row({ space: 6 }) {
        Button("promise")
          .onClick((): void => {
            this.run("promiseRoundtrip", (): string | boolean | Array<number> | Array<string> =>
              promiseRoundtrip(),
            );
          })
        Button("typeTag")
          .onClick((): void => {
            this.run("typeTagRoundtrip", (): string | boolean | Array<number> | Array<string> =>
              typeTagRoundtrip(),
            );
          })
      }
      Row({ space: 6 }) {
        Button("wrap")
          .onClick((): void => {
            this.run("wrapRoundtrip", (): string | boolean | Array<number> | Array<string> =>
              wrapRoundtrip(),
            );
          })
        Button("exception")
          .onClick((): void => {
            this.run("exceptionMessage", (): string | boolean | Array<number> | Array<string> =>
              exceptionMessage(),
            );
          })
      }
      Row({ space: 6 }) {
        Button("nativeMethod")
          .onClick((): void => {
            this.run("nativeMethodWithData", (): string | boolean | Array<number> | Array<string> =>
              nativeMethodWithData(),
            );
          })
        Button("clear")
          .onClick((): void => {
            this.lines = [];
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
