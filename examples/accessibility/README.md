# AccessKit accessibility example

This example exposes AccessKit trees through an ArkUI custom-node provider, a
single XComponent provider, and two simultaneous XComponent providers using
the API 15 multi-instance callback contract. The device host and E2E live under
`examples-ui`.

All binding crates are inherited from the repository workspace. Build the
example and run the real-device verification with:

```sh
pnpm --filter @ohos-rs/examples-ui run test:device:accessibility
```

The device script defaults to Arkdown's unsigned HAP, matching CI. When that
artifact is absent, it builds the accessibility Rust example before producing
the unsigned HAP. For a real device that requires a local signature, pass the
signed artifact explicitly:

```sh
ACCESSIBILITY_E2E_HAP=/path/to/signed.hap \
HDC_TARGET=<device> \
pnpm --dir examples-ui run test:device:accessibility
```
