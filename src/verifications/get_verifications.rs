use std::error::Error;
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;
use crate::types::verifications::verifications::VerificationsRoot;

impl Farcaster {
    /// Get verifications of a user by their FID
    pub async fn get_verifications_by_fid(&self, fid: i64) -> Result<VerificationsRoot, Box<dyn Error>> {
        let verifications_reqwest = &self.reqwest_get(format!("{}/v2/verifications?fid={}", API_ROOT, fid).as_str()).await?;

        let verifications: VerificationsRoot = serde_json::from_str(&verifications_reqwest)?;

        Ok(verifications)
    }

    /// Get verifications of a user by their username
    pub async fn get_verifications_by_username(&self, username: &str) -> Result<VerificationsRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_username(username).await?;

        let verifications = &self.get_verifications_by_fid(fid.fid).await?;

        Ok(verifications.clone())
    }

    /// Get verifications of a user by their address
    pub async fn get_verifications_by_address(&self, address: &str) -> Result<VerificationsRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_address(address).await?;

        let verifications = &self.get_verifications_by_fid(fid.fid).await?;

        Ok(verifications.clone())
    }
}