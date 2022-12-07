use crate::constants::merkle::API_ROOT;
use crate::types::assets::user_collections::UserCollectionsRoot;
use crate::Farcaster;
use std::error::Error;

impl Farcaster {
    /// Get all collections an FID owns
    /// ```no_run
    /// use farcaster_rs::{Account, Farcaster};
    ///
    /// let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
    /// let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
    ///
    /// // Returns all Farcaster collection owners of the ID specified
    /// let collections = farcaster.get_collections_by_fid(5, None, None).await?;
    /// ```
    pub async fn get_collections_by_fid(
        &self,
        fid: i64,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<UserCollectionsRoot, Box<dyn Error>> {
        let mut url = format!("{}/v2/user-collections?ownerFid={}", API_ROOT, fid);

        if limit.is_some() {
            url.push_str(format!("&limit={}", limit.unwrap()).as_str())
        }

        if cursor.is_some() {
            url.push_str(format!("&cursor={}", cursor.unwrap()).as_str())
        }

        let collections_reqwest = &self.reqwest_get(&url).await?;

        let collections: UserCollectionsRoot = serde_json::from_str(&collections_reqwest)?;

        Ok(collections)
    }

    /// Get all collections a username owns
    /// ```no_run
    /// use farcaster_rs::{Account, Farcaster};
    ///
    /// let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
    /// let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
    ///
    /// // Returns all Farcaster collection owners of the ID specified
    /// let collections = farcaster.get_collections_by_username("ace", None, None).await?;
    /// ```
    pub async fn get_collections_by_username(
        &self,
        username: &str,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<UserCollectionsRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_username(username).await?;

        let collection = &self.get_collections_by_fid(fid.fid, limit, cursor).await?;

        Ok(collection.clone())
    }

    /// Get all collections an address owns
    /// ```no_run
    /// use farcaster_rs::{Account, Farcaster};
    ///
    /// let account: Account = Account::from_mnemonic("mnemonic phrase", None).await?;
    /// let farcaster: Farcaster = Farcaster::new("eth provider", account).await?;
    ///
    /// // Returns all Farcaster collection owners of the ID specified
    /// let collections = farcaster.get_collections_by_address("0x000.....", None, None).await?;
    /// ```
    pub async fn get_collections_by_address(
        &self,
        address: &str,
        limit: Option<i64>,
        cursor: Option<String>,
    ) -> Result<UserCollectionsRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_address(address).await?;

        let collection = &self.get_collections_by_fid(fid.fid, limit, cursor).await?;

        Ok(collection.clone())
    }
}
