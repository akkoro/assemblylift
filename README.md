<p align="center">
  <img width="600" src="./docs/AssemblyLift_logo.png"/>
  <br/>
  <br/>
  <img src="https://github.com/akkoro/assemblylift/workflows/AssemblyLift%20CI/badge.svg"/>
  <img src="https://img.shields.io/crates/v/assemblylift-cli"/>
  <img src="https://img.shields.io/discord/901946190667595786"/>
  <br/>
  AssemblyLift is an open source platform that gives you a fast & ergonomic way to build cloud-native services on a predictable, 
secure, and portable runtime -- from the edge to the cloud -- powered by WebAssembly.
  <br/>
  <br/>
</p>

-----

# The What and Why of AssemblyLift

**AssemblyLift wants to take the complexity out of building cloud-native applications.**  

Microservice architecture & containers bring a number of benefits including greater scalability, high fault-tolerance, 
and faster deployment. 'Serverless' microservices and functions offer scale-to-zero and per-millisecond billing, with 
implied economic benefits, without having to manage any kind of server. [An explosion of managed cloud services & APIs](https://landscape.cncf.io/) 
give developers more options than ever to delegate to a third party.

The problem with all of this is that there is now a large number of _things_ -- microservices, functions, data services & APIs, etc. 
-- which all need to communicate and move data across all kinds of boundaries. The high surface area this creates becomes 
visible when you use infrastructure-as-code; it can take a large number of virtual cloud resources just to deploy a "simple"
microservice or FaaS function in a typical production environment.

AssemblyLift provides an opinionated platform for building function-oriented microservices. Cloud-native architecture is occasionally [1,2]
referred to as "Lego brick" or "building block" architecture, due to the way nearly any subset of services can fit together to create 
a stack suitable for your application.

[1] [Cloud-native architecture: The era of large-scale IT building blocks](https://www.redhat.com/architect/cloud-native-building-blocks)  
[2] [Cloud Native: The Lego® Bricks to Your Business](https://www.linkedin.com/pulse/cloud-native-lego-your-business-maged-wassim/)

## What's This WebAssembly Thing?
TODO

## When Would I Use AssemblyLift?

## When Would I _Not_ Use AssemblyLift?

# Quick Start

TODO

# Learn More

Please see the [official documentation](https://docs.assemblylift.akkoro.io) for help with installing & learning to use AssemblyLift, and/or for more information about the project.
You can also find more in-depth tutorials on the [Akkoro Dev blog](https://dev.to/akkoro).

# Contributing

If you would like to contribute to the AssemblyLift project, please see [CONTRIBUTING.md for details](CONTRIBUTING.md) on how to get started!
If you want to report a bug, please [open an issue](https://github.com/akkoro/assemblylift/issues/new?labels=bug).

# Contact & Social

We're on [Matrix](https://matrix.org)! Try the [Element client](https://element.io/) and find us at [`#assemblylift:matrix.org`](https://app.element.io/#/room/#assemblylift:matrix.org) :)

Join us on [Discord](https://discord.gg/pVSCqYgra3)

# License

The AssemblyLift source code is licensed under [Hippocratic License 2.1](/LICENSE.md).  
The AssemblyLift CLI delegates some tasks to [HashiCorp Terraform](https://terraform.io), which is licensed under [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).

-----

AssemblyLift is made with love (and ice, probably) in [Canada's far north-east](https://en.wikipedia.org/wiki/Newfoundland_and_Labrador). 🇨🇦❤️❄️