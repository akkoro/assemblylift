IO Threader
--------

[_Threader_](../core/src/threader.rs) is responsible for handling IOmod calls from WASM modules and invoking the 
corresponding module call. As well it is responsible for tracking the status of each in-flight call, and managing the 
responses in the [IO Buffer](core-buffers.md).

Threader maintains its own [Tokio](https://crates.io/crates/tokio) async runtime, separate from the runtime which 
executes WebAssembly.

TODO IO documents, IOIDs, WasmerEnv dependency
