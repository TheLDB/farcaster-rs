use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecastedRoot {
    pub result: RecastedResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecastedResult {
    #[serde(rename = "castHash")]
    cast_hash: String,
}
