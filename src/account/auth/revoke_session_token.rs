use serde_json::{json, Value};

use crate::types::account::auth::revoke::RevokedKeyRoot;
use crate::{constants::merkle::API_ROOT, types::account::auth::secret::SecretToken, Farcaster};
use chrono::Utc;
use std::error::Error;

impl Farcaster {
    /// Best not to use this function on its own, as the Account struct has a method that does this in a clean way
    /// i.e. ``farcaster.account.revoke_auth_token().await?;``
    pub async fn revoke_session_token(
        token: &SecretToken,
    ) -> Result<RevokedKeyRoot, Box<dyn Error>> {
        let payload: Value = json!({
            "method": "revokeToken",
            "params": {
                "timestamp": Utc::now().timestamp_millis()
            }
        });

        let revoke_reqwest = reqwest::Client::new()
            .delete(format!("{}/v2/auth", API_ROOT))
            .header("Content-Type", "application/json")
            .header("Authorization", &token.secret)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let revoke: RevokedKeyRoot = serde_json::from_str(&revoke_reqwest)?;

        Ok(revoke)
    }
}
