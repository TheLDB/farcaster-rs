use serde_json::{json, Value};

use crate::{constants::merkle::API_ROOT, types::account::auth::secret::SecretToken, Farcaster};
use chrono::Utc;
use std::error::Error;

impl Farcaster {
    pub async fn revoke_session_token(token: &SecretToken) -> Result<(), Box<dyn Error>> {
        let payload: Value = json!({
            "method": "revokeToken",
            "params": {
                "timestamp": Utc::now().timestamp_millis()
            }
        });

        let _revoke_reqwest = reqwest::Client::new()
            .delete(format!("{}/v2/auth", API_ROOT))
            .header("Content-Type", "application/json")
            .header("Authorization", &token.secret)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        println!("{:#?}", _revoke_reqwest);

        // API not yet implemented- refer back when structure is available

        Ok(())
    }
}
