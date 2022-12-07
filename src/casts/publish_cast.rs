use crate::constants::merkle::API_ROOT;
use crate::types::casts::published_cast::PublishedCast;
use crate::Farcaster;
use chrono::offset::Utc;

use serde_json::{json, Value};

impl Farcaster {
    pub async fn publish_cast(
        &self,
        content: &str,
        reply_to_hash: Option<&str>,
        reply_to_fid: Option<i64>
    ) -> Result<PublishedCast, Box<dyn std::error::Error>> {
        let payload: Value;

        if reply_to_hash.is_some() && reply_to_fid.is_some() {
            let parent_hash = reply_to_hash.unwrap();
            let parent_fid = reply_to_fid.unwrap();

            payload = json!({
                "parent": {
                    "hash": parent_hash,
                    "fid": parent_fid
                },
                "text": content
            })
        }
        else {
            payload = json!({
                "text": content
            })
        }

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
