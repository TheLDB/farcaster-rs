use crate::constants::merkle::API_ROOT;
use crate::types::user::user::{UserInfo, UserRoot};
use crate::Farcaster;
use std::error::Error;

impl Farcaster {
    /// Get information about a user via their Farcaster ID (more commonly shortened to FID throughout this codebase
    pub async fn get_user_by_fid(&self, fid: u64) -> Result<UserInfo, Box<dyn Error>> {
        // make sure fid exists
        if let Some(_addr) = self.registry.get_address_by_fid(fid) {
            let response = self
                .reqwest_get(&format!("{}/v2/user?fid={}", API_ROOT, fid))
                .await?;
            let user_root: UserRoot = serde_json::from_str(&response)?;

            return Ok(user_root.result.user);
        }

        Err(Box::from(format!(
            "FID '{}' not found in Farcaster ID Registry",
            fid
        )))
    }

    /// Get information about a user via their Farcaster Username
    pub async fn get_user_by_username(&self, username: &str) -> Result<UserInfo, Box<dyn Error>> {
        if let Some(fid) = self.registry.get_fid_by_username(username) {
            return self.get_user_by_fid(fid).await;
        }

        Err(Box::from(format!(
            "User '{}' not found in Farcaster Name Registry",
            username
        )))
    }

    /// Get information about a user via their Ethereum address
    pub async fn get_user_by_address(&self, address: &str) -> Result<UserInfo, Box<dyn Error>> {
        if let Some(fid) = self.registry.get_fid_by_address(address) {
            return self.get_user_by_fid(fid).await;
        }

        Err(Box::from(format!(
            "Address '{}' not found in Farcaster ID Registry",
            address
        )))
    }
}
