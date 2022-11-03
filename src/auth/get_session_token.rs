use serde_json::{json, Value};

use crate::{
    constants::api_root::API_ROOT,
    types::auth::bearer::Bearer,
    types::auth::secret::{Secret, SecretToken},
    Farcaster,
};

impl Farcaster {
    pub async fn get_session_token(
        bearer: &Bearer,
    ) -> Result<SecretToken, Box<dyn std::error::Error>> {
        let payload: Value = json!(bearer.payload);

        let session_reqwest: String = reqwest::Client::new()
            .put(format!("{}/v2/auth", API_ROOT))
            .header("Content-Type", "application/json")
            .header("Authorization", &bearer.bearer)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let secret: Secret = serde_json::from_str(&session_reqwest)?;

        Ok(secret.result.token)
    }
}
