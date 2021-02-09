<p align="center">
  <img width="600" src="./docs/AssemblyLift_logo_with_text.png">
</p>

![AssemblyLift CI](https://github.com/akkoro/assemblylift/workflows/AssemblyLift%20CI/badge.svg)
![Crates.io](https://img.shields.io/crates/v/assemblylift-cli)

AssemblyLift is a framework for building serverless applications powered by WebAssembly (WASM).

Highlight reel:

- ["IO Modules"](https://dev.to/dotxlem/assemblylift-v0-2-preview-rpc-based-io-modules-2d38) provide a plugin interface for both the host and WASM guest,
  allowing guests to **safely** make calls to the outside world without needing elevated access.
- IOmods are implemented on top of [Cap'n Proto RPC](https://capnproto.org), and guests written using Rust fully support **async/await**.
- Currently focusing on support for guests written in **Rust**, but other languages targeting WASM are possible. PR's welcome!
- Planned support for multiple backends, but the focus is currently on [AWS Lambda](https://aws.amazon.com/lambda/)
- Built using the [Wasmer](https://wasmer.io) interpreter

**Examples** can be found [here](https://github.com/akkoro/assemblylift-examples).

# Overview

The three primary aims of this project, are to provide you with an _ergonomic_ development framework for building serverless applications
which are both _efficient_, and _safe_.

TODO 🚧


# Contributing

I'd like to figure this part out collaboratively. Just in terms of getting code merged though,
I'm a big fan of [forking workflow](https://www.atlassian.com/git/tutorials/comparing-workflows/forking-workflow),
so let's start there 🙂.

# License

The AssemblyLift source code is licensed under [Hippocratic License 2.1](/LICENSE.md).  
The AssemblyLift CLI delegates some tasks to [HashiCorp Terraform](https://terraform.io), which is licensed under [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).
