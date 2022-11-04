use crate::types::user::user::Root;
use crate::Farcaster;
use std::error::Error;

impl Farcaster {
    /// get the details of a user for a given `username`
    pub async fn get_user_by_username(&self, username: &str) -> Result<Root, Box<dyn Error>> {
        if let Some(address) = self.registry.get_address_by_username(username) {
            // TODO: this is still v1 API !
            let user = reqwest::get(format!("https://api.farcaster.xyz/v1/profiles/{}", address))
                .await?
                .text()
                .await?;
            let user: Root = serde_json::from_str(&user)?;
            return Ok(user);
        }

        Err(Box::from(format!(
            "User '{}' not found in FNR registry",
            username
        )))
    }
}
