use serde_json::{json, Value};

use crate::{
    constants::merkle::API_ROOT,
    types::account::auth::bearer::Bearer,
    types::account::auth::secret::{Secret, SecretToken},
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

        let mut secret: Secret = serde_json::from_str(&session_reqwest)?;
        secret.result.token.secret = format!("Bearer: {}", secret.result.token.secret);

        Ok(secret.result.token)
    }
}
