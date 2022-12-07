use crate::constants::merkle::API_ROOT;
use crate::types::casts::published_cast::PublishedCast;
use crate::Farcaster;

use serde_json::{json, Value};

impl Farcaster {
    /// Publish a cast to the protocol
    /// ```no_run
    /// use farcaster_rs::{Account, Farcaster};
    ///
    /// let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
    /// let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
    ///
    /// // Publish a cast.
    /// // 1st Param: Content
    /// // 2nd & 3rd Param: Parent Cast Hash & Parent User FID
    ///     // Used for replying to a cast
    /// let cast = farcaster.publish_cast("cast content", Some("optional parent hash to reply to"), Some(0)).await?;
    /// ```
    pub async fn publish_cast(
        &self,
        content: &str,
        reply_to_hash: Option<&str>,
        reply_to_fid: Option<i64>,
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
        } else {
            payload = json!({ "text": content })
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
