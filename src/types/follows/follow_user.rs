use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowUserRoot {
    pub result: FollowUserResult
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowUserResult {
    pub success: bool
}