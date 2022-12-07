use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Secret {
    pub result: SecretResult,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecretResult {
    pub token: SecretToken,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecretToken {
    pub secret: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: i64,
}
