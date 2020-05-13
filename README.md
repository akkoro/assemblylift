<p align="center">
  <img width="600" src="./docs/AssemblyLift_logo_with_text.png">
</p>

AssemblyLift is a framework for building serverless applications with WebAssembly (WASM).  

Highlight reel:  
 * ["IO Modules"](backends/aws-lambda/iomod) (eventually to be shipped as packages/plugins) provide an interface for both the host and WASM guest, 
    allowing guests to __safely__ make calls to the outside world without needing elevated access.
 * Currently focusing on support for guests written in __Rust__, but other languages targeting WASM are possible. PR's welcome!
 * IO mods are __asynchronous__ (using [tokio](https://github.com/tokio-rs/tokio)), and guests written using Rust fully support __async/await__.
 * Planned support for multiple backends, but the focus is currently on _AWS Lambda_
 * Built using the [Wasmer](https://wasmer.io) interpreter

__Examples__ can be found [here](/examples).  

# Overview
The three primary aims of this project, are to provide you with an _ergonomic_ development framework for building serverless applications 
which are both _efficient_, and _safe_.

## Efficiency
WebAssembly modules [are smaller and faster](https://medium.com/@OPTASY.com/webassembly-vs-javascript-is-wasm-faster-than-js-when-does-javascript-perform-better-db86d2ecf2cc) 
than their NodeJS counterparts. Combined with the IOmod framework, most of the heavy lifting (such as a call to an AWS 
service) is handled by the host runtime (which is native code, written in Rust).

## Safety
WebAssembly modules are isolated -- they are sandboxed with their own memory, and have no access to the outside world 
(such as by opening a socket connection). This allows your serverless guest code to be _untrusted_.  

A side-effect of this with respect to an IOmod, is that the guest code has to ask the host to execute 
any third-party dependency code which needs network access. Ideally this will help you prevent unwanted version changes that
have a habit of sneaking into function code, keeping your entire project in sync and giving you tighter control over
your dependency supply chain.

## Ergonomics
It's still early days, so there's nothing in this repo right now which I would characterize as ergonomic. In terms of 
plans in this area, I intend for the tooling to abstract away as much of the underlying backend as possible (ie AWS vs Azure).

# Roadmap

## 0.1
 [ ] Build system incl. Docker [#2](https://github.com/akkoro/assemblylift/issues/2)  
 [ ] Unit tests [#3](https://github.com/akkoro/assemblylift/issues/3)    
 [ ] Start the CLI [#4](https://github.com/akkoro/assemblylift/issues/4)  
 [ ] Set up a CI/CD pipeline  
 [ ] Handle more of those `unwrap`s  
 [ ] Clean up warnings  

## 0.2
 [ ] Async memory manager  
 [ ] Macros  
 [ ] Project init via CLI  
 [ ] More examples  


# Contributing

I'd like to figure this part out collaboratively. Just in terms of getting code merged though,
 I'm a big fan of [forking workflow](https://www.atlassian.com/git/tutorials/comparing-workflows/forking-workflow), 
 so let's start there 🙂.

# License
[Apache 2](/LICENSE)