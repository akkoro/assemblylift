use quote::{quote, quote_spanned};
use syn::{parse2, ItemFn};

#[proc_macro_attribute]
pub fn handler(
    _args: proc_macro::TokenStream,
    stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: ItemFn = parse2(stream.into()).expect("could not parse token stream");
    let block_statements = &input.block.stmts;
    let name = &input.sig.ident;
    let ret = &input.sig.output;

    if name != "main" {
        return proc_macro::TokenStream::from(quote_spanned! { name.span() =>
            compile_error!("only the main function can be tagged with #[handler]"),
        });
    }

    proc_macro::TokenStream::from(quote! {
        use assemblylift_core_io_guest;
        use direct_executor;
        use serde_json;
        pub fn main() #ret {
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
