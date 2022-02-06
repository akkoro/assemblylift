use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, parse2};

#[proc_macro_attribute]
pub fn handler(args: proc_macro::TokenStream, stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse2(stream.into())
        .expect("could not parse token stream");
    let ItemFn { attrs, vis, sig, block } = input;
    let block_statements = &block.stmts;

    proc_macro::TokenStream::from(quote! {
        use assemblylift_core_io_guest;
        use direct_executor;
        use serde_json;
        #(#attrs)* #vis #sig {
            let mut fib = std::io::BufReader::new(assemblylift_core_io_guest::FunctionInputBuffer::new());
            let input: FunctionInput = match serde_json::from_reader(fib) {
                Ok(event) => event,
                Err(why) => {
                    // TODO log error
                    return;
                }
            };
            let ctx = FunctionContext { input };
            direct_executor::run_spinning(async {
                #(#block_statements)*
            });
        }
    })
}
