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
    let _ret = &input.sig.output;

    if name != "main" {
        return proc_macro::TokenStream::from(quote_spanned! { name.span() =>
            compile_error!("only the main function can be tagged with #[handler]"),
        });
    }

    proc_macro::TokenStream::from(quote! {
        use assemblylift_core_guest::asml_rt;
        use assemblylift_core_guest::direct_executor;
        use assemblylift_core_guest::export_wasi_command;
        use assemblylift_core_guest::FunctionContext;
        use assemblylift_core_guest::wasi_command::*;
        use assemblylift_core_guest::wit_bindgen;
        struct Cmd;
        impl WasiCommand for Cmd {
            fn command(
                stdin: u32,
                stdout: u32,
                args: wit_bindgen::rt::vec::Vec<wit_bindgen::rt::string::String>,
                // env_vars: wit_bindgen::rt::vec::Vec<(
                //     wit_bindgen::rt::string::String,
                //     wit_bindgen::rt::string::String,
                // )>,
                // preopens: wit_bindgen::rt::vec::Vec<(
                //     u32,
                //     wit_bindgen::rt::string::String,
                // )>,
            ) -> Result<(), ()> {
                Ok(__handler(FunctionContext { input: asml_rt::get_input() }))
            }
        }
        export_wasi_command!(Cmd);
        fn __handler(ctx: FunctionContext) {
            direct_executor::run_spinning(async {
                #(#block_statements)*
            });
        }
        fn main() {
            __handler(FunctionContext { input: asml_rt::get_input() })
        }
    })
}
