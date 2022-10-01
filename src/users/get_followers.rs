use crate::{Farcaster, types::user::followers::{FollowerRoot, Follower}};
use std::result::Result;

impl Farcaster {
    pub async fn get_followers(&self, username: Option<String>, address: Option<String>) -> Result<Vec<Follower>, Box<dyn std::error::Error>> {
        let mut res_address: String = "".to_string();
        if username.is_some() {
            // Get address
            let address = Farcaster::get_user_by_username(self, username.unwrap()).await.unwrap();
            res_address = address.result.user.address;
        }
        else if address.is_some() {
            res_address = address.unwrap();
        }
        else {
            // Error
        }
        let follower_request = reqwest::get(format!("https://api.farcaster.xyz/indexer/followers/{}", res_address)).await.unwrap().text().await.unwrap();
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

