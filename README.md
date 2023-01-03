<p align="center">
  <img width="600" src="./docs/AssemblyLift_logo.png"/>
  <br/>
  <br/>
  <img src="https://github.com/akkoro/assemblylift/workflows/AssemblyLift%20CI/badge.svg"/>
  <img src="https://img.shields.io/crates/v/assemblylift-cli"/>
  <img src="https://img.shields.io/discord/901946190667595786"/>
  <br/>
  AssemblyLift is an open source platform that gives you a fast & ergonomic way to build cloud-native services on a predictable, 
secure, and portable runtime &mdash; from the edge to the cloud &mdash; powered by WebAssembly.
  <br/>
  <br/>
  <a href="/examples"><b>EXAMPLES</b></a> | <a href="https://dev.to/akkoro"><b>BLOG</b></a> | <a href="/docs"><b>DESIGN DOCS</b></a>
</p>

-----

# The What and Why of AssemblyLift

Microservice architecture & containers bring a number of benefits including greater scalability, high fault-tolerance, 
and faster deployment. 'Serverless' microservices and functions offer scale-to-zero and per-millisecond billing, with 
implied economic benefits, without having to manage any kind of server. [An explosion of managed cloud services & APIs](https://landscape.cncf.io/) 
give developers more options than ever to delegate to a third party.

The problem with all of this is that there is now a large number of _things_ &mdash; microservices, functions, data services & APIs, etc. 
&mdash; which all need to communicate and move data across all kinds of boundaries. The high surface area this creates becomes 
visible when you use infrastructure-as-code; it can take a large number of virtual cloud resources just to deploy a "simple"
microservice or FaaS function in a typical production environment.

>**AssemblyLift wants to take the complexity out of building cloud-native applications.**

AssemblyLift provides a batteries-included framework for building function-oriented microservices, portable across vendors 
and services like AWS and Kubernetes.

An AssemblyLift project is composed of _services_, which are in turn composed of _functions_. A function is code invoked by 
some event, like an HTTP request. AssemblyLift functions are compiled [WebAssembly](https://webassembly.org/) modules, which
right now (as of v0.4) can be written in either the [Rust](https://rust-lang.org) or [Ruby](https://ruby-lang.org) programming 
languages. Services are declared in simple [TOML](https://toml.io) documents called _manifests_. The AssemblyLift CLI `asml` 
automates the process of compiling and/or packaging functions, generating Terraform HCL and/or Kubernetes YAML derived from 
your manifests, and deploying images and code to the selected provider(s).

AssemblyLift leverages Wasm's deny-by-default, sandboxed environment to provide capability-based access to external services.
The AssemblyLift runtime augments the [Wasmtime](https://wasmtime.dev) runtime used internally with an ABI controlling access to 
[IO Modules](https://docs.assemblylift.akkoro.io/learn-assemblylift/io-modules), which are accessed using an RPC protocol.

# Quick Start

AssemblyLift is evolving quickly! See [Releases](https://github.com/akkoro/assemblylift/releases) to find the latest 
version & release details.

Download the latest `asml` CLI for your system.
```bash
curl -O public.assemblylift.akkoro.io/cli/<version>/<triple>/asml
chmod +x asml
```
Where `<version>` is the latest version (e.g. `0.4.0-alpha.11`) and `<triple>` is one of:
 - `x86_64-linux-gnu`
 - `x86_64-apple-darwin`

Alternatively you can install with Cargo using `cargo install assemblylift-cli`.

Create a new project with `asml init -n my-project -l <lang>` where `<lang>` is one of `rust` or `ruby`; this is the language 
of the default function that is generated for you in the default service `my-service`. 

You can add a new function to any service with `asml make function <service-name>.<function-name> -l <lang>`. 
Add a new service with `asml make service <service-name>`.

The process for generating an AssemblyLift deployment is called _casting_, and is invoked by running `asml cast` in the 
project root directory. All deployment artifacts & plans are serialized to the `net` directory. To deploy your project, 
run `asml bind`.

# Learn More

Please see the [official documentation](https://docs.assemblylift.akkoro.io) for help with installing & learning to use 
AssemblyLift, and/or for more information about the project.
You can also find more in-depth tutorials on the [Akkoro Dev blog](https://dev.to/akkoro).

# Contributing

If you would like to contribute to the AssemblyLift project, please see [CONTRIBUTING.md for details](CONTRIBUTING.md) 
on how to get started! If you want to report a bug, please [open an issue](https://github.com/akkoro/assemblylift/issues/new?labels=bug).

# Contact & Social

For help with using AssemblyLift or to keep up with announcements, join us on [Discord](https://discord.gg/pVSCqYgra3)!
If you prefer we're also on [Matrix](https://matrix.org). Try the [Element client](https://element.io/) and find us at [`#assemblylift:matrix.org`](https://app.element.io/#/room/#assemblylift:matrix.org).

You can follow [@akkorocorp](https://twitter.com/akkorocorp/) on Twitter.

# License

The AssemblyLift source code is licensed under [Hippocratic License 2.1](/LICENSE.md).  
The AssemblyLift CLI delegates some tasks to [HashiCorp Terraform](https://terraform.io), which is licensed under [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).

-----

AssemblyLift is made in [Canada's far north-east](https://en.wikipedia.org/wiki/Newfoundland_and_Labrador) 🇨🇦 and is made possible by 
our contributors and supporters around the world 🌐
