use std::error::Error;
use crate::constants::merkle::API_ROOT;
use crate::Farcaster;

impl Farcaster {
    pub async fn get_api_health() -> Result<String, Box<dyn Error>> {
        let api_health = reqwest::Client::new()
            .get(format!("{}/healthcheck", API_ROOT))
            .header("Accept", "application/json")
            .send()
            .await?
            .text()
            .await?;

        Ok(api_health)
    }
}