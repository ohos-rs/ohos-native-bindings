## XComponent and OpenGL ES example

### How to use it?

1. Create a xcomponent in your app.

```ts
@Entry
@Component
struct Index {
    @State message: string = 'Hello World'
    xComponentContext: object | undefined = undefined;
    xComponentAttrs: XComponentAttrs = {
        id: 'xcomponentId',
        type: XComponentType.SURFACE,
        libraryname: 'nativerender'
    }

    build() {
    Row() {
        // ...
        // 在xxx.ets 中定义 XComponent
        XComponent(this.xComponentAttrs)
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

2. Use ohrs to build this crate.

```
ohrs build
```

3. Copy `libnativerender.so` file to your harmony project which is in `dist/arm64-v8a` or `dist/x86_64`.

4. Have fun it!