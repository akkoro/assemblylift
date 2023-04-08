use std::collections::BTreeMap;

use anyhow::anyhow;
use serde::{Serialize, de::DeserializeOwned};
use tracing::debug;

pub struct Cache {
    data: BTreeMap<String, Vec<u8>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }

    pub fn put<T: Serialize>(&mut self, key: &str, object: &T) -> anyhow::Result<()> {
        debug!("storing cache object @ {}", key);
        let encoded: Vec<u8> = bincode::serialize(object)
            .map_err(|e| anyhow!(e.to_string()))?;
        self.data.insert(key.to_string(), encoded);
        Ok(())
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> anyhow::Result<Option<T>> {
        let encoded = match self.data.get(key) {
            Some(e) => {
                debug!("cache HIT getting object @ {}", key);
                e
            },
            None => {
                debug!("cache MISS getting object @ {}", key);
                return Ok(None)
            },
        };
        let decoded: T = bincode::deserialize(&encoded)
            .map_err(|e| anyhow!(e.to_string()))?;
        Ok(Some(decoded))
    }
}
