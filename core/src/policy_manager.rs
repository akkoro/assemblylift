use std::collections::BTreeMap;

use anyhow::anyhow;

pub struct PolicyManager {
    policies: BTreeMap<String, opa::wasm::Opa>,
}

impl PolicyManager {
    pub fn new() -> Self {
        Self {
            policies: Default::default(),
        }
    }

    pub fn eval(&mut self, policy_id: String, data: String, input: &str) -> anyhow::Result<bool> {
        let policy = match self.policies.get_mut(&*policy_id) {
            Some(policy) => policy,
            None => return Err(anyhow!("no policy to eval with id={}", policy_id)),
        };
        policy.set_data(&data)?;
        let eps = policy
            .entrypoints()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let entrypoint = eps.get(0).expect("no entrypoint in module");
        let result: serde_json::Value = policy.eval(entrypoint, &input)?;
        Ok(result.as_bool().unwrap())
    }

    pub fn load_policy_bundle(
        &mut self,
        policy_id: String,
        bundle_bytes: &[u8],
    ) -> anyhow::Result<Vec<String>> {
        let bundle = match opa::bundle::Bundle::from_bytes(bundle_bytes) {
            Ok(bundle) => bundle,
            Err(_) => return Err(anyhow!("invalid bundle")),
        };
        let policy = opa::wasm::Opa::new().build_from_bundle(&bundle).unwrap();
        let entrypoints = policy
            .entrypoints()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        self.policies.insert(policy_id, policy);
        Ok(entrypoints)
    }
}
