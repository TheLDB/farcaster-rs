use std::error::Error;
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;
use crate::types::reactions::recasters::RecastersRoot;

impl Farcaster {
    /// Get recasters by cast hash
    ///
    /// # Params
    /// cast_hash: &str,
    /// limit: Option<i32> - Optional # of recasters to get (max 100)
    /// cursor: Option<&str) - Optional cursor for pagination
    ///
    /// # Example
    /// ```no_run
    /// let recasters = farcaster.get_recasters.by_cast_hash("cast hash", None, None).await?;
    /// ```
    pub async fn get_recasters_by_cast_hash(&self, cast_hash: &str, limit: Option<i32>, cursor: Option<&str>) -> Result<RecastersRoot, Box<dyn Error>> {
        let mut url = format!("{}/v2/cast-recasters?castHash={}", API_ROOT, cast_hash);

        if limit.is_some() {
            url.push_str(format!("&limit={}", limit.unwrap()).as_str())
        }

        if cursor.is_some() {
            url.push_str(format!("&cursor={}", cursor.unwrap()).as_str())
        }

        let recasters_reqwest = reqwest::Client::new()
            .delete(url)
            .header("Content-Type", "application/json ")
            .header("Authorization", &self.account.session_token.as_ref().unwrap().secret)
            .send()
            .await?
            .text()
            .await?;

        let recasters: RecastersRoot = serde_json::from_str(&recasters_reqwest)?;

        Ok(recasters)
    }
}