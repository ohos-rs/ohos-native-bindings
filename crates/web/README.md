# ohos-web-binding

This crate is a binding for the web module in OpenHarmony.

## Install

```shell
cargo add ohos-web-binding
```

## Usage

Add this crate to your native module and import it from Rust code:

```rust
use ohos_web_binding as web;

// Use the safe Rust APIs exposed by `ohos-web-binding` from your native module.
```

## HTTP request bodies

Body streams belong to the ArkWeb I/O thread. Reads initialize automatically and
consume the handle: the operation owns the native stream and output buffer until
completion, so overlapping reads cannot compile. No executor is required.

```rust,no_run
use ohos_web_binding::ResourceRequest;

// Call from the scheme handler's request-start callback on the ArkWeb I/O thread.
fn read_body(request: &ResourceRequest) {
    if let Some(stream) = request.http_body_stream() {
        stream.read_to_end(|result| match result {
            Ok(body) => println!("Read {} bytes", body.len()),
            Err(error) => eprintln!("Cannot read body: {error}"),
        });
    }
}
```

- `read_to_end(self, callback)` collects the remaining bytes through EOF and
  closes the stream. Known empty and unknown-length bodies are both supported.
- `read(self, size, callback)` delivers `Result<(HttpBodyStream, Vec<u8>), Error>`
  to the callback.
  Use the returned handle to continue reading from the same position. A zero
  read initializes the stream without consuming data.
- `initialize(self, callback)` is optional, for querying metadata before reading.
  It returns the initialized stream and never rewinds an already initialized one.
  `size()` returns `None` for unknown lengths and `Some(0)` for known empty bodies;
  metadata queries before initialization return `NotInitialized`.
- Each operation reports exactly one result through its callback, including
  submission errors. Errors close the stream. Callbacks may run inline or later
  on the same I/O thread and must own their captures (`'static`).
- Successful `read`/`initialize` calls return ownership: dropping that returned
  handle closes the stream. While an operation is pending, it retains ownership
  until completion. If native code never completes it, resources remain allocated
  because ArkWeb provides no cancellation acknowledgement for outstanding writes.

These are API changes: metadata getters return `Result`; operations consume
`self`; read callbacks receive a `Result` and require `'static` captures.
`HttpBodyStream::new` is unsafe and takes exclusive ownership.

The implementation uses one owned `BodyReader` per operation. Inline native
callbacks record completion for a loop to process after native Read returns;
deferred callbacks resume that loop. This prevents recursive short reads without
a shared stream state machine, reference counting, or a self-retaining cycle.

Host regression tests exercise the production reader with a deterministic native
test double, including synchronous reentry, deferred writes, stream destruction
and compile-time rejection of overlapping operations:

```shell
# From the repository root; only a Rust toolchain and host linker are required.
crates/web/tests/run-http-body-stream.sh
```

For native verification, the ArkWeb example posts binary, partial-read, large,
Blob, empty and unknown-length streaming bodies through a real Web component
and checks every echoed byte. The streaming cases use `ReadableStream` with
`duplex: "half"`, producing chunks of 1, 7, 65,539, 3, 131,071, 257 and 1,048,589
bytes at 100 ms intervals. They require native `is_chunked() == true`, unknown
size, non-memory-backed reads and an initial non-EOF state; buffering the upload
into a known-length body fails the test. Both `read_to_end` and repeated `read`
are exercised, with individual read sizes and EOF recorded in device logs.

```shell
pnpm run ui:sync -- --fail-fast ark_web
# Set HDC_BIN and OHOS_SDK_HOME as needed; wait for the device to finish booting.
HDC_TARGET=127.0.0.1:10003 pnpm run test:ui:ark-web
```

The device runner reinstalls the example app, removing its existing app data.
Unknown-length streaming has been verified on the HarmonyOS 7 / API 26 emulator.
Injected native errors and long synchronous short-read sequences are covered
separately by the host tests.

## License

MIT OR Apache-2.0
