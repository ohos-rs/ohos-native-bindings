# window_manager example

Builds a N-API smoke-test module for `ohos-window-manager-binding`.

```shell
ohrs build --arch aarch
```

The dedicated [2in1 QEMU E2E workflow](../../.github/workflows/window-manager-e2e.yml)
builds the API 26 x64 example, grants the user-level screen-capture permission,
and validates window layouts, filters, touch injection, snapshots, cursor and
density controls, and asynchronous density/frame callbacks.
