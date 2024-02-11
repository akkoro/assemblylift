use assemblylift_core_guest::*;

#[handler]
async fn main() {
    // `ctx` is a value injected by the `handler` attribute macro
    let event: serde_json::Value = serde_json::from_slice(&ctx.input)
        .expect("could not parse function input as JSON");

    FunctionContext::success("Function returned OK!".to_string());
}
