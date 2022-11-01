use crate::{
    types::user::verified_address::{self, Root},
    Farcaster,
};

impl Farcaster {
    /// # Get the verified address of a user
    ///
    /// ## Arguments
    ///
    /// * `self: &Farcaster`
    ///     - Takes in a type of Farcaster which is created at the start with ``Farcaster::new("client");``
    ///
    /// * `username: &str`
    ///     - The username you want to fetch casts for
    ///
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("");
    ///
    /// let verified_address = farcaster.get_verified_address("0xen").await.unwrap();
    ///
    /// println!("{:#?}", verified_address);
    /// ```
    pub async fn get_verified_address(
        &self,
        username: &str,
    ) -> Result<Root, Box<dyn std::error::Error>> {
        let user = Farcaster::get_user_by_username(self, username).await?;
        let user_name = user.result.user.username;
        let address = user.result.user.address;
        if username == user_name {
            let user = reqwest::get(format!(
                "https://api.farcaster.xyz/v1/verified_addresses/{}",
                address
            ))
            .await?
            .text()
            .await?;
            let user: verified_address::Root = serde_json::from_str(&user)?;
            return Ok(user);
        };

        Err(Box::from("Couldn't find username"))
    }
}
