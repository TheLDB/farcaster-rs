use std::error::Error;
use serde_json::{json, Value};
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;
use crate::types::reactions::deleted_recast::DeletedRecastRoot;

impl Farcaster {
    pub async fn delete_recast_by_cast_hash(&self, cast_hash: &str) -> Result<DeletedRecastRoot, Box<dyn Error>> {
        let payload: Value = json!({
            "castHash": cast_hash
        });

        let delete_recast_reqwest = reqwest::Client::new()
            .delete(format!("{}/v2/recasts", API_ROOT))
            .header("Content-Type", "application/json")
            .header("Authorization", &self.account.session_token.as_ref().unwrap().secret)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;

        let deleted_recast: DeletedRecastRoot = serde_json::from_str(&delete_recast_reqwest)?;

        Ok(deleted_recast)
    }
}