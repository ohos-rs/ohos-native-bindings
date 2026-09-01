# AccessKit accessibility example

This example exposes the same AccessKit tree through both ArkUI custom-node
and XComponent native accessibility providers. The device host and E2E live
under `examples-ui`.

All binding crates are inherited from the repository workspace. Build the
example and run the real-device verification with:

```sh
pnpm --filter @ohos-rs/examples-ui run test:device:accessibility
```

The device script defaults to Arkdown's unsigned HAP, matching CI. For a real
device that requires a local signature, pass the signed artifact explicitly:

```sh
ACCESSIBILITY_E2E_HAP=/path/to/signed.hap \
HDC_TARGET=<device> \
pnpm --dir examples-ui run test:device:accessibility
```
