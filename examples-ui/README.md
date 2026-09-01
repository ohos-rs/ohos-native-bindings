# examples-ui

ArkTS demo 与 ohosTest 宿主工程，源码直接放在 `ohos-native-bindings` 仓库中。
Rust 动态库和类型声明由根目录 `examples/` 构建，不依赖 submodule。

## 环境

- Node.js 20.19+、pnpm
- Rust、Cargo、[ohrs](https://github.com/ohos-rs/ohrs)
- 已配置 `OHOS_SDK_HOME` 的 OpenHarmony SDK
- hdc；运行 XComponent QEMU 用例还需要 jq

`pnpm install` 会安装 workspace 固定版本的 arkdown 与 oxk。首次构建测试包时，
脚本会在 `examples-ui/.tools/` 自动安装 `ohpm-rs 0.2` 和 hapsigner，不污染全局环境。

## 常用命令

在仓库根目录运行：

```sh
pnpm install
pnpm run init

pnpm run ui:sync                    # 构建并同步全部 Rust example
pnpm run ui:sync -- arkui vsync     # 只同步指定 example
pnpm run ui:sync -- --arch x64      # 显式构建 x64，默认 arm64
pnpm run ui:sync -- --fail-fast     # 首个构建失败后立即结束
pnpm run ui:build                   # 构建并签名主 HAP
pnpm run ui:run                     # sync -> build -> install -> start

pnpm run test:ui                    # 分模块执行全部 ohosTest
pnpm run test:ui -- sensor vsync    # 只执行指定模块
pnpm run test:ui -- --arch x64      # 在 x64 设备执行全部 ohosTest
pnpm run test:ui -- --fail-fast     # 首个失败模块出现后立即结束
pnpm run test:ui:xcomponent -- --arch x64 # QEMU XComponent 手势 E2E
pnpm run test:ui:accessibility -- --arch x64 # QEMU/设备 AccessKit E2E

pnpm run format:ui
pnpm run lint:ui
pnpm run prek:check
```

## 自动化约定

- `entry/src/ohosTest/ets/test/modules/`：每个 binding 一个 Hypium 用例。
- `scripts/run-ohostest.sh`：每个 binding 使用独立 `aa test` 进程，避免多个 napi
  模块耗尽单进程 pthread TLS key。
- `scripts/run-qemu-xcomponent-gestures.sh`：按 `--arch` 指定的架构（默认 `arm64`）
  构建 `arkui`、`xcomponent`、`xcomponent_multi`，自动注入并验证 Tap、Pan、
  Swipe 和多点触控。
- `entry/libs/`、`entry/src/main/ets/types/`、`.tools/` 均为本地生成内容，不提交。

`scripts/sync-rust.sh` 默认使用父目录的 bindings workspace；仅在验证其他 checkout
时设置 `OHOS_NATIVE_BINDINGS=<path>`。HMS OpenGTX 需要 HMS SDK，可单独执行：

```sh
pnpm run ui:sync -- opengtx
```
