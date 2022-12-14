use crate::constants::merkle::API_ROOT;
use crate::types::casts::casts::CastRoot;
use crate::types::user::user::UserInfo;
use crate::Farcaster;
use std::error::Error;

impl Farcaster {
    /// Get a users casts by their FID
    ///
    /// ## Params
    /// fid: i64
    /// limit: Option<i32>
    /// cursor: Option<&str>
    ///
    /// ## Example
    /// ```no_run
    /// let casts = farcaster.get_casts_by_fid(0, None, None).await?;
    /// ```
    pub async fn get_casts_by_fid(
        &self,
        fid: i64,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<CastRoot, Box<dyn Error>> {
        let mut url = format!("{}/v2/casts?fid={}", API_ROOT, fid);
        if limit.is_some() {
            url.push_str(format!("&limit={}", limit.unwrap()).as_str())
        }

        if cursor.is_some() {
            url.push_str(format!("&cursor={}", cursor.unwrap()).as_str())
        }

        println!("{}", url);

        let casts_reqwest = &self.reqwest_get(url.as_str()).await?;

        let casts: CastRoot = serde_json::from_str(casts_reqwest)?;

        Ok(casts)
    }

    /// Get a users casts by their Username
    ///
    /// ## Params
    /// username: &str
    /// limit: Option<i32>
    /// cursor: Option<&str>
    ///
    /// ## Example
    /// ```no_run
    /// let casts = farcaster.get_casts_by_username("cassie", None, None).await?;
    /// ```
    pub async fn get_casts_by_username(
        &self,
        username: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<CastRoot, Box<dyn Error>> {
        let fid = &self.get_user_by_username(username).await?;

        let casts = &self.get_casts_by_fid(fid.fid, limit, cursor).await?;

        Ok(casts.clone())
    }

    /// Get a users casts by their Address
    ///
    /// ## Params
    /// address: &str
    /// limit: Option<i32>
    /// cursor: Option<&str>
    ///
    /// ## Example
    /// ```no_run
    /// let casts = farcaster.get_casts_by_address("0x0000....", None, None).await?;
    /// ```
    pub async fn get_casts_by_address(
        &self,
        address: &str,
        limit: Option<i32>,
        cursor: Option<&str>,
    ) -> Result<CastRoot, Box<dyn Error>> {
        let address: &UserInfo = &self.get_user_by_address(address).await?;

        let casts = &self.get_casts_by_fid(address.fid, limit, cursor).await?;

        Ok(casts.clone())
    }
}
