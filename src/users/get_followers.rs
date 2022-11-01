use crate::{types::user::followers::Follower, Farcaster};
use std::error::Error;

impl Farcaster {
    /// fetches followers of a given Farcaster `address`
    pub async fn get_followers_by_address(
        &self,
        address: &str,
    ) -> Result<Vec<Follower>, Box<dyn Error>> {
        let fetch_url = format!("https://api.farcaster.xyz/indexer/followers/{}", address);
        let response = reqwest::get(fetch_url).await?.text().await?;
        let followers: Vec<Follower> = serde_json::from_str(&response)?;

        Ok(followers)
    }

    /// fetches followers of a given `username`
    pub async fn get_followers_by_username(
        &self,
        username: &str,
    ) -> Result<Vec<Follower>, Box<dyn Error>> {
        let user = self.get_user_by_username(&username).await?;
        self.get_followers_by_address(&user.result.user.address)
            .await
    }
}
