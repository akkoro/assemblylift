Hyper Runtime
-------------

The Hyper runtime is meant to run inside a "generic" environment -- nonspecific to any particular vendor. 
As indicated by its name, it's based on the [Hyper HTTP crate](https://crates.io/crates/hyper) for the Rust programming 
language.

The runtime uses [`crossbeam`](https://crates.io/crates/crossbeam-utils) to spawn two threads, for an HTTP server and a 
WASM module runner. The server listens for traffic on port `5543` and forwards HTTP requests to guests using the shape:
```rust
struct LauncherRequest {
    method: String,
    headers: BTreeMap<String, String>,
    body_encoding: String,
    body: Option<String>,
}
```
where `body_encoding` is currently always `base64` (but probably shouldn't be :)).

The response from the guest via `success` is returned as the body of an HTTP 200 response. A guest error is returned as 
an HTTP 500.

The runtime requires the `ASML_WASM_MODULE_NAME` environment variable to be set to the filename of the module; the module 
is expected to be in the `/opt/assemblylift` directory (i.e. `/opt/assemblylift/$ASML_WASM_MODULE_NAME`).
