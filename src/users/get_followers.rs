use crate::{Farcaster, types::user::followers::{FollowerRoot, Follower}};
use std::result::Result;

#[allow(unused_assignments)]
impl Farcaster {
    /// # Get all followers of a user
    /// 
    /// ## Arguments
    /// 
    /// * `self: &Farcaster`
    ///     - Takes in a type of Farcaster which is created at the start with ``Farcaster::new("client".to_string());``
    /// 
    /// * `username: Option<String>`
    ///     - The username you want to fetch casts for
    ///     - Optional
    /// 
    /// * `address: Option<String>`
    ///     - The address for the user you want to get
    ///     - Optional
    /// 
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("".to_string());
    /// 
    /// let followers = farcaster.get_followers(Some("ace".to_string()), None).await.unwrap();
    /// 
    /// for follower in followers {
    ///     println!("{:#?}", follower);
    /// }
    /// ```
    pub async fn get_followers(&self, username: Option<String>, address: Option<String>) -> Result<Vec<Follower>, Box<dyn std::error::Error>> {
        let mut res_address: String = "".to_string();
        if username.is_some() {
            // Get address
            let address = Farcaster::get_user_by_username(self, username.unwrap()).await?;
            res_address = address.result.user.address;
        }
        else if address.is_some() {
            res_address = address.unwrap();
        }
        else {
            return Err(Box::from("Please provide either a username or an address"));
            // Error
        }
        let follower_request = reqwest::get(format!("https://api.farcaster.xyz/indexer/followers/{}", res_address)).await?.text().await?;
        let followers: Result<FollowerRoot, serde_json::Error> = serde_json::from_str(&follower_request);

        match followers {
            Ok(success) => {
                Ok(success)
            }
            Err(e) => {
                Err(Box::from(format!("Not able to parse follower list into struct. Please ensure the address is correct. Error: {}", e)))
            }
        }
    }
}

