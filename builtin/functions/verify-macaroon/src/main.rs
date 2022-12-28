use macaroon::{Macaroon, MacaroonKey, Verifier};

use asml_core::*;

use secretsmanager;
use secretsmanager::structs::*;

#[handler]
async fn main() {
    let event: serde_json::Value =
        serde_json::from_str(&ctx.input).expect("could not parse function input as JSON");

    let identity = event["identitySource"].as_array().unwrap()[0]
        .as_str()
        .unwrap()
        .to_string();
    let macaroon = match Macaroon::deserialize(&identity) {
        Ok(m) => m,
        Err(err) => {
            FunctionContext::log(err.to_string());
            return FunctionContext::success("{\"isAuthorized\":false}".to_string());
        }
    };
    let verifier = Verifier::default();
    let user_key = get_user_key(
        std::str::from_utf8(macaroon.identifier().0.as_slice())
            .unwrap()
            .to_string(),
    )
    .await
    .unwrap();

    match verifier.verify(
        &macaroon,
        &MacaroonKey::generate(user_key.as_str().as_bytes()),
        vec![],
    ) {
        Ok(_) => FunctionContext::success("{\"isAuthorized\":true}".to_string()),
        Err(_) => FunctionContext::success("{\"isAuthorized\":false}".to_string()),
    }
}

async fn get_user_key(user_id: String) -> Result<String, String> {
    let mut get_secret_req = GetSecretValueRequest::default();
    get_secret_req.secret_id = format!("echopod/user/{}", &user_id);
    match secretsmanager::get_secret_value(get_secret_req).await {
        Ok(res) => Ok(res.secret_string.unwrap()),
        Err(err) => Err(err.to_string()),
    }
}
