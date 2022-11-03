use serde_json::{json, Value};

use crate::{
    types::auth::{bearer::Bearer, secret::Secret},
    Farcaster,
};

impl Farcaster {
    pub async fn get_session_token(bearer: Bearer) -> Result<Secret, Box<dyn std::error::Error>> {
        let payload: Value = json!({
            "method": "generateToken",
            "params": {
                "timestamp": bearer.payload.params.timestamp,
                "expiresAt": bearer.payload.params.expires_at,
            }
        });

        let session_reqwest: String = reqwest::Client::new()
            .put("https://api.farcaster.xyz/v2/auth")
            .header("Content-Type", "application/json")
            .header("Authorization", bearer.bearer)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let secret: Secret = serde_json::from_str(&session_reqwest)?;

        Ok(secret)
    }
}
