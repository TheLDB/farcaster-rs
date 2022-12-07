use std::error::Error;
use serde_json::{json, Value};
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;
use crate::types::follows::unfollow_user::UnfollowUserRoot;

impl Farcaster {
    pub async fn unfollow_user_by_fid(&self, fid: i64) -> Result<UnfollowUserRoot, Box<dyn Error>> {
        let payload: Value = json!({
            "targetFid": fid
        });

        let unfollow_reqwest = reqwest::Client::new()
            .delete(format!("{}/v2/follows", API_ROOT))
            .header("Content-Type", "application/json")
            .header("Authorization", &self.account.session_token.as_ref().unwrap().secret)
            .json(&payload)
            .send()
            .await?
            .text()
            .await?;


        let unfollow: UnfollowUserRoot = serde_json::from_str(&follow_reqwest)?;

        Ok(unfollow)
    }

    pub async fn follow_user_by_username(&self, username: &str) -> Result<UnfollowUserRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_username(username).await?;

        let unfollow = &self.unfollow_user_by_fid(fid.fid).await?;

        Ok(unfollow.clone())
    }

    pub async fn follow_user_by_address(&self, address: &str) -> Result<UnfollowUserRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_address(address).await?;

        let unfollow = &self.unfollow_user_by_fid(fid.fid).await?;

        Ok(unfollow.clone())
    }
}