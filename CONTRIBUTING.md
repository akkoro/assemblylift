# Exploring the source code
AssemblyLift is written in the Rust programming language. It is made up of a number of crates, which make up the workspace
defined in [Cargo.toml](Cargo.toml) at the root.

The crate tree is as follows:
```
├── backends
│   └── aws-lambda
│       ├── guest
│       └── host
├── cli
└── core
    ├── guest
    ├── io
    │   ├── common
    │   └── guest
    └── iomod
        └── guest

```

You will notice there are several `guest` crates. These are library crates which are meant to be consumed by Rust language 
functions, ie the WebAssembly guest code. All guest code _must_ be wasm32 compatible. We do currently not have a location 
in this repository for guest libraries targeting languages other than Rust.

## Backends
Runtimes for supported infrastructure providers. Each should be made up of at least two crates, `guest` and `host`. The 
host is the runtime binary itself, deployed & executed on the chosen provider. The guest crate is a Rust library supporting 
AssemblyLift functions on the provider. 

## CLI
The AssemblyLift Command Line Interface (CLI), `asml`.

## Core
The `core` crates are libraries implementing the AssemblyLift runtime, ABI, IO Modules, etc. To belong in `core`, code 
must not be tied to any particular backend provider.

### assemblylift-core
Provides building blocks to implement a runtime for a provider.
* The `abi` module implements the AssemblyLift ABI which is exposed to the WASM guest.
* The `buffers` module implements the buffer structures needed for AssemblyLift function input and IOmod support. These 
    buffers facilitate data transfer into the WASM guest, and as such are relatively low-level (_IoBuffer_ for example is used by _Threader_).
* The `threader` module implements the "Threader" runtime, which handles IOmod call execution.

### assemblylift-core-io-{guest | common}
The `assemblylift-core-io` crate is effectively deprecated as the IO implementation is now (as of v0.2) all guest-side.

The `guest` crate provides the _Io_ struct, which is a _Future_ wrapping the AssemblyLift IOmod ABI. It also provides 
_Read_ implementations wrapping the ABI for reading buffers from the guest side.

### assemblylift-core-iomod-{guest}
The host-side crate (without the guest suffix) implements the IOmod RPC protocol, the IOmod package manifest spec, and 
exports macros to simplify IOmod development. The guest crate exports macros for defining IOmod guest-side calls.

# Proposing a change
If there's a feature you'd like to implement, please start a [discussion](https://github.com/akkoro/assemblylift/discussions) describing 
your proposal before moving forward with a PR. This helps ensure that changes stay in scope and work isn't duplicated. 🙂

Changes don't have to be limited to new features! Feel free to propose something if there is code you think could be 
refactored to be clearer, documentation you'd like to contribute, CI/CD enhancements, etc.

# Getting your change merged
Please use [forking workflow](https://www.atlassian.com/git/tutorials/comparing-workflows/forking-workflow) to open a new Pull Request.

# Reporting a bug
Please [open an issue](https://github.com/akkoro/assemblylift/issues/new?labels=bug).

# Guidelines
Remember to abide by the [Contributor Covenant](CODE_OF_CONDUCT.md).