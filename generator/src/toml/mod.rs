use serde::{Deserialize, Serialize};

use super::StringMap;

pub mod asml;
pub mod service;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Provider {
    pub name: String,
    #[serde(skip_serializing_if = "StringMap::is_empty", default)]
    pub options: StringMap<String>,
    pub platform_id: Option<String>,
}
