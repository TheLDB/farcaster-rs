use crate::constants::merkle::API_ROOT;
use crate::types::reactions::deleted_like::DeletedLikeRoot;
use crate::Farcaster;
use serde_json::{json, Value};
use std::error::Error;

impl Farcaster {
    /// Unlike a cast
    ///
    /// # Params
    /// cast_hash: &str
    ///
    /// # Example
    /// ```no_run
    /// farcaster.delete_like_by_cast_hash("cast hash").await?;
    /// ```
    pub async fn delete_like_by_cast_hash(
        &self,
        cast_hash: &str,
    ) -> Result<DeletedLikeRoot, Box<dyn Error>> {
        let payload: Value = json!({ "castHash": cast_hash });

        let deleted_like_reqwest = reqwest::Client::new()
            .delete(format!("{}/v2/cast-likes", API_ROOT))
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

        println!("{}", deleted_like_reqwest);

        let deleted_like: DeletedLikeRoot = serde_json::from_str(&deleted_like_reqwest)?;

        Ok(deleted_like)
    }
}
