# NativeImage

Thin native consumer ownership and acquire-fence handling (API 12).
`ConsumerSurface::new()` keeps the SDK's default usage; API 13 adds
`with_usage(BufferUsage)` / `set_usage` and native dimension changes.
Usage flags come from the generated native-buffer SDK, preserve caller bits
and combine with `|`.

`surface_id()` queries only the surface ID.
`acquire_native_window()` separately initializes/borrows the SDK-owned producer
window. `acquire()` returns a `Frame` guard or `None` for the named native
no-buffer status. Each frame retains the consumer and native buffer wrapper;
Drop returns an unsignalled acquire fence to the SDK and balances the reference.
`wait(timeout)` checks the native fence using the caller's timeout.
`native_window_buffer()` borrows its opaque handle for native interoperability.

`unsafe set_frame_available_listener` forwards the SDK listener descriptor.
The caller owns the callback context and synchronizes producer-thread access.
Unregister and Drop release the registration but do not prove in-flight
callback quiescence; retain the context until the host has established that
boundary. Callbacks must not unwind or reenter NativeImage operations.
All native cleanup failures are logged through `log`.

Pixel mapping, format conversion, frame budgets and rendering-worker scheduling
belong to consumers. Native declarations are generated from
`native_image/native_image.h` through `tools/generate/build/config/native_image.rs`.

## Breaking

The unpublished closure constructor/global registry and RGBA conversion API
are withdrawn. Use the native listener's explicit unsafe lifetime contract,
caller-selected usage, separate window/identity methods and fenced raw buffer.
No closure identity allocator, pixel conversion or EGL readback protocol remains.

## Validation

`bash crates/native_image/tests/run-host.sh` executes production owners against
statically linked host substitutes and compile-fail lifetime/thread tests.
Its archives and output are stored under the repository's `target/`.
