use ohos_hilog_binding::hilog_info;
use ohos_web_binding::{
    ArkWebResponse, HttpBodyStream, HttpBodyStreamError, ResourceHandle, ResourceRequest,
};

pub(crate) const PAGE: &str = include_str!("body_stream_e2e.html");

/// Runs body reads on the scheme handler's I/O thread, using real ArkWeb streams.
struct BodyProbe {
    url: String,
    handle: ResourceHandle,
}

impl BodyProbe {
    fn start(request: ResourceRequest, handle: ResourceHandle) {
        let probe = Self {
            url: request.url(),
            handle,
        };
        let stream = match request.http_body_stream() {
            Some(stream) => stream,
            None => {
                probe.respond(Ok(Vec::new()));
                return;
            }
        };
        if stream.size() != Err(HttpBodyStreamError::NotInitialized) {
            probe.respond(Err(
                "metadata must reject queries before initialization".into()
            ));
            return;
        }
        if probe.url.contains("/chunked") {
            stream.initialize(move |result| match result {
                Ok(stream) => {
                    hilog_info!(
                        "ark-web-body: {} initialized chunked={:?} size={:?} memory={:?} eof={:?}",
                        probe.url,
                        stream.is_chunked(),
                        stream.size(),
                        stream.is_in_memory(),
                        stream.is_eof()
                    );
                    if stream.is_chunked() != Ok(true)
                        || stream.size() != Ok(None)
                        || stream.is_in_memory() != Ok(false)
                        || stream.is_eof() != Ok(false)
                    {
                        probe.respond(Err("unexpected native chunked stream metadata".into()));
                    } else if probe.url.ends_with("/chunked-reads") {
                        probe.read_chunks(stream, Vec::new(), 0);
                    } else {
                        probe.read_remaining(stream, Vec::new());
                    }
                }
                Err(error) => probe.respond(Err(error.to_string())),
            });
        } else if probe.url.ends_with("/partial") {
            probe.read_prefix(stream);
        } else if probe.url.ends_with("/blob") || probe.url.ends_with("/empty") {
            stream.initialize(move |result| match result {
                Ok(stream) => {
                    hilog_info!(
                        "ark-web-body: {} initialized size={:?} memory={:?}",
                        probe.url,
                        stream.size(),
                        stream.is_in_memory()
                    );
                    probe.read_remaining(stream, Vec::new());
                }
                Err(error) => probe.respond(Err(error.to_string())),
            });
        } else {
            // The common path needs no separate initialization or retained handle.
            probe.read_remaining(stream, Vec::new());
        }
    }

    fn read_chunks(self, stream: HttpBodyStream, mut body: Vec<u8>, index: usize) {
        const SIZES: [usize; 5] = [1, 13, 4096, 32768, 65536];
        let requested = SIZES[index % SIZES.len()];
        stream.read(requested, move |result| match result {
            Ok((stream, chunk)) => {
                body.extend_from_slice(&chunk);
                hilog_info!(
                    "ark-web-body: chunk-read index={index} requested={requested} actual={} total={} eof={:?}",
                    chunk.len(),
                    body.len(),
                    stream.is_eof()
                );
                match stream.is_eof() {
                    Ok(true) => self.respond(Ok(body)),
                    Ok(false) => self.read_chunks(stream, body, index + 1),
                    Err(error) => self.respond(Err(error.to_string())),
                }
            }
            Err(error) => self.respond(Err(error.to_string())),
        });
    }

    fn read_prefix(self, stream: HttpBodyStream) {
        stream.read(7, move |result| match result {
            Err(error) => self.respond(Err(error.to_string())),
            Ok((stream, prefix)) => {
                // A second initialize must not rewind the first seven bytes.
                stream.initialize(move |result| match result {
                    Ok(stream) => self.read_remaining(stream, prefix),
                    Err(error) => self.respond(Err(error.to_string())),
                });
            }
        });
    }

    fn read_remaining(self, stream: HttpBodyStream, mut prefix: Vec<u8>) {
        // Ownership moves into the operation. Native writes may outlive this call.
        stream.read_to_end(move |result| match result {
            Ok(body) => {
                prefix.extend_from_slice(&body);
                self.respond(Ok(prefix));
            }
            Err(error) => self.respond(Err(error.to_string())),
        });
    }

    fn respond(self, result: Result<Vec<u8>, String>) {
        let response = ArkWebResponse::new();
        response.set_mime_type("application/octet-stream");
        response.set_header("Access-Control-Allow-Origin", "*", true);
        let body = match result {
            Ok(body) => {
                response.set_status(200);
                response.set_status_text("OK");
                hilog_info!("ark-web-body: {} complete bytes={}", self.url, body.len());
                body
            }
            Err(error) => {
                response.set_status(500);
                response.set_status_text("Body stream failure");
                hilog_info!("ark-web-body: {} FAILED {error}", self.url);
                error.into_bytes()
            }
        };
        self.handle.receive_response(response);
        if !body.is_empty() {
            self.handle.receive_data(body);
        }
        self.handle.finish();
    }
}

pub(crate) fn handle_body(request: ResourceRequest, handle: ResourceHandle) {
    BodyProbe::start(request, handle);
}
