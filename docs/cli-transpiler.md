The Transpiler and Providers
-----------------------------

The AssemblyLift CLI is at its core a wrapper around a TOML-to-HCL pipeline, contained in the [`transpiler`](../cli/src/transpiler) 
and [`providers`](../cli/src/providers) modules. It looks like a transpiler in the loosest sense of the word so that's what it 
got named, but _generator_ might be a better word.

When the `cast` command is invoked, a new [`Context`](../cli/src/transpiler/context.rs) is constructed from the project's 
manifests. Context is analogous to an immutable state of the project. 
[The `Context` object itself is `Castable`](../cli/src/transpiler/context.rs#L183), and serves as the entrypoint 
for casting each [`Provider`](../cli/src/providers/mod.rs#L57). The cast proceeds for each _unique_ service provider (or DNS provider), where each provider 
may operate on the entire context or require a "selector". The results (i.e. the [`Artifact`s](../cli/src/transpiler/mod.rs#L51)) from the provider `cast` are concatenated and written 
to their respective locations in the `net/` directory (e.g. `net/plan.tf`).

It's up to each provider to correctly/fully implement the functionality implied by each definition in the `Context`; there's 
currently no mechanism to verify the output (if one is even possible). It _is_ required that a `Provider` implements `Castable`, 
`Bindable`, and `Bootable`.

The providers that are currently implemented generate HCL and YAML using embedded moustache/handlebars templates 
(for which there is a trait, [`Template`](../cli/src/transpiler/mod.rs#L40)).

### The boot step
The `boot` step is currently used only by the [`k8s`](../cli/src/providers/k8s.rs) provider, but exists in general to provide a means to configure the 
target environment in some way that is prerequisite to deployment during `bind`. The [`gloo`](../cli/src/providers/gloo/mod.rs) API provider for example uses 
`boot` to install `certificate-manager` on the target cluster. 

The `boot` step is invoked for each provider prior to the provider's `bind` step.

> It should be noted that the Gloo _Gateway_ is actually installed on `cast` for the `k8s` provider, so that the 
> CRD's are available to Terraform when planning the K8s manifests. There's probably a cleaner way to do that :)

### The bind step
Each `Provider` implements `bind` -- however at the moment the only binding operation is the `terraform apply`. _Apply_
is executed independent of `Context` as the last step of the `bind` command, since there is only a singular plan file. 
It may be worth it (or necessary) to refactor this to a unique plan-per-provider!