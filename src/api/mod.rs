use crate::Farcaster;
use std::error::Error;

impl Farcaster {
    /// bridge to reqwest get call
    pub(crate) async fn reqwest_get(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let req_builder = reqwest::Client::new().get(url);
        let response = req_builder
            .header("Content-Type", "application/json")
            .header("Authorization", self.account.get_auth_token()?)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}
