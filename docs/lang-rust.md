Rust
----

The Rust programming language has first-class support for WebAssembly as a build target. AssemblyLift Functions written 
in Rust are compiled using the `wasm32-wasi` target.

Rust language guests must import the crates `assemblylift-core-guest` and `assemblylift-core-io-guest`.
