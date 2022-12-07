use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeletedRecastRoot {
    pub result: DeletedRecastResult
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeletedRecastResult {
    pub success: bool
}