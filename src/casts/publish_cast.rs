use crate::constants::merkle::API_ROOT;
use crate::types::casts::published_cast::PublishedCast;
use crate::Farcaster;
use chrono::offset::Utc;

use serde_json::{json, Value};

impl Farcaster {
    pub async fn publish_cast(
        &self,
        content: &str,
    ) -> Result<PublishedCast, Box<dyn std::error::Error>> {
        let payload: Value = json!({
            "timestamp": Utc::now().timestamp_millis(),
            "text": content
        });

        let publish_cast_reqwest = reqwest::Client::new()
            .post(format!("{}/v2/casts", API_ROOT))
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                &self.account.session_token.as_ref().unwrap().secret,
            )
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let published_cast: PublishedCast = serde_json::from_str(&publish_cast_reqwest)?;

        Ok(published_cast)
    }
}
