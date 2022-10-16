AssemblyLift WASM ABI
---------------------

AssemblyLift provides its own ABI to WebAssembly modules to complement standard ABIs like WASI. AssemblyLift's ABI provides
specific functions for interacting with the Function runtime (e.g. responding with a 'success' message to Lambda), as 
well as functions for interacting with IOmods. 

AssemblyLift Function modules also use the ABI to receive function input, rather than relying on _stdin_ from WASI. 
This was originally because _stdin_ wasn't available, but remains in favour of Asml's static buffers 
(see [core-buffers](core-buffers.md)).

AssemblyLift's WASI implementation does **not** include sockets (`accept`/`read`/`send` etc). AssemblyLift's policy for 
the time being is to keep net access restricted to higher-level APIs via IOmods.

```rust
// IO
fn __asml_abi_io_invoke(name_ptr: *const u8, name_len: usize, input_ptr: *const u8, input_len: usize) -> i32;
fn __asml_abi_io_poll(id: u32) -> i32;
fn __asml_abi_io_len(id: u32) -> u32;
fn __asml_abi_io_load(id: u32) -> i32;
fn __asml_abi_io_next() -> i32;

// System clock
fn __asml_abi_clock_time_get() -> u64;

// Runtime
fn __asml_abi_runtime_log(ptr: *const u8, len: usize);
fn __asml_abi_runtime_success(ptr: *const u8, len: usize);

// Function Input
fn __asml_abi_input_start() -> i32;
fn __asml_abi_input_next() -> i32;
fn __asml_abi_input_length_get() -> u64;
```
> The `io` group of functions are used to poll for and read responses from IOmod calls.
> The system clock is not really needed anymore; it exists because AssemblyLit predates WASI :)

