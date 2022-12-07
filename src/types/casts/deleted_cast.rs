use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeletedCastRoot {
    pub result: DeletedCastResult
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeletedCastResult {
    pub success: bool
}