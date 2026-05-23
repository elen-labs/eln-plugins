//! Tool 호출자 정체성 — SHOULD scope, default `Human`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Identity {
    Human,
    Agent { name: String },
    System,
}

impl Default for Identity {
    fn default() -> Self {
        Identity::Human
    }
}
