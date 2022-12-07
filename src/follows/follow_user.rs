use crate::constants::merkle::API_ROOT;
use crate::types::follows::follow_user::FollowUserRoot;
use crate::Farcaster;
use serde_json::{json, Value};
use std::error::Error;

impl Farcaster {
    /// Get a users followers via their FID
    ///
    /// # Params
    /// fid: i64
    ///
    /// # Example
    /// ```no_run
    /// let follow = farcaster.follow_user_by_fid(0).await?;
    /// ```
    pub async fn follow_user_by_fid(&self, fid: i64) -> Result<FollowUserRoot, Box<dyn Error>> {
        let payload: Value = json!({ "targetFid": fid });

        let follow_reqwest = reqwest::Client::new()
            .put(format!("{}/v2/follows", API_ROOT))
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

        let follow: FollowUserRoot = serde_json::from_str(&follow_reqwest)?;

        Ok(follow)
    }

    /// Get a users followers via their username
    ///
    /// # Params
    /// username: &str
    ///
    /// # Example
    /// ```no_run
    /// let follow = farcaster.follow_user_by_username("abc").await?;
    /// ```
    pub async fn follow_user_by_username(
        &self,
        username: &str,
    ) -> Result<FollowUserRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_username(username).await?;

        let follow = &self.follow_user_by_fid(fid.fid).await?;

        Ok(follow.clone())
    }

    /// Get a users followers via their address
    ///
    /// # Params
    /// address: &str
    ///
    /// # Example
    /// ```no_run
    /// let follow = farcaster.follow_user_by_address("0x000....").await?;
    /// ```
    pub async fn follow_user_by_address(
        &self,
        address: &str,
    ) -> Result<FollowUserRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_address(address).await?;

        let follow = &self.follow_user_by_fid(fid.fid).await?;

        Ok(follow.clone())
    }
}
