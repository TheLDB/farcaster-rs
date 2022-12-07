use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RevokedKeyRoot {
    pub result: RevokedKeyResult,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RevokedKeyResult {
    pub success: bool,
}
