use crate::{Farcaster, types::casts::Root};

impl Farcaster {
    pub async fn get_casts(&self, username: String) -> Result<Root, Box<dyn std::error::Error>> {
        let user = Farcaster::get_user_by_username(self, username).await.unwrap();
        let address = user.result.user.address;

        let user = reqwest::get(format!("https://api.farcaster.xyz/v1/profiles/{}/casts", address)).await?.text().await?;
        let casts: Root = serde_json::from_str(&user).unwrap();
        Ok(casts)
    }
}