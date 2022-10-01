use crate::{Farcaster, types::user::verified_address::{self, Root}};

impl Farcaster {
    /// # Get the verified address of a user
    /// 
    /// ## Arguments
    /// 
    /// * `self: &Farcaster`
    ///     - Takes in a type of Farcaster which is created at the start with ``Farcaster::new("client".to_string());``
    /// 
    /// * `username: String`
    ///     - The username you want to fetch casts for
    ///     - Optional
    /// 
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("".to_string());
    /// 
    /// let verified_address = farcaster.get_verified_address("0xen".to_string()).await.unwrap();
    /// 
    /// println!("{:#?}", verified_address);
    /// ```
    pub async fn get_verified_address(&self, username: String) -> Result<Root, Box<dyn std::error::Error>> {
        let user = Farcaster::get_user_by_username(self, username.clone()).await?;
        let user_name = user.result.user.username;
        let address = user.result.user.address;
        if username == user_name {
            let user = reqwest::get(format!("https://api.farcaster.xyz/v1/verified_addresses/{}", address)).await?.text().await?;
            let user: verified_address::Root = serde_json::from_str(&user)?;
            return Ok(user)
        };

        Err(Box::from("Couldn't find username"))
    }
}