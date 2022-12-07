use std::error::Error;
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;
use crate::types::user::custody_address::CustodyAddressRoot;

impl Farcaster {
    /// Get a custody address by FID
    pub async fn get_custody_address_by_fid(&self, fid: i64) -> Result<CustodyAddressRoot, Box<dyn Error>> {
        let url = format!("{}/v2/custody-address?fid={}", API_ROOT, fid);

        let custody_address_reqwest = &self.reqwest_get(&url).await?;

        let custody_address: CustodyAddressRoot = serde_json::from_str(&custody_address_reqwest)?;

        Ok(custody_address)
    }

    /// Get a custody address by FID
    pub async fn get_custody_address_by_username(&self, username: &str) -> Result<CustodyAddressRoot, Box<dyn Error>> {
        let url = format!("{}/v2/custody-address?fname={}", API_ROOT, username);

        let custody_address_reqwest = &self.reqwest_get(&url).await?;

        let custody_address: CustodyAddressRoot = serde_json::from_str(&custody_address_reqwest)?;

        Ok(custody_address)
    }

    /// Get a custody address by FID
    pub async fn get_custody_address_by_address(&self, address: &str) -> Result<CustodyAddressRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_address(address).await?;

        let url = format!("{}/v2/custody-address?fid={}", API_ROOT, fid.fid);

        let custody_address_reqwest = &self.reqwest_get(&url).await?;

        let custody_address: CustodyAddressRoot = serde_json::from_str(&custody_address_reqwest)?;

        Ok(custody_address)
    }
}