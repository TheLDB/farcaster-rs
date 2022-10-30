use crate::types::abi::registry::Registry;
use crate::types::logs::events::Events;
use crate::types::user::user::Root;
use crate::Farcaster;

impl Farcaster {
    /// # Get the details of a user
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
    /// let user = farcaster.get_user_by_username("lndnnft").await.unwrap();
    ///
    /// println!("{:#?}", user);
    /// ```
    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> Result<Root, Box<dyn std::error::Error>> {
        let name_registry_logs = self.get_name_registry_logs().await?;

        let username = String::from(username);

        for name in name_registry_logs {
            let parsed_log = &self
                .parse_log(name, Registry::NAME, Events::Transfer)
                .await?;
            let token_id = parsed_log.params.get(2).unwrap();
            let fname = Farcaster::token_to_fname(token_id.value.clone()).await?;
            if fname == username {
                let address = parsed_log.params.get(1).unwrap();
                let address = format!("0x{}", address.value.to_string());
                let user =
                    reqwest::get(format!("https://api.farcaster.xyz/v1/profiles/{}", address))
                        .await?
                        .text()
                        .await?;
                let user: Root = serde_json::from_str(&user)?;
                return Ok(user);
            }
        }

        Err(Box::from(
            "Unable to fetch Name Registry Logs - Fatal Error",
        ))
    }
}
