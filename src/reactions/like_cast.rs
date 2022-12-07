use std::error::Error;
use serde_json::{json, Value};
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;
use crate::types::reactions::liked_cast::LikedCastRoot;

impl Farcaster {
    /// Like cast by the cast hash
    ///
    /// # Params
    /// cast_hash: &str
    ///
    /// # Example
    /// ```no_run
    /// farcaster.like_cast_by_cast_hash("cast hash").await?;
    /// ```
    pub async fn like_cast_by_cast_hash(&self, cast_hash: &str) -> Result<LikedCastRoot, Box<dyn Error>> {
        let payload: Value = json!({
            "castHash": cast_hash
        });

        let liked_cast_reqwest = reqwest::Client::new()
            .put(format!("{}/v2/cast-likes", API_ROOT))
            .header("Content-Type", "application/json")
            .header("Authorization", &self.account.session_token.as_ref().unwrap().secret)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let liked_cast: LikedCastRoot = serde_json::from_str(&liked_cast_reqwest)?;

        Ok(liked_cast)
    }
}