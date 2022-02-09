use quote::quote;
use syn::{ItemFn, parse2};

#[proc_macro_attribute]
pub fn handler(_args: proc_macro::TokenStream, stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse2(stream.into())
        .expect("could not parse token stream");
    let ItemFn { attrs, vis, sig, block } = input;
    let block_statements = &block.stmts;

    proc_macro::TokenStream::from(quote! {
        use assemblylift_core_io_guest;
        use direct_executor;
        use serde_json;
        #(#attrs)* #vis #sig {
            use std::io::Read;
            let mut fib = std::io::BufReader::new(assemblylift_core_io_guest::FunctionInputBuffer::new());
            let mut input = String::new();
            fib.read_to_string(&mut input).expect("could not read FIB to String");
            let ctx = FunctionContext { input: input.trim_matches(char::from(0)).to_string() };
            direct_executor::run_spinning(async {
                #(#block_statements)*
            });
        }
    })
}
