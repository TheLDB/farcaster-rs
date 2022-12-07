use crate::constants::merkle::API_ROOT;
use crate::types::user::user::UserRoot;
use crate::Farcaster;
use std::error::Error;

impl Farcaster {
    /// Get information about the authenticated user
    pub async fn get_me(&self) -> Result<UserRoot, Box<dyn Error>> {
        let me_reqwest = &self
            .reqwest_get(format!("{}/v2/me", API_ROOT).as_str())
            .await?;

        let me: UserRoot = serde_json::from_str(&me_reqwest)?;

        Ok(me)
    }
}
