AssemblyLift Design Docs
========================
> This documentation is related to developing AssemblyLift. For documentation related to using AssemblyLift in practice, 
> see [the user docs](https://docs.assemblylift.akkoro.io).

The Design Docs are intended to capture details and explanations of various concepts within the AssemblyLift source code.

## CLI
 * [Project Transpiler & Infra. Providers](cli-transpiler.md)

## Function Runtimes
 * [AWS Lambda](rt-lambda.md)
 * [Hyper](rt-hyper.md)

## Function Languages
 * [Rust](lang-rust.md)
 * [Ruby](lang-ruby.md)

## IO Modules [TODO]

## Providers [TODO]
 * API
   * [Amazon API Gateway](provider-apigw-gloo.md)
   * [Gloo API Gateway](provider-apigw-amz.md)
 * DNS
   * [Amazon Route53](provider-dns-route53.md)
 * Service
   * [AWS Lambda](provider-service-aws.md)
   * [Kubernetes](provider-service-k8s.md)

## WebAssembly Core
 * [AssemblyLift ABI](core-abi.md)
 * [Function Buffers](core-buffers.md)
 * [Threader](core-threader.md)
