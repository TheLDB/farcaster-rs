use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeletedLikeRoot {
    pub result: DeletedLikeResult
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeletedLikeResult {
    pub success: bool
}
