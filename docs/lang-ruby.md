Ruby
----

AssemblyLift's support for Ruby is built on a [slightly customized fork of Ruby 3.1](https://github.com/akkoro/ruby/tree/assemblylift).  
The forked build includes an extra [built-in gem called `asml`](https://github.com/akkoro/ruby/tree/assemblylift/ext/asml), 
which provides an implementation of the [AssemblyLift ABI](core-abi.md).

Ruby Functions are packaged with a prebuilt Ruby interpreter compiled to WASM, along with the contents of the Function 
source directory. The interpreter is hardcoded to load & execute a script called `handler.rb`, which may `require_relative` 
other Ruby source in the directory. If you wish to use a Gem in your Function, it must be vendored into the source directory 
with Bundler `bundle install --path=./vendor/gems` (get Bundler with `gem install bundler`). Gems **must not** depend on 
any native C code.

Ruby Functions require the `ASML_FUNCTION_ENV` environment variable to be set to either `ruby-docker` or `ruby-lambda`. 
Each will map the `src` and `usr` directories of the Ruby environment to the loaded WASM module [via WASI](../core/src/wasm.rs#L66), 
the difference being from where they are mapped.
