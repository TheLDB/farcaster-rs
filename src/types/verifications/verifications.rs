use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationsRoot {
    pub result: VerificationsResult
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationsResult {
    pub verifications: Vec<Verification>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Verification {
    pub fid: i64,
    pub address: String,
    pub timestamp: i64
}