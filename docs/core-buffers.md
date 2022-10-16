AssemblyLift WASM IO Buffers
----------------------------

AssemblyLift WASM guests and their host pass data thru linear buffers defined in each guest. There are [two buffers](../core/src/buffers.rs), 
the Function Input Buffer which is exactly what it sounds like, and the IO Buffer which contains responses to IOmod calls.

Each buffer implements `PagedWasmBuffer`, which is designed to wrap the `start`/`next`/`length` ABI calls corresponding 
to each buffer. TODO briefly explain how paging works and why paging is used
