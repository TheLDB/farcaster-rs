use crate::{Farcaster, types::casts::casts::CastRoot};

#[allow(unreachable_code)]
impl Farcaster {
    /// # Get all Casts of a user
    /// With pagination!
    /// 
    /// ## Arguments
    /// 
    /// * `self: &Farcaster`
    ///     - Takes in a type of Farcaster which is created at the start with ``Farcaster::new("client".to_string());``
    /// 
    /// * `username: String`
    ///     - The username you want to fetch casts for
    /// 
    /// * `casts_per_page: i64`
    ///     - The amount of casts per page
    /// 
    /// * `page: i64`
    ///     - What page number you get.
    /// 
    /// ## Usage
    /// ```no_run
    /// let farcaster = Farcaster::new("".to_string());
    /// 
    /// let casts = farcaster.get_casts("dwr".to_string(), 30, 2).await.unwrap();
    /// 
    /// println!("{:#?}", casts);
    /// ```
    pub async fn get_casts(&self, username: String, casts_per_page: i64, page: i64) -> Result<CastRoot, Box<dyn std::error::Error>> {
        let user = Farcaster::get_user_by_username(self, username).await?;
        let address = user.result.user.address;

        let user = reqwest::get(format!("https://api.farcaster.xyz/v1/profiles/{}/casts?per_page={}", address, casts_per_page)).await?.text().await?;
        let casts: CastRoot = serde_json::from_str(&user)?;
        if page > 0 {
            let mut loopy = 0;
            let mut new_user: String = String::from("");
            while loopy != page {
                new_user = reqwest::get(casts.meta.next.clone()).await?.text().await?;
                loopy += 1;
            }
            
            let casts: CastRoot = serde_json::from_str(&new_user)?;
            return Ok(casts)
        }
        else {
            return Ok(casts)
        }

        Err(Box::from("This error should be unreachable - if you got here either I fucked up or you fucked up"))
    }
}