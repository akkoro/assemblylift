AssemblyLift WASM IO Buffers
----------------------------

AssemblyLift WASM guests and their host pass data thru linear buffers defined in each guest. There are [two buffers](../core/src/buffers.rs), 
the Function Input Buffer which is exactly what it sounds like, and the IO Buffer which contains responses to IOmod calls.

Each buffer implements `PagedWasmBuffer`, which is designed to wrap the `start`/`next`/`length` ABI calls corresponding 
to each buffer.

Buffer paging is implemented to allow a guest-side buffer to be reasonably small, while allowing the backing buffer on 
the host side to be arbitrarily (in theory) large. The maximum request payload size for AWS Lambda for example is 10MB, 
but we don't want to keep a 10MB static buffer in the guest. We _also_ don't have the luxury of a dynamically allocated 
buffer for this purpose. Thus, paging! 

The Function Input Buffer has three ABI functions for "load first page", "next page", and "input length" operations. 
The IO Buffer is similar, however it allows swapping _between_ buffers with a `load` function which both sets the buffer 
index (by IOID) and loads its first page.

The WASM guest must export the following functions which must return a pointer to each buffer to the host:
```rust
fn __asml_guest_get_io_buffer_pointer() -> *const u8;
fn __asml_guest_get_function_input_buffer_pointer() -> *const u8;
```

How this is accomplished is language-dependant. Rust requires each guest to pull in a crate which will provide definitions. 
For Ruby these calls are embedded in the interpreter.

