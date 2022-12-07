use std::error::Error;
use serde_json::{json, Value};
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;
use crate::types::reactions::recasted::RecastedRoot;

impl Farcaster {
    /// Recast a cast by its hash
    ///
    /// # Params
    /// cast_hash: &str
    ///
    /// # Example
    /// ```no_run
    /// let recast = farcaster.recast_by_cast_hash("cast hash").await?;
    /// ```
    pub async fn recast_by_cast_hash(&self, cast_hash: &str) -> Result<RecastedRoot, Box<dyn Error>> {
        let payload: Value = json!({
            "castHash": cast_hash
        });

        let recast_reqwest = reqwest::Client::new()
            .put(format!("{}/v2/recasts", API_ROOT))
            .header("Content-Type", "application/json")
            .header("Authorization", &self.account.session_token.as_ref().unwrap().secret)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let recast: RecastedRoot = serde_json::from_str(&recast_reqwest)?;

        Ok(recast)
    }
}