use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfollowUserRoot {
    pub result: UnfollowUserResult
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfollowUserResult {
    pub success: bool
}