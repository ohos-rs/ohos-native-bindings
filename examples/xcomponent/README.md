## XComponent and OpenGL ES example

### How to use it?

1. Create a xcomponent in your app.

```ts
@Entry
@Component
struct Index {
    @State message: string = 'Hello World'
    xComponentContext: ESObject | undefined = undefined;
    xComponentAttrs: XComponentAttrs = {
        id: 'xcomponentId',
        type: XComponentType.SURFACE,
        libraryname: 'xcomponent_test'
    }

    build() {
    Column() {
        Button("draw").onClick(() => {
          this.xComponentContext!.drawXcomponent();
        })
        // ...
        // 在xxx.ets 中定义 XComponent
        XComponent(this.xComponentAttrs)
            .width('100%')
            .layoutWeight(1)
            .focusable(true) // 可响应键盘事件
            .onLoad((xComponentContext) => {
            this.xComponentContext = xComponentContext;
            })
            .onDestroy(() => {
            console.log("onDestroy");
            })
        // ...
        }
        .height('100%')
    }
}
    
interface XComponentAttrs {
    id: string;
    type: number;
    libraryname: string;
}
```

The native module receives the original XComponent down, move, up, and cancel
events through `on_touch_event`. Applications that need semantic gestures
should create an ArkUI native XComponent node and attach ArkUI system gesture
recognizers instead of deriving gestures from this raw stream.

2. Use ohrs to build this crate.

```
ohrs build
```

3. Copy `libxcomponent_test.so` file to your harmony project which is in `dist/arm64-v8a` or `dist/x86_64`.

4. Have fun it!
