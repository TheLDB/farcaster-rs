use crate::constants::merkle::API_ROOT;
use crate::types::casts::deleted_cast::DeletedCastRoot;
use crate::Farcaster;
use serde_json::{json, Value};
use std::error::Error;

impl Farcaster {
    /// Delete a cast via its hash
    ///
    /// ## Params
    /// cast_hash: &str
    ///
    /// ## Example
    /// ```no_run
    /// let casts = farcaster.delete_cast_by_cast_hash("cast_hash").await?;
    /// ```
    pub async fn delete_cast_by_cast_hash(
        &self,
        cast_hash: &str,
    ) -> Result<DeletedCastRoot, Box<dyn Error>> {
        let payload: Value = json!({ "castHash": cast_hash });

        let delete_reqwest = reqwest::Client::new()
            .delete(format!("{}/v2/casts", API_ROOT))
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

        let deleted_cast: DeletedCastRoot = serde_json::from_str(&delete_reqwest)?;

        Ok(deleted_cast)
    }
}
