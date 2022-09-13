use asml_core::*;

#[handler]
async fn main() {
    // `ctx` is a value injected by the `handler` attribute macro
    let event: serde_json::Value = serde_json::from_str(&ctx.input)
        .expect("could not parse function input as JSON");
    FunctionContext::log(format!("Received function input: {:?}", event));
    FunctionContext::success("\"Hello world!\"".to_string());
}
