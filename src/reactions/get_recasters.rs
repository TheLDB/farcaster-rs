use std::error::Error;
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;
use crate::types::reactions::recasters::RecastersRoot;

impl Farcaster {
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